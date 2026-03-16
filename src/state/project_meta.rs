// ProjectMetaStore - 项目列表的持久化元数据
//
// 职责：在磁盘上保存用户对项目的操作状态（收藏、隐藏、最后打开时间）以及
//      用户手动导入的项目路径（不在 projects_dir 目录下的项目）。
//
// 存储路径：~/.gdhub/projects.json
// 格式    ：JSON（serde_json pretty-print，便于人工查看与编辑）

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// 数据结构
// ============================================================================

/// 单个项目的用户状态元数据。
///
/// 仅保存**用户主动操作产生的状态**，不重复存储可从磁盘扫描还原的字段
/// （项目路径、名称等均由扫描填充）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    /// 是否被用户收藏
    pub is_favorite: bool,
    /// 是否被用户从列表中移除（隐藏）
    pub is_hidden: bool,
    /// 最后一次通过本应用打开的时间（UTC）
    #[serde(default)]
    pub last_opened: Option<DateTime<Utc>>,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            is_favorite: false,
            is_hidden: false,
            last_opened: None,
        }
    }
}

/// 所有项目元数据的持久化存储。
///
/// # 文件路径
/// `~/.gdhub/projects.json`
///
/// # 数据生命周期
/// - **加载**：`AppState::default()` 时调用 `load()`。
/// - **写入**：切换收藏、隐藏项目、打开项目、导入项目时调用 `save()`。
/// - **容错**：文件缺失或解析失败时均返回空存储并记录警告，不影响正常启动。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMetaStore {
    /// 按项目路径（规范化字符串）索引的元数据映射。
    #[serde(default)]
    pub entries: HashMap<String, ProjectMeta>,

    /// 用户手动导入的项目路径列表（不在 `projects_dir` 目录下的项目）。
    #[serde(default)]
    pub imported_paths: Vec<PathBuf>,
}

// ============================================================================
// 实现
// ============================================================================

impl ProjectMetaStore {
    // ── 持久化 ────────────────────────────────────────────────────────────────

