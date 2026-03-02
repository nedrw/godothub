// GitHub API Service - 从 GitHub 获取 Godot 版本信息
// 支持国内镜像源加速

use serde::{Deserialize, Serialize};
use crate::state::DownloadSource;

/// GitHub Release 信息
/// 使用 deny_unknown_fields(false) 允许忽略未知字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitHubRelease {
    /// 发布标签 (如 "4.3-stable")
    pub tag_name: String,
    /// 发布名称
    #[serde(default)]
    pub name: String,
    /// 发布时间
    #[serde(default)]
    pub published_at: String,
    /// 是否预发布
    #[serde(default)]
    pub prerelease: bool,
    /// 是否草稿
    #[serde(default)]
    pub draft: bool,
    /// 发布资源列表
    #[serde(default)]
    pub assets: Vec<GitHubAsset>,
    /// 发布说明
    #[serde(default)]
    pub body: Option<String>,
}

/// GitHub Release Asset 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubAsset {
    /// 资源名称
    pub name: String,
    /// 下载 URL
    pub browser_download_url: String,
    /// 文件大小 (字节)
    #[serde(default)]
    pub size: u64,
    /// 下载次数
    #[serde(default)]
    pub download_count: u64,
    /// 状态
    #[serde(default)]
    pub state: String,
}

/// GitHub API 客户端
pub struct GitHubApi {
    /// HTTP 客户端
    client: reqwest::Client,
    /// 下载源（镜像站）
    download_source: DownloadSource,
    /// 自定义镜像URL（用户填写）
    custom_mirror_url: String,
}

impl GitHubApi {
    /// 创建新的 GitHub API 客户端（默认使用官方源）
    pub fn new() -> Self {
        Self::with_source(DownloadSource::GitHub)
    }

    /// 创建使用指定下载源的 GitHub API 客户端
    pub fn with_source(download_source: DownloadSource) -> Self {
        Self::with_source_and_custom(download_source, String::new())
    }

    /// 创建使用指定下载源和自定义镜像URL的 GitHub API 客户端
    pub fn with_source_and_custom(download_source: DownloadSource, custom_mirror_url: String) -> Self {
        log::info!("Creating GitHub API client with source: {:?}, custom_url: {}", download_source, custom_mirror_url);
        Self {
            client: reqwest::Client::builder()
                .user_agent("GodotHub/0.1.2")
                .timeout(std::time::Duration::from_secs(60))
                .connect_timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            download_source,
            custom_mirror_url,
        }
    }

    /// 获取 API URL（支持自定义镜像）
    fn get_api_url(&self, path: &str) -> String {
        match self.download_source {
            DownloadSource::GitHub => {
                format!("https://api.github.com{}", path)
            }
            DownloadSource::Custom => {
                // 自定义镜像 URL
                if !self.custom_mirror_url.is_empty() {
                    let mirror_url = self.custom_mirror_url.trim().trim_end_matches('/');
                    format!("{}/https://api.github.com{}", mirror_url, path)
                } else {
                    // 如果没有配置自定义镜像URL，使用官方API
                    format!("https://api.github.com{}", path)
                }
            }
        }
    }

    /// 从 GitHub API 获取 Godot 发布版本
    /// 包含镜像回退机制：如果镜像失败，自动尝试官方 API
    pub async fn fetch_releases(&self) -> Result<Vec<GitHubRelease>, String> {
        let repo = "godotengine/godot";
        log::info!("Fetching releases for {} using source: {:?}", repo, self.download_source);

        // 首先尝试使用配置的下载源
        let result = self.fetch_releases_with_source(repo, self.download_source).await;

        // 如果失败且使用的是镜像，尝试回退到官方源
        // 无论错误是什么，只要使用镜像失败了就回退
        if result.is_err() && self.download_source.needs_proxy() {
            let error_msg = result.as_ref().err().cloned().unwrap_or_else(|| "Unknown error".to_string());
            log::warn!("Mirror failed: {}, falling back to official GitHub API", error_msg);
            return self.fetch_releases_with_source(repo, DownloadSource::GitHub).await;
        }

        result
    }

