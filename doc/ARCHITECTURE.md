# Godot Hub 架构文档

## 1. 项目概述

Godot Hub 是一个用 Rust 编写的跨平台 Godot 引擎管理器。它通过桌面 GUI 帮助开发者管理多个 Godot 引擎版本、浏览本地 Godot 项目，并提供版本下载和启动功能。

### 核心功能

- 从 GitHub Releases API 获取并展示可用的 Godot 引擎版本
- 下载、安装和删除 Godot 引擎版本（含 Standard / Mono 两种变体）
- 实时下载进度显示、取消和失败重试
- 扫描本地项目目录，展示 Godot 项目列表
- 可配置的下载源（GitHub 官方 / 自定义镜像）
- 深色 / 浅色主题切换

### 当前版本

`v0.1.0`（功能可用，部分模块仍有占位实现，见第 10 节）

---

## 2. 技术栈

| 依赖 | 版本 | 用途 |
|------|------|------|
| `eframe` | 0.31 | 跨平台原生窗口框架 |
| `egui` | 0.31 | 即时模式 GUI 渲染 |
| `tokio` | 1 (full) | 异步运行时 |
| `reqwest` | 0.12 | HTTP 客户端（流式下载、JSON） |
| `serde` / `serde_json` | 1.0 | 序列化（配置持久化） |
| `zip` | 2.0 | ZIP 解压 |
| `dirs` | 6.0 | 跨平台目录路径获取 |
| `rfd` | 0.15 | 原生文件选择对话框 |
| `chrono` | 0.4 | 时间处理 |
| `anyhow` / `thiserror` | 1.0 / 2.0 | 错误处理 |
| `futures` | 0.3 | 异步流工具 |
| `log` / `env_logger` | 0.4 / 0.11 | 日志系统 |
| `winreg` (Windows only) | 0.55 | Windows 注册表（平台相关） |

---

## 3. 目录结构

> `state/install_meta.rs` 为本轮新增文件，负责已安装版本用户状态的持久化。


```
src/
├── main.rs              # 入口点、GodotHubApp、eframe App 实现
├── models/              # 数据模型
│   ├── godot_variant.rs # GodotVariant 枚举
│   ├── godot_version.rs # GodotVersion 结构体（可用版本）
│   ├── godot_install.rs # GodotInstall 结构体（已安装版本）
│   └── mod.rs
├── state/               # 应用状态管理
│   ├── app_config.rs    # AppConfig、Theme、DownloadSource
│   ├── app_state.rs     # AppState 结构体定义
│   ├── app_state_impl.rs# AppState 方法实现
│   └── mod.rs
├── services/            # 业务逻辑服务
│   ├── github_api.rs    # GitHub API 客户端，版本解析
│   ├── download.rs      # 文件下载、ZIP 解压、取消机制
│   ├── launcher.rs      # Godot 进程启动（跨平台）
│   └── mod.rs
├── ui/                  # 用户界面组件
│   ├── style.rs         # 主题颜色系统、通用 UI 组件
│   ├── sidebar.rs       # 侧边栏（导航 + 统计）
│   ├── versions_panel.rs# 版本管理面板
│   ├── projects_panel.rs# 项目管理面板
│   ├── settings_panel.rs# 设置面板
│   ├── download_dialog.rs# 下载对话框
│   └── mod.rs
└── utils/               # 工具函数
    ├── file_utils.rs    # 文件路径、大小格式化等工具
    ├── region.rs        # 系统地区/时区检测
    └── mod.rs
```

---

## 4. 数据模型

### 4.1 GodotVariant

```rust
pub enum GodotVariant {
    Standard,         // 标准版（不含 Mono）
    Mono,             // 含 C# 支持的版本
    ExportTemplates,  // 导出模板（当前未通过下载流程处理）
}
```

### 4.2 GodotVersion

表示从 GitHub 获取的可用版本（尚未安装）。

```rust
pub struct GodotVersion {
    pub version: String,             // "4.3"、"4.2.2" 等
    pub variant: GodotVariant,
    pub platform: String,            // "Linux64"、"macOS (ARM)" 等
    pub download_url: String,        // 下载直链（可能经镜像转换）
    pub release_date: String,        // "YYYY-MM-DD"
    pub is_installed: bool,
    pub install_path: Option<PathBuf>,
}
```

