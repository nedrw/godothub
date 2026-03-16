// GodotVariant - 表示 Godot 版本的不同类型

use serde::{Deserialize, Serialize};

/// 表示 Godot 引擎的变体类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GodotVariant {
    /// 标准版 - 不包含 Mono 运行时
    Standard,
    /// Mono 版 - 包含 C# 支持
    Mono,
    /// 导出模板 - 用于导出项目到不同平台
    ExportTemplates,
}

impl GodotVariant {
    /// 获取变体名称
    pub fn name(&self) -> &'static str {
        match self {
            GodotVariant::Standard => "Standard",
            GodotVariant::Mono => "Mono",
            GodotVariant::ExportTemplates => "Export Templates",
        }
    }

    /// 检查是否为 Mono 变体
    #[allow(dead_code)]
    pub fn is_mono(&self) -> bool {
        matches!(self, GodotVariant::Mono)
    }
}

impl std::fmt::Display for GodotVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Default for GodotVariant {
    fn default() -> Self {
        GodotVariant::Standard
    }
}
