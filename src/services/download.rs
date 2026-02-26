// Download - Godot 版本下载服务

use std::path::PathBuf;
use std::sync::Arc;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::models::{GodotVariant, GodotVersion};
use crate::state::AppConfig;

/// 下载进度回调类型
pub type ProgressCallback = Box<dyn Fn(f32) + Send + Sync>;

/// 下载状态
#[derive(Debug, Clone)]
pub enum DownloadState {
    /// 等待中
    Pending,
    /// 下载中
    Downloading {
        /// 已下载字节数
        downloaded: u64,
        /// 总字节数
        total: u64,
    },
    /// 解压中
    Extracting,
    /// 完成
    Completed,
    /// 失败
    Failed(String),
}

/// 下载任务
#[derive(Clone)]
pub struct DownloadTask {
    /// 要下载的版本信息
    pub version: GodotVersion,
    /// 下载状态
    pub state: DownloadState,
    /// 进度 (0.0 - 1.0)
    pub progress: f32,
}

impl DownloadTask {
    /// 创建新的下载任务
    pub fn new(version: GodotVersion) -> Self {
        Self {
            version,
            state: DownloadState::Pending,
            progress: 0.0,
        }
    }
}

/// Godot 下载服务
pub struct DownloadService {
    /// 应用配置
    config: AppConfig,
    /// 临时下载目录
    temp_dir: PathBuf,
}

impl DownloadService {
    /// 创建新的下载服务
    pub fn new(config: &AppConfig) -> Self {
        let temp_dir = config.install_dir.join(".downloads");

        Self {
            config: config.clone(),
            temp_dir,
        }
    }

    /// 获取版本的下载目标路径
    pub fn get_download_path(&self, version: &GodotVersion) -> PathBuf {
        let filename = format!(
            "Godot_v{}_{}.zip",
            version.version,
            match version.variant {
                GodotVariant::Mono => "mono",
                _ => "stable",
            }
        );

        self.temp_dir.join(filename)
    }

    /// 获取版本的安装目标目录
    pub fn get_install_path(&self, version: &GodotVersion) -> PathBuf {
        let dir_name = format!(
            "{}{}",
            version.version,
            match version.variant {
                GodotVariant::Mono => "-mono",
                _ => "",
            }
        );

        self.config.install_dir.join(dir_name)
    }

    /// 确保临时目录存在
    pub async fn ensure_temp_dir(&self) -> std::io::Result<()> {
        tokio::fs::create_dir_all(&self.temp_dir).await
    }

    /// 下载文件（模拟实现）
    ///
    /// 注意：这是一个模拟实现，实际下载需要使用 reqwest
    pub async fn download_version(
        &self,
        version: &GodotVersion,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<PathBuf, String> {
        // 确保临时目录存在
        self.ensure_temp_dir()
            .await
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;

        let download_path = self.get_download_path(version);
        log::info!("Starting download: {}", version.download_url);
        log::info!("Download path: {}", download_path.display());

        // 模拟下载进度
        // 实际实现中，这里应该使用 reqwest 进行流式下载
        for i in 0..=100 {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            let progress = i as f32 / 100.0;

            if let Some(ref callback) = progress_callback {
                callback(progress);
            }
        }

        // 创建占位文件（模拟下载完成）
        let mut file = File::create(&download_path)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        file.write_all(b"placeholder")
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        file.flush()
            .await
            .map_err(|e| format!("Failed to flush file: {}", e))?;

        log::info!("Download completed: {}", download_path.display());

        Ok(download_path)
    }

    /// 解压 Godot 压缩包
    pub async fn extract_version(&self, zip_path: &PathBuf) -> Result<PathBuf, String> {
        let install_path = self.get_install_path(&GodotVersion::new(
            String::new(),
            GodotVariant::Standard,
            String::new(),
            String::new(),
            String::new(),
        ));

        log::info!("Extracting: {} to {}", zip_path.display(), install_path.display());

        // 确保安装目录存在
        tokio::fs::create_dir_all(&install_path)
            .await
            .map_err(|e| format!("Failed to create install directory: {}", e))?;

        // 注意：实际实现中应该使用 zip crate 进行解压
        // 这里只是一个占位实现
        log::info!("Extraction completed (placeholder)");

        Ok(install_path)
    }

    /// 清理临时文件
    pub async fn cleanup_temp_files(&self) -> std::io::Result<()> {
        if self.temp_dir.exists() {
            tokio::fs::remove_dir_all(&self.temp_dir).await?;
        }
        Ok(())
    }

    /// 删除已安装的版本
    pub async fn uninstall_version(&self, install_path: &PathBuf) -> Result<(), String> {
        if !install_path.exists() {
            return Err("Version not installed".to_string());
        }

        log::info!("Removing Godot version at: {}", install_path.display());

        if install_path.is_dir() {
            tokio::fs::remove_dir_all(install_path)
                .await
                .map_err(|e| format!("Failed to remove directory: {}", e))?;
        } else {
            tokio::fs::remove_file(install_path)
                .await
                .map_err(|e| format!("Failed to remove file: {}", e))?;
        }

        log::info!("Version removed successfully");
        Ok(())
    }
}

/// 开始下载（供 UI 调用）
///
/// 这是一个便捷函数，用于启动下载
pub fn start_download(version: &GodotVersion, state: &mut crate::state::AppState) {
    log::info!("Starting download for Godot {}", version.version);
    state.downloads_in_progress.insert(version.version.clone(), 0.0);
}

/// 取消下载
pub fn cancel_download(version_key: &str, state: &mut crate::state::AppState) -> bool {
    state.downloads_in_progress.remove(version_key).is_some()
}
