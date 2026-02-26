// AppConfig - 应用程序配置

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// 应用主题
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    /// 深色主题
    #[default]
    Dark,
    /// 浅色主题
    Light,
    /// 跟随系统
    System,
}

/// 下载源（镜像站）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DownloadSource {
    /// GitHub 官方源
    #[default]
    GitHub,
    /// 国内镜像源 (ghproxy.com)
    ChinaMirror,
    /// 国内镜像源 (gitclone.com)
    GitClone,
}

impl DownloadSource {
    /// 获取显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            DownloadSource::GitHub => "GitHub (Official)",
            DownloadSource::ChinaMirror => "GitHub Mirror (ghproxy.com)",
            DownloadSource::GitClone => "GitClone Mirror (gitclone.com)",
        }
    }

    /// 获取镜像URL前缀
    pub fn mirror_prefix(&self) -> &'static str {
        match self {
            DownloadSource::GitHub => "",
            DownloadSource::ChinaMirror => "https://ghproxy.com/",
            DownloadSource::GitClone => "https://gitclone.com/github.com/",
        }
    }

    /// 获取 GitHub API 代理URL
    pub fn api_proxy_url(&self) -> Option<&'static str> {
        match self {
            DownloadSource::GitHub => None,
            DownloadSource::ChinaMirror => Some("https://ghproxy.com/https://api.github.com"),
            DownloadSource::GitClone => Some("https://gitclone.com/github.com/api.github.com"),
        }
    }

    /// 是否需要代理
    pub fn needs_proxy(&self) -> bool {
        !matches!(self, DownloadSource::GitHub)
    }
}

impl Theme {
    /// 获取主题名称
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::System => "System",
        }
    }

    /// 获取主题图标
    pub fn icon(&self) -> &'static str {
        match self {
            Theme::Dark => "🌙",
            Theme::Light => "☀️",
            Theme::System => "💻",
        }
    }
}

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Godot 版本安装目录
    pub install_dir: PathBuf,
    /// Godot 项目目录
    pub projects_dir: PathBuf,
    /// 启动时是否检查更新
    pub check_updates_on_start: bool,
    /// 应用主题
    #[serde(default)]
    pub theme: Theme,
    /// 下载源（镜像站）
    #[serde(default)]
    pub download_source: DownloadSource,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let gdhub_dir = home_dir.join(".gdhub");

        Self {
            install_dir: gdhub_dir.join("versions"),
            projects_dir: home_dir.join("Godot"),
            check_updates_on_start: true,
            theme: Theme::default(),
            download_source: DownloadSource::default(),
        }
    }
}

impl AppConfig {
    /// 创建自定义配置
    pub fn new(install_dir: PathBuf, projects_dir: PathBuf, check_updates_on_start: bool, theme: Theme, download_source: DownloadSource) -> Self {
        Self {
            install_dir,
            projects_dir,
            check_updates_on_start,
            theme,
            download_source,
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
