# Godot Hub - UI 设计规范文档 v2.0

## 📋 概述

本文档记录了 Godot Hub 项目的 UI 设计规范和实现细节。基于 Godot Hub 界面设计方案，采用统一的样式系统、卡片式布局和清晰的信息层次结构。

---

## 🎨 视觉规范

### 颜色系统

所有颜色常量定义在 `src/ui/style.rs` 中：

#### 背景颜色
```rust
pub const BG_PRIMARY: Color32 = Color32::from_rgb(26, 29, 35);      // #1a1d23 主背景
pub const BG_SECONDARY: Color32 = Color32::from_rgb(37, 40, 48);    // #252830 卡片/面板
pub const BG_SIDEBAR: Color32 = Color32::from_rgb(22, 24, 29);      // #16181d 侧边栏
pub const BG_HOVER: Color32 = Color32::from_rgb(45, 49, 58);        // #2d313a 悬停状态
```

#### 强调色
```rust
pub const ACCENT_BLUE: Color32 = Color32::from_rgb(71, 140, 191);   // #478cbf 主强调色
pub const ACCENT_BLUE_LIGHT: Color32 = Color32::from_rgb(92, 143, 184); // #5c8fb8
```

#### 文字颜色
```rust
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(224, 224, 224); // #e0e0e0 主文字
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 146, 168); // #8b92a8 次要文字
pub const TEXT_MUTED: Color32 = Color32::from_rgb(107, 114, 128);   // #6b7280 禁用/提示
```

#### 标签颜色
```rust
pub const BADGE_BLUE: Color32 = Color32::from_rgb(71, 140, 191);    // 工具标签
pub const BADGE_PURPLE: Color32 = Color32::from_rgb(142, 68, 173);  // 版本号
pub const BADGE_GREEN: Color32 = Color32::from_rgb(39, 174, 96);    // 状态标签
pub const BADGE_ORANGE: Color32 = Color32::from_rgb(255, 152, 0);   // 警告标签
```

#### 状态颜色
```rust
pub const SUCCESS: Color32 = Color32::from_rgb(46, 139, 87);        // 成功状态
pub const WARNING: Color32 = Color32::from_rgb(255, 165, 0);        // 警告状态
pub const ERROR: Color32 = Color32::from_rgb(220, 53, 69);          // 错误状态
```

### 尺寸规范

所有尺寸常量定义在 `src/ui/style.rs` 中：

```rust
pub const SIDEBAR_WIDTH_COLLAPSED: f32 = 60.0;    // 侧边栏宽度（图标模式）
pub const SIDEBAR_WIDTH_EXPANDED: f32 = 220.0;    // 侧边栏宽度（展开模式）
pub const CARD_GAP: f32 = 16.0;                   // 卡片间隙
pub const PAGE_PADDING: f32 = 24.0;               // 页面内边距
pub const CARD_ROUNDING: f32 = 12.0;              // 卡片圆角
pub const BUTTON_ROUNDING: f32 = 6.0;             // 按钮圆角
pub const PILL_ROUNDING: f32 = 12.0;              // Pill 形状圆角
pub const BUTTON_HEIGHT: f32 = 32.0;              // 标准按钮高度
pub const BUTTON_HEIGHT_LARGE: f32 = 40.0;        // 大按钮高度
```

---

## 🏗️ 架构设计

### 统一样式模块

所有 UI 相关的样式和组件都集中在 `src/ui/style.rs` 模块中：

```
src/ui/
├── style.rs           # 统一样式模块
│   ├── colors         # 颜色常量
│   ├── spacing        # 尺寸常量
│   ├── setup_visuals  # 样式配置
│   └── components     # 可复用组件
├── sidebar.rs         # 侧边栏组件
├── versions_panel.rs  # 版本管理面板
├── projects_panel.rs  # 项目管理面板
├── settings_panel.rs  # 设置面板
└── download_dialog.rs # 下载对话框
```

### 模块职责

