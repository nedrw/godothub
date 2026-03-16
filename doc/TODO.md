# Godot Hub - 待办事项

**当前版本**: v0.1.0  
**构建状态**: 可编译运行，核心下载/管理功能可用，P0 缺陷已全部修复，编译零 warning

---

## 🔥 P0 - 必须修复的缺陷

### 功能错误

- [x] **macOS 可执行文件查找失败** ✅ 已修复（`state/app_state_impl.rs`）  
  `find_godot_executable()` 新增 `#[cfg(target_os = "macos")]` 分支，优先查找 `.app`
  目录包并直接返回其路径（供 `open` 命令启动）；Unix 通用回退逻辑追加
  `metadata.is_file()` 检查，避免目录被误匹配为可执行文件。

- [x] **下载数量统计错误** ✅ 已修复（`ui/download_dialog.rs`）  
  `draw_download_queue_status` 及 `draw_download_dialog_content` 的可见性判断均改为
  过滤 `_error`、`_extracting`、`_complete` 后缀 key 后再计数；"Cancel All" 按钮也
  仅对活跃 base key 调用 `cancel_download`，避免重复操作伴生 key。

- [x] **搜索栏无效** ✅ 已修复（`ui/download_dialog.rs` + `state/app_state.rs`）  
  在 `AppState` 中新增 `download_search_text: String`（`#[serde(skip)]`，帧间持久化）；
  `draw_search_bar` 函数签名改为接收 `state: &mut AppState`，`TextEdit` 绑定到
  `state.download_search_text`，彻底解决每帧重置问题。  
  `draw_version_groups` 中增加实时过滤，同时匹配**版本号**和**变体名称**（大小写不敏感）：
  - 输入 `"4.3"` → 过滤版本号
  - 输入 `"mono"` → 仅显示 Mono 变体
  - 输入 `"standard"` → 仅显示标准版  
  空查询显示全部；无数据时显示 Retry 按钮；有数据但无匹配时显示 `"No results for '...'"` + **✕ Clear Search** 按钮。

- [x] **`validate_godot_executable` macOS 误报** ✅ 已修复（`services/launcher.rs`）  
  原实现仅检查 `is_file()`，macOS `.app` bundle 是目录，导致验证失败。  
  现改为 `exec_path.is_file() || exec_path.is_dir()`，兼容所有平台。

---

## 🌟 P1 - 核心功能缺失

### 状态持久化

- [ ] **收藏和使用时间不持久化**  
  `GodotInstall.is_favorite` 和 `last_used` 重启后丢失。  
  需要新增 `~/.gdhub/installed.json` 存储这两个字段，启动时与扫描结果合并。

### 项目管理

- [ ] **`parse_godot_version()` 是硬编码占位**（`projects_panel.rs`）  
  始终返回 `"4.x"`。需要解析 `project.godot` 文件中的 `config_version` 字段，  
  并映射到实际 Godot 版本号。

- [ ] **"Open" 项目按钮无功能**（`draw_project_item()`）  
  点击后仅打印日志。需要找到匹配的已安装 Godot 版本并以项目路径为参数启动。

- [ ] **"New Project" 按钮无功能**（`draw_action_buttons()`）  
  需要：弹出对话框 → 选择 Godot 版本 → 选择目录 → 初始化 `project.godot`。

- [ ] **"Import Project" 按钮无功能**（`draw_action_buttons()`）  
  需要：弹出文件夹选择对话框 → 验证 `project.godot` 存在 → 加入项目列表。

- [ ] **项目收藏/删除无功能**（`draw_project_menu()`）  
  "Toggle Favorite" 和 "Remove" 均只打印日志，无实际逻辑。

### 设置面板

- [ ] **GitHub / Website 按钮无功能**（`draw_about_section()`）  
  点击后仅打印日志。需要调用系统浏览器打开对应 URL。

- [ ] **`check_updates_on_start` 无实现**  
  配置项 UI 已有，但应用启动时没有对应的版本检查逻辑调用。

---

## 💡 P2 - 体验优化

### 代码质量

- [ ] **`open_folder` 三处重复定义**  
  `versions_panel.rs`、`projects_panel.rs`、`settings_panel.rs` 各自实现了相同函数。  
  提取至 `utils/file_utils.rs` 并 re-export。

- [ ] **`region.rs` 死代码**  
  `should_use_china_mirror()` 及相关函数实现完整但从未被调用。  
  选项一：接入自动检测流程（启动时自动设置 `DownloadSource`）。  
  选项二：确认不需要则删除该模块。

