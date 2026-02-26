// AppConfig - 应用程序配置

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Godot 版本安装目录
    pub install_dir: PathBuf,
    /// Godot 项目目录
    pub projects_dir: PathBuf,
    /// 启动时是否检查更新
    pub check_updates_on_start: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let gdhub_dir = home_dir.join(".gdhub");

        Self {
            install_dir: gdhub_dir.join("versions"),
            projects_dir: home_dir.join("Godot"),
            check_updates_on_start: true,
        }
    }
}

impl AppConfig {
    /// 创建自定义配置
    pub fn new(install_dir: PathBuf, projects_dir: PathBuf, check_updates_on_start: bool) -> Self {
        Self {
            install_dir,
            projects_dir,
            check_updates_on_start,
        }
    }

    /// 确保必要的目录存在
    pub fn ensure_directories(&self) -> std::io::Result<()> {
        if !self.install_dir.exists() {
            std::fs::create_dir_all(&self.install_dir)?;
        }
        if !self.projects_dir.exists() {
            std::fs::create_dir_all(&self.projects_dir)?;
        }
        Ok(())
    }

    /// 获取配置文件的路径
    pub fn config_file_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("gdhub").join("config.json"))
    }

    /// 加载配置（如果存在）
    pub fn load() -> Self {
        if let Some(config_path) = Self::config_file_path() {
            if config_path.exists() {
                if let Ok(contents) = std::fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str(&contents) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    /// 保存配置到文件
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(config_path) = Self::config_file_path() {
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let contents = serde_json::to_string_pretty(self)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            std::fs::write(config_path, contents)?;
        }
        Ok(())
    }
}
