# Godot Hub - UI 设计规范文档

## 📐 设计原则

### 核心原则
1. **简洁明了** - 界面应清晰直观，减少用户认知负担
2. **一致性** - 保持视觉和交互的一致性
3. **响应式** - 适应不同窗口大小和分辨率
4. **可访问性** - 提供清晰的视觉反馈和辅助信息
5. **高效性** - 减少操作步骤，提高工作效率

### 设计理念
- **原生优先** - 使用 egui 原生组件，保持跨平台一致性
- **功能导向** - UI 为功能服务，不过度设计
- **渐进披露** - 重要信息优先展示，次要信息按需展开
- **即时反馈** - 用户操作应立即得到视觉反馈

---

## 🎨 视觉规范

### 颜色系统

#### 语义颜色
| 颜色类型 | 用途 | egui 颜色 |
|---------|------|-----------|
| Primary | 主要操作按钮、选中状态 | `Color32::from_rgb(70, 130, 180)` |
| Success | 成功状态、确认操作 | `Color32::from_rgb(46, 139, 87)` |
| Warning | 警告状态、注意事项 | `Color32::from_rgb(255, 165, 0)` |
| Error | 错误状态、删除操作 | `Color32::from_rgb(220, 53, 69)` |
| Info | 信息提示、帮助文本 | `Color32::from_rgb(70, 130, 180)` |

#### 文本颜色
| 类型 | 用途 | 大小/颜色 |
|-----|------|----------|
| 标题 | Heading (h1-h6) | 14-20px, Strong |
| 正文 | Body text | 12-14px, Normal |
| 辅助 | Secondary text | 10-12px, Weak |
| 代码 | Code/Path | 11px, Monospace |

### 字体规范

#### 字体大小层级
```
Heading 1 (h1): 20px - 主标题（应用名称）
Heading 2 (h2): 18px - 次标题（面板标题）
Heading 3 (h3): 16px - 小标题（区域标题）
Body:           14px - 正文内容
Small:          12px - 辅助信息
Tiny:           10px - 标签、提示
```

#### 字体样式
```rust
// 标题样式
ui.heading("Title");                    // 20px, bold
ui.h1("Heading 1");                     // 18px
ui.h2("Heading 2");                     // 17px
ui.h3("Heading 3");                     // 16px
ui.h4("Heading 4");                     // 15px
ui.h5("Heading 5");                     // 14px
ui.h6("Heading 6");                     // 13px

// 文本样式
ui.label("Normal text");                // 正常文本
ui.label(RichText::new("Strong").strong());  // 强调文本
ui.label(RichText::new("Small").small());    // 小号文本
ui.label(RichText::new("Weak").weak());      // 弱化文本
ui.label(RichText::new("Code").code());      // 代码样式
```

### 间距规范

#### 标准间距
```
xs:  4px  - 元素内部间距
sm:  8px  - 紧凑元素间距
md:  16px - 标准元素间距
lg:  24px - 区域间距
xl:  32px - 大块区域间距
```

#### 应用示例
```rust
// 元素内部间距
ui.add_space(4.0);   // xs
// 紧凑元素间距
ui.add_space(8.0);   // sm
// 标准间距
ui.add_space(16.0);  // md
// 区域分隔
ui.add_space(24.0);  // lg
```

---

## 🏗️ 布局结构

### 窗口整体布局

```
┌─────────────────────────────────────────────────────────────┐
│  Title Bar: Godot Hub                          [─][□][×]  │
├────────────────┬────────────────────────────────────────────┤
│                │                                            │
│   Sidebar      │            Central Panel                  │
│   (200-300px)  │            (自适应宽度)                    │
│                │                                            │
│   ┌──────────┐│  ┌──────────────────────────────────────┐ │
│   │ Logo     ││  │  Header                              │ │
│   │ Title    ││  │  (Title + Description + Actions)     │ │
│   └──────────┘│  └──────────────────────────────────────┘ │
│                │                                            │
│   Navigation   │  ┌──────────────────────────────────────┐ │
│   ┌──────────┐│  │                                      │ │
│   │ Versions ││  │  Main Content                        │ │
│   │ Projects ││  │  (ScrollArea)                        │ │
│   │ Settings ││  │                                      │ │
│   └──────────┘│  │                                      │ │
│                │  │                                      │ │
│   Statistics   │  │                                      │ │
│   ┌──────────┐│  │                                      │ │
│   │ Info     ││  │                                      │ │
│   └──────────┘│  └──────────────────────────────────────┘ │
│                │                                            │
│   [Download]   │                                            │
│                │                                            │
└────────────────┴────────────────────────────────────────────┘
```

