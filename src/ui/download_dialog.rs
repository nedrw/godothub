// DownloadDialog - 下载对话框 UI 组件
// 优化版本：修复进度条、添加版本分组、改进布局和交互、支持主题切换

use egui::{Align2, RichText, ScrollArea, Ui, Vec2, Window};

use crate::models::GodotVersion;
use crate::services::{self, download_state};
use crate::state::AppState;
use crate::ui::style::ThemeColors;

/// 绘制下载对话框
pub fn draw_download_dialog(ui: &mut Ui, state: &mut AppState) {
    let theme = state.config.theme;
    let colors = ThemeColors::from_theme(theme);

    Window::new("⬇️ Download Godot")
        .collapsible(false)
        .resizable(true)
        .default_size([650.0, 550.0])
        .min_width(550.0)
        .min_height(400.0)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ui.ctx(), |ui| {
            draw_download_dialog_content(ui, state, &colors);
        });
}

/// 绘制下载对话框内容
fn draw_download_dialog_content(ui: &mut Ui, state: &mut AppState, colors: &ThemeColors) {
    // 头部区域
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Select a Godot version to download")
                    .weak()
                    .color(colors.text_secondary)
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

                    ui.label(RichText::new(time_text).small().weak().color(colors.text_muted));
                    ui.add_space(8.0);
                }

                // 刷新按钮
                let refresh_text = if state.version_refresh_state.is_refreshing {
                    "⏳"
                } else {
                    "🔄"
                };

                let refresh_btn = egui::Button::new(refresh_text)
                    .fill(egui::Color32::TRANSPARENT);

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
    draw_search_bar(ui, colors);

    ui.add_space(8.0);

    // 下载队列状态
    if !state.downloads_in_progress.is_empty() {
        draw_download_queue_status(ui, state, colors);
        ui.add_space(8.0);
    }

    // 版本列表
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .max_height(350.0)
        .show(ui, |ui| {
            // 按版本分组
            draw_version_groups(ui, state, colors);
        });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // 底部按钮
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let close_btn = egui::Button::new(
                RichText::new("Close").color(colors.text_primary)
            )
            .fill(colors.bg_secondary)
            .min_size(Vec2::new(80.0, 28.0));

            if ui.add(close_btn).clicked() {
                state.show_download_dialog = false;
            }
        });
    });
}

/// 绘制搜索栏
fn draw_search_bar(ui: &mut Ui, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        // 搜索框
        let mut search_text = String::new();
        ui.add_sized(
            [ui.available_width() - 120.0, 28.0],
            egui::TextEdit::singleline(&mut search_text)
                .hint_text("🔍 Search versions...")
        );

        // 筛选按钮
        let filter_btn = egui::Button::new(
            RichText::new("Filter ▼").color(colors.text_secondary)
        )
        .fill(colors.bg_hover);

        let response = ui.add(filter_btn);
        response.on_hover_text("Filter by variant or platform");

        // TODO: 实现筛选菜单
    });
}

/// 绘制下载队列状态
fn draw_download_queue_status(ui: &mut Ui, state: &mut AppState, colors: &ThemeColors) {
    egui::Frame::group(ui.style())
        .inner_margin(8.0)
        .outer_margin(0.0)
        .corner_radius(6.0)
        .fill(colors.accent_blue.linear_multiply(0.1))
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
                        .color(colors.text_primary)
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 取消全部按钮
                    let cancel_all_btn = egui::Button::new(
                        RichText::new("Cancel All").color(egui::Color32::WHITE)
                    )
                    .small()
                    .fill(colors.error);

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
fn draw_version_groups(ui: &mut Ui, state: &mut AppState, colors: &ThemeColors) {
    // 如果正在刷新，显示加载指示器
    if state.version_refresh_state.is_refreshing {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.spinner();
            ui.add_space(16.0);
            ui.label(RichText::new("Fetching versions from GitHub...").weak().color(colors.text_secondary));
            ui.add_space(40.0);
        });
        return;
    }

    // 显示错误信息（如果有）
    if let Some(ref error) = state.version_refresh_state.last_error {
        egui::Frame::group(ui.style())
            .inner_margin(10.0)
            .corner_radius(6.0)
            .fill(colors.error.linear_multiply(0.1))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚠️").size(16.0));
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Failed to fetch version list").strong().color(colors.text_primary));
                        ui.label(RichText::new(error).small().weak().color(colors.text_muted));
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
            ui.label(RichText::new("No versions available").weak().color(colors.text_muted));
            ui.add_space(8.0);

            let retry_btn = egui::Button::new(
                RichText::new("🔄 Retry").color(egui::Color32::WHITE)
            )
            .fill(colors.accent_blue);

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
                draw_version_item(ui, version, state, colors);
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
                draw_version_item(ui, version, state, colors);
                ui.add_space(6.0);
            }
        });
    }
}

