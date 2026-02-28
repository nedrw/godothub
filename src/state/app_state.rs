// AppState - 应用程序状态

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::models::{GodotInstall, GodotVersion};

/// 版本刷新状态
#[derive(Debug, Clone, Default)]
pub struct VersionRefreshState {
    /// 是否正在刷新
    pub is_refreshing: bool,
    /// 最后一次刷新的错误信息
    pub last_error: Option<String>,
    /// 上次成功刷新的时间戳（Unix时间戳）
    pub last_refresh_time: Option<u64>,
}

/// 异步刷新结果
#[derive(Debug)]
pub struct RefreshResult {
    pub versions: Result<Vec<crate::models::GodotVersion>, String>,
}

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
#[derive(Debug, Serialize, Deserialize)]
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
    /// Tokio 运行时（不序列化）
    #[serde(skip)]
    pub runtime: Option<Arc<Runtime>>,
    /// 版本列表刷新状态（不序列化）
    #[serde(skip)]
    pub version_refresh_state: VersionRefreshState,
    /// 版本刷新结果接收器（不序列化）
    #[serde(skip)]
    pub refresh_receiver: Option<std::sync::mpsc::Receiver<RefreshResult>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            installed_versions: Vec::new(),
            available_versions: Vec::new(), // 初始为空，启动后异步加载
            downloads_in_progress: HashMap::new(),
            selected_version_index: None,
            show_download_dialog: false,
            current_tab: MainTab::Versions,
            config: crate::state::AppConfig::default(),
            runtime: None,
            version_refresh_state: VersionRefreshState::default(),
            refresh_receiver: None,
        }
    }
}
