// AppState 实现方法

use std::path::{Path, PathBuf};

use crate::models::{GodotInstall, GodotVariant, GodotVersion};
use super::AppState;

impl AppState {
    /// 获取可用的 Godot 版本列表（模拟数据）
    pub fn fetch_available_versions() -> Vec<GodotVersion> {
        vec![
            GodotVersion {
                version: "4.3".to_string(),
                variant: GodotVariant::Standard,
                platform: "Linux64".to_string(),
                download_url: "https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_linux.x86_64.zip".to_string(),
                release_date: "2024-09-20".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "4.3".to_string(),
                variant: GodotVariant::Mono,
                platform: "Linux64".to_string(),
                download_url: "https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_mono_linux.x86_64.zip".to_string(),
                release_date: "2024-09-20".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "4.2.2".to_string(),
                variant: GodotVariant::Standard,
                platform: "Linux64".to_string(),
                download_url: "https://github.com/godotengine/godot/releases/download/4.2.2-stable/Godot_v4.2.2-stable_linux.x86_64.zip".to_string(),
                release_date: "2024-02-03".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "4.2.1".to_string(),
                variant: GodotVariant::Standard,
                platform: "Linux64".to_string(),
                download_url: "https://github.com/godotengine/godot/releases/download/4.2.1-stable/Godot_v4.2.1-stable_linux.x86_64.zip".to_string(),
                release_date: "2024-01-05".to_string(),
                is_installed: false,
                install_path: None,
            },
            GodotVersion {
                version: "3.5.3".to_string(),
                variant: GodotVariant::Standard,
                platform: "Linux64".to_string(),
                download_url: "https://github.com/godotengine/godot/releases/download/3.5.3-stable/Godot_v3.5.3-stable_linux.x86_64.zip".to_string(),
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
    fn find_godot_executable(path: &Path, variant: &GodotVariant) -> PathBuf {
        let exe_name = match variant {
            GodotVariant::Mono => "Godot_v4.3-stable_mono",
            _ => "Godot_v4.3-stable",
        };

        let possible_names = vec![
            "Godot",
            "Godot_v4.3-stable",
            "Godot_v4.2.2-stable",
            "Godot_v4.2.1-stable",
            "Godot_v3.5.3-stable",
            exe_name,
        ];

        for name in possible_names {
            let exe = path.join(name);
            if exe.exists() {
                return exe;
            }
            #[cfg(windows)]
            {
                let exe = path.join(format!("{}.exe", name));
                if exe.exists() {
                    return exe;
                }
            }
        }

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
