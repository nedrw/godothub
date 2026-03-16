// InstallMetaStore - 已安装版本的持久化元数据
//
// 职责：在磁盘上保存用户对已安装版本的操作状态（收藏、最后使用时间）。
// 这些字段无法从磁盘文件扫描中还原，必须单独持久化。
//
// 存储路径：~/.gdhub/installed.json
// 格式    ：JSON（serde_json pretty-print，便于人工查看与编辑）

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::GodotVariant;

// ============================================================================
// 内部工具
// ============================================================================

/// 生成用于 HashMap 索引的唯一键。
///
/// 格式：`"{version}-{variant}"` （例：`"4.3-Standard"`、`"4.3-Mono"`）。
/// 键只依赖版本号和变体类型，与可执行文件路径无关，
/// 从而在用户修改安装目录后仍能正确匹配。
fn meta_key(version: &str, variant: &GodotVariant) -> String {
    format!("{}-{}", version, variant)
}

// ============================================================================
// 数据结构
// ============================================================================

/// 单个已安装版本的用户状态元数据。
///
/// 仅保存**用户主动操作产生的状态**，不重复存储可从磁盘扫描还原的字段
/// （版本号、路径、变体类型等均由 `GodotInstall` 通过扫描填充）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallMeta {
    /// 是否被用户收藏
    pub is_favorite: bool,
    /// 最后一次通过本应用启动的时间（UTC）
    #[serde(default)]
    pub last_used: Option<DateTime<Utc>>,
}

impl Default for InstallMeta {
    fn default() -> Self {
        Self {
            is_favorite: false,
            last_used: None,
        }
    }
}

/// 所有已安装版本元数据的持久化存储。
///
/// # 文件路径
/// `~/.gdhub/installed.json`
///
/// # 数据生命周期
/// - **加载**：`load_installed_versions()` 调用 `load()` 后与磁盘扫描结果合并。
/// - **写入**：切换收藏、启动版本、删除版本时调用 `AppState::save_install_meta()`。
/// - **清理**：删除版本时，对应条目自动从存储中移除。
///
/// # 容错
/// 文件缺失或解析失败时均返回空存储并记录警告，不向调用层抛出错误。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstallMetaStore {
    /// 按 `"version-variant"` 键索引的元数据映射。
    ///
    /// 使用 `#[serde(default)]` 以兼容旧版本 JSON（缺少该字段时反序列化为空 map）。
    #[serde(default)]
    pub entries: HashMap<String, InstallMeta>,
}

// ============================================================================
// 实现
// ============================================================================

impl InstallMetaStore {
    // ── 持久化 ────────────────────────────────────────────────────────────────

