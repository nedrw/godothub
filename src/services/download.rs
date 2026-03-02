// Download - Godot 版本下载服务
// 实现真正的下载和解压功能，支持进度更新和完成通知

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::runtime::Runtime;
use std::sync::Mutex;

use crate::models::{GodotVariant, GodotVersion};
use crate::state::{AppConfig, AppState, DownloadSource};

/// 下载进度回调类型
pub type ProgressCallback = Arc<dyn Fn(f32) + Send + Sync>;

/// 共享状态指针类型
pub type SharedState = Arc<Mutex<AppState>>;

/// 取消令牌类型
pub type CancellationToken = Arc<AtomicBool>;

/// 下载状态标记常量
/// 用于在 downloads_in_progress HashMap 中标识特殊状态
pub mod download_state {
    /// 正常下载进度范围: 0.0 - 1.0
    pub const PROGRESS_MIN: f32 = 0.0;
    pub const PROGRESS_MAX: f32 = 1.0;

    /// 下载失败的标记 (key: "{version}_error", value: ERROR)
    pub const ERROR: f32 = -1.0;

    /// 解压中的标记 (key: "{version}_extracting", value: EXTRACTING)
    pub const EXTRACTING: f32 = -2.0;

    /// 安装完成的标记 (key: "{version}_complete", value: COMPLETE)
    pub const COMPLETE: f32 = -3.0;

    /// 生成错误状态的 key
    pub fn error_key(version_key: &str) -> String {
        format!("{}_error", version_key)
    }

    /// 生成解压中状态的 key
    pub fn extracting_key(version_key: &str) -> String {
        format!("{}_extracting", version_key)
    }

    /// 生成完成状态的 key
    pub fn complete_key(version_key: &str) -> String {
        format!("{}_complete", version_key)
    }

    /// 检查是否为错误状态
    pub fn is_error(progress: f32) -> bool {
        progress == ERROR
    }

    /// 检查是否为解压状态
    pub fn is_extracting(progress: f32) -> bool {
        progress == EXTRACTING
    }

    /// 检查是否为完成状态
    pub fn is_complete(progress: f32) -> bool {
        progress == COMPLETE
    }
}

