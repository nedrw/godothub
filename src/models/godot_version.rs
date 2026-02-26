// GodotVersion - 表示一个可用的 Godot 版本信息

use std::path::PathBuf;

use super::GodotVariant;

/// 表示一个可用的 Godot 版本
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GodotVersion {
    /// 版本号 (如 "4.3", "4.2.2")
    pub version: String,
    /// 变体类型 (Standard/Mono/ExportTemplates)
    pub variant: GodotVariant,
    /// 平台标识 (如 "Linux64", "Windows64", "macOS")
    pub platform: String,
    /// 下载链接
    pub download_url: String,
    /// 发布日期 (YYYY-MM-DD)
    pub release_date: String,
    /// 是否已安装
    pub is_installed: bool,
    /// 已安装版本的路径 (如果已安装)
    pub install_path: Option<PathBuf>,
}

impl GodotVersion {
    /// 创建一个新的 GodotVersion
    pub fn new(
        version: String,
        variant: GodotVariant,
        platform: String,
        download_url: String,
        release_date: String,
    ) -> Self {
        Self {
            version,
            variant,
            platform,
            download_url,
            release_date,
            is_installed: false,
            install_path: None,
        }
    }

    /// 获取显示名称 (版本号 + 变体)
    pub fn display_name(&self) -> String {
        format!("{} ({})", self.version, self.variant.name())
    }

    /// 标记为已安装
    pub fn mark_installed(&mut self, path: PathBuf) {
        self.is_installed = true;
        self.install_path = Some(path);
    }

    /// 标记为未安装
    pub fn mark_not_installed(&mut self) {
        self.is_installed = false;
        self.install_path = None;
    }
}
