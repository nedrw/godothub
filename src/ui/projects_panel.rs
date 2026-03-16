// ProjectsPanel - 项目管理面板 UI 组件
// 优化版本：使用统一样式系统、卡片式布局、清晰信息层次、支持主题切换

use egui::{RichText, ScrollArea, Stroke, Vec2};

use crate::state::{AppState, Theme};
use crate::ui::style::{
    badge, card_frame, danger_button, empty_state, panel_header, path_label, primary_button,
    section_header, spacing, status_pill, success_button, ThemeColors,
};

/// 项目信息
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    /// 项目名称
    pub name: String,
    /// 项目路径
    pub path: std::path::PathBuf,
    /// Godot 版本
    pub godot_version: String,
    /// 是否收藏
    pub is_favorite: bool,
    /// 最后打开时间
    pub last_opened: Option<chrono::DateTime<chrono::Utc>>,
}

#[allow(dead_code)]
impl ProjectInfo {
    /// 创建新项目
    pub fn new(name: String, path: std::path::PathBuf, godot_version: String) -> Self {
        Self {
            name,
            path,
            godot_version,
            is_favorite: false,
            last_opened: None,
        }
    }

    /// 检查项目是否存在
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// 获取 project.godot 文件路径
    pub fn project_file_path(&self) -> std::path::PathBuf {
        self.path.join("project.godot")
    }

    /// 检查是否为有效的 Godot 项目
    pub fn is_valid_godot_project(&self) -> bool {
        self.project_file_path().exists()
    }
}

/// 绘制项目管理面板
pub fn draw_projects_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let theme = state.config.theme;
    let colors = ThemeColors::from_theme(theme);

    // 设置面板背景
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.inner_margin(16.0))
        .show_inside(ui, |ui| {
            // 顶部标题区域
            egui::TopBottomPanel::top("projects_header")
                .frame(egui::Frame::NONE)
                .show_inside(ui, |ui| {
                    draw_panel_header(ui, state, &colors);
                });

            ui.add_space(16.0);

            // 主内容区域
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    // 操作按钮区域
                    draw_action_buttons(ui, state, &colors);

                    ui.add_space(24.0);

                    // 项目列表
                    draw_projects_list(ui, state, &colors);
                });
        });
}

/// 绘制面板头部
fn draw_panel_header(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        panel_header(
            ui,
            state.config.theme,
            "Projects",
            "Manage your Godot projects",
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 扫描按钮
            let scan_btn = egui::Button::new(RichText::new("🔍 Scan").color(colors.text_primary))
                .fill(colors.bg_secondary)
                .stroke(Stroke::new(1.0, colors.border))
                .min_size(Vec2::new(100.0, spacing::BUTTON_HEIGHT));

            let response = ui
                .add(scan_btn)
                .on_hover_text("Scan projects directory for Godot projects");

            if response.clicked() {
                // TODO: 实现项目扫描功能
                log::info!("Scanning projects directory");
            }
        });
    });
}

/// 绘制操作按钮区域
fn draw_action_buttons(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        // 新建项目按钮
        let new_btn = success_button("➕ New Project");
        let response = ui.add(new_btn).on_hover_text("Create a new Godot project");

        if response.clicked() {
            log::info!("New project button clicked");
            // TODO: 实现新建项目功能
        }

        ui.add_space(8.0);

        // 导入项目按钮
        let import_btn = primary_button("📂 Import Project", state.config.theme);
        let response = ui
            .add(import_btn)
            .on_hover_text("Import an existing Godot project");

        if response.clicked() {
            log::info!("Import project button clicked");
            // TODO: 实现导入项目功能
        }

        ui.add_space(8.0);

        // 打开项目目录按钮
        let open_dir_btn =
            egui::Button::new(RichText::new("📁 Open Projects Folder").color(colors.text_primary))
                .fill(colors.bg_secondary)
                .stroke(Stroke::new(1.0, colors.border))
                .min_size(Vec2::new(160.0, spacing::BUTTON_HEIGHT));

        let response = ui
            .add(open_dir_btn)
            .on_hover_text("Open projects directory in file manager");

        if response.clicked() {
            // 打开项目目录
            open_folder(&state.config.projects_dir);
        }
    });
}

/// 绘制项目列表
fn draw_projects_list(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    // 扫描项目
    let projects = scan_projects_directory(&state.config.projects_dir);

    ui.vertical(|ui| {
        // 区域标题
        section_header(
            ui,
            state.config.theme,
            "📁",
            "Recent Projects",
            Some(projects.len()),
        );

        ui.add_space(12.0);

        if projects.is_empty() {
            draw_empty_projects_state(ui, state, colors);
        } else {
            // 显示项目列表
            for (index, project) in projects.iter().enumerate() {
                draw_project_item(ui, project, index, state.config.theme, colors);
                ui.add_space(12.0);
            }
        }
    });
}