### 4.3 GodotInstall

表示已安装到本地的 Godot 实例。

```rust
pub struct GodotInstall {
    pub version: String,
    pub variant: GodotVariant,
    pub path: PathBuf,              // 可执行文件路径
    pub is_favorite: bool,
    pub last_used: Option<DateTime<Utc>>,
}
```

> **注意**：`is_favorite` 和 `last_used` 字段目前**不持久化**。
> 应用每次启动时通过扫描安装目录重新构建 `GodotInstall` 列表，
> 这些字段会被重置为默认值。

### 4.4 AppConfig

```rust
pub struct AppConfig {
    pub install_dir: PathBuf,           // Godot 版本安装目录，默认 ~/.gdhub/versions
    pub projects_dir: PathBuf,          // 项目扫描目录，默认 ~/Godot
    pub check_updates_on_start: bool,   // 启动时检查更新（UI 开关，无实际逻辑）
    pub theme: Theme,                   // Dark | Light | System
    pub download_source: DownloadSource,// GitHub | Custom
    pub custom_mirror_url: String,      // 自定义镜像 URL（仅 Custom 模式有效）
}
```

配置文件路径（JSON 格式）：
- macOS / Linux：`~/.config/gdhub/config.json`
- Windows：`%APPDATA%\gdhub\config.json`

### 4.5 DownloadSource

```rust
pub enum DownloadSource {
    GitHub,  // 直连 GitHub 官方 API 和下载地址
    Custom,  // 通过用户填写的镜像 URL 代理请求
}
```

Custom 模式下：
- API 请求格式：`{custom_mirror_url}/https://api.github.com{path}`
- 文件下载格式：`{custom_mirror_url}/{original_github_url_without_https://}`

### 4.6 AppState

```rust
pub struct AppState {
    pub installed_versions: Vec<GodotInstall>,
    pub available_versions: Vec<GodotVersion>,
    pub downloads_in_progress: HashMap<String, f32>, // 版本 key -> 进度/状态值
    pub selected_version_index: Option<usize>,
    pub show_download_dialog: bool,
    pub current_tab: MainTab,
    pub config: AppConfig,
    // 以下字段不序列化
    pub runtime: Option<Arc<Runtime>>,
    pub version_refresh_state: VersionRefreshState,
    pub refresh_receiver: Option<mpsc::Receiver<RefreshResult>>,
    pub shared_state: Option<Arc<Mutex<AppState>>>, // 供异步任务写入进度
    pub delete_confirm: Option<DeleteConfirmState>,
    pub cancellation_tokens: HashMap<String, Arc<AtomicBool>>,
    pub download_search_text: String,               // 下载对话框搜索文本（帧间持久化，#[serde(skip)]）
}
```

---

## 5. 下载状态系统

`downloads_in_progress` HashMap 使用特殊的 key/value 约定来表达多种状态，
由 `services::download_state` 模块统一管理。

### Key 命名规则

| Key 格式 | 示例 | 含义 |
|---------|------|------|
| `{version}` | `4.3` | 正常下载中（值为 0.0~1.0 进度） |
| `{version}-mono` | `4.3-mono` | Mono 版本正常下载中 |
| `{version}_error` | `4.3_error` | 下载失败（值为 -1.0） |
| `{version}_extracting` | `4.3_extracting` | 正在解压（值为 -2.0） |
| `{version}_complete` | `4.3_complete` | 安装完成（值为 -3.0，短暂存在后被清除） |

### 状态流转

```
初始化 (0.0)
    ↓ 下载中 (0.0 → 1.0)
    ↓ 解压中 (_extracting = -2.0)
    ↓ 完成 (_complete = -3.0)  → 由 sync_download_progress 清除
    ↓ 已安装（从 available_versions 中反映）

    ← 失败 (_error = -1.0)  → 用户可 Retry 或 Remove
    ← 取消（所有相关 key 被清除）
```

---

## 6. 异步架构

### 线程模型

UI 线程运行在主线程（eframe 要求），下载/API 请求在 Tokio 运行时的工作线程上执行。

### 进度同步机制

