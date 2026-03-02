// Sidebar - 侧边栏 UI 组件
// 优化版本：使用统一样式系统、改进视觉层次、增强交互反馈、支持主题切换

use egui::{Align, Layout, RichText, Ui, Vec2};

use crate::services::download_state;
use crate::state::{AppState, MainTab};
use crate::ui::style::{ThemeColors, spacing};

/// 绘制侧边栏
pub fn draw_sidebar(ui: &mut Ui, state: &mut AppState) {
    let theme = state.config.theme;
    let colors = ThemeColors::from_theme(theme);

    // 设置侧边栏样式
    ui.set_min_width(spacing::SIDEBAR_WIDTH_EXPANDED);
    ui.set_max_width(spacing::SIDEBAR_WIDTH_EXPANDED);

    // 侧边栏容器（使用侧边栏背景色）
    egui::Frame::NONE
        .fill(colors.bg_sidebar)
        .inner_margin(egui::Margin::same(0))
        .show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                // 应用标题区
                draw_app_header(ui, &colors);

                ui.add_space(8.0);
                draw_separator(ui, &colors);
                ui.add_space(12.0);

                // 导航区
                draw_navigation_section(ui, state, &colors);

                ui.add_space(16.0);
                draw_separator(ui, &colors);
                ui.add_space(12.0);

                // 统计信息区
                draw_statistics_section(ui, state, &colors);
            });

            // 底部下载按钮（固定在底部）
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add_space(8.0);

                // 版本信息
                ui.label(
                    RichText::new("v0.1.0")
                        .small()
                        .color(colors.text_muted)
                );

                ui.add_space(12.0);
                draw_download_button(ui, state, &colors);
                ui.add_space(16.0);
            });
        });
}

/// 绘制应用标题
fn draw_app_header(ui: &mut Ui, colors: &ThemeColors) {
    ui.add_space(16.0);

    ui.horizontal(|ui| {
        ui.add_space(16.0);

        // 使用 emoji 作为图标
        ui.label(
            RichText::new("🎮")
                .size(32.0)
        );

        ui.add_space(8.0);

        ui.vertical(|ui| {
            ui.label(
                RichText::new("Godot Hub")
                    .size(20.0)
                    .strong()
                    .color(colors.text_primary)
            );
            ui.label(
                RichText::new("Engine Manager")
                    .size(12.0)
                    .color(colors.text_secondary)
            );
        });
    });

    ui.add_space(4.0);
}

/// 绘制导航区
fn draw_navigation_section(ui: &mut Ui, state: &mut AppState, colors: &ThemeColors) {
    // 导航标题
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        ui.label(
            RichText::new("NAVIGATION")
                .size(11.0)
                .color(colors.text_muted)
        );
    });

    ui.add_space(8.0);

    // 导航按钮
    draw_nav_button(
        ui,
        "📦",
        "Versions",
        "Manage Godot engine installations",
        MainTab::Versions,
        state,
        colors
    );

    draw_nav_button(
        ui,
        "📁",
        "Projects",
        "Browse and manage your projects",
        MainTab::Projects,
        state,
        colors
    );

    draw_nav_button(
        ui,
        "⚙️",
        "Settings",
        "Configure application preferences",
        MainTab::Settings,
        state,
        colors
    );
}

/// 绘制导航按钮
fn draw_nav_button(
    ui: &mut Ui,
    icon: &str,
    text: &str,
    tooltip: &str,
    tab: MainTab,
    state: &mut AppState,
    colors: &ThemeColors
) {
    let is_selected = state.current_tab == tab;

    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // 创建按钮容器
        let button_frame = egui::Frame::NONE
            .fill(if is_selected { colors.bg_hover } else { colors.bg_sidebar })
            .corner_radius(8.0)
            .inner_margin(egui::Margin::symmetric(12, 10));

        let response = button_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // 图标
                ui.label(
                    RichText::new(icon)
                        .size(20.0)
                );

                ui.add_space(8.0);

                // 文字
                ui.label(
                    RichText::new(text)
                        .size(14.0)
                        .color(if is_selected {
                            colors.accent_blue
                        } else {
                            colors.text_primary
                        })
                        .strong()
                );
            });
        });

        // 添加交互效果
        let response = response.response.interact(egui::Sense::click());

        let response = response.on_hover_text(tooltip);

        // 悬停时的视觉效果
        if response.hovered() {
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
        }

        if response.clicked() {
            state.current_tab = tab;
        }
    });

    ui.add_space(4.0);
}

