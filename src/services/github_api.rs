// GitHub API Service - 从 GitHub 获取 Godot 版本信息

use serde::{Deserialize, Serialize};

/// GitHub Release 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRelease {
    /// 发布标签 (如 "4.3-stable")
    pub tag_name: String,
    /// 发布名称
    #[serde(default)]
    pub name: String,
    /// 发布时间
    pub published_at: String,
    /// 是否预发布
    pub prerelease: bool,
    /// 发布资源列表
    #[serde(default)]
    pub assets: Vec<GitHubAsset>,
    /// 发布说明
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
    pub size: u64,
    /// 下载次数
    #[serde(default)]
    pub download_count: u64,
}

/// GitHub API 客户端
pub struct GitHubApi {
    /// HTTP 客户端
    client: reqwest::Client,
}

impl GitHubApi {
    /// 创建新的 GitHub API 客户端
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("GodotHub/0.1.1")
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// 从 GitHub API 获取 Godot 发布版本
    pub async fn fetch_releases(&self) -> Result<Vec<GitHubRelease>, String> {
        let repo = "godotengine/godot";
        log::info!("Fetching releases from GitHub API for {}", repo);

        let url = format!(
            "https://api.github.com/repos/{}/releases?per_page=50",
            repo
        );

        let response = self.client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch releases: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()));
        }

        let releases: Vec<GitHubRelease> = response
            .json::<Vec<GitHubRelease>>()
            .await
            .map_err(|e| format!("Failed to parse releases: {}", e))?;

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
) -> Option<crate::models::GodotVersion> {
    let version = GitHubApi::parse_version(&release.tag_name)?;
    let platform = GitHubApi::detect_platform();

    let (filename, download_url) = GitHubApi::find_download_url(
        &release.assets,
        platform,
        mono,
    )?;

    // 解析发布日期
    let release_date = release.published_at
        .split('T')
        .next()
        .unwrap_or("Unknown")
        .to_string();

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

/// 获取所有可用版本（标准版和 Mono 版）
pub async fn fetch_all_versions() -> Result<Vec<crate::models::GodotVersion>, String> {
    let api = GitHubApi::new();
    let releases = api.fetch_releases().await?;

    let mut versions = Vec::new();

    for release in releases {
        // 添加标准版
        if let Some(version) = release_to_version(&release, false) {
            versions.push(version);
        }

        // 添加 Mono 版
        if let Some(version) = release_to_version(&release, true) {
            versions.push(version);
        }
    }

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
    fn test_detect_platform() {
        let platform = GitHubApi::detect_platform();
        // 测试平台检测是否正常工作
        match platform {
            Platform::Linux64 | Platform::Linux32 | Platform::LinuxARM64 => {
                #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
                assert_eq!(platform, Platform::Linux64);
            }
            Platform::MacOSIntel | Platform::MacOSARM => {
                #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
                assert_eq!(platform, Platform::MacOSIntel);
            }
            Platform::Windows64 | Platform::Windows32 => {
                #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
                assert_eq!(platform, Platform::Windows64);
            }
        }
    }

    #[test]
    fn test_platform_patterns() {
        let patterns = Platform::Linux64.patterns(false);
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.contains("linux.x86_64")));

        let patterns_mono = Platform::Linux64.patterns(true);
        assert!(patterns_mono.iter().any(|p| p.contains("_mono")));
    }
}
