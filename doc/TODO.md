# Godot Hub - 待办事项

**当前版本**: v0.1.0  
**构建状态**: 可编译运行，核心下载/管理功能可用，P0/P1（持久化 + 设置面板）/P2 缺陷已全部修复，编译零 warning

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

- [x] **收藏和使用时间不持久化** ✅ 已修复（`state/install_meta.rs`）  
  新增 `InstallMetaStore` 结构体，将 `is_favorite` 和 `last_used` 持久化到  
  `~/.gdhub/installed.json`（与安装目录同级，便于统一备份）。  
  - **加载**：`load_installed_versions()` 扫描磁盘后自动合并元数据文件  
  - **写入**：切换收藏（`draw_version_menu`）、启动版本（Run 按钮成功后调用  
    `mark_used()`）、删除版本（`remove_installed_version` 内部）三处均触发  
    `AppState::save_install_meta()`  
  - **原子写入**：先写 `.json.tmp` 临时文件，再 rename 覆盖，防止写入中途崩溃  
    导致文件损坏  
  - **容错**：文件缺失或 JSON 解析失败时均静默返回空存储，不影响正常启动  
  - **单元测试**：6 项测试覆盖 key 格式、增删查、序列化往返、空文件加载

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

- [x] **GitHub / Website 按钮无功能** ✅ 已修复（`ui/settings_panel.rs` + `utils/file_utils.rs`）  
  新增 `pub fn open_url(url: &str)` 至 `utils/file_utils.rs`（跨平台：macOS 用 `open`，Linux 用 `xdg-open`，Windows 用 `cmd /c start`），在 `utils/mod.rs` 中 `pub use file_utils::open_url` re-export；  
  `draw_about_section()` 中 GitHub 按钮调用 `open_url("https://github.com/gdHub/gdhub")`，Website 按钮调用 `open_url("https://github.com/gdHub/gdhub#readme")`，TODO 注释与 `log::info!` 占位逻辑一并移除。

- [x] **`check_updates_on_start` 无实现** ✅ 已修复（`main.rs`）  
  `GodotHubApp::default()` 中将原先无条件调用的 `app_state.refresh_available_versions()` 改为受 `app_state.config.check_updates_on_start` 控制：  
  - 默认值 `true`：启动时立即联网拉取可用版本列表，行为与修复前一致；  
  - 用户手动关闭后：跳过自动刷新，下载对话框初始为空，用户可在对话框内手动点击 Retry 触发刷新。

---

## 💡 P2 - 体验优化

### 代码质量

- [x] **`open_folder` 三处重复定义** ✅ 已修复（`utils/file_utils.rs` + `utils/mod.rs`）  
  新增 `pub fn open_folder(path: &Path)` 至 `utils/file_utils.rs`，在 `utils/mod.rs` 中  
  `pub use file_utils::open_folder` re-export；三个面板文件改为 `use crate::utils::open_folder`，  
  各自的局部 `fn open_folder` 定义全部删除。

- [x] **`region.rs` 死代码** ✅ 已删除（上一轮 warning 清零时移除整个模块）

- [x] **`style.rs` 空模块 `pub mod colors {}`** ✅ 已删除（上一轮 warning 清零时移除）

- [x] **`DownloadSource::mirror_prefix()` 和 `api_proxy_url()` 无效接口** ✅ 已修复（`state/app_config.rs`）  
  两个始终返回空值的方法已从 `DownloadSource` impl 中删除；`full_api_url()` 直接返回  
  `format!("https://api.github.com{}", path)`，不再间接调用 `api_proxy_url()`；  
  `github_api.rs` 中的测试 `test_download_source` 同步更新，改为断言 `needs_proxy()` 和 `is_custom()`。

- [x] **无用公开函数清理**（`download_dialog.rs`）✅ 已删除（上一轮 warning 清零时移除）  
  `draw_download_details()`、`initiate_download()`、`cancel_download()`、`get_download_stats()` 均已移除。

### 功能完善

