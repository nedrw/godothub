// AppState 实现方法

use std::path::{Path, PathBuf};

use crate::models::{GodotInstall, GodotVariant, GodotVersion};
use super::AppState;

/// 检测当前平台
fn detect_platform() -> String {
    #[cfg(target_os = "linux")]
    {
        #[cfg(target_arch = "x86_64")]
        return "Linux64".to_string();
        #[cfg(target_arch = "x86")]
        return "Linux32".to_string();
        #[cfg(target_arch = "aarch64")]
        return "LinuxARM64".to_string();
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "x86_64")]
        return "macOS.Intel".to_string();
        #[cfg(target_arch = "aarch64")]
        return "macOS.ARM".to_string();
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(target_arch = "x86_64")]
        return "Windows64".to_string();
        #[cfg(target_arch = "x86")]
        return "Windows32".to_string();
    }

    "Unknown".to_string()
}

/// 根据平台和版本格式化下载 URL
fn format_platform_url(version: &str, _release_type: &str, variant: GodotVariant) -> String {
    let variant_str = match variant {
        GodotVariant::Mono => "_mono",
        _ => "",
    };

    #[cfg(target_os = "linux")]
    {
        #[cfg(target_arch = "x86_64")]
        return format!(
            "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_linux.x86_64.zip",
            version, version, variant_str
        );
        #[cfg(target_arch = "x86")]
        return format!(
            "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_linux.x86_32.zip",
            version, version, variant_str
        );
        #[cfg(target_arch = "aarch64")]
        return format!(
            "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_linux.arm64.zip",
            version, version, variant_str
        );
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "x86_64")]
        return format!(
            "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_macos.universal.zip",
            version, version, variant_str
        );
        #[cfg(target_arch = "aarch64")]
        return format!(
            "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_macos.universal.zip",
            version, version, variant_str
        );
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(target_arch = "x86_64")]
        return format!(
            "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_win64.exe.zip",
            version, version, variant_str
        );
        #[cfg(target_arch = "x86")]
        return format!(
            "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_win32.exe.zip",
            version, version, variant_str
        );
    }

    // 默认返回 Linux x86_64
    format!(
        "https://github.com/godotengine/godot/releases/download/{}-stable/Godot_v{}-stable{}_linux.x86_64.zip",
        version, version, variant_str
    )
}

impl AppState {
    /// 获取可用的 Godot 版本列表（更新到最新版本）
    pub fn fetch_available_versions() -> Vec<GodotVersion> {
        vec![
            // Godot 4.6 - Latest
            GodotVersion {
                version: "4.6".to_string(),
                variant: GodotVariant::Standard,
                platform: detect_platform(),
                download_url: format_platform_url("4.6", "stable", GodotVariant::Standard),
                release_date: "2025-01-16".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "4.6".to_string(),
                variant: GodotVariant::Mono,
                platform: detect_platform(),
                download_url: format_platform_url("4.6", "stable", GodotVariant::Mono),
                release_date: "2025-01-16".to_string(),
                is_installed: false,
                install_path: None,
            },
            // Godot 4.5
            GodotVersion {
                version: "4.5".to_string(),
                variant: GodotVariant::Standard,
                platform: detect_platform(),
                download_url: format_platform_url("4.5", "stable", GodotVariant::Standard),
                release_date: "2024-11-15".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "4.5".to_string(),
                variant: GodotVariant::Mono,
                platform: detect_platform(),
                download_url: format_platform_url("4.5", "stable", GodotVariant::Mono),
                release_date: "2024-11-15".to_string(),
                is_installed: false,
                install_path: None,
            },
            // Godot 4.4
            GodotVersion {
                version: "4.4".to_string(),
                variant: GodotVariant::Standard,
                platform: detect_platform(),
                download_url: format_platform_url("4.4", "stable", GodotVariant::Standard),
                release_date: "2024-10-08".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "4.4".to_string(),
                variant: GodotVariant::Mono,
                platform: detect_platform(),
                download_url: format_platform_url("4.4", "stable", GodotVariant::Mono),
                release_date: "2024-10-08".to_string(),
                is_installed: false,
                install_path: None,
            },
            // Godot 4.3
            GodotVersion {
                version: "4.3".to_string(),
                variant: GodotVariant::Standard,
                platform: detect_platform(),
                download_url: format_platform_url("4.3", "stable", GodotVariant::Standard),
                release_date: "2024-09-20".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "4.3".to_string(),
                variant: GodotVariant::Mono,
                platform: detect_platform(),
                download_url: format_platform_url("4.3", "stable", GodotVariant::Mono),
                release_date: "2024-09-20".to_string(),
                is_installed: false,
                install_path: None,
            },
            // Godot 4.2.2
            GodotVersion {
                version: "4.2.2".to_string(),
                variant: GodotVariant::Standard,
                platform: detect_platform(),
                download_url: format_platform_url("4.2.2", "stable", GodotVariant::Standard),
                release_date: "2024-02-03".to_string(),
                is_installed: false,
                install_path: None,
            },
            // Godot 3.5.3 - LTS
            GodotVersion {
                version: "3.5.3".to_string(),
                variant: GodotVariant::Standard,
                platform: detect_platform(),
                download_url: format_platform_url("3.5.3", "stable", GodotVariant::Standard),
                release_date: "2023-09-11".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "3.5.3".to_string(),
                variant: GodotVariant::Mono,
                platform: detect_platform(),
                download_url: format_platform_url("3.5.3", "stable", GodotVariant::Mono),
                release_date: "2023-09-11".to_string(),
                is_installed: false,
                install_path: None,
            },
        ]
    }

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
        for available in &mut self.available_versions {
            available.is_installed = self.installed_versions.iter().any(|installed| {
                installed.version == available.version && installed.variant == available.variant
            });
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

    /// 移除已安装的版本
    pub fn remove_installed_version(&mut self, index: usize) -> Option<GodotInstall> {
        if index < self.installed_versions.len() {
            let removed = self.installed_versions.remove(index);
            // 更新可用版本状态
            for available in &mut self.available_versions {
                if available.version == removed.version && available.variant == removed.variant {
                    available.is_installed = false;
                    available.install_path = None;
                }
            }
            Some(removed)
        } else {
            None
        }
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
}