1. `AppState::shared_state` 是一个 `Arc<Mutex<AppState>>` 的克隆副本（不含 runtime），专门用于异步任务写入进度。
2. 每帧调用 `AppState::sync_download_progress()`，将 `shared_state` 中的进度合并到主状态。
3. 版本列表刷新使用标准库 `mpsc::channel`，结果通过 `AppState::poll_refresh_result()` 每帧轮询。

### 取消机制

每次下载时创建一个 `Arc<AtomicBool>` 取消令牌，存储在 `cancellation_tokens` 中。
调用 `cancel_download()` 时将该标志设为 `true`，下载和解压过程中定期检查并提前返回。

---

## 7. UI 结构

### 整体布局

```
┌─────────────────────────────────────────────────┐
│  SidePanel::left("sidebar")                     │
│  ┌──────────────┐  ┌────────────────────────┐   │
│  │   Sidebar    │  │   CentralPanel         │   │
│  │  - 导航按钮  │  │   - Versions Panel     │   │
│  │  - 统计信息  │  │   - Projects Panel     │   │
│  │  - 下载按钮  │  │   - Settings Panel     │   │
│  └──────────────┘  │   [Download Dialog]    │   │
│                    └────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

初始窗口尺寸：1000×700，最小 800×500。

### 主题系统

`ui/style.rs` 提供两套颜色方案（`dark_colors` / `light_colors`），通过 `ThemeColors::from_theme()` 按当前主题选取。`Theme::System` 目前回退为深色主题（系统主题检测未实现）。

每帧通过 `ui::setup_visuals(ctx, theme)` 应用到 egui 全局样式。

### 可复用组件（style.rs）

| 函数 | 说明 |
|------|------|
| `badge()` | 带背景色的小标签 |
| `status_pill()` | 圆角胶囊状态标签 |
| `card_frame()` | 卡片容器 Frame |
| `primary_button()` | 主要操作按钮 |
| `secondary_button()` | 次要操作按钮 |
| `danger_button()` | 危险操作按钮（红色） |
| `success_button()` | 成功/运行按钮（绿色） |
| `empty_state()` | 空状态提示组件 |
| `section_header()` | 带图标的区域标题 |
| `panel_header()` | 面板页眉 |
| `path_label()` | 带省略和悬停提示的路径显示 |

---

## 8. 业务流程

### 8.1 启动流程

```
main()
  ├─ env_logger 初始化
  ├─ GodotHubApp::default()
  │    ├─ AppState::default()
  │    │    └─ AppConfig::load()（读取配置文件）
  │    ├─ Tokio Runtime 创建
  │    ├─ AppState::load_installed_versions()（扫描安装目录）
  │    ├─ AppState::create_shared_state()（创建异步共享状态）
  │    └─ AppState::refresh_available_versions()（异步拉取 GitHub 版本列表）
  └─ eframe::run_native() 进入事件循环
```

### 8.2 版本下载流程

```
用户点击 "Download"
  ↓
services::start_download(version, state, runtime)
  ├─ downloads_in_progress 插入初始进度 0.0
  ├─ 创建 CancellationToken
  └─ runtime.spawn(async)
       ├─ download_and_install()
       │    ├─ 创建临时目录和安装目录
       │    ├─ download_file_with_fallback()（流式下载，每块更新进度）
       │    ├─ validate_zip_file()（验证 ZIP 完整性）
       │    ├─ extract_zip()（解压，支持取消检查）
       │    ├─ 删除临时 ZIP 文件
       │    └─ find_executable()（查找解压后的可执行文件）
       └─ 写入 shared_state：
            成功 → 插入 _extracting → 等待 500ms → 插入 _complete，添加到 installed_versions
            失败 → 插入 _error

每帧：sync_download_progress() 合并 shared_state 到主状态
```

### 8.3 版本启动流程

```
用户点击 "Run"
  ↓
services::launch_godot(&install.path)
  ├─ 检查可执行文件是否存在
  └─ 平台分支：
       Windows → cmd /C start "" {path}
       Linux   → Command::new(path).spawn()
       macOS   → open {path}