#### style.rs
- **颜色常量定义**: 统一管理所有颜色
- **尺寸常量定义**: 统一管理所有尺寸
- **样式配置**: 配置 egui 的视觉效果
- **可复用组件**: 提供常用的 UI 组件

#### 面板模块
- 使用统一样式系统
- 实现卡片式布局
- 保持清晰的信息层次
- 提供一致的交互体验

---

## 📦 可复用组件

### 1. 状态标签 (Badge)

```rust
pub fn badge(ui: &mut egui::Ui, text: &str, color: Color32)
```

**用途**: 显示小型的状态标签
**示例**: 版本标签、分类标签

### 2. Pill 形状标签

```rust
pub fn status_pill(ui: &mut egui::Ui, text: &str, color: Color32)
```

**用途**: 显示更突出的状态标签
**示例**: 变体标签（Standard/Mono/Export）

### 3. 卡片容器

```rust
pub fn card_frame() -> egui::Frame
```

**用途**: 创建统一的卡片容器
**特点**: 
- 统一的背景色和圆角
- 标准的内边距
- 边框样式

### 4. 按钮组件

```rust
pub fn primary_button(text: &str) -> egui::Button    // 主要按钮
pub fn secondary_button(text: &str) -> egui::Button  // 次要按钮
pub fn danger_button(text: &str) -> egui::Button     // 危险操作按钮
pub fn success_button(text: &str) -> egui::Button    // 成功/运行按钮
```

### 5. 空状态组件

```rust
pub fn empty_state(
    ui: &mut egui::Ui,
    icon: &str,
    title: &str,
    description: &str,
    action_text: Option<&str>,
    action: Option<&mut dyn FnMut()>,
)
```

**用途**: 显示友好的空状态提示
**特点**: 
- 图标 + 标题 + 描述
- 可选的操作按钮
- 统一的视觉风格

### 6. 统计卡片

```rust
pub fn stat_card(ui: &mut egui::Ui, label: &str, value: &str, icon: &str, color: Color32)
```

**用途**: 显示统计信息
**示例**: 已安装版本数、可用版本数

### 7. 区域标题

```rust
pub fn section_header(ui: &mut egui::Ui, icon: &str, text: &str, count: Option<usize>)
```

**用途**: 显示区域标题和计数
**示例**: "📦 Installed Versions (3)"

### 8. 路径标签

```rust
pub fn path_label(ui: &mut egui::Ui, path: &str, max_len: usize)
```

**用途**: 显示路径并自动截断
**特点**: 
- 自动截断长路径
- 悬停显示完整路径
- 使用代码字体样式

---

## 🎨 组件设计规范

### 侧边栏 (Sidebar)

#### 布局结构
```
┌────────────────┐
│  🎮 Godot Hub  │  应用标题区
│  Engine Mgr    │
│                │
│  ───────────── │  分隔线
│                │
│  NAVIGATION    │  导航区
│  📦 Versions   │
│  📁 Projects   │
│  ⚙️ Settings   │
│                │
│  ───────────── │  分隔线
│                │
│  STATISTICS    │  统计区
│  ┌──────────┐ │
│  │📦 3      │ │
│  │Installed │ │
│  └──────────┘ │
│                │
│  v0.1.0        │  版本信息
│  [⬇️ Download]│  下载按钮
└────────────────┘
```

#### 设计要点
- 宽度固定为 220px
- 背景色使用 `BG_SIDEBAR`
- 导航按钮选中时高亮
- 统计卡片使用颜色条标识
- 底部固定下载按钮

### 版本管理面板 (Versions Panel)

