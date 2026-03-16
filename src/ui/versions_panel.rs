// VersionsPanel - 版本管理面板 UI 组件
// 优化版本：使用统一样式系统、卡片式布局、清晰信息层次、支持主题切换

use egui::{RichText, ScrollArea, Stroke, Vec2};

use crate::models::{GodotInstall, GodotVariant, GodotVersion};
use crate::services::{self, download_state};
use crate::state::{AppState, Theme};
use crate::ui::style::{
    badge, card_frame, danger_button, empty_state, panel_header, path_label, primary_button,
    section_header, spacing, status_pill, success_button, ThemeColors,
};
use crate::utils::open_folder;

/// 绘制版本管理面板
pub fn draw_versions_panel(ui: &mut egui::Ui, state: &mut AppState) {
    // 先收集需要显示的删除确认对话框信息（避免借用冲突）
    let delete_confirm = state.delete_confirm.clone();
    let theme = state.config.theme;
    let colors = ThemeColors::from_theme(theme);

    // 顶部标题区域
    egui::TopBottomPanel::top("versions_header")
        .frame(egui::Frame::NONE.inner_margin(16.0))
        .show_inside(ui, |ui| {
            draw_panel_header(ui, state, &colors);
        });

    // 主内容区域
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.add_space(8.0);

            // 已安装版本部分
            draw_installed_section(ui, state, &colors);

            ui.add_space(24.0);

            // 可用版本部分
            draw_available_section(ui, state, &colors);

            ui.add_space(16.0);
        });

    // 显示删除确认对话框
    if let Some(ref confirm) = delete_confirm {
        draw_delete_confirm_dialog(ui.ctx(), confirm, state, &colors);
    }
}

/// 绘制删除确认对话框
fn draw_delete_confirm_dialog(
    ctx: &egui::Context,
    delete_confirm: &crate::state::DeleteConfirmState,
    state: &mut AppState,
    colors: &ThemeColors,
) {
    egui::Window::new("Delete Confirmation")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // 警告图标
                ui.label(
                    RichText::new("⚠")
                        .size(48.0)
                );

                ui.add_space(12.0);

                // 标题
                ui.label(
                    RichText::new("Delete Godot Version?")
                        .size(18.0)
                        .strong()
                        .color(colors.text_primary)
                );

                ui.add_space(8.0);

                // 删除警告信息
                ui.label(
                    RichText::new(format!(
                        "Are you sure you want to delete {}?\n\nThis will permanently remove the engine files from your system.",
                        delete_confirm.version_info
                    ))
                    .color(colors.text_secondary)
                );

                ui.add_space(16.0);

                // 按钮区域
                ui.horizontal(|ui| {
                    // 取消按钮
                    let cancel_btn = egui::Button::new(
                        RichText::new("Cancel").color(colors.text_primary)
                    )
                    .fill(colors.bg_secondary)
                    .stroke(Stroke::new(1.0, colors.border));

                    if ui.add(cancel_btn).clicked() {
                        state.delete_confirm = None;
                    }

                    ui.add_space(12.0);

                    // 确认删除按钮
                    let confirm_btn = egui::Button::new(
                        RichText::new("Delete").color(egui::Color32::WHITE)
                    )
                    .fill(colors.error);

                    if ui.add(confirm_btn).clicked() {
                        let index = delete_confirm.version_index;
                        state.delete_confirm = None;

                        // 执行删除
                        match state.remove_installed_version(index) {
                            Ok(removed) => {
                                log::info!("Successfully removed Godot {}", removed.version);
                            }
                            Err(e) => {
                                log::error!("Failed to remove Godot: {}", e);
                            }
                        }
                    }
                });
            });
        });
}

/// 绘制面板头部
fn draw_panel_header(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        panel_header(
            ui,
            state.config.theme,
            "Godot Versions",
            "Manage your Godot engine installations",
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 刷新按钮
            let refresh_btn =
                egui::Button::new(RichText::new("Refresh").color(colors.text_primary))
                    .fill(colors.bg_secondary)
                    .stroke(Stroke::new(1.0, colors.border))
                    .min_size(Vec2::new(100.0, spacing::BUTTON_HEIGHT));

            let response = ui
                .add(refresh_btn)
                .on_hover_text("Refresh installed versions list");

            if response.clicked() {
                state.load_installed_versions();
            }
        });
    });
}