### 侧边栏布局规范

```rust
// 侧边栏配置
egui::SidePanel::left("sidebar")
    .width_range(200.0..=300.0)  // 最小200px，最大300px
    .default_width(220.0)        // 默认宽度220px
    .show(ctx, |ui| {
        // 内部布局
    });
```

#### 侧边栏元素
1. **应用标题区** (padding: 16px)
   - Logo/图标
   - 应用名称
   - 版本号（小号文本）

2. **导航区** (margin-top: 16px)
   - 导航按钮组
   - 当前选中项高亮
   - 图标 + 文本

3. **统计信息区** (margin-top: auto)
   - 已安装版本数
   - 项目数量
   - 下载状态

4. **操作按钮区** (padding: 16px)
   - 主要操作按钮（下载新版本）

---

## 📦 组件设计

### 1. 版本卡片组件

#### 设计规范
```
┌────────────────────────────────────────────────────────┐
│  ⚪ Godot 4.3                              [Stable]   │
│     Standard · Linux64                               │
│                                                        │
│  📂 /home/user/.gdhub/versions/4.3                   │
│                                                        │
│  Last used: 2025-01-15                    [Run] [⋮]  │
└────────────────────────────────────────────────────────┘
```

#### 实现规范
```rust
// 卡片容器
egui::Frame::group(ui.style())
    .inner_margin(12.0)    // 内边距
    .outer_margin(4.0)     // 外边距
    .rounding(6.0)         // 圆角
    .stroke(Stroke::new(   // 边框
        1.0, 
        ui.style().visuals.widgets.noninteractive.bg_stroke.color
    ))
    .show(ui, |ui| {
        // 卡片内容
    });
```

#### 信息层次
1. **第一层级**：版本号（大号加粗）、状态标签
2. **第二层级**：变体类型、平台信息
3. **第三层级**：路径、使用时间等详细信息
4. **操作区**：主要操作按钮、更多操作菜单

### 2. 下载对话框组件

#### 设计规范
```
┌──────────────────────────────────────────────────┐
│  Download Godot                           [×]    │
├──────────────────────────────────────────────────┤
│  [Search...                          ] [Filter▼] │
├──────────────────────────────────────────────────┤
│  ▼ Godot 4.x                                     │
│    ┌──────────────────────────────────────────┐  │
│    │ Godot 4.3 - Standard        [Download]   │  │
│    │ Released: 2024-09-20                     │  │
│    └──────────────────────────────────────────┘  │
│    ┌──────────────────────────────────────────┐  │
│    │ Godot 4.2.2 - Standard     [Installed]  │  │
│    │ Released: 2024-02-03                     │  │
│    └──────────────────────────────────────────┘  │
│                                                  │
│  ▶ Godot 3.x                                     │
│    ...                                           │
│                                                  │
├──────────────────────────────────────────────────┤
│  Download Queue: 1 task                          │
│  ┌────────────────────────────────────────────┐ │
│  │ Godot 4.3 Mono  ████████░░░░░░  60% [×]   │ │
│  └────────────────────────────────────────────┘ │
│                                    [Cancel All] │
└──────────────────────────────────────────────────┘
```

#### 实现要点
```rust
Window::new("Download Godot")
    .collapsible(false)
    .resizable(true)
    .default_size([600.0, 500.0])
    .min_width(500.0)
    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
    .show(ui.ctx(), |ui| {
        // 对话框内容
    });
```

### 3. 进度条组件

#### 设计规范
```rust
// 标准进度条
egui::ProgressBar::new(progress)
    .desired_width(200.0)
    .text(format!("{:.0}%", progress * 100.0))
    .animate(true);  // 启用动画效果

// 带状态的进度条
let color = if is_error {
    Color32::ERROR_RGB
} else if is_complete {
    Color32::GREEN
} else {
    ui.style().visuals.selection.bg_fill
};

egui::ProgressBar::new(progress)
    .desired_width(200.0)
    .fill(color);
```

### 4. 按钮组件