```

### 8.4 版本列表刷新

```
AppState::refresh_available_versions()
  ├─ 创建 mpsc::channel
  ├─ runtime.spawn(fetch_all_versions_with_source_and_custom())
  │    ├─ GitHubApi::fetch_releases()（GET /repos/godotengine/godot/releases?per_page=50）
  │    │    └─ 若镜像失败自动回退到官方 GitHub API
  │    └─ 对每个 release 解析 Standard + Mono 版本，按版本号降序排列
  └─ 结果通过 channel 发送

每帧：poll_refresh_result() → handle_refresh_result() → update_install_status()
```

### 8.5 版本删除流程

```
用户点击 "Remove"
  ↓
state.delete_confirm = Some(DeleteConfirmState { version_index, version_info })
  ↓ (用户点击确认)
AppState::remove_installed_version(index)
  ├─ 从 installed_versions 中移除记录
  ├─ std::fs::remove_dir_all(install_path.parent())（同步删除文件）
  └─ 更新 available_versions 中对应版本的 is_installed 为 false
```

---

## 9. 配置持久化

### 持久化范围

| 数据 | 是否持久化 | 方式 |
|------|-----------|------|
| `AppConfig`（目录、主题、下载源等） | ✅ 是 | JSON 文件 |
| `installed_versions` | ❌ 否 | 每次启动扫描目录重建 |
| `is_favorite`、`last_used` | ❌ 否 | 内存状态，重启丢失 |
| egui 窗口位置/大小 | ✅ 是 | eframe 内置存储 |

### 配置文件示例

```json
{
  "install_dir": "/Users/user/.gdhub/versions",
  "projects_dir": "/Users/user/Godot",
  "check_updates_on_start": true,
  "theme": "Dark",
  "download_source": "GitHub",
  "custom_mirror_url": ""
}
```

---

## 10. 占位接口与已知缺陷

以下是代码中存在但**尚未实现**的功能接口，已用 `// TODO` 或日志占位，需要后续实现：

### 10.1 Projects 面板（projects_panel.rs）

| 接口 | 位置 | 问题描述 |
|------|------|---------|
| `parse_godot_version()` | `projects_panel.rs` | 始终返回硬编码的 `"4.x"`，未解析 `project.godot` 文件 |
| "New Project" 按钮 | `draw_action_buttons()` | 仅打印日志，无实际功能 |
| "Import Project" 按钮 | `draw_action_buttons()` | 仅打印日志，无实际功能 |
| "Open" 项目按钮 | `draw_project_item()` | 仅打印日志，未调用 Godot 打开项目 |
| "Toggle Favorite" 项目 | `draw_project_menu()` | 仅打印日志，收藏状态不更新 |
| "Remove" 项目 | `draw_project_menu()` | 仅打印日志，无确认对话框和删除逻辑 |
| "Scan" 按钮 | `draw_panel_header()` | 仅打印日志，实际已在 `draw_projects_list()` 中自动扫描 |
| `create_sample_projects()` | `projects_panel.rs` | 死代码，从未被调用 |

### 10.2 Settings 面板（settings_panel.rs）

| 接口 | 位置 | 问题描述 |
|------|------|---------|
| "GitHub" 按钮 | `draw_about_section()` | ✅ **已修复**：调用 `utils::open_url("https://github.com/gdHub/gdhub")`，通过系统默认浏览器打开；跨平台实现见 `utils/file_utils.rs::open_url()` |
| "Website" 按钮 | `draw_about_section()` | ✅ **已修复**：调用 `utils::open_url("https://github.com/gdHub/gdhub#readme")`；与 GitHub 按钮共用同一 `open_url` 工具函数 |
| `check_updates_on_start` | `AppConfig` + `main.rs` | ✅ **已修复**：`GodotHubApp::default()` 中原无条件调用的 `refresh_available_versions()` 改为受配置开关控制；默认 `true` 行为不变，设为 `false` 时跳过自动刷新，用户可在下载对话框手动触发 Retry |

### 10.3 下载对话框（download_dialog.rs）