/// 绘制单个版本项
/// 创建版本标识键（与 download.rs 中的 create_version_key 保持一致）
fn create_version_key(version: &GodotVersion) -> String {
    match version.variant {
        crate::models::GodotVariant::Mono => format!("{}-mono", version.version),
        _ => version.version.clone(),
    }
}

fn draw_version_item(ui: &mut Ui, version: &GodotVersion, state: &mut AppState, colors: &ThemeColors) {
    let version_key = create_version_key(version);
    let is_downloading = state.downloads_in_progress.contains_key(&version_key)
        || state.downloads_in_progress.contains_key(&download_state::error_key(&version_key))
        || state.downloads_in_progress.contains_key(&download_state::extracting_key(&version_key))
        || state.downloads_in_progress.contains_key(&download_state::complete_key(&version_key));

    egui::Frame::group(ui.style())
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(6.0)
        .stroke(egui::Stroke::new(
            1.0,
            if version.is_installed {
                colors.success.linear_multiply(0.5)
            } else {
                colors.border
            }
        ))
        .fill(if version.is_installed {
            colors.success.linear_multiply(0.08)
        } else {
            egui::Color32::TRANSPARENT
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
                                .color(colors.text_primary)
                        );

                        // 变体标签
                        let (variant_text, variant_color) = match version.variant {
                            crate::models::GodotVariant::Mono => ("Mono", colors.badge_purple),
                            crate::models::GodotVariant::Standard => ("Standard", colors.badge_green),
                            crate::models::GodotVariant::ExportTemplates => ("Export", colors.badge_orange),
                        };

                        draw_variant_tag(ui, variant_text, variant_color);

                        // 平台信息
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&version.platform)
                                .small()
                                .weak()
                                .color(colors.text_muted)
                        );
                    });

                    ui.add_space(2.0);

                    // 第二行：发布日期
                    ui.label(
                        RichText::new(format!("📅 {}", version.release_date))
                            .small()
                            .weak()
                            .color(colors.text_secondary)
                    );
                });

                // 右侧：状态和操作
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if version.is_installed {
                        // 已安装状态
                        ui.label(
                            RichText::new("✓ Installed")
                                .color(colors.success)
                                .strong()
                        );
                    } else if is_downloading {
                        // 下载中状态
                        draw_downloading_status(ui, &version_key, state, colors);
                    } else {
                        // 可下载状态
                        draw_download_button(ui, version, state, colors);
                    }
                });
            });
        });
}

/// 绘制变体标签
fn draw_variant_tag(ui: &mut Ui, text: &str, color: egui::Color32) {
    ui.label(
        RichText::new(format!(" {} ", text))
            .small()
            .background_color(color.linear_multiply(0.3))
            .color(color)
    );
}

