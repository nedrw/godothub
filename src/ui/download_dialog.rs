// DownloadDialog - 下载对话框 UI 组件
// 优化版本：修复进度条、添加版本分组、改进布局和交互

use egui::{Align2, Color32, RichText, ScrollArea, Ui, Vec2, Window};

use crate::models::GodotVersion;
use crate::services;
use crate::state::AppState;

/// 绘制下载对话框
pub fn draw_download_dialog(ui: &mut Ui, state: &mut AppState) {
    Window::new("⬇️ Download Godot")
        .collapsible(false)
        .resizable(true)
        .default_size([650.0, 550.0])
        .min_width(550.0)
        .min_height(400.0)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ui.ctx(), |ui| {
            draw_download_dialog_content(ui, state);
        });
}

/// 绘制下载对话框内容
fn draw_download_dialog_content(ui: &mut Ui, state: &mut AppState) {
    // 头部区域
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Select a Godot version to download")
                    .weak()
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 显示上次刷新时间
                if let Some(last_time) = state.version_refresh_state.last_refresh_time {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    let elapsed = now - last_time;

                    let time_text = if elapsed < 60 {
                        format!("Updated {}s ago", elapsed)
                    } else if elapsed < 3600 {
                        format!("Updated {}m ago", elapsed / 60)
                    } else {
                        format!("Updated {}h ago", elapsed / 3600)
                    };

                    ui.label(RichText::new(time_text).small().weak());
                    ui.add_space(8.0);
                }

                // 刷新按钮
                let refresh_text = if state.version_refresh_state.is_refreshing {
                    "⏳"
                } else {
                    "🔄"
                };

                let refresh_btn = egui::Button::new(refresh_text)
                    .fill(Color32::TRANSPARENT);

                let response = ui.add_enabled(
                    !state.version_refresh_state.is_refreshing,
                    refresh_btn
                );
                let response = response.on_hover_text(
                    if state.version_refresh_state.is_refreshing {
                        "Refreshing..."
                    } else {
                        "Refresh version list from GitHub"
                    }
                );

                if response.clicked() {
                    log::info!("Refresh version list requested");
                    // 启动异步刷新
                    state.refresh_available_versions();
                }
            });
        });
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // 搜索栏（占位）
    draw_search_bar(ui);

    ui.add_space(8.0);

    // 下载队列状态
    if !state.downloads_in_progress.is_empty() {
        draw_download_queue_status(ui, state);
        ui.add_space(8.0);
    }

    // 版本列表
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .max_height(350.0)
        .show(ui, |ui| {
            // 按版本分组
            draw_version_groups(ui, state);
        });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // 底部按钮
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let close_btn = egui::Button::new("Close")
                .min_size(Vec2::new(80.0, 28.0));

            if ui.add(close_btn).clicked() {
                state.show_download_dialog = false;
            }
        });
    });
}

/// 绘制搜索栏
fn draw_search_bar(ui: &mut Ui) {
    ui.horizontal(|ui| {
        // 搜索框
        let mut search_text = String::new();
        ui.add_sized(
            [ui.available_width() - 120.0, 28.0],
            egui::TextEdit::singleline(&mut search_text)
                .hint_text("🔍 Search versions...")
        );

        // 筛选按钮
        let filter_btn = egui::Button::new("Filter ▼")
            .fill(Color32::from_rgba_unmultiplied(128, 128, 128, 30));

        let response = ui.add(filter_btn);
        response.on_hover_text("Filter by variant or platform");

        // TODO: 实现筛选菜单
    });
}

/// 绘制下载队列状态
fn draw_download_queue_status(ui: &mut Ui, state: &mut AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(8.0)
        .outer_margin(0.0)
        .corner_radius(6.0)
        .fill(Color32::from_rgba_unmultiplied(70, 130, 180, 20))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("⬇️")
                        .size(16.0)
                );

                let count = state.downloads_in_progress.len();
                ui.label(
                    RichText::new(format!("{} download{} in progress", count, if count > 1 { "s" } else { "" }))
                        .strong()
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 取消全部按钮
                    let cancel_all_btn = egui::Button::new("Cancel All")
                        .small()
                        .fill(Color32::from_rgb(220, 53, 69));

                    if ui.add(cancel_all_btn).clicked() {
                        // 取消所有下载
                        let keys: Vec<String> = state.downloads_in_progress.keys().cloned().collect();
                        for key in keys {
                            services::cancel_download(&key, state);
                        }
                    }
                });
            });
        });
}