| 接口 | 位置 | 问题描述 |
|------|------|---------|
| 搜索栏 | `draw_search_bar()` | ✅ **已修复**：搜索文本绑定 `AppState::download_search_text`，帧间持久化；`draw_version_groups` 同步增加版本号实时过滤（大小写不敏感，空查询显示全部） |
| Filter 按钮 | `draw_search_bar()` | 仅渲染按钮，无变体筛选逻辑（P2） |
| "Cancel All" 按钮 | `draw_download_queue_status()` | ✅ **已修复**：数量统计和可见性判断均过滤 `_error`/`_extracting`/`_complete` 后缀 key；Cancel All 仅操作活跃 base key |
| `draw_download_details()` | 公开函数 | ✅ **已删除**：从未在 UI 流程中调用，已移除 |
| `initiate_download()` | 公开函数 | ✅ **已删除**：封装层，从未调用，已移除 |
| `cancel_download()` | 公开函数 | ✅ **已删除**：与 `services::cancel_download` 重复，已移除 |
| `get_download_stats()` | 公开函数 | ✅ **已删除**：从未被调用且逻辑有误，已移除 |

### 10.4 样式模块（style.rs）

| 接口 | 位置 | 问题描述 |
|------|------|---------|
| `Theme::System` | `ThemeColors::from_theme()` | ✅ **已实现（动态轮询）**：新增 `pub fn detect_system_dark_mode() -> bool`，采用**定时轮询**策略：缓存层由两个静态原子变量构成——`CACHED: AtomicBool`（深色标志）+ `LAST_CHECK: AtomicU64`（上次检测时间戳），读写均使用 `Relaxed` 序，无锁线程安全；轮询间隔由常量 `DARK_MODE_POLL_INTERVAL_SECS = 30` 控制，`LAST_CHECK` 初始值为 0，保证首次调用必触发实际检测。检测方式：macOS 调用 `defaults read -g AppleInterfaceStyle`，Windows 读取注册表 `AppsUseLightTheme`（需 `winreg` crate），Linux 调用 `gsettings get org.gnome.desktop.interface color-scheme`。`ThemeColors::from_theme` 和 `setup_visuals` 均已接入；由于 `update()` 每 100 ms 重绘，系统主题切换后最多 30 秒内自动生效。 |
| `pub mod colors {}` | `style.rs` | ✅ **已删除**（上一轮 warning 清零时移除） |

### 10.5 工具模块（utils/file_utils.rs）

| 接口 | 位置 | 问题描述 |
|------|------|---------|
| `should_use_china_mirror()` | `region.rs` | ✅ **已删除**（上一轮 warning 清零时移除整个模块） |
| `open_url(url: &str)` | `utils/file_utils.rs` | ✅ **已新增**：跨平台 URL 打开工具函数；macOS 用 `open`，Linux 用 `xdg-open`，Windows 用 `cmd /c start`；在 `utils/mod.rs` 中 `pub use file_utils::open_url` re-export，供设置面板 GitHub/Website 按钮调用 |

### 10.6 状态持久化

| 问题 | 描述 |
|------|------|
| `is_favorite` 不持久化 | ✅ **已修复**：通过 `InstallMetaStore` 持久化到 `~/.gdhub/installed.json`；`load_installed_versions()` 在磁盘扫描后自动合并元数据文件；`draw_version_menu` 切换收藏时调用 `save_install_meta()`。 |
| `last_used` 不持久化 | ✅ **已修复**：Run 按钮启动成功后调用 `install.mark_used()` 更新时间戳，随即调用 `save_install_meta()` 写盘。 |
| 删除版本后元数据残留 | ✅ **已处理**：`remove_installed_version()` 末尾调用 `save_install_meta()`，以当前（已移除该项的）`installed_versions` 覆盖写入，自动清理已删版本的元数据条目。 |
| `AppState` 有 `Serialize`/`Deserialize` | 仍保留（未来可扩展），但实际整体序列化/恢复未实现；当前仅 `AppConfig` 和 `InstallMetaStore` 分别独立序列化。 |

#### InstallMetaStore 设计要点

- **存储路径**：`~/.gdhub/installed.json`，与安装目录（`~/.gdhub/versions/`）同级，便于统一备份。
- **数据结构**：`HashMap<String, InstallMeta>`，键格式为 `"version-variant"`（如 `"4.3-Standard"`），与路径无关，在用户修改安装目录后仍可正确匹配。
- **原子写入**：先写 `.json.tmp` 临时文件，再 `rename` 覆盖目标文件，防止写入中途崩溃导致文件损坏。
- **容错加载**：文件缺失（正常首次运行）或 JSON 解析失败时均静默返回空存储，不阻断应用启动。
- **单元测试**：6 项测试覆盖键格式、增删查改、序列化往返、空存储边界条件。

