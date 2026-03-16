// ProjectsPanel - 项目管理面板 UI 组件
// 完整实现版本：parse_godot_version 真实解析、Open/New/Import 全部接入、
//              收藏/隐藏持久化、新建项目对话框

use std::path::{Path, PathBuf};

use ini::Ini;

use egui::{RichText, ScrollArea, Stroke, Vec2};

use crate::models::GodotVariant;
use crate::services::launch_godot_with_project;
use crate::state::{
    AppState, DeleteProjectConfirmState, NewProjectDialogState, Theme, VersionMismatchConfirmState,
};
use crate::ui::style::{
    badge, card_frame, danger_button, empty_state, panel_header, path_label, primary_button,
    section_header, spacing, status_pill, success_button, ThemeColors,
};
use crate::utils::open_folder;

// ============================================================================
// 数据模型
// ============================================================================

/// 版本匹配结果（包含匹配的安装版本和是否精确匹配的标志）
#[derive(Debug, Clone)]
pub struct VersionMatch<'a> {
    /// 匹配到的安装版本
    pub install: &'a crate::models::GodotInstall,
    /// 是否精确匹配（版本号完全相同）
    pub is_exact_match: bool,
}

/// 项目信息（由 collect_projects 从磁盘扫描 + 元数据合并生成）
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub path: PathBuf,
    /// 从 project.godot 解析的 Godot 版本（如 "4.3"、"3.x"）
    pub godot_version: String,
    pub is_favorite: bool,
    pub last_opened: Option<chrono::DateTime<chrono::Utc>>,
    /// 是否为手动导入的项目（不在 projects_dir 下）
    pub is_imported: bool,
}

/// 项目动作（从项目卡片触发）
#[derive(Debug, Clone)]
pub enum ProjectAction {
    /// 打开项目（路径, Godot版本）
    Open(PathBuf, String),
    /// 切换收藏状态
    ToggleFavorite(PathBuf),
    /// 从列表中移除（仅隐藏，不删除文件）
    Hide(PathBuf),
    /// 删除项目文件（需要二次确认）
    DeleteFiles(PathBuf),
    /// 打开项目所在文件夹
    OpenFolder(PathBuf),
}

// ============================================================================
// 顶层入口
// ============================================================================

/// 绘制项目管理面板
pub fn draw_projects_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let theme = state.config.theme;
    let colors = ThemeColors::from_theme(theme);
    let has_new_dialog = state.new_project_dialog.is_some();

    // 先收集需要显示的确认对话框信息（避免借用冲突）
    let version_mismatch_confirm = state.version_mismatch_confirm.clone();
    let delete_project_confirm = state.delete_project_confirm.clone();

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.inner_margin(16.0))
        .show_inside(ui, |ui| {
            egui::TopBottomPanel::top("projects_header")
                .frame(egui::Frame::NONE)
                .show_inside(ui, |ui| {
                    draw_panel_header(ui, state, &colors);
                });

            ui.add_space(16.0);

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    draw_action_buttons(ui, state, &colors);
                    ui.add_space(24.0);
                    draw_projects_list(ui, state, &colors);
                });
        });

    // 新建项目对话框（浮动窗口，在 CentralPanel 外绘制）
    if has_new_dialog {
        draw_new_project_dialog(ui.ctx(), state, &colors);
    }

    // 显示版本不匹配确认对话框
    if let Some(ref confirm) = version_mismatch_confirm {
        draw_version_mismatch_confirm_dialog(ui.ctx(), confirm, state, &colors);
    }

    // 显示项目删除确认对话框
    if let Some(ref confirm) = delete_project_confirm {
        draw_delete_project_confirm_dialog(ui.ctx(), confirm, state, &colors);
    }
}

// ============================================================================
// 面板头部
// ============================================================================

