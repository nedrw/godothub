# Godot Hub - UI 设计规范

## 1. 概述

本文档描述 Godot Hub 的 UI 设计系统，包括颜色规范、组件库、各面板布局，以及当前实现状态。
所有 UI 代码位于 `src/ui/` 目录，样式系统统一由 `src/ui/style.rs` 管理。

---

## 2. 颜色系统

### 深色主题（`dark_colors`）

| 常量名 | RGB | 用途 |
|--------|-----|------|
| `BG_PRIMARY` | (26, 29, 35) | 主背景、窗口/面板底色 |
| `BG_SECONDARY` | (37, 40, 48) | 卡片、输入框背景 |
| `BG_SIDEBAR` | (22, 24, 29) | 侧边栏背景 |
| `BG_HOVER` | (45, 49, 58) | 悬停状态背景 |
| `ACCENT_BLUE` | (71, 140, 191) | 主要强调色、主按钮 |
| `ACCENT_BLUE_LIGHT` | (92, 143, 184) | 激活状态描边 |
| `TEXT_PRIMARY` | (224, 224, 224) | 主文本 |
| `TEXT_SECONDARY` | (139, 146, 168) | 次要文本、说明文字 |
| `TEXT_MUTED` | (107, 114, 128) | 弱化文本、标签 |
| `BORDER` | (58, 63, 75) | 边框、分隔线 |
| `BADGE_BLUE` | (71, 140, 191) | Standard 变体标签 |
| `BADGE_PURPLE` | (142, 68, 173) | Mono 变体标签 |
| `BADGE_GREEN` | (39, 174, 96) | 已安装状态标签 |
| `BADGE_ORANGE` | (255, 152, 0) | Export 变体标签、警告 |
| `SUCCESS` | (46, 139, 87) | 成功按钮、成功状态 |
| `WARNING` | (255, 165, 0) | 警告状态、收藏标签 |
| `ERROR` | (220, 53, 69) | 危险按钮、错误状态 |

### 浅色主题（`light_colors`）

| 常量名 | RGB | 用途 |
|--------|-----|------|
| `BG_PRIMARY` | (245, 247, 250) | 主背景 |
| `BG_SECONDARY` | (255, 255, 255) | 卡片背景 |
| `BG_SIDEBAR` | (240, 242, 245) | 侧边栏背景 |
| `BG_HOVER` | (232, 235, 240) | 悬停背景 |
| `ACCENT_BLUE` | (41, 98, 255) | 主要强调色 |
| `TEXT_PRIMARY` | (32, 33, 36) | 主文本 |
| `TEXT_SECONDARY` | (95, 99, 104) | 次要文本 |
| `BORDER` | (218, 220, 224) | 边框 |
| `ERROR` | (244, 67, 54) | 错误状态 |

### 主题获取方式

```rust
// 根据当前主题获取颜色集合
let colors = ThemeColors::from_theme(state.config.theme);

// Theme::System 目前回退为深色主题（系统检测未实现）
```

---

## 3. 尺寸规范（`spacing` 模块）

| 常量 | 值 | 说明 |
|------|-----|------|
| `SIDEBAR_WIDTH_COLLAPSED` | 60.0 | 侧边栏折叠宽度（未启用） |
| `SIDEBAR_WIDTH_EXPANDED` | 220.0 | 侧边栏展开宽度 |
| `CARD_GAP` | 16.0 | 卡片间距 |
| `PAGE_PADDING` | 24.0 | 页面内边距 |
| `CARD_ROUNDING` | 12.0 | 卡片圆角 |
| `BUTTON_ROUNDING` | 6.0 | 按钮圆角 |
| `PILL_ROUNDING` | 12.0 | 胶囊标签圆角 |
| `BUTTON_HEIGHT` | 32.0 | 标准按钮高度 |
| `BUTTON_HEIGHT_LARGE` | 40.0 | 大按钮高度（下载按钮） |

---

## 4. 可复用组件（`style.rs`）

### 4.1 标签类