/// 绘制下载按钮
fn draw_download_button(ui: &mut Ui, version: &GodotVersion, state: &mut AppState, colors: &ThemeColors) {
    let download_btn = egui::Button::new(
        RichText::new("⬇️ Download").color(egui::Color32::WHITE)
    )
    .fill(colors.accent_blue)
    .min_size(Vec2::new(110.0, 28.0));

    let response = ui.add(download_btn);

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
fn draw_downloading_status(ui: &mut Ui, version_key: &str, state: &mut AppState, colors: &ThemeColors) {
    // 先获取各种状态
    let error_key = download_state::error_key(version_key);
    let extracting_key = download_state::extracting_key(version_key);
    let complete_key = download_state::complete_key(version_key);

    let progress = state.downloads_in_progress.get(version_key).copied();
    let error_progress = state.downloads_in_progress.get(&error_key).copied();
    let is_extracting = state.downloads_in_progress.contains_key(&extracting_key);
    let is_complete = state.downloads_in_progress.contains_key(&complete_key);

    // 检查是否为错误状态
    let is_error = error_progress.map(download_state::is_error).unwrap_or(false);

    if is_error {
        // 显示错误状态
        ui.vertical(|ui| {
            ui.label(
                RichText::new("❌ Failed")
                    .color(colors.error)
                    .strong()
            );

            ui.add_space(4.0);

            // 重试按钮
            let retry_btn = egui::Button::new(
                RichText::new("🔄 Retry").color(egui::Color32::WHITE)
            )
            .small()
            .fill(colors.accent_blue);

            if ui.add(retry_btn).clicked() {
                // 移除错误状态标记
                state.downloads_in_progress.remove(&download_state::error_key(version_key));
                // 重新开始下载
                if let Some(runtime) = &state.runtime {
                    let version_clone = state.available_versions.iter().find(|v| {
                        let key = match v.variant {
                            crate::models::GodotVariant::Mono => format!("{}-mono", v.version),
                            _ => v.version.clone(),
                        };
                        key == version_key
                    }).cloned();
                    if let Some(available_version) = version_clone {
                        services::start_download(&available_version, state, runtime.clone());
                    }
                }
            }

            ui.add_space(4.0);

            // 移除按钮
            let remove_btn = egui::Button::new(
                RichText::new("Remove").color(egui::Color32::WHITE)
            )
            .small()
            .fill(colors.error);

            if ui.add(remove_btn).clicked() {
                services::cancel_download(version_key, state);
                log::info!("Download removed: {}", version_key);
            }
        });
    } else if is_extracting {
        // 显示解压中状态
        ui.vertical(|ui| {
            ui.label(
                RichText::new("📦 Extracting...")
                    .color(colors.accent_blue)
                    .strong()
            );

            ui.add_space(4.0);

            // 取消按钮
            let cancel_btn = egui::Button::new(
                RichText::new("Cancel").color(egui::Color32::WHITE)
            )
            .small()
            .fill(colors.warning);

            if ui.add(cancel_btn).clicked() {
                services::cancel_download(version_key, state);
                log::info!("Extraction cancelled: {}", version_key);
            }
        });
    } else if is_complete {
        // 显示完成状态（短暂显示后会被安装状态替代）
        ui.vertical(|ui| {
            ui.label(
                RichText::new("✅ Complete!")
                    .color(colors.success)
                    .strong()
            );
        });
    } else if let Some(progress) = progress {
        // 正常下载进度
        ui.vertical(|ui| {
            // 进度条
            ui.add(
                egui::ProgressBar::new(progress)
                    .desired_width(130.0)
                    .text(format!("{:.0}%", progress * 100.0))
                    .animate(true)
                    .fill(colors.accent_blue)
            );

            ui.add_space(4.0);

            // 取消按钮
            let cancel_btn = egui::Button::new(
                RichText::new("Cancel").color(egui::Color32::WHITE)
            )
            .small()
            .fill(colors.error);

            if ui.add(cancel_btn).clicked() {
                services::cancel_download(version_key, state);
                log::info!("Download cancelled: {}", version_key);
            }
        });
    }
}

/// 显示下载详情对话框
pub fn draw_download_details(ui: &mut Ui, version: &GodotVersion, colors: &ThemeColors) {
    ui.separator();

    ui.heading(
        RichText::new("Download Details")
            .color(colors.text_primary)
    );
    ui.add_space(8.0);

    // 版本信息
    egui::Grid::new("download_details_grid")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            ui.label(RichText::new("Version:").strong().color(colors.text_primary));
            ui.label(RichText::new(&version.version).color(colors.text_secondary));

            ui.label(RichText::new("Variant:").strong().color(colors.text_primary));
            ui.label(RichText::new(AppState::get_variant_name(&version.variant)).color(colors.text_secondary));

            ui.label(RichText::new("Platform:").strong().color(colors.text_primary));
            ui.label(RichText::new(&version.platform).color(colors.text_secondary));

            ui.label(RichText::new("Release Date:").strong().color(colors.text_primary));
            ui.label(RichText::new(&version.release_date).color(colors.text_secondary));

            ui.label(RichText::new("Download URL:").strong().color(colors.text_primary));
            ui.label(
                RichText::new(&version.download_url)
                    .small()
                    .weak()
                    .color(colors.text_muted)
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
