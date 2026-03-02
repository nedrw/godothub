// AppState 实现方法

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::models::{GodotInstall, GodotVariant};
use super::{AppState, RefreshResult};

/// 检测当前平台
pub fn detect_platform() -> String {
    #[cfg(target_os = "linux")]
    {
        #[cfg(target_arch = "x86_64")]
        return "Linux64".to_string();
        #[cfg(target_arch = "x86")]
        return "Linux32".to_string();
        #[cfg(target_arch = "aarch64")]
        return "LinuxARM64".to_string();
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
        return "Linux.Unknown".to_string();
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "x86_64")]
        return "macOS.Intel".to_string();
        #[cfg(target_arch = "aarch64")]
        return "macOS.ARM".to_string();
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        return "macOS.Unknown".to_string();
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(target_arch = "x86_64")]
        return "Windows64".to_string();
        #[cfg(target_arch = "x86")]
        return "Windows32".to_string();
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        return "Windows.Unknown".to_string();
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    "Unknown".to_string()
}

impl AppState {
    /// 从磁盘加载已安装的版本
    pub fn load_installed_versions(&mut self) {
        let versions_dir = &self.config.install_dir;
        if !versions_dir.exists() {
            return;
        }

        self.installed_versions.clear();

        if let Ok(entries) = std::fs::read_dir(versions_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name() {
                        let name_str = name.to_string_lossy();
                        let (version, variant) = Self::parse_version_dir(&name_str);

                        let executable = Self::find_godot_executable(&path, &variant);
                        if executable.exists() {
                            self.installed_versions.push(GodotInstall {
                                version,
                                variant,
                                path: executable,
                                is_favorite: false,
                                last_used: None,
                            });
                        }
                    }
                }
            }
        }

        // 更新可用版本的安装状态
        self.update_install_status();
    }

    /// 更新可用版本的安装状态
    pub fn update_install_status(&mut self) {
        for available in &mut self.available_versions {
            available.is_installed = self.installed_versions.iter().any(|installed| {
                installed.version == available.version && installed.variant == available.variant
            });
            if available.is_installed {
                if let Some(installed) = self.installed_versions.iter()
                    .find(|i| i.version == available.version && i.variant == available.variant)
                {
                    available.install_path = Some(installed.path.clone());
                }
            }
        }
    }

    /// 解析版本目录名称，提取版本号和变体类型
    fn parse_version_dir(name: &str) -> (String, GodotVariant) {
        let name_lower = name.to_lowercase();
        if name_lower.contains("mono") {
            (name.replace("-mono", "").replace("_mono", "").to_string(), GodotVariant::Mono)
        } else {
            (name.to_string(), GodotVariant::Standard)
        }
    }

    /// 在指定目录中查找 Godot 可执行文件
    fn find_godot_executable(path: &Path, _variant: &GodotVariant) -> PathBuf {
        // 尝试查找任何 Godot 可执行文件
        // 首先遍历目录查找可执行文件
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy().to_string();

                // 跳过明显的非可执行文件
                if name.starts_with('.') || name.contains(".zip") || name.contains(".txt") {
                    continue;
                }

                // 检查是否是可执行文件
                #[cfg(unix)]
                {
                    if let Ok(metadata) = entry.metadata() {
                        use std::os::unix::fs::PermissionsExt;
                        if metadata.permissions().mode() & 0o111 != 0 {
                            // 这是一个可执行文件
                            return entry.path();
                        }
                    }
                }

                #[cfg(windows)]
                {
                    if name.ends_with(".exe") {
                        return entry.path();
                    }
                }
            }
        }

        // 如果没有找到，回退到默认名称
        path.join("Godot")
    }

    /// 获取变体名称
    pub fn get_variant_name(variant: &GodotVariant) -> &'static str {
        match variant {
            GodotVariant::Standard => "Standard",
            GodotVariant::Mono => "Mono",
            GodotVariant::ExportTemplates => "Export Templates",
        }
    }

    /// 移除已安装的版本（同步版本，删除文件）
    /// 返回删除的版本信息，如果失败返回错误信息
    pub fn remove_installed_version(&mut self, index: usize) -> Result<GodotInstall, String> {
        if index >= self.installed_versions.len() {
            return Err("Invalid index".to_string());
        }

        let removed = self.installed_versions.remove(index);

        // 获取要删除的目录路径
        let install_path = removed.path.parent().unwrap_or(&removed.path).to_path_buf();

        // 删除安装目录
        if install_path.exists() {
            log::info!("Deleting installation directory: {}", install_path.display());
            std::fs::remove_dir_all(&install_path)
                .map_err(|e| format!("Failed to delete directory: {}", e))?;
            log::info!("Successfully deleted: {}", install_path.display());
        }

        // 更新可用版本状态
        for available in &mut self.available_versions {
            if available.version == removed.version && available.variant == removed.variant {
                available.is_installed = false;
                available.install_path = None;
            }
        }

        Ok(removed)
    }

    /// 异步移除已安装的版本（带进度回调）
    pub async fn remove_installed_version_async(
        &mut self,
        index: usize,
        progress_callback: Option<Arc<dyn Fn(f32, &str) + Send + Sync>>,
    ) -> Result<GodotInstall, String> {
        if index >= self.installed_versions.len() {
            return Err("Invalid index".to_string());
        }

        let removed = self.installed_versions.remove(index);

        // 获取要删除的目录路径
        let install_path = removed.path.parent().unwrap_or(&removed.path).to_path_buf();

        // 报告开始删除
        if let Some(ref cb) = progress_callback {
            cb(0.0, "Preparing to delete...");
        }

        // 异步删除安装目录
        if install_path.exists() {
            log::info!("Deleting installation directory: {}", install_path.display());

            let install_path_clone = install_path.clone();

            // 使用 spawn_blocking 在后台线程执行文件删除
            tokio::task::spawn_blocking(move || {
                std::fs::remove_dir_all(&install_path_clone)
            })
            .await
            .map_err(|e| format!("Failed to delete directory: {}", e))?
            .map_err(|e| format!("Failed to delete directory: {}", e))?;

            log::info!("Successfully deleted: {}", install_path.display());
        }

        // 报告删除完成
        if let Some(ref cb) = progress_callback {
            cb(1.0, "Deletion complete");
        }

        // 更新可用版本状态
        for available in &mut self.available_versions {
            if available.version == removed.version && available.variant == removed.variant {
                available.is_installed = false;
                available.install_path = None;
            }
        }

        Ok(removed)
    }

    /// 获取已安装版本数量
    pub fn installed_count(&self) -> usize {
        self.installed_versions.len()
    }

    /// 获取可用但未安装的版本数量
    pub fn available_count(&self) -> usize {
        self.available_versions.iter().filter(|v| !v.is_installed).count()
    }

    /// 切换下载对话框显示状态
    pub fn toggle_download_dialog(&mut self) {
        self.show_download_dialog = !self.show_download_dialog;
    }

    /// 切换标签页
    pub fn switch_tab(&mut self, tab: super::MainTab) {
        self.current_tab = tab;
    }

    /// 异步刷新可用版本列表
    /// 将结果接收器存储在 state 中，由 UI 层定期检查
    pub fn refresh_available_versions(&mut self) {
        // 如果已经在刷新，则直接返回
        if self.version_refresh_state.is_refreshing {
            log::warn!("Version refresh already in progress");
            return;
        }

        let runtime = match self.runtime.as_ref() {
            Some(r) => r.clone(),
            None => {
                log::error!("No runtime available for version refresh");
                return;
            }
        };

        // 获取下载源配置
        let download_source = self.config.download_source;
        let custom_mirror_url = self.config.custom_mirror_url.clone();

        // 设置刷新状态
        self.version_refresh_state.is_refreshing = true;
        self.version_refresh_state.last_error = None;

        // 创建通道
        let (tx, rx) = mpsc::channel();
        self.refresh_receiver = Some(rx);

        // 启动异步任务
        runtime.spawn(async move {
            log::info!("Starting async version refresh from API (source: {:?}, custom_url: {:?})...", download_source, custom_mirror_url);

            let result = crate::services::fetch_all_versions_with_source_and_custom(download_source, custom_mirror_url).await;

            match &result {
                Ok(versions) => {
                    log::info!("Successfully fetched {} versions", versions.len());
                }
                Err(e) => {
                    log::error!("Failed to fetch versions: {}", e);
                }
            }

            // 发送结果
            if let Err(e) = tx.send(RefreshResult { versions: result }) {
                log::error!("Failed to send refresh result: {}", e);
            }
        });
    }

    /// 检查并处理刷新结果
    /// 应在每帧调用
    pub fn poll_refresh_result(&mut self) {
        if let Some(ref receiver) = self.refresh_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.handle_refresh_result(result);
                self.refresh_receiver = None;
            }
        }
    }

    /// 处理刷新结果
    pub fn handle_refresh_result(&mut self, result: RefreshResult) {
        self.version_refresh_state.is_refreshing = false;

        match result.versions {
            Ok(versions) => {
                // 更新版本列表
                self.available_versions = versions;
                self.version_refresh_state.last_error = None;
                self.version_refresh_state.last_refresh_time = Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                );

                // 更新安装状态
                self.update_install_status();

                log::info!("Version list updated successfully");
            }
            Err(e) => {
                // 网络请求失败，保留错误信息，不使用备用列表
                log::error!("Failed to fetch version list: {}", e);
                self.version_refresh_state.last_error = Some(e);
                // 不修改 available_versions，保持原状（可能为空）
            }
        }
    }

    /// 从共享状态同步下载进度到主状态
    /// 供 UI 每帧调用以获取异步任务的进度和完成状态
    pub fn sync_download_progress(&mut self) {
        if let Some(ref shared) = self.shared_state {
            if let Ok(mut s) = shared.lock() {
                // 收集需要清理的标记
                let mut keys_to_remove: Vec<String> = Vec::new();

                // 同步下载进度
                for (key, progress) in &s.downloads_in_progress {
                    self.downloads_in_progress.insert(key.clone(), *progress);

                    // 检查是否有完成标记，如果有则需要清理
                    if key.ends_with("_complete") {
                        keys_to_remove.push(key.clone());
                    }
                }

                // 同步已安装版本列表（检查是否有新安装完成）
                for install in &s.installed_versions {
                    if !self.installed_versions.iter().any(|i|
                        i.version == install.version && i.variant == install.variant)
                    {
                        self.installed_versions.push(install.clone());
                    }
                }

                // 同步可用版本的安装状态
                for available in &mut self.available_versions {
                    let is_installed = s.installed_versions.iter().any(|i|
                        i.version == available.version && i.variant == available.variant
                    );
                    available.is_installed = is_installed;
                    if is_installed {
                        if let Some(installed) = s.installed_versions.iter()
                            .find(|i| i.version == available.version && i.variant == available.variant)
                        {
                            available.install_path = Some(installed.path.clone());
                        }
                    }
                }

                // 清理完成标记
                for key in keys_to_remove {
                    s.downloads_in_progress.remove(&key);
                    self.downloads_in_progress.remove(&key);
                }
            }
        }
    }

    /// 检查是否需要刷新版本列表（距离上次刷新超过指定秒数）
    pub fn should_refresh_versions(&self, max_age_secs: u64) -> bool {
        if self.version_refresh_state.is_refreshing {
            return false;
        }

        if self.available_versions.is_empty() {
            return true;
        }

        if let Some(last_time) = self.version_refresh_state.last_refresh_time {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            now - last_time > max_age_secs
        } else {
            true
        }
    }
}