/// 绘制已安装版本区域
fn draw_installed_section(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.vertical(|ui| {
        // 区域标题
        section_header(
            ui,
            state.config.theme,
            "▸",
            "Installed Versions",
            Some(state.installed_versions.len()),
        );

        ui.add_space(12.0);

        if state.installed_versions.is_empty() {
            draw_empty_installed_state(ui, state);
        } else {
            // 显示已安装版本列表
            let installs: Vec<GodotInstall> = state.installed_versions.clone();
            for (index, install) in installs.iter().enumerate() {
                draw_installed_version_card(ui, index, install, state, colors);
                ui.add_space(12.0);
            }
        }
    });
}

/// 绘制空状态（已安装版本）
fn draw_empty_installed_state(ui: &mut egui::Ui, state: &mut AppState) {
    empty_state(
        ui,
        state.config.theme,
        "▸",
        "No Godot Versions Installed",
        "Click 'Download New Version' to get started",
        Some("Download Now"),
        Some(&mut || {
            state.show_download_dialog = true;
        }),
    );
}

/// 绘制已安装版本卡片
fn draw_installed_version_card(
    ui: &mut egui::Ui,
    index: usize,
    install: &GodotInstall,
    state: &mut AppState,
    colors: &ThemeColors,
) {
    card_frame(state.config.theme).show(ui, |ui| {
        ui.horizontal(|ui| {
            // 左侧：版本图标
            ui.vertical(|ui| {
                ui.add_space(4.0);
                ui.label(RichText::new(if install.is_favorite { "★" } else { "◆" }).size(32.0));
            });

            ui.add_space(12.0);

            // 中间：版本信息
            ui.vertical(|ui| {
                // 第一行：版本号 + 标签
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&install.version)
                            .size(16.0)
                            .strong()
                            .color(colors.text_primary),
                    );

                    ui.add_space(8.0);

                    // 变体标签
                    let (variant_text, variant_color) = match install.variant {
                        GodotVariant::Mono => ("Mono", colors.badge_purple),
                        GodotVariant::Standard => ("Standard", colors.badge_green),
                        GodotVariant::ExportTemplates => ("Export", colors.badge_orange),
                    };

                    status_pill(ui, variant_text, variant_color);

                    // 收藏标签
                    if install.is_favorite {
                        ui.add_space(4.0);
                        badge(ui, "★ Favorite", colors.warning);
                    }
                });

                ui.add_space(6.0);

                // 第二行：路径
                let path_str = install.path.display().to_string();
                path_label(ui, state.config.theme, &path_str, 60);

                ui.add_space(4.0);

                // 第三行：使用时间
                if let Some(last_used) = &install.last_used {
                    ui.label(
                        RichText::new(format!("Last used: {}", last_used.format("%Y-%m-%d %H:%M")))
                            .small()
                            .color(colors.text_secondary),
                    );
                }
            });

            // 右侧：操作按钮
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 更多操作菜单
                draw_version_menu(ui, index, state, colors);

                ui.add_space(8.0);

                // 运行按钮
                let run_btn = success_button("Run");
                let response = ui
                    .add(run_btn)
                    .on_hover_text(format!("Launch Godot {}", install.version));

                if response.clicked() {
                    if let Err(e) = services::launch_godot(&install.path) {
                        log::error!("Failed to launch Godot: {}", e);
                    } else {
                        // 启动成功：更新最后使用时间并持久化
                        if let Some(item) = state.installed_versions.get_mut(index) {
                            item.mark_used();
                        }
                        state.save_install_meta();
                    }
                }
            });
        });
    });
}