#### 按钮类型
```rust
// 1. 主要按钮
let primary_btn = egui::Button::new("Download")
    .fill(Color32::from_rgb(70, 130, 180));

// 2. 次要按钮
let secondary_btn = egui::Button::new("Cancel");

// 3. 危险按钮
let danger_btn = egui::Button::new("Delete")
    .fill(Color32::from_rgb(220, 53, 69));

// 4. 图标按钮
let icon_btn = ui.add_sized([28.0, 28.0], egui::Button::new("⚙"));

// 5. 小型按钮
let small_btn = egui::Button::new("Details").small();
```

#### 按钮布局
```rust
ui.horizontal(|ui| {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // 按钮从右向左排列
        if ui.button("Cancel").clicked() {}
        if ui.button("Save").clicked() {}
    });
});
```

---

## 🎯 当前 UI 问题及优化方案

### 问题 1: 侧边栏视觉层次不明显

#### 当前问题
- 按钮样式单调，缺少状态反馈
- 没有图标辅助识别
- 统计信息展示不直观

#### 优化方案
```rust
// 侧边栏优化代码框架
pub fn draw_sidebar(ui: &mut Ui, state: &mut AppState) {
    // 应用标题区
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.heading("🎮 Godot Hub");
    });
    ui.add_space(4.0);
    ui.label(RichText::new("v0.1.0").small().weak());
    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);
    
    // 导航区 - 使用更好的按钮样式
    ui.label(RichText::new("NAVIGATION").small().weak());
    ui.add_space(8.0);
    
    draw_nav_button(ui, "📦 Versions", MainTab::Versions, state);
    draw_nav_button(ui, "📁 Projects", MainTab::Projects, state);
    draw_nav_button(ui, "⚙️ Settings", MainTab::Settings, state);
    
    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);
    
    // 统计卡片
    draw_stat_card(ui, "Installed", state.installed_versions.len());
    
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        // 底部下载按钮
        ui.add_space(8.0);
        if ui.add_sized([180.0, 36.0], egui::Button::new("⬇️ Download New").fill(Color32::from_rgb(70, 130, 180))).clicked() {
            state.show_download_dialog = true;
        }
    });
}

// 导航按钮样式
fn draw_nav_button(ui: &mut Ui, text: &str, tab: MainTab, state: &mut AppState) {
    let is_selected = state.current_tab == tab;
    
    let btn = if is_selected {
        egui::Button::new(RichText::new(text).strong())
            .fill(Color32::from_rgb(60, 120, 170))
    } else {
        egui::Button::new(RichText::new(text))
    };
    
    if ui.add_sized([200.0, 32.0], btn).clicked() {
        state.current_tab = tab;
    }
}
```

### 问题 2: 版本面板信息展示混乱

#### 当前问题
- 信息层次不清晰
- 操作按钮排列不整齐
- 缺少状态标签

#### 优化方案
```rust
pub fn draw_installed_version(ui: &mut Ui, install: &GodotInstall, state: &mut AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(12.0)
        .outer_margin(4.0)
        .rounding(6.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // 左侧：版本信息
                ui.vertical(|ui| {
                    // 第一行：版本号 + 标签
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&install.version).size(16.0).strong());
                        
                        // 变体标签
                        let variant_label = match install.variant {
                            GodotVariant::Mono => "Mono",
                            _ => "Standard"
                        };
                        ui.label(
                            RichText::new(variant_label)
                                .small()
                                .background_color(Color32::from_rgb(100, 100, 100))
                        );
                        
                        // 收藏标签
                        if install.is_favorite {
                            ui.label(RichText::new("⭐ Favorite").small());
                        }
                    });
                    
                    // 第二行：路径信息
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("📂 {}", install.path.display()))
                            .small()
                            .weak()
                            .code()
                    );
                    
                    // 第三行：使用时间
                    if let Some(last_used) = &install.last_used {
                        ui.label(
                            RichText::new(format!("Last used: {}", last_used.format("%Y-%m-%d")))
                                .small()
                                .weak()
                        );
                    }
                });
                
                // 右侧：操作按钮
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 更多操作菜单
                    ui.menu_button("⋮", |ui| {
                        if ui.button("Open Folder").clicked() {
                            // 打开文件夹
                        }
                        if ui.button("Create Shortcut").clicked() {
                            // 创建快捷方式
                        }
                        ui.separator();
                        if ui.button(RichText::new("Remove").color(Color32::RED)).clicked() {
                            // 删除
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    // 运行按钮
                    let run_btn = egui::Button::new("▶ Run")
                        .fill(Color32::from_rgb(46, 139, 87));
                    if ui.add(run_btn).clicked() {
                        services::launch_godot(&install.path).ok();
                    }
                });
            });
        });
}
```

