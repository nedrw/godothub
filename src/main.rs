// Godot Hub - A Godot Engine Management Application
// Built with eframe/egui for cross-platform UI

mod models;
mod services;
mod state;
mod ui;
mod utils;

use eframe::{egui, App, Frame, NativeOptions};
use std::sync::Arc;
use std::time::Duration;
use state::Theme;
use tokio::runtime::Runtime;

/// 主应用程序
struct GodotHubApp {
    state: state::AppState,
}

impl Default for GodotHubApp {
    fn default() -> Self {
        // 创建 Tokio 运行时
        let runtime = Arc::new(
            Runtime::new()
                .expect("Failed to create Tokio runtime")
        );

        let mut app_state = state::AppState::default();
        app_state.load_installed_versions();

        // 将运行时设置到 state 中，供下载等功能使用
        app_state.runtime = Some(runtime);

        // 立即启动版本列表刷新
        app_state.refresh_available_versions();

        Self {
            state: app_state,
        }
    }
}

impl App for GodotHubApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // 应用主题
        apply_theme(ctx, self.state.config.theme);

        // 检查并处理版本刷新结果
        self.state.poll_refresh_result();

        // 更新下载进度（模拟）
        for (_version, progress) in &mut self.state.downloads_in_progress {
            if *progress < 1.0 {
                *progress += 0.01;
                if *progress >= 1.0 {
                    log::info!("Download complete");
                }
            }
        }

        // 请求定期重绘
        ctx.request_repaint_after(Duration::from_millis(100));

        // 绘制侧边栏
        egui::SidePanel::left("sidebar")
            .width_range(200.0..=300.0)
            .show(ctx, |ui| {
                ui::draw_sidebar(ui, &mut self.state);
            });

        // 绘制主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state.current_tab {
                state::MainTab::Versions => ui::draw_versions_panel(ui, &mut self.state),
                state::MainTab::Projects => ui::draw_projects_panel(ui, &mut self.state),
                state::MainTab::Settings => ui::draw_settings_panel(ui, &mut self.state),
            }

            // 显示下载对话框
            if self.state.show_download_dialog {
                ui::draw_download_dialog(ui, &mut self.state);
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // 保存配置
        if let Err(e) = self.state.config.save() {
            log::error!("Failed to save config: {}", e);
        }
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

/// 应用主题
fn apply_theme(ctx: &egui::Context, theme: Theme) {
    let mut style = (*ctx.style()).clone();

    match theme {
        Theme::Dark => {
            style.visuals = egui::Visuals::dark();
            style.visuals.window_fill = egui::Color32::from_rgb(25, 25, 25);
            style.visuals.panel_fill = egui::Color32::from_rgb(30, 30, 30);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(20, 20, 20);
        }
        Theme::Light => {
            style.visuals = egui::Visuals::light();
            style.visuals.window_fill = egui::Color32::from_rgb(245, 245, 245);
            style.visuals.panel_fill = egui::Color32::from_rgb(240, 240, 240);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(235, 235, 235);
        }
        Theme::System => {
            // 暂时使用深色主题，后续可以通过系统 API 检测
            style.visuals = egui::Visuals::dark();
        }
    }

    // 自定义一些通用样式
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.interact_size = egui::vec2(40.0, 20.0);
    style.visuals.button_frame = true;
    style.visuals.collapsing_header_frame = true;
    style.visuals.selection.bg_fill = egui::Color32::from_rgb(70, 130, 180);

    ctx.set_style(style);
}

/// 应用程序入口点
fn main() -> eframe::Result<()> {
    // 初始化日志系统
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Godot Hub");

    // 创建应用程序实例
    let app = GodotHubApp::default();

    // 配置原生窗口选项
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 500.0])
            .with_title("Godot Hub"),
        persist_window: true,
        ..Default::default()
    };

    // 运行应用程序
    eframe::run_native("Godot Hub", native_options, Box::new(|_cc| Ok(Box::new(app))))
}