#### `badge(ui, text, color)`
带背景色的小型文本标签，白色文字，11px，用于 "Favorite"、技术栈名称等。

#### `status_pill(ui, text, color)`
圆角胶囊形状的状态标签，白色文字，11px，加粗，水平内边距 8px。
用于变体标签（Standard/Mono）和安装状态（✓ Installed）。

### 4.2 容器类

#### `card_frame(theme) -> egui::Frame`
卡片容器，使用 `BG_SECONDARY` 填充，`CARD_ROUNDING` 圆角，`BORDER` 描边，12px 内边距。

### 4.3 按钮类

所有按钮函数返回 `egui::Button`，需通过 `ui.add(btn)` 渲染。

| 函数 | 颜色 | 最小尺寸 | 用途 |
|------|------|---------|------|
| `primary_button(text, theme)` | `ACCENT_BLUE` 填充，白色文字 | 120×32 | 主操作（下载、保存） |
| `secondary_button(text, theme)` | 透明填充，`BORDER` 描边 | 120×32 | 次要操作（取消、浏览） |
| `danger_button(text)` | `ERROR` 填充，白色文字 | 120×32 | 危险操作（删除、移除） |
| `success_button(text)` | `SUCCESS` 填充，白色文字 | 64×32 | 执行操作（Run、Open） |

### 4.4 信息展示类

#### `empty_state(ui, theme, icon, title, description, action_text, action)`
空状态提示组件，垂直居中，包含大图标（48px）、标题（16px 加粗）、描述文字，
可选渲染一个主操作按钮。

#### `section_header(ui, theme, icon, text, count)`
带 emoji 图标的区域标题（16px 加粗），右侧可选显示数量标注。

#### `panel_header(ui, theme, title, description)`
面板页眉，标题 20px 加粗，下方跟随小字描述。

#### `path_label(ui, theme, path, max_len)`
带路径缩短显示（从末尾截断并加 `...` 前缀）和完整路径悬停提示的路径展示。

#### `stat_card(ui, theme, label, value, icon, color)`
统计卡片，包含图标、数值（18px 加粗）和标签，用于侧边栏统计展示。

---

## 5. 整体布局

```
┌──────────────────────────────────────────────────────────────┐
│  SidePanel::left("sidebar")  │  CentralPanel::default()     │
│  宽度固定 220px               │                              │
│  ┌────────────────────────┐  │  ┌────────────────────────┐  │
│  │ 🎮 Godot Hub           │  │  │  TopBottomPanel::top   │  │
│  │    Engine Manager      │  │  │  面板标题 + 操作按钮    │  │
│  │ ─────────────────────  │  │  └────────────────────────┘  │
│  │ NAVIGATION             │  │                              │
│  │  📦 Versions           │  │  ScrollArea::vertical()      │
│  │  📁 Projects           │  │  主内容区域（卡片列表）       │
│  │  ⚙️ Settings           │  │                              │
│  │ ─────────────────────  │  │  [Download Dialog Window]    │
│  │ STATISTICS             │  │  居中浮动，650×550            │
│  │  📦 3  Installed       │  └────────────────────────────┘  │
│  │  🌐 12 Available       │                                  │
│  │                        │                                  │
│  │ [⬇️ Download New Ver.] │                                  │
│  │ v0.1.0                 │                                  │
│  └────────────────────────┘                                  │
└──────────────────────────────────────────────────────────────┘
```

---

## 6. 各面板设计

### 6.1 侧边栏（`sidebar.rs`）

**结构（从上到下）：**
1. 应用标题区：`🎮` 图标（32px）+ "Godot Hub"（20px 加粗）+ "Engine Manager"（12px）
2. 分隔线
3. 导航区：标题 "NAVIGATION"（11px 灰色），三个导航按钮
4. 分隔线
5. 统计区：标题 "STATISTICS"（11px 灰色），紧凑型统计卡片
6. 底部（`Layout::bottom_up`）：版本号 + 大下载按钮

