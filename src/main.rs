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

        // 创建共享状态指针，用于异步任务更新进度
        app_state.create_shared_state();

        // 立即启动版本列表刷新
        app_state.refresh_available_versions();

        Self {
            state: app_state,
        }
    }
}

impl App for GodotHubApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // 应用主题和样式
        ui::setup_visuals(ctx, self.state.config.theme);

        // 检查并处理版本刷新结果
        self.state.poll_refresh_result();

        // 从共享状态同步下载进度到主状态
        // 这样 UI 可以实时显示异步任务的进度和完成状态
        self.state.sync_download_progress();

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