fn draw_panel_header(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        panel_header(
            ui,
            state.config.theme,
            "Projects",
            "Manage your Godot projects",
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let scan_btn = egui::Button::new(RichText::new("Scan").color(colors.text_primary))
                .fill(colors.bg_secondary)
                .stroke(Stroke::new(1.0, colors.border))
                .min_size(Vec2::new(100.0, spacing::BUTTON_HEIGHT));

            if ui
                .add(scan_btn)
                .on_hover_text("Rescan projects directory")
                .clicked()
            {
                log::info!("Manual rescan triggered");
            }
        });
    });
}

// ============================================================================
// 操作按钮区域
// ============================================================================

fn draw_action_buttons(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        // 新建项目
        if ui
            .add(success_button("+ New Project"))
            .on_hover_text("Create a new Godot project")
            .clicked()
        {
            state.new_project_dialog = Some(NewProjectDialogState::new(
                state.config.projects_dir.clone(),
            ));
        }

        ui.add_space(8.0);

        // 导入项目
        if ui
            .add(primary_button("Import Project", state.config.theme))
            .on_hover_text("Import an existing Godot project from any directory")
            .clicked()
        {
            import_project(state);
        }

        ui.add_space(8.0);

        // 打开项目目录
        let open_dir_btn =
            egui::Button::new(RichText::new("Open Projects Folder").color(colors.text_primary))
                .fill(colors.bg_secondary)
                .stroke(Stroke::new(1.0, colors.border))
                .min_size(Vec2::new(160.0, spacing::BUTTON_HEIGHT));

        if ui
            .add(open_dir_btn)
            .on_hover_text("Open projects directory in file manager")
            .clicked()
        {
            open_folder(&state.config.projects_dir);
        }
    });
}

/// 使用 rfd 弹出文件夹选择框导入外部 Godot 项目
fn import_project(state: &mut AppState) {
    let picked = rfd::FileDialog::new()
        .set_title("Select Godot Project Folder")
        .pick_folder();

    if let Some(path) = picked {
        let project_file = path.join("project.godot");
        if project_file.exists() {
            log::info!("Importing project from: {}", path.display());
            state.project_meta_store.add_imported_path(path);
            state.project_meta_store.save_quiet();
        } else {
            log::warn!(
                "Selected folder is not a valid Godot project (missing project.godot): {}",
                path.display()
            );
        }
    }
}

// ============================================================================
// 项目列表
// ============================================================================

fn draw_projects_list(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    let projects = collect_projects(state);

    ui.vertical(|ui| {
        section_header(
            ui,
            state.config.theme,
            "●",
            "Projects",
            Some(projects.len()),
        );
        ui.add_space(12.0);

        if projects.is_empty() {
            draw_empty_projects_state(ui, state, colors);
        } else {
            // 收集动作后统一处理，避免借用冲突
            let mut pending_action: Option<ProjectAction> = None;

            for (index, project) in projects.iter().enumerate() {
                if let Some(act) = draw_project_item(ui, project, index, state.config.theme, colors)
                {
                    pending_action = Some(act);
                    break;
                }
                ui.add_space(12.0);
            }

            if let Some(act) = pending_action {
                handle_project_action(act, state);
            }
        }
    });
}

fn draw_empty_projects_state(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    let mut open_new = false;

    empty_state(
        ui,
        state.config.theme,
        "●",
        "No Projects Found",
        "Create a new project or import an existing one to get started",
        Some("Create Project"),
        Some(&mut || {
            open_new = true;
        }),
    );

    if open_new {
        state.new_project_dialog = Some(NewProjectDialogState::new(
            state.config.projects_dir.clone(),
        ));
    }

    ui.add_space(16.0);

    egui::Frame::NONE
        .fill(colors.bg_secondary)
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Tip:").size(20.0));
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Tip: You can change the projects directory in Settings")
                        .size(12.0)
                        .color(colors.text_secondary),
                );
            });
        });
}

