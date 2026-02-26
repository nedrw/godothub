// ProjectsPanel - 项目管理面板 UI 组件
// 优化版本：卡片式布局、统一设计风格、改进交互体验

use egui::{Color32, RichText, ScrollArea, Stroke, Vec2};

use crate::state::AppState;

/// 项目信息结构体
///
/// 用于显示项目中收集到的基本信息
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    /// 项目名称
    pub name: String,
    /// 项目路径
    pub path: std::path::PathBuf,
    /// 使用的 Godot 版本
    pub godot_version: Option<String>,
    /// 是否为 favorite
    pub is_favorite: bool,
    /// 最后打开时间
    pub last_opened: Option<chrono::DateTime<chrono::Utc>>,
}

impl ProjectInfo {
    /// 创建新的项目信息
    pub fn new(name: String, path: std::path::PathBuf) -> Self {
        Self {
            name,
            path,
            godot_version: None,
            is_favorite: false,
            last_opened: None,
        }
    }

    /// 检查项目目录是否存在
    pub fn exists(&self) -> bool {
        self.path.exists() && self.path.is_dir()
    }

    /// 获取项目描述文件路径
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
    // 顶部标题区域
    egui::TopBottomPanel::top("projects_header")
        .frame(egui::Frame::NONE.inner_margin(egui::Margin::same(16)))
        .show_inside(ui, |ui| {
            draw_panel_header(ui, state);
        });

    // 主内容区域
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.add_space(8.0);

            // 操作按钮区域
            draw_action_buttons(ui, state);

            ui.add_space(16.0);

            // 项目列表
            draw_projects_list(ui, state);

            ui.add_space(16.0);
        });
}

/// 绘制面板头部
fn draw_panel_header(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Projects");
            ui.label(
                RichText::new("Manage your Godot projects")
                    .small()
                    .weak()
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 扫描按钮
            let scan_btn = egui::Button::new("🔍 Scan")
                .fill(Color32::from_rgb(70, 130, 180));

            let mut response = ui.add(scan_btn);
            response = response.on_hover_text("Scan projects directory for Godot projects");

            if response.clicked() {
                // TODO: 实现项目扫描功能
                log::info!("Scanning projects directory");
            }
        });
    });
}

/// 绘制操作按钮区域
fn draw_action_buttons(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.horizontal(|ui| {
        // 新建项目按钮
        let new_btn = egui::Button::new("➕ New Project")
            .fill(Color32::from_rgb(46, 139, 87))
            .min_size(Vec2::new(120.0, 32.0));

        let mut response = ui.add(new_btn);
        response = response.on_hover_text("Create a new Godot project");

        if response.clicked() {
            log::info!("New project button clicked");
            // TODO: 实现新建项目功能
        }

        ui.add_space(8.0);

        // 导入项目按钮
        let import_btn = egui::Button::new("📂 Import Project")
            .fill(Color32::from_rgb(70, 130, 180))
            .min_size(Vec2::new(130.0, 32.0));

        let mut response = ui.add(import_btn);
        response = response.on_hover_text("Import an existing Godot project");

        if response.clicked() {
            log::info!("Import project button clicked");
            // TODO: 实现导入项目功能
        }

        ui.add_space(8.0);

        // 打开项目目录按钮
        let open_dir_btn = egui::Button::new("📁 Open Projects Folder")
            .min_size(Vec2::new(150.0, 32.0));

        let mut response = ui.add(open_dir_btn);
        response = response.on_hover_text("Open projects directory in file manager");

        if response.clicked() {
            // 打开项目目录
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .arg(&_state.config.projects_dir)
                    .spawn()
                    .ok();
            }

            #[cfg(target_os = "linux")]
            {
                std::process::Command::new("xdg-open")
                    .arg(&_state.config.projects_dir)
                    .spawn()
                    .ok();
            }

            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer")
                    .arg(&_state.config.projects_dir)
                    .spawn()
                    .ok();
            }
        }
    });
}

/// 绘制项目列表
fn draw_projects_list(ui: &mut egui::Ui, state: &mut AppState) {
    // 扫描项目
    let projects = scan_projects_directory(&state.config.projects_dir);

    ui.vertical(|ui| {
        // 区域标题
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("📁 Recent Projects")
                    .size(16.0)
                    .strong()
            );

            ui.add_space(8.0);

            ui.label(
                RichText::new(format!("({})", projects.len()))
                    .small()
                    .weak()
            );
        });

        ui.add_space(8.0);

        if projects.is_empty() {
            draw_empty_projects_state(ui, state);
        } else {
            // 显示项目列表
            for (index, project) in projects.iter().enumerate() {
                draw_project_item(ui, project, index);
                ui.add_space(8.0);
            }
        }
    });
}