    /// 使用指定的下载源获取 releases
    async fn fetch_releases_with_source(&self, repo: &str, source: DownloadSource) -> Result<Vec<GitHubRelease>, String> {
        log::info!("Fetching releases with source: {:?}", source);

        // 使用自定义 API URL 方法构建完整的 API URL
        let api_path = format!("/repos/{}/releases?per_page=50", repo);
        let url = self.get_api_url(&api_path);

        log::info!("Final API URL: {}", url);

        let response = self.client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| {
                let error_msg = format!("Failed to fetch releases: {}", e);
                log::error!("{}", error_msg);
                error_msg
            })?;

        if !response.status().is_success() {
            let error_msg = format!("GitHub API error: {} - URL: {}", response.status(), url);
            log::error!("{}", error_msg);
            return Err(error_msg);
        }

        // 先获取响应文本，便于调试
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        log::info!("Response length: {} bytes", response_text.len());

        // 如果响应太短，可能是错误
        if response_text.len() < 100 {
            log::error!("Response too short, might be an error: {}", response_text);
            return Err(format!("API returned unexpected response: {}", response_text));
        }

        // 尝试解析 JSON
        let releases: Vec<GitHubRelease> = match serde_json::from_str(&response_text) {
            Ok(r) => r,
            Err(e) => {
                // 尝试解析错误信息
                if let Ok(error_obj) = serde_json::from_str::<serde_json::Value>(&response_text) {
                    if let Some(message) = error_obj.get("message").and_then(|m| m.as_str()) {
                        return Err(format!("GitHub API error: {}", message));
                    }
                }
                let error_msg = format!("Failed to parse releases: {}. Response preview: {}",
                    e, &response_text[..response_text.len().min(500)]);
                log::error!("{}", error_msg);
                return Err(error_msg);
            }
        };

        // 过滤掉预发布版本
        let stable_releases: Vec<GitHubRelease> = releases
            .into_iter()
            .filter(|r| !r.prerelease)
            .collect();

        log::info!("Fetched {} stable releases", stable_releases.len());
        Ok(stable_releases)
    }

    /// 解析版本号
    pub fn parse_version(tag: &str) -> Option<String> {
        // 从标签中提取版本号，如 "4.3-stable" -> "4.3"
        let version = tag
            .strip_prefix('v')
            .unwrap_or(tag)
            .split('-')
            .next()?;

        // 验证版本号格式
        if version.chars().all(|c| c.is_digit(10) || c == '.') {
            Some(version.to_string())
        } else {
            None
        }
    }

    /// 检测当前平台
    pub fn detect_platform() -> Platform {
        #[cfg(target_os = "linux")]
        {
            #[cfg(target_arch = "x86_64")]
            return Platform::Linux64;
            #[cfg(target_arch = "x86")]
            return Platform::Linux32;
            #[cfg(target_arch = "aarch64")]
            return Platform::LinuxARM64;
        }

        #[cfg(target_os = "macos")]
        {
            #[cfg(target_arch = "x86_64")]
            return Platform::MacOSIntel;
            #[cfg(target_arch = "aarch64")]
            return Platform::MacOSARM;
        }

        #[cfg(target_os = "windows")]
        {
            #[cfg(target_arch = "x86_64")]
            return Platform::Windows64;
            #[cfg(target_arch = "x86")]
            return Platform::Windows32;
        }

        #[allow(unreachable_code)]
        Platform::Linux64 // 默认
    }

    /// 从资源列表中查找适合当前平台的下载 URL
    pub fn find_download_url(
        assets: &[GitHubAsset],
        platform: Platform,
        mono: bool,
    ) -> Option<(String, String)> {
        let platform_patterns = platform.patterns(mono);

        for asset in assets {
            let name_lower = asset.name.to_lowercase();

            for pattern in &platform_patterns {
                if name_lower.contains(pattern) {
                    // 验证文件类型
                    if name_lower.ends_with(".zip")
                        || name_lower.ends_with(".tar.xz")
                        || name_lower.ends_with(".exe")
                        || name_lower.ends_with(".dmg")
                        || name_lower.ends_with(".app")
                    {
                        return Some((asset.name.clone(), asset.browser_download_url.clone()));
                    }
                }
            }
        }
        None
    }

    /// 转换下载 URL 为镜像 URL
    pub fn convert_to_mirror_url(&self, original_url: &str) -> String {
        match self.download_source {
            DownloadSource::GitHub => original_url.to_string(),
            DownloadSource::Custom => {
                // 自定义镜像 URL
                if !self.custom_mirror_url.is_empty() {
                    let mirror_url = self.custom_mirror_url.trim().trim_end_matches('/');
                    // 尝试去除原始URL的 https:// 前缀
                    let url_without_protocol = original_url.strip_prefix("https://").unwrap_or(original_url);
                    format!("{}/{}", mirror_url, url_without_protocol)
                } else {
                    // 如果没有配置自定义镜像URL，使用官方源
                    original_url.to_string()
                }
            }
        }
    }
}

