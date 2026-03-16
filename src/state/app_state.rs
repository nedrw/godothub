// AppState - 应用程序状态

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

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
    /// 共享状态指针，用于异步任务更新进度（不序列化）
    #[serde(skip)]
    pub shared_state: Option<Arc<Mutex<AppState>>>,
    /// 删除确认对话框状态（不序列化）
    #[serde(skip)]
    pub delete_confirm: Option<DeleteConfirmState>,
    /// 下载取消令牌 (版本标识 -> 取消标志)（不序列化）
    #[serde(skip)]
    pub cancellation_tokens: HashMap<String, Arc<AtomicBool>>,
    /// 下载对话框搜索文本（帧间持久化，不序列化）
    #[serde(skip)]
    pub download_search_text: String,
}

/// 删除确认对话框状态
#[derive(Debug, Clone)]
pub struct DeleteConfirmState {
    /// 要删除的版本索引
    pub version_index: usize,
    /// 要删除的版本信息（用于显示）
    pub version_info: String,
}

/// 手动实现 Clone，跳过无法克隆的字段
impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            installed_versions: self.installed_versions.clone(),
            available_versions: self.available_versions.clone(),
            downloads_in_progress: self.downloads_in_progress.clone(),
            selected_version_index: self.selected_version_index,
            show_download_dialog: self.show_download_dialog,
            current_tab: self.current_tab.clone(),
            config: self.config.clone(),
            runtime: None, // Runtime 不支持 Clone
            version_refresh_state: self.version_refresh_state.clone(),
            refresh_receiver: None, // Receiver 不支持 Clone
            shared_state: None,     // 避免循环引用
            delete_confirm: None,
            cancellation_tokens: HashMap::new(), // 克隆时不保留取消令牌
            download_search_text: self.download_search_text.clone(),
        }
    }
}

impl AppState {
    /// 创建共享状态指针（用于异步任务更新状态）
    /// 注意：这个克隆版本会移除 Runtime 字段，因为 Runtime 不支持 Clone
    pub fn create_shared_state(&mut self) -> Arc<Mutex<AppState>> {
        let state_for_async = self.clone();
        let shared = Arc::new(Mutex::new(state_for_async));
        self.shared_state = Some(Arc::clone(&shared));
        shared
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            installed_versions: Vec::new(),
            available_versions: Vec::new(),
            downloads_in_progress: HashMap::new(),
            selected_version_index: None,
            show_download_dialog: false,
            current_tab: MainTab::Versions,
            config: crate::state::AppConfig::default(),
            runtime: None,
            version_refresh_state: VersionRefreshState::default(),
            refresh_receiver: None,
            shared_state: None,
            delete_confirm: None,
            cancellation_tokens: HashMap::new(),
            download_search_text: String::new(),
        }
    }
}