/// 绘制版本分组
fn draw_version_groups(ui: &mut Ui, state: &mut AppState) {
    // 如果正在刷新，显示加载指示器
    if state.version_refresh_state.is_refreshing {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.spinner();
            ui.add_space(16.0);
            ui.label(RichText::new("Fetching versions from GitHub...").weak());
            ui.add_space(40.0);
        });
        return;
    }

    // 显示错误信息（如果有）
    if let Some(ref error) = state.version_refresh_state.last_error {
        egui::Frame::group(ui.style())
            .inner_margin(10.0)
            .corner_radius(6.0)
            .fill(Color32::from_rgba_unmultiplied(220, 53, 69, 30))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚠️").size(16.0));
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Failed to fetch version list").strong());
                        ui.label(RichText::new(error).small().weak());
                    });
                });
            });
        ui.add_space(8.0);
    }

    let versions: Vec<GodotVersion> = state.available_versions.clone();

    // 如果版本列表为空
    if versions.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new("No versions available").weak());
            ui.add_space(8.0);

            let retry_btn = egui::Button::new("🔄 Retry")
                .fill(Color32::from_rgb(70, 130, 180));

            if ui.add(retry_btn).clicked() {
                state.refresh_available_versions();
            }
            ui.add_space(40.0);
        });
        return;
    }

    // 统计各分组数量
    let godot4_count = versions.iter().filter(|v| v.version.starts_with('4') && !v.is_installed).count();
    let godot3_count = versions.iter().filter(|v| v.version.starts_with('3') && !v.is_installed).count();

    // Godot 4.x 分组
    if godot4_count > 0 {
        ui.collapsing(format!("🚀 Godot 4.x ({} available)", godot4_count), |ui| {
            ui.add_space(8.0);
            for version in versions.iter().filter(|v| v.version.starts_with('4')) {
                draw_version_item(ui, version, state);
                ui.add_space(6.0);
            }
        });
        ui.add_space(8.0);
    }

    // Godot 3.x 分组
    if godot3_count > 0 {
        ui.collapsing(format!("📦 Godot 3.x ({} available)", godot3_count), |ui| {
            ui.add_space(8.0);
            for version in versions.iter().filter(|v| v.version.starts_with('3')) {
                draw_version_item(ui, version, state);
                ui.add_space(6.0);
            }
        });
    }
}

/// 绘制单个版本项
fn draw_version_item(ui: &mut Ui, version: &GodotVersion, state: &mut AppState) {
    let is_downloading = state.downloads_in_progress.contains_key(&version.version);

    egui::Frame::group(ui.style())
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(6.0)
        .stroke(egui::Stroke::new(
            1.0,
            if version.is_installed {
                Color32::from_rgba_unmultiplied(46, 139, 87, 100)
            } else {
                ui.style().visuals.widgets.noninteractive.bg_stroke.color
            }
        ))
        .fill(if version.is_installed {
            Color32::from_rgba_unmultiplied(46, 139, 87, 15)
        } else {
            Color32::TRANSPARENT
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // 左侧：版本信息
                ui.vertical(|ui| {
                    // 第一行：版本号 + 变体
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&version.version)
                                .size(15.0)
                                .strong()
                        );

                        // 变体标签
                        let (variant_text, variant_color) = match version.variant {
                            crate::models::GodotVariant::Mono => ("Mono", Color32::from_rgb(156, 39, 176)),
                            crate::models::GodotVariant::Standard => ("Standard", Color32::from_rgb(76, 175, 80)),
                            crate::models::GodotVariant::ExportTemplates => ("Export", Color32::from_rgb(255, 152, 0)),
                        };

                        draw_variant_tag(ui, variant_text, variant_color);

                        // 平台信息
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&version.platform)
                                .small()
                                .weak()
                        );
                    });

                    ui.add_space(2.0);

                    // 第二行：发布日期
                    ui.label(
                        RichText::new(format!("📅 {}", version.release_date))
                            .small()
                            .weak()
                    );
                });

                // 右侧：状态和操作
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if version.is_installed {
                        // 已安装状态
                        ui.label(
                            RichText::new("✓ Installed")
                                .color(Color32::from_rgb(46, 139, 87))
                                .strong()
                        );
                    } else if is_downloading {
                        // 下载中状态
                        draw_downloading_status(ui, &version.version, state);
                    } else {
                        // 可下载状态
                        draw_download_button(ui, version, state);
                    }
                });
            });
        });
}