/// 绘制单个项目卡片，返回用户触发的动作（如有）
fn draw_project_item(
    ui: &mut egui::Ui,
    project: &ProjectInfo,
    _index: usize,
    theme: Theme,
    colors: &ThemeColors,
) -> Option<ProjectAction> {
    let mut action: Option<ProjectAction> = None;

    card_frame(theme).show(ui, |ui| {
        ui.horizontal(|ui| {
            // 左侧：图标
            ui.vertical(|ui| {
                ui.add_space(4.0);
                ui.label(RichText::new(if project.is_favorite { "★" } else { "D" }).size(28.0));
            });

            ui.add_space(12.0);

            // 中间：项目信息
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&project.name)
                            .size(16.0)
                            .strong()
                            .color(colors.text_primary),
                    );
                    ui.add_space(8.0);
                    status_pill(ui, &project.godot_version, colors.accent_blue);

                    if project.is_favorite {
                        ui.add_space(4.0);
                        badge(ui, "★ Favorite", colors.warning);
                    }
                    if project.is_imported {
                        ui.add_space(4.0);
                        badge(ui, "● Imported", colors.badge_purple);
                    }
                });

                ui.add_space(6.0);
                let path_str = project.path.display().to_string();
                path_label(ui, theme, &path_str, 60);
                ui.add_space(4.0);

                if let Some(last_opened) = &project.last_opened {
                    ui.label(
                        RichText::new(format!(
                            "Last opened: {}",
                            last_opened.format("%Y-%m-%d %H:%M")
                        ))
                        .small()
                        .color(colors.text_secondary),
                    );
                }
            });

            // 右侧：操作按钮
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(act) = draw_project_menu(ui, project) {
                    action = Some(act);
                }

                ui.add_space(8.0);

                if ui
                    .add(success_button("Open"))
                    .on_hover_text(format!("Open in Godot {}", project.godot_version))
                    .clicked()
                {
                    action = Some(ProjectAction::Open(
                        project.path.clone(),
                        project.godot_version.clone(),
                    ));
                }
            });
        });
    });

    action
}

fn draw_project_menu(ui: &mut egui::Ui, project: &ProjectInfo) -> Option<ProjectAction> {
    let mut action: Option<ProjectAction> = None;

    ui.menu_button("...", |ui| {
        ui.set_min_width(180.0);

        if ui.button("Open Folder").clicked() {
            action = Some(ProjectAction::OpenFolder(project.path.clone()));
            ui.close_menu();
        }

        let fav_text = if project.is_favorite {
            "☆ Remove from Favorites"
        } else {
            "★ Add to Favorites"
        };
        if ui.button(fav_text).clicked() {
            action = Some(ProjectAction::ToggleFavorite(project.path.clone()));
            ui.close_menu();
        }

        ui.separator();

        // 从列表中移除（仅隐藏，不删除文件）
        if ui.button("Remove from List").clicked() {
            action = Some(ProjectAction::Hide(project.path.clone()));
            ui.close_menu();
        }

        // 删除项目文件（需要二次确认）
        ui.add_space(4.0);
        if ui.add(danger_button("Delete Project Files")).clicked() {
            action = Some(ProjectAction::DeleteFiles(project.path.clone()));
            ui.close_menu();
        }
    });

    action
}

/// 执行项目动作（在 UI 迭代结束后调用）
fn handle_project_action(action: ProjectAction, state: &mut AppState) {
    match action {
        ProjectAction::Open(path, version) => open_project(state, &path, &version),
        ProjectAction::ToggleFavorite(path) => {
            state.project_meta_store.toggle_favorite(&path);
            state.project_meta_store.save_quiet();
            log::info!(
                "Toggled favorite: {} → {}",
                path.display(),
                state.project_meta_store.is_favorite(&path)
            );
        }
        ProjectAction::Hide(path) => {
            state.project_meta_store.hide(&path);
            state
                .project_meta_store
                .imported_paths
                .retain(|p| p != &path);
            state.project_meta_store.save_quiet();
            log::info!("Removed project from list: {}", path.display());
        }
        ProjectAction::DeleteFiles(path) => {
            // 设置删除确认对话框状态，等待用户确认
            let project_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown Project")
                .to_string();
            state.delete_project_confirm = Some(DeleteProjectConfirmState {
                project_path: path,
                project_name,
            });
        }
        ProjectAction::OpenFolder(path) => open_folder(&path),
    }
}