### 问题 3: 下载进度条显示错误

#### 当前问题
```rust
// 当前代码问题：进度条没有添加到 UI
ui.vertical(|ui| {
    ui.label(format!("{}%", progress_percent));
    egui::ProgressBar::new(progress)
        .desired_width(150.0)
        .animate(true);
});
```

#### 优化方案
```rust
// 正确的进度条实现
fn draw_download_progress(ui: &mut Ui, progress: f32) {
    ui.vertical(|ui| {
        // 进度条
        ui.add(
            egui::ProgressBar::new(progress)
                .desired_width(150.0)
                .text(format!("{:.0}%", progress * 100.0))
                .animate(true)
        );
        
        // 进度详情
        ui.label(
            RichText::new(format!("Downloading... {:.0}%", progress * 100.0))
                .small()
        );
    });
}
```

---

## 🎨 样式配置

### egui 主题配置

```rust
// 自定义样式配置
fn configure_visuals(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // 间距配置
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.interact_size = egui::vec2(40.0, 20.0);
    
    // 视觉配置
    style.visuals.button_frame = true;
    style.visuals.collapsing_header_frame = true;
    
    // 颜色配置
    style.visuals.selection.bg_fill = Color32::from_rgb(70, 130, 180);
    style.visuals.hyperlink_color = Color32::from_rgb(70, 130, 180);
    
    ctx.set_style(style);
}
```

### 暗色主题（默认）

```rust
fn dark_theme() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();
    
    visuals.window_fill = Color32::from_rgb(25, 25, 25);
    visuals.panel_fill = Color32::from_rgb(30, 30, 30);
    visuals.extreme_bg_color = Color32::from_rgb(20, 20, 20);
    
    visuals
}
```

### 亮色主题

```rust
fn light_theme() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();
    
    visuals.window_fill = Color32::from_rgb(245, 245, 245);
    visuals.panel_fill = Color32::from_rgb(240, 240, 240);
    
    visuals
}
```

---

## 📱 响应式设计

### 断点定义

```rust
// 窗口尺寸断点
const MIN_WIDTH: f32 = 800.0;
const MOBILE_WIDTH: f32 = 600.0;
const TABLET_WIDTH: f32 = 900.0;
const DESKTOP_WIDTH: f32 = 1200.0;

// 根据宽度调整布局
fn get_layout_mode(width: f32) -> LayoutMode {
    if width < MOBILE_WIDTH {
        LayoutMode::Mobile
    } else if width < TABLET_WIDTH {
        LayoutMode::Tablet
    } else {
        LayoutMode::Desktop
    }
}

enum LayoutMode {
    Mobile,   // 隐藏侧边栏，使用抽屉式菜单
    Tablet,   // 缩小侧边栏宽度
    Desktop,  // 完整布局
}
```

### 自适应组件

```rust
// 根据宽度调整卡片布局
fn draw_versions_grid(ui: &mut Ui, versions: &[GodotInstall]) {
    let available_width = ui.available_width();
    let columns = if available_width > 800.0 { 2 } else { 1 };
    
    egui::Grid::new("versions_grid")
        .num_columns(columns)
        .spacing([16.0, 16.0])
        .show(ui, |ui| {
            for version in versions {
                draw_version_card(ui, version);
                ui.end_row();
            }
        });
}
```

---

## 🔧 可复用组件库

### 1. 信息卡片组件

```rust
struct InfoCard {
    title: String,
    value: String,
    icon: String,
    color: Color32,
}

impl InfoCard {
    fn show(&self, ui: &mut Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(12.0)
            .outer_margin(4.0)
            .rounding(8.0)
            .fill(self.color.linear_multiply(0.1))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.icon);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(&self.title).small().weak());
                        ui.label(RichText::new(&self.value).size(20.0).strong());
                    });
                });
            });
    }
}
```

### 2. 状态标签组件