#### 布局结构
```
┌────────────────────────────────────────────────────────────┐
│  Godot Versions                          [🔄 Refresh]      │
│  Manage your Godot engine installations                    │
├────────────────────────────────────────────────────────────┤
│  📦 Installed Versions (3)                                 │
│  ┌──────────────────────────────────────────────────────┐ │
│  │ 🎮  4.3  [Standard]  [⭐ Favorite]        [⋮] [▶ Run]│ │
│  │      📂 .../Godot/4.3/standard/godot                  │ │
│  │      🕐 Last used: 2025-01-16 14:30                  │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  🌐 Available Versions (12)                                │
│  ▼ Godot 4.x                                               │
│    ┌────────────────────────────────────────────────────┐ │
│    │  4.3  [Mono]  Linux64        [⬇️ Download]        │ │
│    │  📅 Released: 2024-08-15                           │ │
│    └────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────┘
```

#### 设计要点
- 使用卡片式布局
- 变体标签使用不同颜色
- 状态标签清晰标识
- 操作按钮右对齐
- 长路径自动截断

### 项目管理面板 (Projects Panel)

#### 布局结构
```
┌────────────────────────────────────────────────────────────┐
│  Projects                                  [🔍 Scan]       │
│  Manage your Godot projects                                │
├────────────────────────────────────────────────────────────┤
│  [➕ New Project] [📂 Import Project] [📁 Open Folder]    │
│                                                            │
│  📁 Recent Projects (5)                                    │
│  ┌──────────────────────────────────────────────────────┐ │
│  │ 🎮  My Game  [✓ Valid]  [⭐ Favorite]    [⋮] [▶ Open]│ │
│  │      📂 .../Godot/Projects/MyGame                     │ │
│  │      🎮 Godot 4.3  🕐 Last opened: 2025-01-15       │ │
│  └──────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────┘
```

#### 设计要点
- 顶部操作按钮区
- 项目卡片显示完整信息
- 有效/无效状态标识
- 收藏项目星标显示
- 快捷操作菜单

### 设置面板 (Settings Panel)

#### 布局结构
```
┌────────────────────────────────────────────────────────────┐
│  Settings                          [🔄 Reset] [💾 Save]    │
│  Configure application preferences                         │
├────────────────────────────────────────────────────────────┤
│  📂 Directories                                            │
│  Configure installation and project directories            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  Installation Directory                              │ │
│  │  [/path/to/install              ] [Browse] [📂 Open] │ │
│  │  Where Godot versions will be installed              │ │
│  │                                                      │ │
│  │  Projects Directory                                  │ │
│  │  [/path/to/projects             ] [Browse] [📂 Open] │ │
│  │  Default location for your Godot projects            │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ⚙️ Behavior                                               │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  Check for Updates on Startup          [✓]          │ │
│  │  Application Theme                                   │ │
│  │  [🌙 Dark] [☀️ Light] [💻 System]                   │ │
│  └──────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────┘
```

#### 设计要点
- 使用区块式布局
- 每个设置项包含标题和说明
- 目录设置提供快捷操作
- 主题选择使用按钮组
- 下载源选择清晰标识

---

## 🔧 样式配置

### egui 视觉效果配置

```rust
pub fn setup_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    
    // 窗口/面板背景
    visuals.window_fill = colors::BG_PRIMARY;
    visuals.panel_fill = colors::BG_PRIMARY;
    visuals.extreme_bg_color = colors::BG_SIDEBAR;
    
    // 文字
    visuals.override_text_color = Some(colors::TEXT_PRIMARY);
    visuals.text_cursor.stroke.color = colors::ACCENT_BLUE;
    
    // 组件样式
    visuals.widgets.inactive.weak_bg_fill = colors::BG_SECONDARY;
    visuals.widgets.inactive.bg_fill = colors::BG_SECONDARY;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, colors::BORDER);
    
    visuals.widgets.hovered.weak_bg_fill = colors::BG_HOVER;
    visuals.widgets.hovered.bg_fill = colors::BG_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, colors::ACCENT_BLUE);
    
    visuals.widgets.active.bg_fill = colors::ACCENT_BLUE;
    
    // 选择高亮
    visuals.selection.bg_fill = colors::ACCENT_BLUE;
    
    ctx.set_visuals(visuals);
}
```