/// 打开项目：找到匹配的 Godot 安装，以项目路径启动
fn open_project(state: &mut AppState, project_path: &Path, version: &str) {
    match find_best_godot_for_version(&state.installed_versions, version) {
        Some(match_result) => {
            // 如果版本精确匹配，直接打开项目
            if match_result.is_exact_match {
                let exec_path = match_result.install.path.clone();
                match launch_godot_with_project(&exec_path, project_path) {
                    Ok(_) => {
                        log::info!(
                            "Opened '{}' with Godot {}",
                            project_path.display(),
                            match_result.install.version
                        );
                        state.project_meta_store.update_last_opened(project_path);
                        state.project_meta_store.save_quiet();
                    }
                    Err(e) => log::error!("Failed to open project: {}", e),
                }
            } else {
                // 版本不匹配，弹出确认对话框
                state.version_mismatch_confirm = Some(VersionMismatchConfirmState {
                    project_path: project_path.to_path_buf(),
                    required_version: version.to_string(),
                    available_version: match_result.install.version.clone(),
                    install_path: match_result.install.path.clone(),
                });
            }
        }
        None => {
            log::warn!(
                "No installed Godot found for version '{}'. Please install a matching version first.",
                version
            );
        }
    }
}

/// 在已安装版本中找到最合适的 Godot（排除 ExportTemplates，优先 Standard，精确→prefix→major）
/// 返回 VersionMatch，包含匹配的安装版本和是否精确匹配的标志
fn find_best_godot_for_version<'a>(
    installed: &'a [crate::models::GodotInstall],
    version: &str,
) -> Option<VersionMatch<'a>> {
    let candidates: Vec<_> = installed
        .iter()
        .filter(|i| i.variant != GodotVariant::ExportTemplates)
        .collect();

    if candidates.is_empty() {
        return None;
    }

    // 精确匹配 + Standard 优先
    if let Some(m) = candidates
        .iter()
        .find(|i| i.version == version && i.variant == GodotVariant::Standard)
    {
        return Some(VersionMatch {
            install: m,
            is_exact_match: true,
        });
    }
    if let Some(m) = candidates.iter().find(|i| i.version == version) {
        return Some(VersionMatch {
            install: m,
            is_exact_match: true,
        });
    }

    // major.minor 前缀（非精确匹配）
    let prefix = version.split('.').take(2).collect::<Vec<_>>().join(".");
    if !prefix.is_empty() {
        if let Some(m) = candidates
            .iter()
            .find(|i| i.version.starts_with(&prefix) && i.variant == GodotVariant::Standard)
        {
            return Some(VersionMatch {
                install: m,
                is_exact_match: false,
            });
        }
        if let Some(m) = candidates.iter().find(|i| i.version.starts_with(&prefix)) {
            return Some(VersionMatch {
                install: m,
                is_exact_match: false,
            });
        }
    }

    // major 版本（非精确匹配）
    let major = version.split('.').next().unwrap_or("");
    if !major.is_empty() && major.chars().all(|c| c.is_ascii_digit()) {
        let mp = format!("{}.", major);
        if let Some(m) = candidates
            .iter()
            .find(|i| i.version.starts_with(&mp) && i.variant == GodotVariant::Standard)
        {
            return Some(VersionMatch {
                install: m,
                is_exact_match: false,
            });
        }
        if let Some(m) = candidates.iter().find(|i| i.version.starts_with(&mp)) {
            return Some(VersionMatch {
                install: m,
                is_exact_match: false,
            });
        }
    }

    None
}

// ============================================================================
// 新建项目对话框
// ============================================================================