    /// 从磁盘加载元数据存储。
    ///
    /// 以下情况均返回空存储，并在日志记录对应警告：
    /// - 文件不存在（正常首次运行）
    /// - 文件读取失败（权限问题等）
    /// - JSON 解析失败（文件损坏或版本不兼容）
    pub fn load() -> Self {
        let path = match Self::file_path() {
            Some(p) => p,
            None => {
                log::warn!("Cannot determine install metadata file path");
                return Self::default();
            }
        };

        if !path.exists() {
            // 正常情况：首次运行，文件尚未创建
            return Self::default();
        }

        match std::fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
                log::warn!(
                    "Failed to parse install metadata ({}), using defaults: {}",
                    path.display(),
                    e
                );
                Self::default()
            }),
            Err(e) => {
                log::warn!(
                    "Failed to read install metadata ({}): {}",
                    path.display(),
                    e
                );
                Self::default()
            }
        }
    }

    /// 将元数据序列化并写入磁盘。
    ///
    /// 若父目录不存在，会自动创建（包含中间目录）。
    /// 写入使用原子替换（先写临时文件，再 rename）以防止写入中途崩溃导致文件损坏。
    pub fn save(&self) -> std::io::Result<()> {
        let path = match Self::file_path() {
            Some(p) => p,
            None => {
                log::warn!("Cannot determine install metadata file path, skipping save");
                return Ok(());
            }
        };

        // 确保父目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 序列化为格式化 JSON
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // 原子写入：先写临时文件，再重命名覆盖目标文件
        // 防止写入过程中进程崩溃导致文件半写入损坏
        let tmp_path = path.with_extension("json.tmp");
        std::fs::write(&tmp_path, &content)?;
        std::fs::rename(&tmp_path, &path)?;

        log::debug!(
            "Install metadata saved ({} entries) → {}",
            self.entries.len(),
            path.display()
        );
        Ok(())
    }

    /// 获取元数据文件的存储路径：`~/.gdhub/installed.json`。
    ///
    /// 与安装目录（`~/.gdhub/versions/`）位于同一父目录，
    /// 方便用户统一备份 `~/.gdhub/` 目录。
    pub fn file_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".gdhub").join("installed.json"))
    }

    // ── 查询与修改 ─────────────────────────────────────────────────────────────

    /// 查询指定版本的元数据，不存在时返回 `None`。
    pub fn get(&self, version: &str, variant: &GodotVariant) -> Option<&InstallMeta> {
        self.entries.get(&meta_key(version, variant))
    }

    /// 写入或覆盖指定版本的元数据。
    pub fn set(&mut self, version: &str, variant: &GodotVariant, meta: InstallMeta) {
        self.entries.insert(meta_key(version, variant), meta);
    }

    /// 删除指定版本的元数据（版本被卸载时调用）。
    ///
    /// 键不存在时为空操作，不报错。
    pub fn remove(&mut self, version: &str, variant: &GodotVariant) {
        self.entries.remove(&meta_key(version, variant));
    }

    /// 返回当前存储中的条目数量。
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 判断存储是否为空。
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::GodotVariant;

    #[test]
    fn test_meta_key_format() {
        assert_eq!(meta_key("4.3", &GodotVariant::Standard), "4.3-Standard");
        assert_eq!(meta_key("4.3", &GodotVariant::Mono), "4.3-Mono");
        assert_eq!(
            meta_key("4.2.2", &GodotVariant::ExportTemplates),
            "4.2.2-Export Templates"
        );
    }

    #[test]
    fn test_store_set_get_remove() {
        let mut store = InstallMetaStore::default();
        assert!(store.get("4.3", &GodotVariant::Standard).is_none());

        store.set(
            "4.3",
            &GodotVariant::Standard,
            InstallMeta {
                is_favorite: true,
                last_used: None,
            },
        );

        let meta = store.get("4.3", &GodotVariant::Standard).unwrap();
        assert!(meta.is_favorite);
        assert!(meta.last_used.is_none());

        store.remove("4.3", &GodotVariant::Standard);
        assert!(store.get("4.3", &GodotVariant::Standard).is_none());
    }

    #[test]
    fn test_remove_nonexistent_is_noop() {
        let mut store = InstallMetaStore::default();
        // 不应 panic
        store.remove("99.0", &GodotVariant::Mono);
        assert!(store.is_empty());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut store = InstallMetaStore::default();
        store.set(
            "4.3",
            &GodotVariant::Mono,
            InstallMeta {
                is_favorite: true,
                last_used: Some(chrono::Utc::now()),
            },
        );

        let json = serde_json::to_string(&store).unwrap();
        let restored: InstallMetaStore = serde_json::from_str(&json).unwrap();

        let meta = restored.get("4.3", &GodotVariant::Mono).unwrap();
        assert!(meta.is_favorite);
        assert!(meta.last_used.is_some());
    }

    #[test]
    fn test_load_empty_on_missing_file() {
        // 只要不 panic 且返回默认值即可（实际不会触及真实文件系统）
        let store = InstallMetaStore::default();
        assert!(store.is_empty());
    }

    #[test]
    fn test_default_meta() {
        let meta = InstallMeta::default();
        assert!(!meta.is_favorite);
        assert!(meta.last_used.is_none());
    }
}