**导航按钮状态：**
- 默认：`BG_SIDEBAR` 背景
- 选中：`BG_HOVER` 背景，文字使用 `ACCENT_BLUE`
- 悬停：光标变为 `PointingHand`

**统计卡片（仅在数量 > 0 时显示）：**
- Installed：始终显示
- Available：始终显示（未安装版本数）
- Downloading：下载中数量 > 0 时显示
- Favorites：收藏数 > 0 时显示

> 注意：Downloading 计数通过过滤 `_error`、`_extracting`、`_complete` 后缀 key 获取，
> 但 `draw_download_queue_status`（下载对话框）中的计数存在同类 bug 未修复。

---

### 6.2 版本管理面板（`versions_panel.rs`）

**布局：**
```
TopBottomPanel::top   → "Godot Versions" 标题 + "🔄 Refresh" 按钮
ScrollArea::vertical
  ├─ "📦 Installed Versions (n)"
  │    └─ 已安装版本卡片列表（或空状态提示）
  └─ "🌐 Available Versions (n)"
       ├─ Godot 4.x（可折叠分组）
       └─ Godot 3.x（可折叠分组）
[Window] Delete Confirmation 对话框（居中浮动）
```

**已安装版本卡片内容：**
- 左侧：收藏时显示 `⭐`，否则显示 `🎮`（32px）
- 中间：版本号（16px 加粗）+ 变体标签（Pill）+ 收藏标签（如有）
- 路径显示（截断，悬停显示完整路径）
- 最后使用时间（如有）
- 右侧：`▶ Run` 按钮 + `⋮` 菜单

**`⋮` 菜单选项：**
- 📂 Open Folder
- ★/☆ Add/Remove from Favorites
- 🗑 Remove（触发确认对话框）

**可用版本卡片内容：**
- 左侧：版本号 + 变体标签 + 平台标签 + 发布日期
- 右侧：已安装显示 `✓ Installed` Pill；下载中显示进度区域；否则显示 `⬇️ Download` 按钮

**下载进度区域状态：**

| 状态 | 显示内容 |
|------|---------|
| 正常下载 | 进度条（120px）+ 百分比 + Cancel 按钮 |
| 解压中 | `📦 Extracting...`（蓝色文字） |
| 完成 | `✓ Installed`（绿色文字） |
| 失败 | `❌ Failed` + Retry 按钮 + Remove 按钮 |

---

### 6.3 项目管理面板（`projects_panel.rs`）

**布局：**
```
CentralPanel（内边距 16px）
  TopBottomPanel::top  → "Projects" 标题 + "🔍 Scan" 按钮（无实际功能）
  操作按钮行：[➕ New Project] [📂 Import Project] [📁 Open Projects Folder]
  ScrollArea::vertical
    └─ "📁 Recent Projects (n)"
         └─ 项目卡片列表（或空状态提示）
```

**项目卡片内容：**
- 左侧：收藏时 `⭐`，否则 `📁`（28px）
- 中间：项目名称（16px）+ Godot 版本标签 + 收藏标签
- 项目路径（截断显示）
- 最后打开时间（如有）
- 右侧：`▶ Open` 按钮（占位）+ `⋮` 菜单

> ⚠️ **当前限制**：项目扫描功能可用（扫描 `projects_dir` 查找 `project.godot`），
> 但版本解析返回硬编码 `"4.x"`，Open/New/Import/Favorite/Remove 均为占位实现。

---

### 6.4 设置面板（`settings_panel.rs`）

**布局：**
```
CentralPanel（内边距 16px）
  TopBottomPanel::top  → "Settings" 标题 + [💾 Save Settings] [🔄 Reset]
  ScrollArea::vertical
    ├─ "📂 Directories"（卡片）
    │    ├─ Installation Directory：文本框 + Browse 按钮 + Open 按钮
    │    └─ Projects Directory：文本框 + Browse 按钮 + Open 按钮
    ├─ "⚙️ Behavior"（卡片）
    │    ├─ Check for Updates on Startup：Checkbox 开关
    │    ├─ Application Theme：[🌙 Dark] [☀️ Light] [💻 System] 三按钮选择
    │    └─ Download Source：[🐙 GitHub] [⚙️ Custom] + 自定义 URL 输入框
    └─ "ℹ️ About"（卡片）
         ├─ 应用图标 + 名称 + 版本标签
         ├─ 描述文字
         ├─ [🐙 GitHub] [🌐 Website] 按钮（占位）
         └─ 技术栈标签行 + 版权信息
```