fn draw_new_project_dialog(ctx: &egui::Context, state: &mut AppState, colors: &ThemeColors) {
    let mut close = false;
    let mut do_create = false;

    egui::Window::new("New Project")
        .collapsible(false)
        .resizable(false)
        .min_width(440.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            let dlg = match state.new_project_dialog.as_mut() {
                Some(d) => d,
                None => return,
            };

            ui.add_space(4.0);

            // 项目名称
            ui.label(
                RichText::new("Project Name")
                    .color(colors.text_primary)
                    .strong(),
            );
            ui.add_space(4.0);
            let name_resp = ui.add(
                egui::TextEdit::singleline(&mut dlg.name)
                    .hint_text("e.g. my_awesome_game")
                    .min_size(Vec2::new(420.0, 0.0)),
            );
            if name_resp.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                do_create = true;
            }

            ui.add_space(12.0);

            // 父目录
            ui.label(
                RichText::new("Parent Directory")
                    .color(colors.text_primary)
                    .strong(),
            );
            ui.add_space(4.0);

            let parent_dir_clone = dlg.parent_dir.clone();
            ui.horizontal(|ui| {
                let mut dir_str = parent_dir_clone.display().to_string();
                ui.add(
                    egui::TextEdit::singleline(&mut dir_str)
                        .hint_text("Select a folder…")
                        .min_size(Vec2::new(330.0, 0.0))
                        .interactive(false),
                );
                ui.add_space(4.0);
                if ui.button("Browse…").clicked() {
                    if let Some(picked) = rfd::FileDialog::new()
                        .set_directory(&parent_dir_clone)
                        .set_title("Choose Parent Directory")
                        .pick_folder()
                    {
                        if let Some(d) = state.new_project_dialog.as_mut() {
                            d.parent_dir = picked;
                        }
                    }
                }
            });

            ui.add_space(4.0);

            // 预览最终路径
            if let Some(d) = state.new_project_dialog.as_ref() {
                if !d.name.trim().is_empty() {
                    let preview = d.project_path().display().to_string();
                    ui.label(
                        RichText::new(format!("Will be created at: {}", preview))
                            .small()
                            .color(colors.text_secondary),
                    );
                }
            }

            ui.add_space(12.0);

            // Godot 版本下拉
            ui.label(
                RichText::new("Godot Version")
                    .color(colors.text_primary)
                    .strong(),
            );
            ui.add_space(4.0);

            let launchable: Vec<(usize, String)> = state
                .installed_versions
                .iter()
                .enumerate()
                .filter(|(_, i)| i.variant != GodotVariant::ExportTemplates)
                .map(|(idx, i)| (idx, format!("{} ({})", i.version, i.variant.name())))
                .collect();

            if launchable.is_empty() {
                ui.label(
                    RichText::new("No Godot versions installed. Please install one first.")
                        .color(colors.warning),
                );
            } else {
                let selected_label = state
                    .new_project_dialog
                    .as_ref()
                    .and_then(|d| d.selected_godot_index)
                    .and_then(|idx| launchable.iter().find(|(i, _)| *i == idx))
                    .map(|(_, lbl)| lbl.clone())
                    .unwrap_or_else(|| "Select a version…".to_string());

                egui::ComboBox::from_id_salt("new_project_godot_version")
                    .selected_text(selected_label)
                    .width(420.0)
                    .show_ui(ui, |ui| {
                        for (idx, label) in &launchable {
                            let dlg_ref = state.new_project_dialog.as_mut().unwrap();
                            let selected = dlg_ref.selected_godot_index == Some(*idx);
                            if ui.selectable_label(selected, label).clicked() {
                                dlg_ref.selected_godot_index = Some(*idx);
                            }
                        }
                    });
            }

            ui.add_space(12.0);

            // 错误提示
            if let Some(d) = state.new_project_dialog.as_ref() {
                if let Some(ref err) = d.error.clone() {
                    ui.label(RichText::new(format!("{}", err)).color(colors.error));
                    ui.add_space(8.0);
                }
            }

            // 底部按钮
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let can_create = state
                    .new_project_dialog
                    .as_ref()
                    .map(|d| d.is_valid())
                    .unwrap_or(false);

                let create_btn = if can_create {
                    success_button("✅ Create Project")
                } else {
                    egui::Button::new(RichText::new("✅ Create Project").color(colors.text_muted))
                        .fill(colors.bg_secondary)
                };

                if ui.add_enabled(can_create, create_btn).clicked() {
                    do_create = true;
                }

                ui.add_space(8.0);

                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });

    if do_create {
        execute_create_project(state);
    }
    if close {
        state.new_project_dialog = None;
    }
}

