// GodotInstall - 表示已安装的 Godot 实例

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::GodotVariant;

/// 表示一个已安装的 Godot 引擎实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodotInstall {
    /// 版本号 (如 "4.3", "4.2.2")
    pub version: String,
    /// 变体类型 (Standard/Mono)
    pub variant: GodotVariant,
    /// 可执行文件路径
    pub path: PathBuf,
    /// 是否收藏
    pub is_favorite: bool,
    /// 最后使用时间
    pub last_used: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
impl GodotInstall {
    /// 创建一个新的 GodotInstall
    pub fn new(version: String, variant: GodotVariant, path: PathBuf) -> Self {
        Self {
            version,
            variant,
            path,
            is_favorite: false,
            last_used: None,
        }
    }

    /// 获取显示名称
    pub fn display_name(&self) -> String {
        format!("{} ({})", self.version, self.variant.name())
    }

    /// 标记为已使用
    pub fn mark_used(&mut self) {
        self.last_used = Some(Utc::now());
    }

    /// 切换收藏状态
    pub fn toggle_favorite(&mut self) {
        self.is_favorite = !self.is_favorite;
    }

    /// 检查可执行文件是否存在
    pub fn is_valid(&self) -> bool {
        self.path.exists()
    }
}
