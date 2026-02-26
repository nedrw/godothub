// Sidebar - 侧边栏 UI 组件
// 优化版本：添加图标、改进视觉层次、增强交互反馈

use egui::{Color32, RichText, Ui, Vec2};

use crate::state::{AppState, MainTab};

/// 绘制侧边栏
pub fn draw_sidebar(ui: &mut Ui, state: &mut AppState) {
    // 设置侧边栏内边距
    ui.set_min_width(200.0);

    // 应用标题区
    draw_app_header(ui);

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // 导航区
    draw_navigation_section(ui, state);

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // 统计信息区
    draw_statistics_section(ui, state);

    // 底部下载按钮（固定在底部）
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.add_space(16.0);
        draw_download_button(ui, state);
        ui.add_space(8.0);

        // 版本信息
        ui.label(
            RichText::new("v0.1.0")
                .small()
                .weak()
        );
    });
}

/// 绘制应用标题
fn draw_app_header(ui: &mut Ui) {
    ui.add_space(12.0);

    // 应用图标和名称
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // 使用 emoji 作为图标
        ui.label(
            RichText::new("🎮")
                .size(28.0)
        );

        ui.vertical(|ui| {
            ui.label(
                RichText::new("Godot Hub")
                    .size(18.0)
                    .strong()
            );
            ui.label(
                RichText::new("Engine Manager")
                    .small()
                    .weak()
            );
        });
    });
}

/// 绘制导航区
fn draw_navigation_section(ui: &mut Ui, state: &mut AppState) {
    // 导航标题
    ui.label(
        RichText::new("NAVIGATION")
            .small()
            .weak()
    );
    ui.add_space(8.0);

    // 导航按钮
    draw_nav_button(
        ui,
        "📦  Versions",
        "Manage Godot engine installations",
        MainTab::Versions,
        state
    );

    draw_nav_button(
        ui,
        "📁  Projects",
        "Browse and manage your projects",
        MainTab::Projects,
        state
    );

    draw_nav_button(
        ui,
        "⚙️  Settings",
        "Configure application preferences",
        MainTab::Settings,
        state
    );
}

/// 绘制导航按钮
fn draw_nav_button(
    ui: &mut Ui,
    text: &str,
    tooltip: &str,
    tab: MainTab,
    state: &mut AppState
) {
    let is_selected = state.current_tab == tab;

    // 根据选中状态设置不同的样式
    let button = if is_selected {
        egui::Button::new(
            RichText::new(text)
                .strong()
                .color(Color32::WHITE)
        )
        .fill(Color32::from_rgb(70, 130, 180))
        .min_size(Vec2::new(ui.available_width() - 16.0, 36.0))
    } else {
        egui::Button::new(RichText::new(text))
        .fill(Color32::TRANSPARENT)
        .min_size(Vec2::new(ui.available_width() - 16.0, 36.0))
    };

    // 添加左侧缩进
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        let mut response = ui.add(button);

        // 添加工具提示
        let response = response.on_hover_text(tooltip);

        if response.clicked() {
            state.current_tab = tab;
        }
    });

    ui.add_space(4.0);
}

/// 绘制统计信息区
fn draw_statistics_section(ui: &mut Ui, state: &AppState) {
    // 统计标题
    ui.label(
        RichText::new("STATISTICS")
            .small()
            .weak()
    );
    ui.add_space(8.0);

    // 已安装版本统计
    draw_stat_card(
        ui,
        "Installed",
        state.installed_versions.len().to_string().as_str(),
        "📦"
    );

    ui.add_space(8.0);

    // 可用版本统计
    let available_count = state.available_versions.iter()
        .filter(|v| !v.is_installed)
        .count();
    draw_stat_card(
        ui,
        "Available",
        available_count.to_string().as_str(),
        "🌐"
    );

    ui.add_space(8.0);

    // 下载中统计
    let downloading_count = state.downloads_in_progress.len();
    if downloading_count > 0 {
        draw_stat_card(
            ui,
            "Downloading",
            downloading_count.to_string().as_str(),
            "⬇️"
        );
        ui.add_space(8.0);
    }

    // 收藏统计
    let favorite_count = state.installed_versions.iter()
        .filter(|v| v.is_favorite)
        .count();
    if favorite_count > 0 {
        draw_stat_card(
            ui,
            "Favorites",
            favorite_count.to_string().as_str(),
            "⭐"
        );
    }
}

/// 绘制统计卡片
fn draw_stat_card(ui: &mut Ui, label: &str, value: &str, icon: &str) {
    egui::Frame::group(ui.style())
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(6.0)
        .fill(Color32::from_rgba_unmultiplied(128, 128, 128, 20))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(icon)
                        .size(20.0)
                );

                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(value)
                            .size(18.0)
                            .strong()
                    );
                    ui.label(
                        RichText::new(label)
                            .small()
                            .weak()
                    );
                });
            });
        });
}

/// 绘制下载按钮
fn draw_download_button(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // 主下载按钮
        let download_btn = egui::Button::new(
            RichText::new("⬇️  Download New Version")
                .strong()
        )
        .fill(Color32::from_rgb(70, 130, 180))
        .min_size(Vec2::new(ui.available_width() - 16.0, 40.0));

        let mut response = ui.add(download_btn);

        let response = response.on_hover_text(
            "Download new Godot versions from GitHub releases"
        );

        if response.clicked() {
            state.show_download_dialog = true;
        }
    });
}