/// 绘制版本不匹配确认对话框
fn draw_version_mismatch_confirm_dialog(
    ctx: &egui::Context,
    confirm: &VersionMismatchConfirmState,
    state: &mut AppState,
    colors: &ThemeColors,
) {
    let mut close = false;
    let mut do_open = false;

    egui::Window::new("Version Mismatch")
        .collapsible(false)
        .resizable(false)
        .min_width(450.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // 警告图标
                ui.label(RichText::new("⚠").size(48.0));

                ui.add_space(12.0);

                // 标题
                ui.label(
                    RichText::new("Version Mismatch Warning")
                        .size(18.0)
                        .strong()
                        .color(colors.text_primary),
                );

                ui.add_space(8.0);

                // 警告信息
                ui.label(
                    RichText::new(format!(
                        "Project requires Godot {} but you only have {} installed.\n\nOpening it may upgrade your project files and could cause compatibility issues.\n\nConsider downloading Godot {} first.",
                        confirm.required_version,
                        confirm.available_version,
                        confirm.required_version
                    ))
                    .color(colors.text_secondary),
                );

                ui.add_space(16.0);

                // 按钮区域
                ui.horizontal(|ui| {
                    // 取消按钮
                    let cancel_btn = egui::Button::new(RichText::new("Cancel").color(colors.text_primary))
                        .fill(colors.bg_secondary)
                        .stroke(egui::Stroke::new(1.0, colors.border));

                    if ui.add(cancel_btn).clicked() {
                        close = true;
                    }

                    ui.add_space(12.0);

                    // 仍然打开按钮（警告样式）
                    let open_btn = danger_button("Open Anyway");

                    if ui.add(open_btn).clicked() {
                        do_open = true;
                        close = true;
                    }
                });
            });
        });

    if do_open {
        // 用户确认，使用选中的版本打开项目
        let exec_path = confirm.install_path.clone();
        let project_path = confirm.project_path.clone();
        match launch_godot_with_project(&exec_path, &project_path) {
            Ok(_) => {
                log::info!(
                    "Opened '{}' with Godot {} (version mismatch confirmed by user)",
                    project_path.display(),
                    confirm.available_version
                );
                state.project_meta_store.update_last_opened(&project_path);
                state.project_meta_store.save_quiet();
            }
            Err(e) => log::error!("Failed to open project: {}", e),
        }
    }

    if close {
        state.version_mismatch_confirm = None;
    }
}