/// 开始下载（供 UI 调用）
///
/// # Arguments
/// * `version` - 要下载的 Godot 版本信息
/// * `state` - 应用程序状态
/// * `runtime` - Tokio 运行时
pub fn start_download(
    version: &GodotVersion,
    state: &mut AppState,
    runtime: Arc<Runtime>,
) {
    log::info!("Starting download for Godot {}", version.version);

    // 创建版本标识键
    let version_key = create_version_key(version);

    // 初始化下载进度
    state.downloads_in_progress.insert(version_key.clone(), 0.0);

    // 创建取消令牌
    let cancellation_token = Arc::new(AtomicBool::new(false));
    state.cancellation_tokens.insert(version_key.clone(), cancellation_token.clone());

    // 克隆必要的数据
    let version = version.clone();
    let config = state.config.clone();
    let version_key_clone = version_key.clone();

    // 获取共享状态（如果可用）
    let shared_state = state.shared_state.clone();

    // 使用传入的运行时在后台执行下载
    runtime.spawn(async move {
        // 克隆 shared_state 以在闭包中使用
        let shared_state_for_progress = shared_state.clone();

        // 创建进度回调
        let progress_cb = Arc::new(move |progress: f32| {
            // 更新共享状态中的进度
            if let Some(ref shared) = shared_state_for_progress {
                if let Ok(mut s) = shared.lock() {
                    s.downloads_in_progress.insert(version_key_clone.clone(), progress);
                }
            }
        });

        // 执行下载
        match download_and_install(&version, &config, Some(progress_cb), cancellation_token).await {
            Ok(path) => {
                log::info!("Successfully installed Godot {} at: {}", version.version, path.display());

                // 使用正确的版本 key
                let version_key = create_version_key(&version);

                // 添加解压标记，让UI有时间显示"Extracting..."状态
                if let Some(ref shared) = shared_state {
                    if let Ok(mut s) = shared.lock() {
                        s.downloads_in_progress.insert(
                            download_state::extracting_key(&version_key),
                            download_state::EXTRACTING
                        );
                    }
                }

                // 等待一小段时间让UI显示解压状态
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                // 更新共享状态，标记下载完成
                if let Some(ref shared) = shared_state {
                    if let Ok(mut s) = shared.lock() {
                        // 移除解压标记
                        s.downloads_in_progress.remove(&download_state::extracting_key(&version_key));

                        // 先添加安装完成标记，让UI有时间显示"Installed"状态
                        s.downloads_in_progress.insert(
                            download_state::complete_key(&version_key),
                            download_state::COMPLETE
                        );

                        // 移除下载进度记录（使用正确的版本 key）
                        s.downloads_in_progress.remove(&version_key);

                        // 查找对应的版本信息并添加到已安装列表
                        if let Some(available_version) = s.available_versions.iter().find(|v| v.version == version.version && v.variant == version.variant) {
                            let install = crate::models::GodotInstall::new(
                                version.version.clone(),
                                available_version.variant.clone(),
                                path.clone(),
                            );
                            s.installed_versions.push(install);

                            // 更新可用版本状态
                            if let Some(av) = s.available_versions.iter_mut().find(|v| v.version == version.version && v.variant == version.variant) {
                                av.is_installed = true;
                                av.install_path = Some(path.clone());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to download Godot {}: {}", version.version, e);

                // 使用正确的版本 key 移除下载进度记录（标记失败）
                let version_key = create_version_key(&version);
                if let Some(ref shared) = shared_state {
                    if let Ok(mut s) = shared.lock() {
                        s.downloads_in_progress.remove(&version_key);
                        // 添加下载失败标记，用于 UI 显示错误状态
                        s.downloads_in_progress.insert(
                            download_state::error_key(&version_key),
                            download_state::ERROR
                        );
                    }
                }
            }
        }
    });
}

/// 创建版本标识键
fn create_version_key(version: &GodotVersion) -> String {
    match version.variant {
        GodotVariant::Mono => format!("{}-mono", version.version),
        _ => version.version.clone(),
    }
}

/// 取消下载
pub fn cancel_download(version_key: &str, state: &mut AppState) -> bool {
    log::info!("Cancelling download for: {}", version_key);

    // 设置取消标志
    if let Some(token) = state.cancellation_tokens.get(version_key) {
        token.store(true, Ordering::SeqCst);
        log::info!("Cancellation flag set for: {}", version_key);
    }

    // 移除取消令牌
    state.cancellation_tokens.remove(version_key);

    // 移除下载进度记录
    let has_progress = state.downloads_in_progress.remove(version_key).is_some();
    // 同时移除错误标记（如果有）
    let has_error = state.downloads_in_progress.remove(&download_state::error_key(version_key)).is_some();
    // 同时移除解压标记（如果有）
    state.downloads_in_progress.remove(&download_state::extracting_key(version_key));
    // 同时移除完成标记（如果有）
    state.downloads_in_progress.remove(&download_state::complete_key(version_key));
    has_progress || has_error
}

/// 下载并安装版本
pub async fn download_and_install(
    version: &GodotVersion,
    config: &AppConfig,
    progress_callback: Option<ProgressCallback>,
    cancellation_token: CancellationToken,
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

    // 根据当前配置重新转换下载 URL（而不是使用缓存的 URL）
    let download_url = convert_url_with_current_source(&version.download_url, &config.download_source, &config.custom_mirror_url);
    log::info!("Using download URL (with current source settings): {}", download_url);

    // 克隆进度回调用于后续解压进度报告
    let extract_callback = progress_callback.clone();

    // 下载文件，支持镜像回退
    download_file_with_fallback(&download_url, &download_path, progress_callback.clone(), cancellation_token.clone()).await?;

    // 验证下载的文件是否为有效的 ZIP
    if let Err(e) = validate_zip_file(&download_path).await {
        // 删除无效文件
        let _ = tokio::fs::remove_file(&download_path).await;
        return Err(format!("Downloaded file is not a valid ZIP: {}. The download source may be unavailable.", e));
    }

    // 解压文件
    extract_zip(&download_path, &install_dir, cancellation_token).await?;

    // 清理临时文件
    if let Err(e) = tokio::fs::remove_file(&download_path).await {
        log::warn!("Failed to remove temp file: {}", e);
    }

    // 找到解压后的可执行文件路径
    let executable_path = find_executable(&install_dir).await?;

    Ok(executable_path)
}

/// 在安装目录中查找 Godot 可执行文件（异步版本）
async fn find_executable(install_dir: &PathBuf) -> Result<PathBuf, String> {
    let mut entries = tokio::fs::read_dir(install_dir)
        .await
        .map_err(|e| format!("Failed to read install directory: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("Read error: {}", e))? {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // 匹配 Godot 可执行文件
                if name.starts_with("Godot") && !name.ends_with(".pdb") {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        // 确保可执行权限
                        if let Ok(metadata) = std::fs::metadata(&path) {
                            let mut perms = metadata.permissions();
                            perms.set_mode(0o755);
                            let _ = std::fs::set_permissions(&path, perms);
                        }
                    }
                    log::info!("Found executable: {}", path.display());
                    return Ok(path);
                }
            }
        }
    }

    // 如果没找到，返回安装目录本身
    log::warn!("Could not find Godot executable, returning install directory");
    Ok(install_dir.clone())
}

/// 验证下载的文件是否为有效的 ZIP 格式
async fn validate_zip_file(path: &PathBuf) -> Result<(), String> {
    let path = path.clone();
    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&path)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        // 尝试读取 ZIP 文件头
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| format!("Invalid ZIP archive: {}", e))?;

        // 检查 ZIP 内部是否有文件
        if archive.len() == 0 {
            return Err("ZIP archive is empty".to_string());
        }

        log::info!("ZIP file validated successfully, contains {} files", archive.len());
        Ok(())
    })
    .await
    .map_err(|e| format!("Validation task failed: {}", e))?
}