/// 绘制空项目状态
fn draw_empty_projects_state(ui: &mut egui::Ui, state: &AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(32.0)
        .outer_margin(0.0)
        .corner_radius(8.0)
        .fill(Color32::from_rgba_unmultiplied(128, 128, 128, 15))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.0);

                ui.label(
                    RichText::new("📁")
                        .size(64.0)
                        .weak()
                );

                ui.add_space(16.0);

                ui.label(
                    RichText::new("No Projects Found")
                        .size(18.0)
                        .strong()
                );

                ui.add_space(8.0);

                ui.label(
                    RichText::new("Create a new project or import an existing one to get started")
                        .weak()
                );

                ui.add_space(24.0);

                ui.horizontal(|ui| {
                    // 新建项目按钮
                    let new_btn = egui::Button::new("➕ New Project")
                        .fill(Color32::from_rgb(46, 139, 87))
                        .min_size(Vec2::new(120.0, 36.0));

                    if ui.add(new_btn).clicked() {
                        log::info!("New project from empty state");
                        // TODO: 实现新建项目功能
                    }

                    ui.add_space(12.0);

                    // 导入项目按钮
                    let import_btn = egui::Button::new("📂 Import Project")
                        .fill(Color32::from_rgb(70, 130, 180))
                        .min_size(Vec2::new(130.0, 36.0));

                    if ui.add(import_btn).clicked() {
                        log::info!("Import project from empty state");
                        // TODO: 实现导入项目功能
                    }
                });

                ui.add_space(16.0);

                // 显示项目目录路径
                ui.label(
                    RichText::new(format!(
                        "Projects directory: {}",
                        state.config.projects_dir.display()
                    ))
                    .small()
                    .weak()
                    .code()
                );
            });
        });
}

/// 绘制单个项目条目
fn draw_project_item(ui: &mut egui::Ui, project: &ProjectInfo, _index: usize) {
    egui::Frame::group(ui.style())
        .inner_margin(12.0)
        .outer_margin(0.0)
        .corner_radius(8.0)
        .stroke(Stroke::new(
            1.0,
            ui.style().visuals.widgets.noninteractive.bg_stroke.color
        ))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // 左侧：项目图标
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(if project.is_favorite { "⭐" } else { "🎮" })
                            .size(32.0)
                    );
                });

                ui.add_space(8.0);

                // 中间：项目信息
                ui.vertical(|ui| {
                    // 第一行：项目名称 + 标签
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&project.name)
                                .size(16.0)
                                .strong()
                        );

                        // 有效项目标签
                        if project.is_valid_godot_project() {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(" Valid")
                                    .small()
                                    .background_color(Color32::from_rgba_unmultiplied(46, 139, 87, 50))
                                    .color(Color32::from_rgb(46, 139, 87))
                            );
                        } else {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(" Invalid")
                                    .small()
                                    .background_color(Color32::from_rgba_unmultiplied(220, 53, 69, 50))
                                    .color(Color32::from_rgb(220, 53, 69))
                            );
                        }

                        // 收藏标签
                        if project.is_favorite {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(" Favorite")
                                    .small()
                                    .color(Color32::from_rgb(255, 193, 7))
                            );
                        }
                    });

                    ui.add_space(4.0);

                    // 第二行：路径
                    let path_str = project.path.display().to_string();
                    let display_path = if path_str.len() > 60 {
                        format!("...{}", &path_str[path_str.len()-57..])
                    } else {
                        path_str
                    };

                    ui.label(
                        RichText::new(format!("📂 {}", display_path))
                            .small()
                            .weak()
                            .code()
                    ).on_hover_text(project.path.display().to_string());

                    // 第三行：Godot 版本和最后打开时间
                    ui.horizontal(|ui| {
                        if let Some(ref version) = project.godot_version {
                            ui.label(
                                RichText::new(format!("🎮 Godot {}", version))
                                    .small()
                                    .weak()
                            );
                            ui.add_space(8.0);
                        }

                        if let Some(last_opened) = &project.last_opened {
                            ui.label(
                                RichText::new(format!(
                                    "🕐 Last opened: {}",
                                    last_opened.format("%Y-%m-%d %H:%M")
                                ))
                                .small()
                                .weak()
                            );
                        }
                    });
                });

                // 右侧：操作按钮
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 更多操作菜单
                    draw_project_menu(ui, project);

                    ui.add_space(8.0);

                    // 打开按钮
                    if project.is_valid_godot_project() {
                        let open_btn = egui::Button::new("▶ Open")
                            .fill(Color32::from_rgb(46, 139, 87))
                            .min_size(Vec2::new(64.0, 28.0));

                        let mut response = ui.add(open_btn);
                        response = response.on_hover_text(format!("Open project in Godot"));

                        if response.clicked() {
                            log::info!("Opening project: {}", project.name);
                            // TODO: 实现打开项目功能
                        }
                    } else {
                        ui.add_enabled(
                            false,
                            egui::Button::new("⚠ Invalid")
                                .min_size(Vec2::new(80.0, 28.0))
                        ).on_hover_text("Project is not a valid Godot project");
                    }
                });
            });
        });
}