/// 绘制项目删除确认对话框
fn draw_delete_project_confirm_dialog(
    ctx: &egui::Context,
    confirm: &DeleteProjectConfirmState,
    state: &mut AppState,
    colors: &ThemeColors,
) {
    let mut close = false;
    let mut do_delete = false;

    egui::Window::new("Delete Project Files")
        .collapsible(false)
        .resizable(false)
        .min_width(450.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // 警告图标
                ui.label(RichText::new("⚠").size(48.0));

                ui.add_space(12.0);

                // 标题
                ui.label(
                    RichText::new("Delete Project Files")
                        .size(18.0)
                        .strong()
                        .color(colors.text_primary),
                );

                ui.add_space(8.0);

                // 警告信息
                ui.label(
                    RichText::new(format!(
                        "This will permanently delete all files in '{}'!\n\nThis action cannot be undone!",
                        confirm.project_name
                    ))
                    .color(colors.text_secondary),
                );

                ui.add_space(8.0);

                // 显示完整路径
                ui.label(
                    RichText::new(format!("Path: {}", confirm.project_path.display()))
                        .size(12.0)
                        .color(colors.text_secondary),
                );

                ui.add_space(16.0);

                // 按钮区域
                ui.horizontal(|ui| {
                    // 取消按钮
                    let cancel_btn = egui::Button::new(RichText::new("Cancel").color(colors.text_primary))
                        .fill(colors.bg_secondary)
                        .stroke(egui::Stroke::new(1.0, colors.border));

                    if ui.add(cancel_btn).clicked() {
                        close = true;
                    }

                    ui.add_space(12.0);

                    // 删除按钮（危险样式）
                    let delete_btn = danger_button("Delete");

                    if ui.add(delete_btn).clicked() {
                        do_delete = true;
                        close = true;
                    }
                });
            });
        });

    if do_delete {
        // 用户确认，执行删除操作
        let project_path = confirm.project_path.clone();

        log::warn!("Deleting project files: {}", project_path.display());

        match std::fs::remove_dir_all(&project_path) {
            Ok(_) => {
                log::info!("Successfully deleted project: {}", project_path.display());

                // 从导入列表中移除（如果在其中）
                state
                    .project_meta_store
                    .imported_paths
                    .retain(|p| p != &project_path);

                // 从元数据中移除
                let key = project_path.to_string_lossy().to_string();
                state.project_meta_store.entries.remove(&key);

                // 保存状态
                state.project_meta_store.save_quiet();
            }
            Err(e) => {
                log::error!(
                    "Failed to delete project '{}': {}",
                    project_path.display(),
                    e
                );
            }
        }
    }

    if close {
        state.delete_project_confirm = None;
    }
}

/// 执行新建项目：创建目录 + 写入 project.godot 模板
fn execute_create_project(state: &mut AppState) {
    // 先把需要的值提取出来，避免后面持有对 state 的可变借用
    let (name, project_path, godot_version) = {
        let dlg = match state.new_project_dialog.as_ref() {
            Some(d) => d,
            None => return,
        };

        let name = dlg.name.trim().to_string();
        if name.is_empty() {
            if let Some(d) = state.new_project_dialog.as_mut() {
                d.error = Some("Project name cannot be empty.".to_string());
            }
            return;
        }

        let project_path = dlg.project_path();

        if project_path.exists() {
            if let Some(d) = state.new_project_dialog.as_mut() {
                d.error = Some(format!(
                    "Directory already exists: {}",
                    project_path.display()
                ));
            }
            return;
        }

        let godot_version = match dlg
            .selected_godot_index
            .and_then(|idx| state.installed_versions.get(idx))
        {
            Some(g) => g.version.clone(),
            None => {
                if let Some(d) = state.new_project_dialog.as_mut() {
                    d.error = Some("Please select a Godot version.".to_string());
                }
                return;
            }
        };

        (name, project_path, godot_version)
    };

    // 创建目录
    if let Err(e) = std::fs::create_dir_all(&project_path) {
        if let Some(d) = state.new_project_dialog.as_mut() {
            d.creating = false;
            d.error = Some(format!("Failed to create directory: {}", e));
        }
        return;
    }

    // 写入 project.godot 模板
    let content = generate_project_godot_template(&name, &godot_version);
    if let Err(e) = std::fs::write(project_path.join("project.godot"), &content) {
        let _ = std::fs::remove_dir_all(&project_path);
        if let Some(d) = state.new_project_dialog.as_mut() {
            d.creating = false;
            d.error = Some(format!("Failed to write project.godot: {}", e));
        }
        return;
    }

    log::info!(
        "Created Godot {} project '{}' at: {}",
        godot_version,
        name,
        project_path.display()
    );

    state.new_project_dialog = None;
}