/// 下载文件（支持进度报告）
pub async fn download_file(
    url: &str,
    dest_path: &PathBuf,
    progress_callback: Option<ProgressCallback>,
    cancellation_token: CancellationToken,
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

    // 检查 Content-Type，确保返回的是文件而不是 HTML 错误页面
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    log::info!("Content-Type: {}", content_type);

    // 如果是 HTML，说明镜像返回了错误页面
    if content_type.contains("text/html") {
        // 尝试读取响应内容的前500字节用于调试
        let body_preview = response.text().await.unwrap_or_default();
        let preview = body_preview.chars().take(500).collect::<String>();
        log::error!("Mirror returned HTML instead of file. Preview: {}", preview);
        return Err("Mirror returned error page instead of file. Please check your mirror URL or use GitHub official source.".to_string());
    }

    let total_size = response.content_length().unwrap_or(0);
    log::info!("Total size: {} bytes", total_size);

    // 确保目标目录存在
    if let Some(parent) = dest_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // 使用流式下载，支持进度报告
    if total_size > 0 {
        // 大文件：流式下载并报告进度
        let mut file = tokio::fs::File::create(dest_path)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        // 导入 StreamExt trait
        use futures::stream::StreamExt;
        use tokio::io::AsyncWriteExt;

        while let Some(chunk) = stream.next().await {
            // 检查是否被取消
            if cancellation_token.load(Ordering::SeqCst) {
                log::info!("Download cancelled by user");
                // 删除未完成的文件
                let _ = tokio::fs::remove_file(dest_path).await;
                return Err("Download cancelled".to_string());
            }

            let chunk = chunk.map_err(|e| format!("Download error: {}", e))?;
            file.write_all(&chunk).await.map_err(|e| format!("Write error: {}", e))?;
            downloaded += chunk.len() as u64;

            // 报告进度
            if let Some(ref callback) = progress_callback {
                let progress = downloaded as f32 / total_size as f32;
                callback(progress);
            }
        }

        file.flush().await.map_err(|e| format!("Flush error: {}", e))?;

        if let Some(callback) = progress_callback {
            callback(1.0);
        }
    } else {
        // 无法获取大小时，一次性下载
        // 先检查是否被取消
        if cancellation_token.load(Ordering::SeqCst) {
            log::info!("Download cancelled by user");
            return Err("Download cancelled".to_string());
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to download: {}", e))?;

        tokio::fs::write(dest_path, &bytes)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        if let Some(callback) = progress_callback {
            callback(1.0);
        }
    }

    log::info!("Download completed");
    Ok(())
}

/// 使用镜像回退机制下载文件
pub async fn download_file_with_fallback(
    url: &str,
    dest_path: &PathBuf,
    progress_callback: Option<ProgressCallback>,
    cancellation_token: CancellationToken,
) -> Result<(), String> {
    // 首先尝试原始 URL
    let result = download_file(url, dest_path, progress_callback.clone(), cancellation_token.clone()).await;

    // 如果失败，检查是否使用了镜像
    if result.is_err() {
        let original_url = url.to_string();

        // 尝试识别是否使用了镜像
        let needs_fallback = original_url.contains("ghproxy.com")
            || original_url.contains("gitclone.com")
            || original_url.contains("mirror.ghproxy")
            || original_url.contains("fastgit.org");

        if needs_fallback {
            log::warn!("Mirror download failed: {}, trying GitHub official URL", result.as_ref().err().unwrap());

            // 尝试从原始 GitHub URL 下载
            if let Some(github_url) = convert_mirror_to_github_url(&original_url) {
                log::info!("Retrying with GitHub official URL: {}", github_url);
                return download_file(&github_url, dest_path, progress_callback, cancellation_token).await;
            }
        }
    }

    result
}

/// 将镜像 URL 转换回原始 GitHub URL
fn convert_mirror_to_github_url(mirror_url: &str) -> Option<String> {
    // 从 ghproxy.com URL 提取原始 URL
    if mirror_url.contains("ghproxy.com/") {
        // 格式: https://ghproxy.com/https://github.com/... -> https://github.com/...
        let without_prefix = mirror_url.replace("https://ghproxy.com/", "");
        return Some(without_prefix);
    }

    // 从 gitclone.com URL 提取原始 URL
    if mirror_url.contains("gitclone.com/github.com/") {
        // 格式: https://gitclone.com/github.com/... -> https://github.com/...
        let without_prefix = mirror_url.replace("https://gitclone.com/github.com/", "https://github.com/");
        return Some(without_prefix);
    }

    // 从 fastgit.org URL 提取原始 URL
    if mirror_url.contains("download.fastgit.org/") {
        // 格式: https://download.fastgit.org/... -> https://github.com/...
        let without_prefix = mirror_url.replace("https://download.fastgit.org/", "https://github.com/");
        return Some(without_prefix);
    }

    None
}

/// 根据当前下载源配置转换下载 URL
/// 确保即使版本列表缓存了旧的 URL，也能使用当前设置的下载源
fn convert_url_with_current_source(cached_url: &str, current_source: &DownloadSource, custom_mirror_url: &str) -> String {
    // 如果 URL 已经是官方 GitHub URL，直接返回
    if cached_url.starts_with("https://github.com/") {
        return cached_url.to_string();
    }

    // 如果 URL 包含镜像前缀，提取原始 URL 并根据当前配置重新转换
    let original_url = if cached_url.contains("ghproxy.com/") {
        // 提取原始 URL: https://ghproxy.com/https://github.com/... -> https://github.com/...
        cached_url.replace("https://ghproxy.com/", "")
    } else if cached_url.contains("gitclone.com/github.com/") {
        // 提取原始 URL: https://gitclone.com/github.com/... -> https://github.com/...
        cached_url.replace("https://gitclone.com/github.com/", "https://github.com/")
    } else if cached_url.contains("download.fastgit.org/") {
        // 提取原始 URL
        cached_url.replace("https://download.fastgit.org/", "https://github.com/")
    } else {
        // 无法识别，直接返回缓存的 URL
        return cached_url.to_string();
    };

    // 根据当前下载源配置重新转换 URL
    match current_source {
        DownloadSource::GitHub => original_url,
        DownloadSource::Custom => {
            // 自定义镜像 URL
            if !custom_mirror_url.is_empty() {
                let mirror_url = custom_mirror_url.trim().trim_end_matches('/');
                // 对于自定义镜像，尝试去除原始URL的 https:// 前缀
                let url_without_protocol = original_url.strip_prefix("https://").unwrap_or(&original_url);
                format!("{}/{}", mirror_url, url_without_protocol)
            } else {
                // 如果没有配置自定义镜像URL，使用官方源
                original_url
            }
        }
    }
}

/// 解压 ZIP 文件
pub async fn extract_zip(zip_path: &PathBuf, dest_dir: &PathBuf, cancellation_token: CancellationToken) -> Result<(), String> {
    log::info!("Extracting: {} to {}", zip_path.display(), dest_dir.display());

    let zip_path = zip_path.clone();
    let dest_dir = dest_dir.clone();

    tokio::task::spawn_blocking(move || {
        // 检查是否已被取消
        if cancellation_token.load(Ordering::SeqCst) {
            log::info!("Extraction cancelled by user before starting");
            return Err("Extraction cancelled".to_string());
        }

        let file =
            std::fs::File::open(&zip_path).map_err(|e| format!("Failed to open zip file: {}", e))?;

        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

        for i in 0..archive.len() {
            // 检查是否已被取消
            if cancellation_token.load(Ordering::SeqCst) {
                log::info!("Extraction cancelled by user during extraction");
                // 尝试清理已解压的目录
                let _ = std::fs::remove_dir_all(&dest_dir);
                return Err("Extraction cancelled".to_string());
            }

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