    /// 获取元数据文件的存储路径：`~/.gdhub/projects.json`。
    pub fn file_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".gdhub").join("projects.json"))
    }

    /// 从磁盘加载元数据存储。
    ///
    /// 以下情况均返回空存储：
    /// - 文件不存在（正常首次运行）
    /// - 文件读取失败
    /// - JSON 解析失败
    pub fn load() -> Self {
        let path = match Self::file_path() {
            Some(p) => p,
            None => {
                log::warn!("Cannot determine project metadata file path");
                return Self::default();
            }
        };

        if !path.exists() {
            return Self::default();
        }

        match std::fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
                log::warn!(
                    "Failed to parse project metadata ({}), using defaults: {}",
                    path.display(),
                    e
                );
                Self::default()
            }),
            Err(e) => {
                log::warn!(
                    "Failed to read project metadata ({}): {}",
                    path.display(),
                    e
                );
                Self::default()
            }
        }
    }

    /// 将元数据序列化并写入磁盘（原子写入）。
    pub fn save(&self) -> std::io::Result<()> {
        let path = match Self::file_path() {
            Some(p) => p,
            None => {
                log::warn!("Cannot determine project metadata file path, skipping save");
                return Ok(());
            }
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // 原子写入：先写临时文件，再重命名覆盖
        let tmp_path = path.with_extension("json.tmp");
        std::fs::write(&tmp_path, &content)?;
        std::fs::rename(&tmp_path, &path)?;

        log::debug!(
            "Project metadata saved ({} entries, {} imported) → {}",
            self.entries.len(),
            self.imported_paths.len(),
            path.display()
        );
        Ok(())
    }

    /// 静默保存（失败时仅记录日志，不向上传播错误）。
    pub fn save_quiet(&self) {
        if let Err(e) = self.save() {
            log::error!("Failed to save project metadata: {}", e);
        }
    }

    // ── 内部工具 ──────────────────────────────────────────────────────────────

    /// 将路径转换为 HashMap 键（使用字符串表示）。
    fn key_for(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }

    // ── 查询 ──────────────────────────────────────────────────────────────────

    /// 查询指定项目的元数据，不存在时返回 `None`。
    pub fn get(&self, path: &Path) -> Option<&ProjectMeta> {
        self.entries.get(&Self::key_for(path))
    }

    /// 查询指定项目的元数据，不存在时返回默认值的克隆。
    #[allow(dead_code)]
    pub fn get_or_default(&self, path: &Path) -> ProjectMeta {
        self.entries
            .get(&Self::key_for(path))
            .cloned()
            .unwrap_or_default()
    }

    /// 判断指定项目是否被收藏。
    pub fn is_favorite(&self, path: &Path) -> bool {
        self.get(path).map_or(false, |m| m.is_favorite)
    }

    /// 判断指定项目是否被隐藏（用户从列表中移除）。
    pub fn is_hidden(&self, path: &Path) -> bool {
        self.get(path).map_or(false, |m| m.is_hidden)
    }

    /// 获取指定项目的最后打开时间。
    #[allow(dead_code)]
    pub fn last_opened(&self, path: &Path) -> Option<DateTime<Utc>> {
        self.get(path).and_then(|m| m.last_opened)
    }

    // ── 写入 ──────────────────────────────────────────────────────────────────

    /// 写入或覆盖指定项目的元数据。
    #[allow(dead_code)]
    pub fn set(&mut self, path: &Path, meta: ProjectMeta) {
        self.entries.insert(Self::key_for(path), meta);
    }

    /// 切换指定项目的收藏状态。
    pub fn toggle_favorite(&mut self, path: &Path) {
        let key = Self::key_for(path);
        let meta = self.entries.entry(key).or_default();
        meta.is_favorite = !meta.is_favorite;
    }

    /// 将指定项目标记为隐藏（从列表中移除）。
    pub fn hide(&mut self, path: &Path) {
        let key = Self::key_for(path);
        let meta = self.entries.entry(key).or_default();
        meta.is_hidden = true;
    }

    /// 取消隐藏指定项目（恢复显示）。
    #[allow(dead_code)]
    pub fn unhide(&mut self, path: &Path) {
        let key = Self::key_for(path);
        let meta = self.entries.entry(key).or_default();
        meta.is_hidden = false;
    }

    /// 更新指定项目的最后打开时间为当前 UTC 时间。
    pub fn update_last_opened(&mut self, path: &Path) {
        let key = Self::key_for(path);
        let meta = self.entries.entry(key).or_default();
        meta.last_opened = Some(Utc::now());
    }

    // ── 导入路径管理 ─────────────────────────────────────────────────────────

    /// 添加一个手动导入的项目路径（已存在则跳过）。
    pub fn add_imported_path(&mut self, path: PathBuf) {
        if !self.imported_paths.contains(&path) {
            self.imported_paths.push(path);
        }
    }

    /// 移除一个手动导入的项目路径。
    #[allow(dead_code)]
    pub fn remove_imported_path(&mut self, path: &Path) {
        self.imported_paths.retain(|p| p != path);
    }

    /// 返回当前导入路径的数量。
    #[allow(dead_code)]
    pub fn imported_count(&self) -> usize {
        self.imported_paths.len()
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_path() -> PathBuf {
        PathBuf::from("/home/user/projects/my_game")
    }

    #[test]
    fn test_default_meta() {
        let meta = ProjectMeta::default();
        assert!(!meta.is_favorite);
        assert!(!meta.is_hidden);
        assert!(meta.last_opened.is_none());
    }

    #[test]
    fn test_toggle_favorite() {
        let mut store = ProjectMetaStore::default();
        let path = sample_path();

        assert!(!store.is_favorite(&path));
        store.toggle_favorite(&path);
        assert!(store.is_favorite(&path));
        store.toggle_favorite(&path);
        assert!(!store.is_favorite(&path));
    }

    #[test]
    fn test_hide_unhide() {
        let mut store = ProjectMetaStore::default();
        let path = sample_path();

        assert!(!store.is_hidden(&path));
        store.hide(&path);
        assert!(store.is_hidden(&path));
        store.unhide(&path);
        assert!(!store.is_hidden(&path));
    }

    #[test]
    fn test_update_last_opened() {
        let mut store = ProjectMetaStore::default();
        let path = sample_path();

        assert!(store.last_opened(&path).is_none());
        store.update_last_opened(&path);
        assert!(store.last_opened(&path).is_some());
    }

    #[test]
    fn test_add_imported_path_dedup() {
        let mut store = ProjectMetaStore::default();
        let path = PathBuf::from("/external/projects/game");

        store.add_imported_path(path.clone());
        store.add_imported_path(path.clone());
        assert_eq!(store.imported_paths.len(), 1);
    }

    #[test]
    fn test_remove_imported_path() {
        let mut store = ProjectMetaStore::default();
        let path = PathBuf::from("/external/projects/game");

        store.add_imported_path(path.clone());
        assert_eq!(store.imported_count(), 1);

        store.remove_imported_path(&path);
        assert_eq!(store.imported_count(), 0);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut store = ProjectMetaStore::default();
        let path = sample_path();

        store.toggle_favorite(&path);
        store.update_last_opened(&path);
        store.add_imported_path(PathBuf::from("/external/game"));

        let json = serde_json::to_string(&store).unwrap();
        let restored: ProjectMetaStore = serde_json::from_str(&json).unwrap();

        assert!(restored.is_favorite(&path));
        assert!(!restored.is_hidden(&path));
        assert!(restored.last_opened(&path).is_some());
        assert_eq!(restored.imported_paths.len(), 1);
    }
}