impl Default for GitHubApi {
    fn default() -> Self {
        Self::new()
    }
}

/// 支持的平台
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Linux64,
    Linux32,
    LinuxARM64,
    MacOSIntel,
    MacOSARM,
    Windows64,
    Windows32,
}

impl Platform {
    /// 获取平台的名称
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Linux64 => "Linux64",
            Platform::Linux32 => "Linux32",
            Platform::LinuxARM64 => "LinuxARM64",
            Platform::MacOSIntel => "macOS (Intel)",
            Platform::MacOSARM => "macOS (ARM)",
            Platform::Windows64 => "Windows64",
            Platform::Windows32 => "Windows32",
        }
    }

    /// 获取匹配模式的列表
    pub fn patterns(&self, mono: bool) -> Vec<String> {
        let mono_str = if mono { "_mono" } else { "" };

        match self {
            Platform::Linux64 => vec![
                format!("_linux.x86_64{}.zip", mono_str),
                format!("_linux.x86_64{}", mono_str),
                format!("_linux64{}.zip", mono_str),
                format!("_linux64{}", mono_str),
            ],
            Platform::Linux32 => vec![
                format!("_linux.x86_32{}.zip", mono_str),
                format!("_linux.x86_32{}", mono_str),
                format!("_linux32{}.zip", mono_str),
                format!("_linux32{}", mono_str),
            ],
            Platform::LinuxARM64 => vec![
                format!("_linux.arm64{}.zip", mono_str),
                format!("_linux.arm64{}", mono_str),
            ],
            Platform::MacOSIntel => vec![
                format!("_macos.universal{}.zip", mono_str),
                format!("_macos.intel{}.zip", mono_str),
                format!("_osx.universal{}.zip", mono_str),
                format!("_osx.intel{}.zip", mono_str),
            ],
            Platform::MacOSARM => vec![
                format!("_macos.universal{}.zip", mono_str),
                format!("_macos.arm{}.zip", mono_str),
                format!("_osx.universal{}.zip", mono_str),
            ],
            Platform::Windows64 => vec![
                format!("_win64.exe{}.zip", mono_str),
                format!("_win64{}", mono_str),
                format!("_windows64{}.zip", mono_str),
            ],
            Platform::Windows32 => vec![
                format!("_win32.exe{}.zip", mono_str),
                format!("_win32{}", mono_str),
                format!("_windows32{}.zip", mono_str),
            ],
        }
    }
}

/// 将 GitHub Release 转换为 GodotVersion
pub fn release_to_version(
    release: &GitHubRelease,
    mono: bool,
    api: &GitHubApi,
) -> Option<crate::models::GodotVersion> {
    // 跳过草稿版本
    if release.draft {
        return None;
    }

    let version = GitHubApi::parse_version(&release.tag_name)?;
    let platform = GitHubApi::detect_platform();

    let (_filename, original_url) = GitHubApi::find_download_url(
        &release.assets,
        platform,
        mono,
    )?;

    // 转换为镜像 URL
    let download_url = api.convert_to_mirror_url(&original_url);

    // 解析发布日期
    let release_date = if release.published_at.is_empty() {
        "Unknown".to_string()
    } else {
        release.published_at
            .split('T')
            .next()
            .unwrap_or("Unknown")
            .to_string()
    };

    Some(crate::models::GodotVersion {
        version: version.clone(),
        variant: if mono {
            crate::models::GodotVariant::Mono
        } else {
            crate::models::GodotVariant::Standard
        },
        platform: platform.name().to_string(),
        download_url,
        release_date,
        is_installed: false,
        install_path: None,
    })
}