/// 生成最小化 project.godot 内容
fn generate_project_godot_template(project_name: &str, godot_version: &str) -> String {
    let major = godot_version
        .split('.')
        .next()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(4);

    if major >= 4 {
        format!(
            "; Engine configuration file.\n\
             ; It's best edited using the editor UI and not directly,\n\
             ; since the properties don't have any descriptive names.\n\
             \n\
             config_version=5\n\
             \n\
             [application]\n\
             \n\
             config/name=\"{name}\"\n\
             config/features=PackedStringArray(\"{ver}\", \"Forward Plus\")\n",
            name = project_name,
            ver = godot_version,
        )
    } else {
        format!(
            "; Engine configuration file.\n\
             ; It's best edited using the editor UI and not directly,\n\
             ; since the properties don't have any descriptive names.\n\
             \n\
             config_version=4\n\
             \n\
             [application]\n\
             \n\
             config/name=\"{name}\"\n",
            name = project_name,
        )
    }
}

// ============================================================================
// 项目扫描与元数据合并
// ============================================================================

/// 扫描 projects_dir 和导入路径，合并元数据，过滤隐藏项目
fn collect_projects(state: &AppState) -> Vec<ProjectInfo> {
    let mut projects: Vec<ProjectInfo> = Vec::new();
    let meta = &state.project_meta_store;

    // 扫描 projects_dir 下的所有 Godot 项目
    if state.config.projects_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&state.config.projects_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let project_file = path.join("project.godot");
                if !project_file.exists() {
                    continue;
                }
                if meta.is_hidden(&path) {
                    continue;
                }

                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let godot_version = parse_godot_version(&project_file);
                let m = meta.get(&path);

                projects.push(ProjectInfo {
                    name,
                    path: path.clone(),
                    godot_version,
                    is_favorite: m.map_or(false, |x| x.is_favorite),
                    last_opened: m.and_then(|x| x.last_opened),
                    is_imported: false,
                });
            }
        }
    }

    // 加入手动导入的路径（有效且未隐藏）
    for imported_path in &meta.imported_paths {
        if meta.is_hidden(imported_path) {
            continue;
        }
        // 避免重复（如果导入路径恰好在 projects_dir 下）
        if projects.iter().any(|p| &p.path == imported_path) {
            continue;
        }
        let project_file = imported_path.join("project.godot");
        if !project_file.exists() {
            continue;
        }

        let name = imported_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let godot_version = parse_godot_version(&project_file);
        let m = meta.get(imported_path);

        projects.push(ProjectInfo {
            name,
            path: imported_path.clone(),
            godot_version,
            is_favorite: m.map_or(false, |x| x.is_favorite),
            last_opened: m.and_then(|x| x.last_opened),
            is_imported: true,
        });
    }

    // 收藏置顶，其次按最后打开时间排序，最后按名称排序
    projects.sort_by(|a, b| {
        a.is_favorite
            .cmp(&b.is_favorite)
            .reverse()
            .then_with(|| a.last_opened.cmp(&b.last_opened).reverse())
            .then_with(|| a.name.cmp(&b.name))
    });

    projects
}

/// 解析 project.godot 文件，读取 config/features 里的 Godot 版本
fn parse_godot_version(project_file: &Path) -> String {
    // 使用 rust-ini 解析 project.godot 文件，更健壮地处理 INI 格式
    if let Ok(conf) = Ini::load_from_file(project_file) {
        // 尝试从 [application] section 读取 config/features
        if let Some(section) = conf.section(Some("application")) {
            if let Some(features) = section.get("config/features") {
                // Godot 4.x: PackedStringArray("4.3", "Forward Plus")
                // 提取第一个引号内的版本号
                if features.contains("PackedStringArray") {
                    if let Some(start) = features.find('"') {
                        if let Some(end) = features[start + 1..].find('"') {
                            let version = &features[start + 1..start + 1 + end];
                            if version.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                                return version.to_string();
                            }
                        }
                    }
                }
            }

            // 如果没找到 features，检查 config_version
            if let Some(config_ver) = section.get("config_version") {
                if config_ver == "5" {
                    return String::from("4.x");
                }
            }
        }
    }

    // 默认回退 3.x
    String::from("3.x")
}