---

## 📝 最佳实践

### 1. 使用统一样式

**推荐**:
```rust
use crate::ui::style::{colors, spacing, card_frame, primary_button};

// 使用统一的颜色
ui.label(RichText::new("Text").color(colors::TEXT_PRIMARY));

// 使用统一的卡片
card_frame().show(ui, |ui| {
    // 内容
});

// 使用统一的按钮
ui.add(primary_button("Download"));
```

**不推荐**:
```rust
// 硬编码颜色
ui.label(RichText::new("Text").color(Color32::from_rgb(255, 0, 0)));

// 重复定义样式
egui::Frame::group(ui.style())
    .fill(Color32::from_rgb(37, 40, 48))
    .show(ui, |ui| { /* ... */ });
```

### 2. 保持信息层次

```rust
// 第一层：区域标题
section_header(ui, "📦", "Installed Versions", Some(3));

ui.add_space(12.0);

// 第二层：卡片
card_frame().show(ui, |ui| {
    // 第三层：卡片内容
    ui.horizontal(|ui| {
        ui.label(RichText::new("4.3").size(16.0).strong());
        status_pill(ui, "Standard", colors::BADGE_GREEN);
    });
    
    ui.add_space(6.0);
    
    // 第四层：详细信息
    path_label(ui, &path_str, 60);
});
```

### 3. 处理交互反馈

```rust
// 按钮悬停提示
let response = ui.add(primary_button("Download"));
let response = response.on_hover_text("Download Godot from GitHub");

if response.clicked() {
    // 执行操作
}
```

### 4. 空状态友好提示

```rust
if items.is_empty() {
    empty_state(
        ui,
        "📦",
        "No Items Found",
        "Click 'Add' to create your first item",
        Some("➕ Add Item"),
        Some(&mut || {
            // 添加操作
        })
    );
} else {
    // 显示列表
}
```

---

## 🚀 实现清单

### ✅ 已完成

- [x] 统一样式模块 (`style.rs`)
- [x] 颜色常量系统
- [x] 尺寸常量系统
- [x] 可复用组件库
- [x] 侧边栏优化
- [x] 版本管理面板优化
- [x] 项目管理面板优化
- [x] 设置面板优化
- [x] egui API 兼容性修复

### 🔄 进行中

- [ ] 下载对话框优化
- [ ] 主题切换功能
- [ ] 响应式布局优化

### 📋 待办

- [ ] 动画效果
- [ ] 键盘快捷键
- [ ] 无障碍支持
- [ ] 性能优化

---

## 📊 技术细节

### egui API 兼容性

#### Frame API 变更
```rust
// 旧版本
egui::Frame::none()
    .rounding(8.0)
    .inner_margin(egui::Margin::symmetric(12.0, 8.0))

// 新版本 (egui 0.31+)
egui::Frame::NONE
    .corner_radius(8.0)
    .inner_margin(egui::Margin::symmetric(12, 8))
```

#### Margin 类型变更
```rust
// Margin 现在使用 i8 类型
egui::Margin::same(16)          // 正确
egui::Margin::same(16.0)        // 错误

egui::Margin::symmetric(12, 8)  // 正确
egui::Margin::symmetric(12.0, 8.0)  // 错误
```

### 编译警告处理

```rust
// 未使用的导入
use crate::ui::style::{colors, spacing};  // 只导入需要的

// 未使用的变量
fn draw_panel_header(ui: &mut egui::Ui, _state: &mut AppState) {
    // 使用下划线前缀
}
```

---

## 📚 参考资料

- [egui 官方文档](https://docs.rs/egui/)
- [egui GitHub 仓库](https://github.com/emilk/egui)
- [Godot Hub 设计方案](./DESIGN.md)
- [项目架构文档](./ARCHITECTURE.md)

---

**文档版本**: 2.0  
**最后更新**: 2025-01-16  
**维护者**: Godot Hub 开发团队