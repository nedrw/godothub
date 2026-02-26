// AppState - 应用程序状态

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{GodotInstall, GodotVariant, GodotVersion};

/// 主面板标签页
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MainTab {
    Versions,
    Projects,
    Settings,
}

impl Default for MainTab {
    fn default() -> Self {
        MainTab::Versions
    }
}

/// 应用程序状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// 已安装的 Godot 版本列表
    pub installed_versions: Vec<GodotInstall>,
    /// 可用的 Godot 版本列表
    pub available_versions: Vec<GodotVersion>,
    /// 下载进度 (版本标识 -> 进度 0.0-1.0)
    pub downloads_in_progress: HashMap<String, f32>,
    /// 当前选中的版本索引
    pub selected_version_index: Option<usize>,
    /// 是否显示下载对话框
    pub show_download_dialog: bool,
    /// 当前显示的标签页
    pub current_tab: MainTab,
    /// 应用程序配置
    pub config: crate::state::AppConfig,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            installed_versions: Vec::new(),
            available_versions: Self::fetch_available_versions(),
            downloads_in_progress: HashMap::new(),
            selected_version_index: None,
            show_download_dialog: false,
            current_tab: MainTab::Versions,
            config: crate::state::AppConfig::default(),
        }
    }
}
