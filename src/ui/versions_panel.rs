// VersionsPanel - 版本管理面板 UI 组件
// 优化版本：卡片式布局、清晰信息层次、状态标签、改进交互

use egui::{Color32, RichText, ScrollArea, Stroke, Vec2};

use crate::models::{GodotInstall, GodotVariant, GodotVersion};
use crate::services;
use crate::state::AppState;

/// 绘制版本管理面板
pub fn draw_versions_panel(ui: &mut egui::Ui, state: &mut AppState) {
    // 顶部标题区域
    egui::TopBottomPanel::top("versions_header")
        .frame(egui::Frame::NONE.inner_margin(egui::Margin::same(16)))
        .show_inside(ui, |ui| {
            draw_panel_header(ui, state);
        });

    // 主内容区域
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.add_space(8.0);

            // 已安装版本部分
            draw_installed_section(ui, state);

            ui.add_space(16.0);

            // 可用版本部分
            draw_available_section(ui, state);

            ui.add_space(16.0);
        });
}

/// 绘制面板头部
fn draw_panel_header(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Godot Versions");
            ui.label(
                RichText::new("Manage your Godot engine installations")
                    .small()
                    .weak()
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 刷新按钮
            let refresh_btn = egui::Button::new("🔄 Refresh")
                .fill(Color32::from_rgb(70, 130, 180));

            let mut response = ui.add(refresh_btn);
            let response = response.on_hover_text("Refresh installed versions list");

            if response.clicked() {
                state.load_installed_versions();
            }
        });
    });
}

/// 绘制已安装版本区域
fn draw_installed_section(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // 区域标题
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("📦 Installed Versions")
                    .size(16.0)
                    .strong()
            );

            ui.add_space(8.0);

            ui.label(
                RichText::new(format!("({})", state.installed_versions.len()))
                    .small()
                    .weak()
            );
        });

        ui.add_space(8.0);

        if state.installed_versions.is_empty() {
            draw_empty_installed_state(ui, state);
        } else {
            // 显示已安装版本列表
            let installs: Vec<GodotInstall> = state.installed_versions.clone();
            for (index, install) in installs.iter().enumerate() {
                draw_installed_version_card(ui, index, install, state);
                ui.add_space(8.0);
            }
        }
    });
}

/// 绘制空状态（已安装版本）
fn draw_empty_installed_state(ui: &mut egui::Ui, state: &mut AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(24.0)
        .outer_margin(0.0)
        .corner_radius(8.0)
        .fill(Color32::from_rgba_unmultiplied(128, 128, 128, 15))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.0);

                ui.label(
                    RichText::new("📦")
                        .size(48.0)
                        .weak()
                );

                ui.add_space(12.0);

                ui.label(
                    RichText::new("No Godot Versions Installed")
                        .size(16.0)
                        .strong()
                );

                ui.add_space(8.0);

                ui.label(
                    RichText::new("Click 'Download New Version' to get started")
                        .weak()
                );

                ui.add_space(16.0);

                let download_btn = egui::Button::new(
                    RichText::new("⬇️  Download Now").strong()
                )
                .fill(Color32::from_rgb(70, 130, 180))
                .min_size(Vec2::new(160.0, 32.0));

                if ui.add(download_btn).clicked() {
                    state.show_download_dialog = true;
                }

                ui.add_space(8.0);
            });
        });
}

/// 绘制已安装版本卡片
fn draw_installed_version_card(
    ui: &mut egui::Ui,
    index: usize,
    install: &GodotInstall,
    state: &mut AppState,
) {
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
                // 左侧：版本图标
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(if install.is_favorite { "⭐" } else { "🎮" })
                            .size(32.0)
                    );
                });

                ui.add_space(8.0);

                // 中间：版本信息
                ui.vertical(|ui| {
                    // 第一行：版本号 + 标签
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&install.version)
                                .size(16.0)
                                .strong()
                        );

                        // 变体标签
                        let (variant_text, variant_color) = match install.variant {
                            GodotVariant::Mono => ("Mono", Color32::from_rgb(156, 39, 176)),
                            GodotVariant::Standard => ("Standard", Color32::from_rgb(76, 175, 80)),
                            GodotVariant::ExportTemplates => ("Export", Color32::from_rgb(255, 152, 0)),
                        };

                        draw_status_tag(ui, variant_text, variant_color);

                        // 收藏标签
                        if install.is_favorite {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new("Favorite")
                                    .small()
                                    .color(Color32::from_rgb(255, 193, 7))
                            );
                        }
                    });

                    ui.add_space(4.0);

                    // 第二行：路径
                    let path_str = install.path.display().to_string();
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
                    ).on_hover_text(install.path.display().to_string());

                    // 第三行：使用时间
                    if let Some(last_used) = &install.last_used {
                        ui.label(
                            RichText::new(format!(
                                "🕐 Last used: {}",
                                last_used.format("%Y-%m-%d %H:%M")
                            ))
                            .small()
                            .weak()
                        );
                    }
                });

                // 右侧：操作按钮
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 更多操作菜单
                    draw_version_menu(ui, index, state);

                    ui.add_space(8.0);

                    // 运行按钮
                    let run_btn = egui::Button::new("▶ Run")
                        .fill(Color32::from_rgb(46, 139, 87))
                        .min_size(Vec2::new(64.0, 28.0));

                    let mut response = ui.add(run_btn);
                    let response = response.on_hover_text(format!(
                        "Launch Godot {}",
                        install.version
                    ));

                    if response.clicked() {
                        if let Err(e) = services::launch_godot(&install.path) {
                            log::error!("Failed to launch Godot: {}", e);
                        }
                    }
                });
            });
        });
}