/// 绘制统计信息区
fn draw_statistics_section(ui: &mut Ui, state: &AppState, colors: &ThemeColors) {
    // 统计标题
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        ui.label(
            RichText::new("STATISTICS")
                .size(11.0)
                .color(colors.text_muted)
        );
    });

    ui.add_space(12.0);

    // 已安装版本统计
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        draw_stat_card_compact(
            ui,
            "Installed",
            state.installed_versions.len().to_string().as_str(),
            "📦",
            colors.accent_blue,
            colors
        );
    });

    ui.add_space(8.0);

    // 可用版本统计
    let available_count = state.available_versions.iter()
        .filter(|v| !v.is_installed)
        .count();

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        draw_stat_card_compact(
            ui,
            "Available",
            available_count.to_string().as_str(),
            "🌐",
            colors.badge_green,
            colors
        );
    });

    // 下载中统计（排除特殊状态：错误、解压、完成）
    let downloading_count = state.downloads_in_progress.iter()
        .filter(|(key, _)| {
            !key.ends_with("_error")
                && !key.ends_with("_extracting")
                && !key.ends_with("_complete")
        })
        .count();
    if downloading_count > 0 {
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.add_space(8.0);
            draw_stat_card_compact(
                ui,
                "Downloading",
                downloading_count.to_string().as_str(),
                "⬇️",
                colors.badge_orange,
                colors
            );
        });
    }

    // 收藏统计
    let favorite_count = state.installed_versions.iter()
        .filter(|v| v.is_favorite)
        .count();
    if favorite_count > 0 {
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.add_space(8.0);
            draw_stat_card_compact(
                ui,
                "Favorites",
                favorite_count.to_string().as_str(),
                "⭐",
                colors.warning,
                colors
            );
        });
    }
}

/// 绘制紧凑型统计卡片
fn draw_stat_card_compact(ui: &mut Ui, label: &str, value: &str, icon: &str, color: egui::Color32, colors: &ThemeColors) {
    egui::Frame::NONE
        .fill(colors.bg_secondary)
        .corner_radius(8.0)
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // 左侧颜色条
                let color_bar = egui::Frame::NONE
                    .fill(color)
                    .corner_radius(4.0)
                    .inner_margin(egui::Margin::symmetric(2, 12));

                color_bar.show(ui, |ui| {
                    ui.set_width(4.0);
                    ui.set_height(24.0);
                });

                ui.add_space(8.0);

                // 图标
                ui.label(
                    RichText::new(icon)
                        .size(18.0)
                );

                ui.add_space(4.0);

                // 数值
                ui.label(
                    RichText::new(value)
                        .size(16.0)
                        .strong()
                        .color(colors.text_primary)
                );

                ui.add_space(4.0);

                // 标签
                ui.label(
                    RichText::new(label)
                        .size(12.0)
                        .color(colors.text_secondary)
                );
            });
        });
}

/// 绘制下载按钮
fn draw_download_button(ui: &mut Ui, state: &mut AppState, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // 主下载按钮
        let download_btn = egui::Button::new(
            RichText::new("⬇️  Download New Version")
                .size(13.0)
                .strong()
                .color(egui::Color32::WHITE)
        )
        .fill(colors.accent_blue)
        .min_size(Vec2::new(spacing::SIDEBAR_WIDTH_EXPANDED - 32.0, spacing::BUTTON_HEIGHT_LARGE));

        let response = ui.add(download_btn);

        let response = response.on_hover_text(
            "Download new Godot versions from GitHub releases"
        );

        if response.clicked() {
            state.show_download_dialog = true;
        }
    });
}

/// 绘制分隔线
fn draw_separator(ui: &mut Ui, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        ui.add_space(16.0);

        let separator = egui::Frame::NONE
            .fill(colors.border)
            .inner_margin(egui::Margin::symmetric(0, 0));

        separator.show(ui, |ui| {
            ui.set_width(spacing::SIDEBAR_WIDTH_EXPANDED - 32.0);
            ui.set_height(1.0);
        });
    });
}