```rust
enum Status {
    Stable,
    Beta,
    Dev,
    Installed,
    Downloading,
}

fn draw_status_tag(ui: &mut Ui, status: Status) {
    let (text, color) = match status {
        Status::Stable => ("Stable", Color32::from_rgb(46, 139, 87)),
        Status::Beta => ("Beta", Color32::from_rgb(255, 165, 0)),
        Status::Dev => ("Dev", Color32::from_rgb(220, 53, 69)),
        Status::Installed => ("Installed", Color32::from_rgb(70, 130, 180)),
        Status::Downloading => ("Downloading", Color32::from_rgb(128, 128, 128)),
    };
    
    ui.label(
        RichText::new(text)
            .small()
            .background_color(color.linear_multiply(0.3))
            .color(color)
    );
}
```

### 3. 空状态组件

```rust
struct EmptyState {
    icon: String,
    title: String,
    description: String,
    action_text: Option<String>,
}

impl EmptyState {
    fn show(&self, ui: &mut Ui) -> Option<()> {
        ui.vertical_centered(|ui| {
            ui.add_space(48.0);
            
            ui.label(RichText::new(&self.icon).size(64.0).weak());
            ui.add_space(16.0);
            
            ui.label(RichText::new(&self.title).size(18.0).strong());
            ui.add_space(8.0);
            
            ui.label(RichText::new(&self.description).weak());
            ui.add_space(16.0);
            
            if let Some(text) = &self.action_text {
                if ui.button(text).clicked() {
                    return Some(());
                }
            }
            
            None
        }).inner
    }
}
```

---

## 📝 最佳实践

### 1. 保持一致性
- 相同功能的按钮使用相同的图标和文本
- 保持相同的间距和对齐方式
- 使用统一的颜色系统

### 2. 提供反馈
```rust
// 按钮点击反馈
if ui.button("Save").clicked() {
    // 立即更新 UI
    state.saved = true;
    ui.ctx().request_repaint();
    
    // 异步执行保存操作
    save_state_async(state.clone());
}

// 悬停提示
ui.add_enabled(false, egui::Button::new("Download"))
    .on_hover_text("Select a version first");
```

### 3. 处理长文本
```rust
// 路径截断
let path_text = if path.display().to_string().len() > 40 {
    format!("...{}", &path.display().to_string()[path.display().to_string().len()-37..])
} else {
    path.display().to_string()
};

ui.label(RichText::new(&path_text).code())
    .on_hover_text(path.display().to_string());  // 完整路径在悬停时显示
```

### 4. 防止 UI 阻塞
```rust
// 使用异步操作
fn start_download(version: GodotVersion, ctx: egui::Context) {
    std::thread::spawn(move || {
        // 执行下载
        for progress in 0..=100 {
            std::thread::sleep(Duration::from_millis(50));
            
            // 更新 UI
            ctx.request_repaint();
        }
    });
}
```

---

## 📋 UI 检查清单

### 每个面板应该包含
- [ ] 清晰的标题和描述
- [ ] 合理的默认布局
- [ ] 响应式设计
- [ ] 空状态提示
- [ ] 加载状态指示
- [ ] 错误状态处理
- [ ] 工具提示

### 每个交互元素应该
- [ ] 有明确的视觉状态（悬停、按下、禁用）
- [ ] 提供即时反馈
- [ ] 有合理的键盘支持
- [ ] 包含工具提示（如果需要）

### 对话框应该
- [ ] 可以通过 Esc 键关闭
- [ ] 有明确的操作按钮
- [ ] 合理的默认焦点
- [ ] 清晰的内容层次

---

## 🚀 未来改进方向

### 短期目标 (v0.2.0)
1. 完成侧边栏视觉优化
2. 实现卡片式版本展示
3. 修复进度条显示问题
4. 添加工具提示系统

### 中期目标 (v0.3.0)
1. 实现主题切换功能
2. 添加响应式布局
3. 完善键盘导航
4. 优化动画效果

### 长期目标 (v1.0.0)
1. 自定义主题编辑器
2. 无障碍访问支持
3. 高 DPI 支持
4. 多窗口支持

---

## 📚 参考资料

- [egui 官方文档](https://docs.rs/egui/)
- [egui 演示应用](https://www.egui.rs/)
- [Godot Hub 设计参考](https://github.com/godotengine/godot)
- [Material Design 指南](https://material.io/design)
- [Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)

---

*最后更新: 2025-01-16*
*文档版本: 1.0*
*维护者: Godot Hub 开发团队*