**目录选择实现：**使用 `rfd::FileDialog::new().pick_folder()` 打开原生文件夹选择对话框。

**主题选择按钮：** 选中态为 `ACCENT_BLUE` 填充 + 白色文字，未选中态为 `BG_SECONDARY` + `BORDER` 描边。

**自定义镜像 URL：** 仅在选择 `Custom` 下载源时显示，URL 为空时显示橙色警告提示。

---

### 6.5 下载对话框（`download_dialog.rs`）

**窗口属性：**
- 标题：`⬇️ Download Godot`
- 默认尺寸：650×550，最小 550×400
- 居中显示（`Align2::CENTER_CENTER`）
- 可调整大小，不可折叠

**内容布局：**
```
描述文字 + 刷新时间 + 🔄 刷新按钮
分隔线
搜索栏（占位）+ Filter 按钮（占位）
[下载队列状态条]（有下载任务时显示）
ScrollArea（max_height 350px）
  ├─ 🚀 Godot 4.x (n available)（可折叠）
  │    └─ 版本条目列表
  └─ 📦 Godot 3.x (n available)（可折叠）
       └─ 版本条目列表
分隔线
[Close] 按钮
```

**版本条目：**
- 背景：已安装的条目使用 `SUCCESS.linear_multiply(0.08)` 淡绿色背景和绿色描边
- 左侧：版本号 + 变体标签（`draw_variant_tag`，背景色半透明）+ 平台（小字）+ 发布日期
- 右侧：已安装显示 `✓ Installed` 文字；下载中显示 `draw_downloading_status`；否则显示下载按钮

**刷新状态：**
- 刷新中：全内容替换为居中 `ui.spinner()` + 提示文字
- 刷新失败：显示橙色警告框 + 错误信息，版本列表为空时显示 Retry 按钮

---

## 7. egui 配置（`setup_visuals`）

每帧调用，通过 `ctx.set_visuals()` 应用以下配置：

```rust
visuals.window_fill        = colors.bg_primary
visuals.panel_fill         = colors.bg_primary
visuals.extreme_bg_color   = colors.bg_sidebar  // 侧边栏背景

// 组件状态颜色
inactive  → bg_fill: bg_secondary, stroke: border
hovered   → bg_fill: bg_hover,     stroke: accent_blue
active    → bg_fill: accent_blue,   stroke: accent_blue_light
open      → bg_fill: bg_hover

// 间距
item_spacing:   (8, 6)
button_padding: (8, 4)
interact_size:  (40, 20)
```

---

## 8. 实现状态

### ✅ 已实现

- 深色 / 浅色主题完整颜色集，每帧动态切换
- 所有可复用组件（badge、pill、card、buttons、empty_state 等）
- 侧边栏导航、统计、下载按钮
- 版本管理面板（已安装卡片、可用版本分组、下载进度状态机、删除确认对话框）
- 下载对话框（版本分组、下载进度展示、取消/重试/移除）
- 设置面板（目录选择、主题切换、下载源配置）
- 项目面板（目录扫描、卡片展示）

### 🔴 未实现 / 占位

- `Theme::System` 系统主题检测
- 版本搜索/筛选（下载对话框搜索栏无效）
- 项目 Open / New / Import / Favorite / Remove 操作
- Settings 面板 GitHub / Website 按钮跳转
- `draw_download_details()`、`initiate_download()`、`get_download_stats()` 公开函数未接入 UI 流程

---

## 9. 参考资源

- [egui 组件展示](https://www.egui.rs/)
- [egui API 文档](https://docs.rs/egui/0.31)
- [eframe 文档](https://docs.rs/eframe/0.31)