- [ ] **`style.rs` 空模块 `pub mod colors {}`**  
  注释称"向后兼容"，实际为空且无任何引用，直接删除。

- [ ] **`DownloadSource::mirror_prefix()` 和 `api_proxy_url()` 无效接口**  
  两个方法均返回空值，实际 URL 构建逻辑分散在 `get_api_url()` 和 `convert_to_mirror_url()` 中。  
  删除这两个无用方法，统一到构建函数中。

- [ ] **无用公开函数清理**（`download_dialog.rs`）  
  `draw_download_details()`、`initiate_download()`、`get_download_stats()` 已公开但未被调用。  
  评估后决定内化或删除。

### 功能完善

- [ ] **`Theme::System` 系统主题检测**  
  目前回退为深色主题。需要根据平台 API 检测系统深/浅色偏好：  
  macOS 使用 `defaults read -g AppleInterfaceStyle`，  
  Windows 使用注册表 `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`。

- [ ] **下载对话框搜索/筛选功能**  
  修复搜索框后，实现按版本号和变体类型（Standard/Mono）筛选版本列表。

- [ ] **键盘快捷键**  
  `Ctrl+R`：刷新版本列表；`Ctrl+,`：打开设置；`Esc`：关闭对话框。

---

## 🐛 已知问题（待确认）

- [ ] 某些情况下版本列表加载后不刷新安装状态（可能与 `sync_download_progress` 时序有关）
- [ ] 自定义镜像 URL 格式校验不完整（仅检查 `http://` / `https://` 前缀）
- [ ] 大量版本时（100+）滚动区域渲染性能未验证

---

## 📊 当前功能状态

| 模块 | 状态 | 说明 |
|------|------|------|
| 版本下载 | ✅ 完整 | 流式下载、进度、取消、重试、解压 |
| 版本安装管理 | ✅ 完整 | 扫描、删除（含确认对话框）、启动 |
| macOS 启动 | ✅ 已修复 | `.app` bundle 检测修复，`open` 命令可正常启动；`validate_godot_executable` 兼容目录路径 |
| GitHub API | ✅ 完整 | 拉取 releases、平台匹配、镜像回退 |
| 自定义镜像 | ✅ 可用 | 用户填写 URL，自动代理 API 和下载 |
| 下载队列计数 | ✅ 已修复 | 过滤特殊 key，数量显示正确；Cancel All 逻辑修正 |
| 搜索栏 | ✅ 已修复 | 输入持久化 + 版本号/变体名称实时过滤均已实现；Filter 按钮下拉筛选待实现（P2） |
| 主题切换 | 🟡 部分 | Dark/Light 正常，System 未实现 |
| 项目管理 | 🔴 占位 | 扫描可用，其余操作均为 TODO |
| 收藏/使用时间 | 🔴 缺失 | 内存状态，重启丢失 |
| 更新检查 | 🔴 缺失 | 配置开关存在，逻辑未实现 |

---

## 🧹 已完成的代码清理（warning 归零）

本次清理将编译 warning 从 51 个降至 0：

| 类型 | 处理方式 | 涉及位置 |
|------|---------|---------|
| **删除死代码** | 彻底移除 | `utils/region.rs`（整个模块）、`style.rs` 空 `colors` 模块及 `BADGE_BLUE` 常量、`github_api.rs` 两个 wrapper 函数、`download_dialog.rs` 四个未接入函数、`projects_panel.rs` `create_sample_projects()` |
| **修复机械错误** | 直接修正 | 4 处未使用 import（`Ordering`、`Mutex`、`download_state`、`fetch_all_versions_with_source`）、`mut archive` 去掉多余 `mut`、`extract_callback` 加 `_` 前缀 |
| **未来 API 保留** | `#[allow(dead_code)]` | 模型层方法（`GodotInstall`/`GodotVersion`/`GodotVariant`）、工具层（`file_utils.rs` 模块级）、服务接口（`launcher.rs`、`download_state` 子项）、配置方法（`AppConfig`/`DownloadSource`/`Theme`）、状态方法（`AppState` impl 中 6 个工具方法）、UI 辅助（`style.rs` 未使用常量/函数，`settings_panel.rs` 2 个工具函数） |

---

## 📌 符号说明

- ✅ 完整实现  
- 🟡 部分实现  
- 🔴 未实现 / 占位  
- [ ] 待办  
- [x] 已完成