/// 绘制版本操作菜单
fn draw_version_menu(ui: &mut egui::Ui, index: usize, state: &mut AppState, _colors: &ThemeColors) {
    ui.menu_button("...", |ui| {
        ui.set_min_width(140.0);

        // 打开文件夹
        if ui.button("Open Folder").clicked() {
            if let Some(install) = state.installed_versions.get(index) {
                let path = install.path.parent().unwrap_or(&install.path);
                open_folder(path);
            }
            ui.close_menu();
        }

        // 切换收藏
        if let Some(install) = state.installed_versions.get(index) {
            let favorite_text = if install.is_favorite {
                "☆ Remove from Favorites"
            } else {
                "★ Add to Favorites"
            };

            if ui.button(favorite_text).clicked() {
                if let Some(item) = state.installed_versions.get_mut(index) {
                    item.is_favorite = !item.is_favorite;
                }
                // 收藏状态变更后立即持久化
                state.save_install_meta();
                ui.close_menu();
            }
        }

        ui.separator();

        // 删除操作（危险操作）
        let delete_btn = danger_button("Remove");
        if ui.add(delete_btn).clicked() {
            // 显示删除确认对话框
            if let Some(install) = state.installed_versions.get(index) {
                let version_info = format!(
                    "Godot {} ({})",
                    install.version,
                    match install.variant {
                        GodotVariant::Mono => "Mono",
                        GodotVariant::Standard => "Standard",
                        GodotVariant::ExportTemplates => "Export Templates",
                    }
                );
                state.delete_confirm = Some(crate::state::DeleteConfirmState {
                    version_index: index,
                    version_info,
                });
            }
            ui.close_menu();
        }
    });
}

/// 绘制可用版本区域
fn draw_available_section(ui: &mut egui::Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.vertical(|ui| {
        // 区域标题
        let available_count = state
            .available_versions
            .iter()
            .filter(|v| !v.is_installed)
            .count();
        section_header(
            ui,
            state.config.theme,
            "●",
            "Available Versions",
            Some(available_count),
        );

        ui.add_space(12.0);

        if state.available_versions.is_empty() {
            draw_empty_available_state(ui, state.config.theme);
        } else {
            // 按版本分组显示
            let versions: Vec<GodotVersion> = state.available_versions.clone();

            // Godot 4.x
            draw_version_group(ui, "Godot 4.x", &versions, "4", state, colors);

            ui.add_space(16.0);

            // Godot 3.x
            draw_version_group(ui, "Godot 3.x", &versions, "3", state, colors);
        }
    });
}

/// 绘制版本分组
fn draw_version_group(
    ui: &mut egui::Ui,
    title: &str,
    versions: &[GodotVersion],
    prefix: &str,
    state: &mut AppState,
    colors: &ThemeColors,
) {
    let filtered_versions: Vec<&GodotVersion> = versions
        .iter()
        .filter(|v| v.version.starts_with(prefix))
        .collect();

    if filtered_versions.is_empty() {
        return;
    }

    // 可折叠的分组
    ui.collapsing(title, |ui| {
        ui.add_space(12.0);

        for version in filtered_versions {
            draw_available_version_card(ui, version, state, colors);
            ui.add_space(12.0);
        }
    });
}

/// 绘制可用版本卡片
fn draw_available_version_card(
    ui: &mut egui::Ui,
    version: &GodotVersion,
    state: &mut AppState,
    colors: &ThemeColors,
) {
    // 使用正确的版本 key（与 download.rs 保持一致）
    let version_key = match version.variant {
        GodotVariant::Mono => format!("{}-mono", version.version),
        _ => version.version.clone(),
    };
    let is_downloading = state.downloads_in_progress.contains_key(&version_key)
        || state
            .downloads_in_progress
            .contains_key(&download_state::error_key(&version_key));

    card_frame(state.config.theme).show(ui, |ui| {
        ui.horizontal(|ui| {
            // 左侧：版本信息
            ui.vertical(|ui| {
                // 第一行：版本号 + 标签
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&version.version)
                            .size(16.0)
                            .strong()
                            .color(colors.text_primary),
                    );

                    ui.add_space(8.0);

                    // 变体标签
                    let (variant_text, variant_color) = match version.variant {
                        GodotVariant::Mono => ("Mono", colors.badge_purple),
                        GodotVariant::Standard => ("Standard", colors.badge_green),
                        GodotVariant::ExportTemplates => ("Export", colors.badge_orange),
                    };

                    status_pill(ui, variant_text, variant_color);

                    ui.add_space(4.0);

                    // 平台标签
                    badge(ui, &version.platform, colors.text_muted);
                });

                ui.add_space(6.0);

                // 第二行：发布日期
                ui.label(
                    RichText::new(format!("📅 Released: {}", version.release_date))
                        .small()
                        .color(colors.text_secondary),
                );
            });

            // 右侧：状态和操作
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if version.is_installed {
                    // 已安装状态
                    status_pill(ui, "✓ Installed", colors.success);
                } else if is_downloading {
                    // 下载中状态
                    draw_download_progress(ui, &version_key, state, colors);
                } else {
                    // 可下载状态
                    let download_btn = primary_button("Download", state.config.theme);
                    let response = ui
                        .add(download_btn)
                        .on_hover_text(format!("Download Godot {} from GitHub", version.version));

                    if response.clicked() {
                        if let Some(runtime) = &state.runtime {
                            services::start_download(version, state, runtime.clone());
                        }
                    }
                }
            });
        });
    });
}

