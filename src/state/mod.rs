// State 模块 - 应用程序状态管理

pub mod app_config;
pub mod app_state;
pub mod app_state_impl;
pub mod install_meta;

pub use app_config::{AppConfig, DownloadSource, Theme};
pub use app_state::{AppState, DeleteConfirmState, MainTab, RefreshResult};