/// 绘制空项目状态
fn draw_empty_projects_state(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    empty_state(
        ui,
        state.config.theme,
        "📁",
        "No Projects Found",
        "Create a new project or import an existing one to get started",
        Some("➕ Create Project"),
        Some(&mut || {
            log::info!("Create project from empty state");
        }),
    );

    ui.add_space(16.0);

    // 提示信息
    egui::Frame::NONE
        .fill(colors.bg_secondary)
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("💡").size(20.0));

                ui.add_space(8.0);

                ui.label(
                    RichText::new("Tip: You can change the projects directory in Settings")
                        .size(12.0)
                        .color(colors.text_secondary),
                );
            });
        });
}

/// 绘制项目项
fn draw_project_item(
    ui: &mut egui::Ui,
    project: &ProjectInfo,
    _index: usize,
    theme: Theme,
    colors: &ThemeColors,
) {
    card_frame(theme).show(ui, |ui| {
        ui.horizontal(|ui| {
            // 左侧：项目图标
            ui.vertical(|ui| {
                ui.add_space(4.0);
                ui.label(RichText::new(if project.is_favorite { "⭐" } else { "📁" }).size(28.0));
            });

            ui.add_space(12.0);

            // 中间：项目信息
            ui.vertical(|ui| {
                // 第一行：项目名称 + 标签
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&project.name)
                            .size(16.0)
                            .strong()
                            .color(colors.text_primary),
                    );

                    ui.add_space(8.0);

                    // Godot 版本标签
                    status_pill(ui, &project.godot_version, colors.accent_blue);

                    // 收藏标签
                    if project.is_favorite {
                        ui.add_space(4.0);
                        badge(ui, "⭐ Favorite", colors.warning);
                    }
                });

                ui.add_space(6.0);

                // 第二行：路径
                let path_str = project.path.display().to_string();
                path_label(ui, theme, &path_str, 60);

                ui.add_space(4.0);

                // 第三行：最后打开时间
                if let Some(last_opened) = &project.last_opened {
                    ui.label(
                        RichText::new(format!(
                            "🕐 Last opened: {}",
                            last_opened.format("%Y-%m-%d %H:%M")
                        ))
                        .small()
                        .color(colors.text_secondary),
                    );
                }
            });

            // 右侧：操作按钮
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 更多操作菜单
                draw_project_menu(ui, project, colors);

                ui.add_space(8.0);

                // 打开按钮
                let open_btn = success_button("▶ Open");
                let response = ui
                    .add(open_btn)
                    .on_hover_text(format!("Open project in Godot {}", project.godot_version));

                if response.clicked() {
                    log::info!("Opening project: {:?}", project.path);
                    // TODO: 实现打开项目功能
                }
            });
        });
    });
}

/// 绘制项目操作菜单
fn draw_project_menu(ui: &mut egui::Ui, project: &ProjectInfo, _colors: &ThemeColors) {
    ui.menu_button("⋮", |ui| {
        ui.set_min_width(140.0);

        // 打开文件夹
        if ui.button("📂 Open Folder").clicked() {
            open_folder(&project.path);
            ui.close_menu();
        }

        // 切换收藏
        let favorite_text = if project.is_favorite {
            "☆ Remove from Favorites"
        } else {
            "★ Add to Favorites"
        };

        if ui.button(favorite_text).clicked() {
            // TODO: 实现收藏切换
            log::info!("Toggle favorite for project: {:?}", project.path);
            ui.close_menu();
        }

        ui.separator();

        // 删除操作（危险操作）
        let delete_btn = danger_button("🗑 Remove");
        if ui.add(delete_btn).clicked() {
            // TODO: 显示确认对话框
            log::warn!("Remove project requested: {:?}", project.path);
            ui.close_menu();
        }
    });
}

/// 打开文件夹（跨平台）
fn open_folder(path: &std::path::Path) {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn().ok();
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .ok();
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .ok();
    }
}

/// 扫描项目目录，查找 Godot 项目
pub fn scan_projects_directory(projects_dir: &std::path::Path) -> Vec<ProjectInfo> {
    let mut projects = Vec::new();

    if !projects_dir.exists() {
        return projects;
    }

    // 读取目录
    if let Ok(entries) = std::fs::read_dir(projects_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // 只处理目录
            if !path.is_dir() {
                continue;
            }

            // 检查是否包含 project.godot 文件
            let project_file = path.join("project.godot");
            if !project_file.exists() {
                continue;
            }

            // 获取项目名称
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            // 解析 Godot 版本（简化版，实际需要解析 project.godot 文件）
            let godot_version = parse_godot_version(&project_file);

            projects.push(ProjectInfo::new(name, path, godot_version));
        }
    }

    projects
}

/// 从 project.godot 文件解析 Godot 版本
fn parse_godot_version(_project_file: &std::path::Path) -> String {
    // 简化实现：返回默认版本
    // TODO: 实际解析 project.godot 文件中的 config_version
    "4.x".to_string()
}
