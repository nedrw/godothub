// Download - Godot 版本下载服务
// 实现基本的下载和解压功能

use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::models::{GodotVariant, GodotVersion};
use crate::state::AppConfig;

/// 下载进度回调类型
pub type ProgressCallback = Arc<dyn Fn(f32) + Send + Sync>;

/// 开始下载（供 UI 调用）
///
/// # Arguments
/// * `version` - 要下载的 Godot 版本信息
/// * `state` - 应用程序状态
/// * `runtime` - Tokio 运行时
pub fn start_download(
    version: &GodotVersion,
    state: &mut crate::state::AppState,
    runtime: Arc<Runtime>,
) {
    log::info!("Starting download for Godot {}", version.version);
    state.downloads_in_progress.insert(version.version.clone(), 0.0);

    // 克隆必要的数据
    let version = version.clone();
    let config = state.config.clone();

    // 使用传入的运行时在后台执行下载
    runtime.spawn(async move {
        match download_and_install(&version, &config, None).await {
            Ok(path) => {
                log::info!("Successfully installed Godot {} at: {}", version.version, path.display());
            }
            Err(e) => {
                log::error!("Failed to download Godot {}: {}", version.version, e);
            }
        }
    });
}

/// 取消下载
pub fn cancel_download(version_key: &str, state: &mut crate::state::AppState) -> bool {
    log::info!("Cancelling download for: {}", version_key);
    state.downloads_in_progress.remove(version_key).is_some()
}

/// 下载并安装版本
pub async fn download_and_install(
    version: &GodotVersion,
    config: &AppConfig,
    progress_callback: Option<ProgressCallback>,
) -> Result<PathBuf, String> {
    let temp_dir = config.install_dir.join(".downloads");
    let install_dir = config.install_dir.join(format!(
        "{}{}",
        version.version,
        match version.variant {
            GodotVariant::Mono => "-mono",
            _ => "",
        }
    ));

    // 确保目录存在
    tokio::fs::create_dir_all(&temp_dir)
        .await
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;

    tokio::fs::create_dir_all(&install_dir)
        .await
        .map_err(|e| format!("Failed to create install directory: {}", e))?;

    let filename = format!(
        "Godot_v{}_{}.zip",
        version.version,
        match version.variant {
            GodotVariant::Mono => "mono",
            _ => "stable",
        }
    );
    let download_path = temp_dir.join(&filename);

    // 下载文件
    download_file(&version.download_url, &download_path, progress_callback).await?;

    // 解压文件
    extract_zip(&download_path, &install_dir).await?;

    // 清理临时文件
    if let Err(e) = tokio::fs::remove_file(&download_path).await {
        log::warn!("Failed to remove temp file: {}", e);
    }

    Ok(install_dir)
}

/// 下载文件
pub async fn download_file(
    url: &str,
    dest_path: &PathBuf,
    progress_callback: Option<ProgressCallback>,
) -> Result<(), String> {
    log::info!("Downloading from: {}", url);
    log::info!("Destination: {}", dest_path.display());

    let client = reqwest::Client::builder()
        .user_agent("GodotHub/0.1.2")
        .timeout(std::time::Duration::from_secs(300)) // 5分钟超时
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to start download: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    log::info!("Total size: {} bytes", total_size);

    // 确保目标目录存在
    if let Some(parent) = dest_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // 下载整个响应体
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;

    // 写入文件
    tokio::fs::write(dest_path, &bytes)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;

    if let Some(callback) = progress_callback {
        callback(1.0);
    }

    log::info!("Download completed: {} bytes", bytes.len());
    Ok(())
}

/// 解压 ZIP 文件
pub async fn extract_zip(zip_path: &PathBuf, dest_dir: &PathBuf) -> Result<(), String> {
    log::info!("Extracting: {} to {}", zip_path.display(), dest_dir.display());

    let zip_path = zip_path.clone();
    let dest_dir = dest_dir.clone();

    tokio::task::spawn_blocking(move || {
        let file =
            std::fs::File::open(&zip_path).map_err(|e| format!("Failed to open zip file: {}", e))?;

        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

        for i in 0..archive.len() {
            let mut file =
                archive.by_index(i).map_err(|e| format!("Failed to get file from archive: {}", e))?;

            let outpath = match file.enclosed_name() {
                Some(path) => dest_dir.join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)
                            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                    }
                }

                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| format!("Failed to create output file: {}", e))?;

                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
            }

            // 设置可执行权限（Unix）
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(name) = file.name().rsplit('/').next() {
                    if name.starts_with("Godot") && !name.contains('.') {
                        std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(0o755))
                            .map_err(|e| format!("Failed to set permissions: {}", e))?;
                    }
                }
            }
        }

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Failed to extract: {}", e))??;

    log::info!("Extraction completed");
    Ok(())
}