/// 绘制下载进度
fn draw_download_progress(
    ui: &mut egui::Ui,
    version_key: &str,
    state: &mut AppState,
    colors: &ThemeColors,
) {
    // 先获取进度值的副本，避免借用冲突
    let error_key = download_state::error_key(version_key);
    let extracting_key = download_state::extracting_key(version_key);
    let complete_key = download_state::complete_key(version_key);
    let progress = state.downloads_in_progress.get(version_key).copied();
    let error_progress = state.downloads_in_progress.get(&error_key).copied();
    let extracting_progress = state.downloads_in_progress.get(&extracting_key).copied();
    let is_complete = state.downloads_in_progress.contains_key(&complete_key);

    // 检查是否为错误状态
    let is_error = error_progress
        .map(download_state::is_error)
        .unwrap_or(false);

    // 检查是否为解压状态
    let is_extracting = extracting_progress
        .map(download_state::is_extracting)
        .unwrap_or(false);

    if is_error {
        // 显示错误状态
        ui.vertical(|ui| {
            ui.label(RichText::new("❌ Failed").color(colors.error).strong());

            ui.add_space(4.0);

            // 重试按钮
            let retry_btn = egui::Button::new(RichText::new("Retry").color(colors.text_primary))
                .fill(colors.bg_secondary)
                .stroke(egui::Stroke::new(1.0, colors.border));

            if ui.add(retry_btn).clicked() {
                // 移除错误状态标记
                state
                    .downloads_in_progress
                    .remove(&download_state::error_key(version_key));
                // 重新开始下载
                if let Some(runtime) = &state.runtime {
                    // 找到对应的版本信息并克隆（避免借用冲突）
                    let version_clone = state
                        .available_versions
                        .iter()
                        .find(|v| {
                            let key = match v.variant {
                                GodotVariant::Mono => format!("{}-mono", v.version),
                                _ => v.version.clone(),
                            };
                            key == version_key
                        })
                        .cloned();
                    if let Some(available_version) = version_clone {
                        services::start_download(&available_version, state, runtime.clone());
                    }
                }
            }

            ui.add_space(4.0);

            // 取消/移除按钮
            let cancel_btn = danger_button("Remove");
            if ui.add(cancel_btn).clicked() {
                services::cancel_download(version_key, state);
            }
        });
    } else if is_extracting {
        // 解压中状态
        ui.vertical(|ui| {
            ui.label(RichText::new("Extracting...").color(colors.accent_blue));
        });
    } else if is_complete {
        // 安装完成状态
        ui.vertical(|ui| {
            ui.label(RichText::new("✓ Installed").color(colors.success).strong());
        });
    } else if let Some(progress) = progress {
        // 正常下载进度
        ui.vertical(|ui| {
            // 进度条
            let progress_bar = egui::ProgressBar::new(progress)
                .desired_width(120.0)
                .text(format!("{:.0}%", progress * 100.0))
                .animate(true)
                .fill(colors.accent_blue);

            ui.add(progress_bar);

            ui.add_space(6.0);

            // 取消按钮
            let cancel_btn = danger_button("Cancel");
            if ui.add(cancel_btn).clicked() {
                services::cancel_download(version_key, state);
            }
        });
    }
}

/// 绘制空状态（可用版本）
fn draw_empty_available_state(ui: &mut egui::Ui, theme: Theme) {
    empty_state(
        ui,
        theme,
        "●",
        "No Versions Available",
        "Unable to fetch version list from GitHub",
        Some("Refresh"),
        Some(&mut || {
            log::info!("Refresh requested");
        }),
    );
}