- [x] **`Theme::System` 系统主题检测** ✅ 已实现（`ui/style.rs`）  
  新增 `pub fn detect_system_dark_mode() -> bool`，采用**定时轮询**策略动态响应系统主题切换：  
  - 缓存层：两个静态原子变量 `CACHED: AtomicBool`（深色标志）+  
    `LAST_CHECK: AtomicU64`（上次检测时间戳），读写均用 `Relaxed` 序，无锁线程安全  
  - 轮询间隔：`DARK_MODE_POLL_INTERVAL_SECS = 30` 秒；初始值 `LAST_CHECK = 0`  
    保证首次调用必触发检测  
  - 检测方式：macOS 调用 `defaults read -g AppleInterfaceStyle`，  
    Windows 读取注册表 `AppsUseLightTheme`（需 `winreg` crate），  
    Linux 调用 `gsettings get org.gnome.desktop.interface color-scheme`  
  - `ThemeColors::from_theme(Theme::System)` 和 `setup_visuals()` 均已接入该函数；  
    由于 `update()` 每 100 ms 重绘一次，每隔 30 秒会自动触发一次实际系统调用

- [x] **下载对话框搜索/筛选功能** ✅ 已在 P0 阶段完成（版本号 + 变体名称实时过滤）

- [x] **键盘快捷键** ✅ 已实现（`main.rs` `GodotHubApp::update()`）  
  在每帧 `update()` 中通过 `ctx.input()` 捕获：  
  - `Ctrl+R`：调用 `refresh_available_versions()`  
  - `Ctrl+,`：切换到 Settings 标签页  
  - `Esc`：关闭下载对话框（`show_download_dialog = false`）

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
| 主题切换 | ✅ 完整 | Dark/Light/System 均已实现；System 通过 `detect_system_dark_mode()` 每 30 秒轮询，动态响应系统主题切换 |
| 项目管理 | 🔴 占位 | 扫描可用，其余操作均为 TODO |
| 收藏/使用时间 | ✅ 已修复 | `~/.gdhub/installed.json` 持久化；切换收藏/启动/删除均触发写盘；原子写入防损坏 |
| 更新检查 | ✅ 已修复 | `check_updates_on_start=true`（默认）时启动自动刷新；`false` 时跳过，用户可在下载对话框手动 Retry |
| 设置面板链接 | ✅ 已修复 | GitHub/Website 按钮调用 `utils::open_url()`，跨平台打开系统默认浏览器 |

---

## 🧹 已完成的代码清理（warning 归零）

本次清理将编译 warning 从 51 个降至 0：

| 类型 | 处理方式 | 涉及位置 |
|------|---------|---------|
| **删除死代码** | 彻底移除 | `utils/region.rs`（整个模块）、`style.rs` 空 `colors {}` 模块及 `BADGE_BLUE` 常量、`github_api.rs` 两个 wrapper 函数、`download_dialog.rs` 四个未接入函数、`projects_panel.rs` `create_sample_projects()`、`DownloadSource::mirror_prefix()` / `api_proxy_url()`（P2 阶段）、三处重复的局部 `fn open_folder`（P2 阶段）  |
| **新增持久化层** | `state/install_meta.rs` | `InstallMetaStore`（`HashMap<String, InstallMeta>`）+ 原子写盘 + 容错加载；`app_state_impl.rs` 接入加载/保存/删除三处调用点；`versions_panel.rs` 接入收藏切换和 Run 按钮 |
| **修复机械错误** | 直接修正 | 4 处未使用 import（`Ordering`、`Mutex`、`download_state`、`fetch_all_versions_with_source`）、`mut archive` 去掉多余 `mut`、`extract_callback` 加 `_` 前缀 |
| **未来 API 保留** | `#[allow(dead_code)]` | 模型层方法（`GodotInstall`/`GodotVersion`/`GodotVariant`）、工具层（`file_utils.rs` 模块级）、服务接口（`launcher.rs`、`download_state` 子项）、配置方法（`AppConfig`/`DownloadSource`/`Theme`）、状态方法（`AppState` impl 中 6 个工具方法）、UI 辅助（`style.rs` 未使用常量/函数，`settings_panel.rs` 2 个工具函数） |

---

## 📌 符号说明

- ✅ 完整实现  
- 🟡 部分实现  
- 🔴 未实现 / 占位  
- [ ] 待办  
- [x] 已完成