/// 绘制版本操作菜单
fn draw_version_menu(ui: &mut egui::Ui, index: usize, state: &mut AppState) {
    ui.menu_button("⋮", |ui| {
        ui.set_min_width(140.0);

        // 打开文件夹
        if ui.button("📂 Open Folder").clicked() {
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
                ui.close_menu();
            }
        }

        ui.separator();

        // 删除操作（危险操作）
        let delete_btn = egui::Button::new(
            RichText::new("🗑 Remove").color(Color32::from_rgb(220, 53, 69))
        );

        if ui.add(delete_btn).clicked() {
            // TODO: 显示确认对话框
            log::warn!("Remove version requested for index {}", index);
            ui.close_menu();
        }
    });
}

/// 绘制可用版本区域
fn draw_available_section(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // 区域标题
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("🌐 Available Versions")
                    .size(16.0)
                    .strong()
            );

            ui.add_space(8.0);

            let available_count = state.available_versions.iter()
                .filter(|v| !v.is_installed)
                .count();

            ui.label(
                RichText::new(format!("({})", available_count))
                    .small()
                    .weak()
            );
        });

        ui.add_space(8.0);

        if state.available_versions.is_empty() {
            draw_empty_available_state(ui);
        } else {
            // 按版本分组显示
            let versions: Vec<GodotVersion> = state.available_versions.clone();

            // Godot 4.x
            draw_version_group(ui, "Godot 4.x", &versions, "4", state);

            ui.add_space(12.0);

            // Godot 3.x
            draw_version_group(ui, "Godot 3.x", &versions, "3", state);
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
        ui.add_space(8.0);

        for version in filtered_versions {
            draw_available_version_card(ui, version, state);
            ui.add_space(8.0);
        }
    });
}

/// 绘制可用版本卡片
fn draw_available_version_card(ui: &mut egui::Ui, version: &GodotVersion, state: &mut AppState) {
    let is_downloading = state.downloads_in_progress.contains_key(&version.version);

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
                // 左侧：版本信息
                ui.vertical(|ui| {
                    // 第一行：版本号 + 标签
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&version.version)
                                .size(16.0)
                                .strong()
                        );

                        // 变体标签
                        let (variant_text, variant_color) = match version.variant {
                            GodotVariant::Mono => ("Mono", Color32::from_rgb(156, 39, 176)),
                            GodotVariant::Standard => ("Standard", Color32::from_rgb(76, 175, 80)),
                            GodotVariant::ExportTemplates => ("Export", Color32::from_rgb(255, 152, 0)),
                        };

                        draw_status_tag(ui, variant_text, variant_color);

                        // 平台标签
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&version.platform)
                                .small()
                                .weak()
                        );
                    });

                    ui.add_space(4.0);

                    // 第二行：发布日期
                    ui.label(
                        RichText::new(format!("📅 Released: {}", version.release_date))
                            .small()
                            .weak()
                    );
                });

                // 右侧：状态和操作
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if version.is_installed {
                        // 已安装状态
                        draw_status_tag(ui, "Installed", Color32::from_rgb(70, 130, 180));
                    } else if is_downloading {
                        // 下载中状态
                        draw_download_progress(ui, &version.version, state);
                    } else {
                        // 可下载状态
                        let download_btn = egui::Button::new("⬇️ Download")
                            .fill(Color32::from_rgb(70, 130, 180))
                            .min_size(Vec2::new(100.0, 28.0));

                        let mut response = ui.add(download_btn);
                        let response = response.on_hover_text(format!(
                            "Download Godot {} from GitHub",
                            version.version
                        ));

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
fn draw_download_progress(ui: &mut egui::Ui, version_key: &str, state: &mut AppState) {
    // 先获取进度值的副本，避免借用冲突
    let progress = state.downloads_in_progress.get(version_key).copied();

    if let Some(progress) = progress {
        ui.vertical(|ui| {
            // 进度条
            ui.add(
                egui::ProgressBar::new(progress)
                    .desired_width(120.0)
                    .text(format!("{:.0}%", progress * 100.0))
                    .animate(true)
            );

            ui.add_space(4.0);

            // 取消按钮
            let cancel_btn = egui::Button::new("Cancel")
                .small()
                .fill(Color32::from_rgb(220, 53, 69));

            if ui.add(cancel_btn).clicked() {
                services::cancel_download(version_key, state);
            }
        });
    }
}

/// 绘制状态标签
fn draw_status_tag(ui: &mut egui::Ui, text: &str, color: Color32) {
    ui.label(
        RichText::new(format!(" {} ", text))
            .small()
            .background_color(color.linear_multiply(0.3))
            .color(color)
    );
}

/// 绘制空状态（可用版本）
fn draw_empty_available_state(ui: &mut egui::Ui) {
    egui::Frame::group(ui.style())
        .inner_margin(24.0)
        .outer_margin(0.0)
        .corner_radius(8.0)
        .fill(Color32::from_rgba_unmultiplied(128, 128, 128, 15))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.0);

                ui.label(
                    RichText::new("🌐")
                        .size(48.0)
                        .weak()
                );

                ui.add_space(12.0);

                ui.label(
                    RichText::new("No Versions Available")
                        .size(16.0)
                        .strong()
                );

                ui.add_space(8.0);

                ui.label(
                    RichText::new("Unable to fetch version list from GitHub")
                        .weak()
                );

                ui.add_space(16.0);

                let refresh_btn = egui::Button::new("🔄 Refresh")
                    .fill(Color32::from_rgb(70, 130, 180));

                if ui.add(refresh_btn).clicked() {
                    log::info!("Refresh requested");
                }

                ui.add_space(8.0);
            });
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