### 10.7 平台兼容性问题

| 问题 | 描述 |
|------|------|
| macOS `.app` 包检测 | ✅ **已修复**：`find_godot_executable()` 新增 `#[cfg(target_os = "macos")]` 分支，优先枚举 `.app` 目录包并返回其路径，供 `open` 命令启动；Unix 通用回退逻辑追加 `metadata.is_file()` 检查，避免目录被误作可执行文件返回 |
| `validate_godot_executable` macOS 误报 | ✅ **已修复**（`services/launcher.rs`）：原实现仅检查 `is_file()`，macOS `.app` bundle 是目录，导致验证失败。现改为 `exec_path.is_file() \|\| exec_path.is_dir()`，兼容所有平台 |
| `open_folder` 三处重复 | ✅ **已修复**：新增 `pub fn open_folder(path: &Path)` 至 `utils/file_utils.rs`，在 `utils/mod.rs` 中 `pub use file_utils::open_folder` re-export；三个面板文件改为 `use crate::utils::open_folder`，各自的局部定义全部删除。 |
| `DownloadSource::mirror_prefix()` / `api_proxy_url()` | ✅ **已删除**：两个始终返回空值的方法从 `DownloadSource` impl 中移除；`full_api_url()` 直接返回 `format!("https://api.github.com{}", path)`；`github_api.rs` 测试同步更新为断言 `needs_proxy()` 和 `is_custom()`。 |

---

---

## 10.8 代码清理（warning 归零）

51 个编译 warning 已全部消除，处理策略如下：

| 处理方式 | 说明 | 主要涉及位置 |
|---------|------|------------|
| **直接删除** | 真正的死代码，无保留价值 | `utils/region.rs`（整个模块）、`style.rs` 空 `colors {}` 模块及 `BADGE_BLUE` 常量、`github_api.rs` 两个 wrapper（`fetch_all_versions`/`fetch_all_versions_with_source`）、`download_dialog.rs` 四个未接入公开函数、`projects_panel.rs::create_sample_projects()`、`DownloadSource::mirror_prefix()` / `api_proxy_url()`（P2 阶段）、三处重复的局部 `fn open_folder`（P2 阶段） |
| **修复机械错误** | 直接改正，无副作用 | 4 处未使用 import（`Ordering`、`Mutex`、`download_state`、`fetch_all_versions_with_source`）、`validate_zip_file` 中多余的 `mut archive`、`extract_callback` 加 `_` 前缀 |
| **`#[allow(dead_code)]`** | 有意保留的未来 API | 模型层方法（`GodotInstall`/`GodotVersion`/`GodotVariant` impl）、`utils/file_utils.rs`（模块级 `#![allow(dead_code)]`）、`services/launcher.rs` 工具函数、`download_state` 子常量/函数、`state/app_config.rs` impl 方法、`state/app_state_impl.rs` 6 个工具方法、`style.rs` 未使用常量/函数、`settings_panel.rs` 2 个工具函数 |

---

## 11. 开发计划

### 短期（v0.2.0）

- 实现"打开项目"功能（通过指定 Godot 版本打开）
- 实现 `parse_godot_version()` 解析 `project.godot` 文件中的 `config_version` 字段

### 中期（v0.3.0）

- 项目管理完整实现（新建、导入、删除、收藏）
- `check_updates_on_start` 接入实际更新检查逻辑
- 工具提示完善

### 长期（v1.0.0）

- 多语言支持（i18n）
- 导出模板管理（`GodotVariant::ExportTemplates`）
- 断点续传下载
- 应用自身更新机制

---

## 12. 参考资源

- [egui 文档](https://docs.rs/egui)
- [eframe 文档](https://docs.rs/eframe)
- [GitHub REST API - Releases](https://docs.github.com/en/rest/releases/releases)
- [Godot Engine 下载页](https://godotengine.org/download)
- [Godot GitHub Releases](https://github.com/godotengine/godot/releases)