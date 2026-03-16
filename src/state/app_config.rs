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
    /// 自定义镜像源
    Custom,
}

#[allow(dead_code)]
impl DownloadSource {
    /// 获取显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            DownloadSource::GitHub => "GitHub (Official)",
            DownloadSource::Custom => "Custom Mirror",
        }
    }

    /// 获取镜像URL前缀
    pub fn mirror_prefix(&self) -> &'static str {
        match self {
            DownloadSource::GitHub => "",
            DownloadSource::Custom => "",
        }
    }

    /// 获取 GitHub API 代理URL
    pub fn api_proxy_url(&self) -> Option<&'static str> {
        match self {
            DownloadSource::GitHub => None,
            // 自定义镜像源需要用户填写 API 地址
            DownloadSource::Custom => None,
        }
    }

    /// 获取完整的 GitHub API URL（包含路径）
    /// 用于直接构建完整的 API 请求 URL
    pub fn full_api_url(&self, path: &str) -> String {
        match self.api_proxy_url() {
            Some(proxy_url) => {
                format!("{}{}", proxy_url, path)
            }
            None => {
                format!("https://api.github.com{}", path)
            }
        }
    }

    /// 是否需要代理
    pub fn needs_proxy(&self) -> bool {
        matches!(self, DownloadSource::Custom)
    }

    /// 是否使用自定义镜像
    pub fn is_custom(&self) -> bool {
        matches!(self, DownloadSource::Custom)
    }
}

#[allow(dead_code)]
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
    /// 自定义镜像站URL（用户填写）
    #[serde(default)]
    pub custom_mirror_url: String,
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
            download_source: DownloadSource::GitHub,
            custom_mirror_url: String::new(),
        }
    }
}

#[allow(dead_code)]
impl AppConfig {
    /// 创建自定义配置
    pub fn new(
        install_dir: PathBuf,
        projects_dir: PathBuf,
        check_updates_on_start: bool,
        theme: Theme,
        download_source: DownloadSource,
        custom_mirror_url: String,
    ) -> Self {
        Self {
            install_dir,
            projects_dir,
            check_updates_on_start,
            theme,
            download_source,
            custom_mirror_url,
        }
    }

    /// 获取自定义镜像的下载URL
    /// 如果custom_mirror_url不为空，则使用它作为基础URL
    pub fn get_custom_download_prefix(&self) -> Option<String> {
        if self.download_source == DownloadSource::Custom && !self.custom_mirror_url.is_empty() {
            let url = self.custom_mirror_url.trim();
            // 确保 URL 格式正确
            if url.starts_with("http://") || url.starts_with("https://") {
                // 去掉末尾的斜杠
                let url = url.trim_end_matches('/');
                Some(url.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 获取自定义镜像的 API URL
    pub fn get_custom_api_url(&self) -> Option<String> {
        if self.download_source == DownloadSource::Custom && !self.custom_mirror_url.is_empty() {
            let url = self.custom_mirror_url.trim();
            // 确保 URL 格式正确
            if url.starts_with("http://") || url.starts_with("https://") {
                let url = url.trim_end_matches('/');
                // 自定义镜像通常需要代理 GitHub API
                Some(format!("{}/https://api.github.com", url))
            } else {
                None
            }
        } else {
            None
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