/// 获取所有可用版本（标准版和 Mono 版）- 使用默认源
pub async fn fetch_all_versions() -> Result<Vec<crate::models::GodotVersion>, String> {
    fetch_all_versions_with_source(DownloadSource::GitHub).await
}

/// 获取所有可用版本（使用指定下载源）
pub async fn fetch_all_versions_with_source(
    download_source: DownloadSource,
) -> Result<Vec<crate::models::GodotVersion>, String> {
    fetch_all_versions_with_source_and_custom(download_source, String::new()).await
}

/// 获取所有可用版本（使用指定下载源和自定义镜像URL）
pub async fn fetch_all_versions_with_source_and_custom(
    download_source: DownloadSource,
    custom_mirror_url: String,
) -> Result<Vec<crate::models::GodotVersion>, String> {
    log::info!("Fetching versions with source: {:?}, custom_url: {}", download_source, custom_mirror_url);

    let api = GitHubApi::with_source_and_custom(download_source, custom_mirror_url);
    let releases = api.fetch_releases().await?;

    log::info!("Received {} releases from API", releases.len());

    let mut versions = Vec::new();
    let mut standard_count = 0;
    let mut mono_count = 0;

    for release in &releases {
        // 跳过预发布和草稿版本
        if release.prerelease || release.draft {
            continue;
        }

        // 添加标准版
        if let Some(version) = release_to_version(release, false, &api) {
            versions.push(version);
            standard_count += 1;
        }

        // 添加 Mono 版
        if let Some(version) = release_to_version(release, true, &api) {
            versions.push(version);
            mono_count += 1;
        }
    }

    log::info!("Parsed {} standard versions and {} mono versions", standard_count, mono_count);

    // 按版本号排序（从新到旧）
    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let b_parts: Vec<u32> = b.version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();

        // 比较版本号各部分
        for i in 0..a_parts.len().min(b_parts.len()) {
            match b_parts.get(i).cmp(&a_parts.get(i)) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }

        b_parts.len().cmp(&a_parts.len())
    });

    log::info!("Returning {} versions", versions.len());
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(GitHubApi::parse_version("4.3-stable"), Some("4.3".to_string()));
        assert_eq!(GitHubApi::parse_version("v4.2.2-stable"), Some("4.2.2".to_string()));
        assert_eq!(GitHubApi::parse_version("3.5.3-stable"), Some("3.5.3".to_string()));
        assert_eq!(GitHubApi::parse_version("invalid"), None);
    }

    #[test]
    fn test_mirror_url_conversion() {
        let api_github = GitHubApi::with_source(DownloadSource::GitHub);
        let url = "https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_linux.x86_64.zip";
        assert_eq!(api_github.convert_to_mirror_url(url), url);

        // Test custom mirror
        let api_custom = GitHubApi::with_source_and_custom(DownloadSource::Custom, "https://mirror.example.com".to_string());
        let mirrored = api_custom.convert_to_mirror_url(url);
        assert!(mirrored.starts_with("https://mirror.example.com/"));
        assert!(mirrored.contains("github.com"));
    }

    #[test]
    fn test_platform_patterns() {
        let patterns = Platform::Linux64.patterns(false);
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.contains("linux.x86_64")));

        let patterns_mono = Platform::Linux64.patterns(true);
        assert!(patterns_mono.iter().any(|p| p.contains("_mono")));
    }

    #[test]
    fn test_download_source() {
        assert_eq!(DownloadSource::GitHub.mirror_prefix(), "");
        assert!(DownloadSource::GitHub.api_proxy_url().is_none());
        assert!(DownloadSource::Custom.needs_proxy());
    }
}