/// 绘制项目操作菜单
fn draw_project_menu(ui: &mut egui::Ui, project: &ProjectInfo) {
    ui.menu_button("⋮", |ui| {
        ui.set_min_width(140.0);

        // 在文件管理器中显示
        if ui.button("📂 Show in Folder").clicked() {
            open_folder(&project.path);
            ui.close_menu();
        }

        // 使用终端打开
        #[cfg(not(windows))]
        {
            if ui.button("💻 Open in Terminal").clicked() {
                // TODO: 实现终端打开功能
                log::info!("Open in terminal: {:?}", project.path);
                ui.close_menu();
            }
        }

        ui.separator();

        // 复制路径
        if ui.button("📋 Copy Path").clicked() {
            ui.ctx().copy_text(project.path.display().to_string());
            log::info!("Project path copied to clipboard");
            ui.close_menu();
        }

        ui.separator();

        // 删除操作（危险操作）
        let delete_btn = egui::Button::new(
            RichText::new("🗑 Remove from List").color(Color32::from_rgb(220, 53, 69))
        );

        if ui.add(delete_btn).clicked() {
            // TODO: 显示确认对话框
            log::warn!("Remove project requested: {}", project.name);
            ui.close_menu();
        }
    });
}

/// 打开文件夹（跨平台）
fn open_folder(path: &std::path::Path) {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .ok();
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

/// 扫描项目目录获取项目列表
///
/// # Arguments
/// * `projects_dir` - 项目根目录
///
/// # Returns
/// * `Vec<ProjectInfo>` - 项目信息列表
pub fn scan_projects_directory(projects_dir: &std::path::Path) -> Vec<ProjectInfo> {
    let mut projects = Vec::new();

    if !projects_dir.exists() {
        return projects;
    }

    if let Ok(entries) = std::fs::read_dir(projects_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                // 检查是否为有效的 Godot 项目
                let project_file = path.join("project.godot");

                if project_file.exists() {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let mut project = ProjectInfo::new(name, path);

                    // 尝试读取 Godot 版本信息
                    if let Ok(contents) = std::fs::read_to_string(&project_file) {
                        // 简单解析 project.godot 文件
                        for line in contents.lines() {
                            if line.starts_with("config_version=") {
                                // 找到配置版本
                                break;
                            }
                        }
                    }

                    projects.push(project);
                }
            }
        }
    }

    // 按修改时间排序（最新的在前）
    projects.sort_by(|a, b| {
        b.path.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            .cmp(
                &a.path.metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
            )
    });

    projects
}

/// 创建示例项目（用于测试）
pub fn create_sample_projects() -> Vec<ProjectInfo> {
    vec![
        ProjectInfo {
            name: "My Awesome Game".to_string(),
            path: std::path::PathBuf::from("/home/user/Godot/MyAwesomeGame"),
            godot_version: Some("4.3".to_string()),
            is_favorite: true,
            last_opened: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
        },
        ProjectInfo {
            name: "Platformer Demo".to_string(),
            path: std::path::PathBuf::from("/home/user/Godot/PlatformerDemo"),
            godot_version: Some("4.2.2".to_string()),
            is_favorite: false,
            last_opened: Some(chrono::Utc::now() - chrono::Duration::days(3)),
        },
        ProjectInfo {
            name: "RPG Project".to_string(),
            path: std::path::PathBuf::from("/home/user/Godot/RPGProject"),
            godot_version: Some("3.5.3".to_string()),
            is_favorite: false,
            last_opened: None,
        },
    ]
}