/// 绘制变体标签
fn draw_variant_tag(ui: &mut Ui, text: &str, color: Color32) {
    ui.label(
        RichText::new(format!(" {} ", text))
            .small()
            .background_color(color.linear_multiply(0.3))
            .color(color)
    );
}

/// 绘制下载按钮
fn draw_download_button(ui: &mut Ui, version: &GodotVersion, state: &mut AppState) {
    let download_btn = egui::Button::new("⬇️ Download")
        .fill(Color32::from_rgb(70, 130, 180))
        .min_size(Vec2::new(110.0, 28.0));

    let mut response = ui.add(download_btn);

    let response = response.on_hover_text(format!(
        "Download Godot {} ({})",
        version.version,
        AppState::get_variant_name(&version.variant)
    ));

    if response.clicked() {
        if let Some(runtime) = &state.runtime {
            services::start_download(version, state, runtime.clone());
            log::info!("Download started for Godot {}", version.version);
        }
    }
}

/// 绘制下载中状态
fn draw_downloading_status(ui: &mut Ui, version_key: &str, state: &mut AppState) {
    // 先获取进度值的副本
    let progress = state.downloads_in_progress.get(version_key).copied();

    if let Some(progress) = progress {
        ui.vertical(|ui| {
            // 进度条
            ui.add(
                egui::ProgressBar::new(progress)
                    .desired_width(130.0)
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
                log::info!("Download cancelled: {}", version_key);
            }
        });
    }
}

/// 显示下载详情对话框
pub fn draw_download_details(ui: &mut Ui, version: &GodotVersion) {
    ui.separator();

    ui.heading("Download Details");
    ui.add_space(8.0);

    // 版本信息
    egui::Grid::new("download_details_grid")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            ui.label(RichText::new("Version:").strong());
            ui.label(&version.version);

            ui.label(RichText::new("Variant:").strong());
            ui.label(AppState::get_variant_name(&version.variant));

            ui.label(RichText::new("Platform:").strong());
            ui.label(&version.platform);

            ui.label(RichText::new("Release Date:").strong());
            ui.label(&version.release_date);

            ui.label(RichText::new("Download URL:").strong());
            ui.label(
                RichText::new(&version.download_url)
                    .small()
                    .weak()
            );
        });

    ui.add_space(8.0);

    // 复制链接按钮
    if ui.button("📋 Copy Download URL").clicked() {
        ui.ctx().copy_text(version.download_url.clone());
        log::info!("Download URL copied to clipboard");
    }
}

/// 启动下载（供外部调用）
pub fn initiate_download(version: &GodotVersion, state: &mut AppState) {
    log::info!(
        "Initiating download for Godot {} ({})",
        version.version,
        AppState::get_variant_name(&version.variant)
    );

    if let Some(runtime) = &state.runtime {
        services::start_download(version, state, runtime.clone());
    }
}

/// 取消下载（供外部调用）
pub fn cancel_download(version_key: &str, state: &mut AppState) -> bool {
    log::info!("Cancelling download for: {}", version_key);
    services::cancel_download(version_key, state)
}

/// 获取下载统计信息
pub fn get_download_stats(state: &AppState) -> (usize, usize) {
    let total = state.downloads_in_progress.len();
    let completed = state.downloads_in_progress.values()
        .filter(|&&progress| progress >= 1.0)
        .count();
    (total, completed)
}
