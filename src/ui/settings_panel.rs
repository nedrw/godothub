// SettingsPanel - 设置面板 UI 组件
// 优化版本：使用统一样式系统、卡片式布局、清晰信息层次

use egui::{RichText, ScrollArea, Stroke, Vec2};

use crate::state::{AppState, Theme, DownloadSource};
use crate::ui::style::{colors, spacing, card_frame, panel_header,
                       primary_button, secondary_button, danger_button, badge};

/// 绘制设置面板
pub fn draw_settings_panel(ui: &mut egui::Ui, state: &mut AppState) {
    // 设置面板背景
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.inner_margin(16.0))
        .show_inside(ui, |ui| {
            // 顶部标题区域
            egui::TopBottomPanel::top("settings_header")
                .frame(egui::Frame::NONE)
                .show_inside(ui, |ui| {
                    draw_panel_header(ui, state);
                });

            ui.add_space(16.0);

            // 主内容区域
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    // 目录设置
                    draw_directory_settings(ui, state);

                    ui.add_space(24.0);

                    // 行为设置
                    draw_behavior_settings(ui, state);

                    ui.add_space(24.0);

                    // 关于信息
                    draw_about_section(ui, state.config.theme);
                });
        });
}

/// 绘制面板头部
fn draw_panel_header(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        panel_header(ui, state.config.theme, "Settings", "Configure application preferences");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 保存按钮
            let save_btn = primary_button("💾 Save Settings", state.config.theme);
            let response = ui.add(save_btn).on_hover_text("Save current settings");

            if response.clicked() {
                if let Err(e) = state.config.save() {
                    log::error!("Failed to save settings: {}", e);
                } else {
                    log::info!("Settings saved successfully");
                }
            }

            ui.add_space(8.0);

            // 重置按钮
            let reset_btn = danger_button("🔄 Reset");
            let response = ui.add(reset_btn).on_hover_text("Reset all settings to default values");

            if response.clicked() {
                state.config = crate::state::AppConfig::default();
                log::info!("Settings reset to default");
            }
        });
    });
}

/// 绘制目录设置部分
fn draw_directory_settings(ui: &mut egui::Ui, state: &mut AppState) {
    draw_settings_section(ui, state.config.theme, "📂 Directories", "Configure installation and project directories", |ui| {
        // 安装目录设置
        draw_directory_setting(
            ui,
            state.config.theme,
            "Installation Directory",
            "Where Godot versions will be installed",
            &mut state.config.install_dir,
            "Select Installation Directory",
        );

        ui.add_space(20.0);

        // 项目目录设置
        draw_directory_setting(
            ui,
            state.config.theme,
            "Projects Directory",
            "Default location for your Godot projects",
            &mut state.config.projects_dir,
            "Select Projects Directory",
        );
    });
}

/// 绘制单个目录设置项
fn draw_directory_setting(
    ui: &mut egui::Ui,
    theme: Theme,
    label: &str,
    description: &str,
    path: &mut std::path::PathBuf,
    dialog_title: &str,
) {
    ui.vertical(|ui| {
        // 标签和操作按钮
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(label)
                    .size(14.0)
                    .strong()
                    .color(colors::TEXT_PRIMARY)
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 打开文件夹按钮
                let open_btn = egui::Button::new(
                    RichText::new("📂 Open")
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY)
                )
                .fill(colors::BG_HOVER)
                .min_size(Vec2::new(70.0, 24.0));

                if ui.add(open_btn).clicked() {
                    open_folder(path);
                }
            });
        });

        ui.add_space(8.0);

        // 路径输入框和浏览按钮
        ui.horizontal(|ui| {
            let mut path_str = path.display().to_string();

            let text_edit = egui::TextEdit::singleline(&mut path_str)
                .desired_width(ui.available_width() - 100.0)
                .margin(egui::Vec2::new(8.0, 6.0));

            let response = ui.add(text_edit);

            if response.changed() {
                *path = std::path::PathBuf::from(path_str);
            }

            // 浏览按钮
            let browse_btn = secondary_button("Browse", theme);
            if ui.add(browse_btn).clicked() {
                if let Some(selected_path) = rfd::FileDialog::new()
                    .set_title(dialog_title)
                    .pick_folder()
                {
                    *path = selected_path;
                    log::info!("Selected directory: {:?}", path);
                }
            }
        });

        ui.add_space(6.0);

        // 描述文字
        ui.label(
            RichText::new(description)
                .size(12.0)
                .color(colors::TEXT_SECONDARY)
        );
    });
}

/// 绘制行为设置部分
fn draw_behavior_settings(ui: &mut egui::Ui, state: &mut AppState) {
    draw_settings_section(ui, state.config.theme, "⚙️ Behavior", "Application startup and runtime behavior", |ui| {
        // 启动时检查更新
        draw_toggle_setting(
            ui,
            "Check for Updates on Startup",
            "Automatically check for Godot updates when the application starts",
            &mut state.config.check_updates_on_start,
        );

        ui.add_space(16.0);

        // 主题选择
        draw_theme_setting(ui, &mut state.config.theme);

        ui.add_space(16.0);

        // 下载源选择
        draw_download_source_setting(ui, &mut state.config.download_source);
    });
}

/// 绘制开关设置项
fn draw_toggle_setting(
    ui: &mut egui::Ui,
    label: &str,
    description: &str,
    value: &mut bool,
) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(
                RichText::new(label)
                    .size(14.0)
                    .strong()
                    .color(colors::TEXT_PRIMARY)
            );

            ui.add_space(4.0);

            ui.label(
                RichText::new(description)
                    .size(12.0)
                    .color(colors::TEXT_SECONDARY)
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(egui::Checkbox::new(value, ""));
        });
    });
}

/// 绘制主题选择设置
fn draw_theme_setting(ui: &mut egui::Ui, theme: &mut Theme) {
    ui.vertical(|ui| {
        ui.label(
            RichText::new("Application Theme")
                .size(14.0)
                .strong()
                .color(colors::TEXT_PRIMARY)
        );

        ui.add_space(4.0);

        ui.label(
            RichText::new("Choose the color theme for the application")
                .size(12.0)
                .color(colors::TEXT_SECONDARY)
        );

        ui.add_space(12.0);

        ui.horizontal(|ui| {
            theme_button(ui, "🌙 Dark", Theme::Dark, theme);
            ui.add_space(8.0);
            theme_button(ui, "☀️ Light", Theme::Light, theme);
            ui.add_space(8.0);
            theme_button(ui, "💻 System", Theme::System, theme);
        });
    });
}

/// 绘制主题选择按钮
fn theme_button(ui: &mut egui::Ui, text: &str, theme_type: Theme, current_theme: &mut Theme) {
    let is_selected = *current_theme == theme_type;

    let btn = if is_selected {
        egui::Button::new(
            RichText::new(text)
                .color(colors::TEXT_PRIMARY)
                .strong()
        )
        .fill(colors::ACCENT_BLUE)
        .min_size(Vec2::new(100.0, spacing::BUTTON_HEIGHT))
    } else {
        egui::Button::new(
            RichText::new(text)
                .color(colors::TEXT_PRIMARY)
        )
        .fill(colors::BG_SECONDARY)
        .stroke(Stroke::new(1.0, colors::BORDER))
        .min_size(Vec2::new(100.0, spacing::BUTTON_HEIGHT))
    };

    if ui.add(btn).clicked() {
        *current_theme = theme_type;
    }
}

/// 绘制下载源选择设置
fn draw_download_source_setting(ui: &mut egui::Ui, source: &mut DownloadSource) {
    ui.vertical(|ui| {
        ui.label(
            RichText::new("Download Source")
                .size(14.0)
                .strong()
                .color(colors::TEXT_PRIMARY)
        );

        ui.add_space(4.0);

        ui.label(
            RichText::new("Select the source for downloading Godot versions")
                .size(12.0)
                .color(colors::TEXT_SECONDARY)
        );

        ui.add_space(12.0);

        ui.horizontal(|ui| {
            source_button(ui, "🐙 GitHub", DownloadSource::GitHub, source);
            ui.add_space(8.0);
            source_button(ui, "🇨🇳 China Mirror", DownloadSource::ChinaMirror, source);
        });
    });
}

/// 绘制下载源选择按钮
fn source_button(ui: &mut egui::Ui, text: &str, source_type: DownloadSource, current_source: &mut DownloadSource) {
    let is_selected = *current_source == source_type;

    let btn = if is_selected {
        egui::Button::new(
            RichText::new(text)
                .color(colors::TEXT_PRIMARY)
                .strong()
        )
        .fill(colors::ACCENT_BLUE)
        .min_size(Vec2::new(110.0, spacing::BUTTON_HEIGHT))
    } else {
        egui::Button::new(
            RichText::new(text)
                .color(colors::TEXT_PRIMARY)
        )
        .fill(colors::BG_SECONDARY)
        .stroke(Stroke::new(1.0, colors::BORDER))
        .min_size(Vec2::new(110.0, spacing::BUTTON_HEIGHT))
    };

    if ui.add(btn).clicked() {
        *current_source = source_type;
    }
}

/// 绘制关于部分
fn draw_about_section(ui: &mut egui::Ui, theme: Theme) {
    draw_settings_section(ui, theme, "ℹ️ About", "Application information", |ui| {
        ui.vertical(|ui| {
            // 应用名称和版本
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🎮")
                        .size(32.0)
                );

                ui.add_space(12.0);

                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Godot Hub")
                            .size(18.0)
                            .strong()
                            .color(colors::TEXT_PRIMARY)
                    );

                    badge(ui, "v0.1.0", colors::BADGE_PURPLE);
                });
            });

            ui.add_space(16.0);

            ui.label(
                RichText::new("A modern Godot Engine management application built with Rust and egui.")
                    .size(13.0)
                    .color(colors::TEXT_SECONDARY)
            );

            ui.add_space(20.0);

            // 链接按钮
            ui.horizontal(|ui| {
                let github_btn = secondary_button("🐙 GitHub", theme);
                if ui.add(github_btn).clicked() {
                    // TODO: 打开 GitHub 页面
                    log::info!("Opening GitHub page");
                }

                ui.add_space(8.0);

                let website_btn = secondary_button("🌐 Website", theme);
                if ui.add(website_btn).clicked() {
                    // TODO: 打开官方网站
                    log::info!("Opening website");
                }
            });

            ui.add_space(16.0);

            // 技术栈信息
            ui.label(
                RichText::new("Built with:")
                    .size(12.0)
                    .strong()
                    .color(colors::TEXT_PRIMARY)
            );

            ui.add_space(8.0);

            ui.horizontal_wrapped(|ui| {
                for tech in &["Rust", "egui", "eframe", "tokio", "serde"] {
                    badge(ui, tech, colors::BG_HOVER);
                    ui.add_space(6.0);
                }
            });

            ui.add_space(16.0);

            // 版权信息
            ui.label(
                RichText::new("© 2025 Godot Hub. Licensed under MIT License.")
                    .size(11.0)
                    .color(colors::TEXT_MUTED)
            );
        });
    });
}

/// 绘制设置区块容器
fn draw_settings_section(
    ui: &mut egui::Ui,
    theme: Theme,
    title: &str,
    description: &str,
    content: impl FnOnce(&mut egui::Ui),
) {
    ui.vertical(|ui| {
        // 区域标题
        ui.label(
            RichText::new(title)
                .size(16.0)
                .strong()
                .color(colors::TEXT_PRIMARY)
        );

        ui.add_space(4.0);

        ui.label(
            RichText::new(description)
                .size(12.0)
                .color(colors::TEXT_SECONDARY)
        );

        ui.add_space(12.0);

        // 内容卡片
        card_frame(theme).show(ui, |ui| {
            content(ui);
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

/// 保存设置到文件
pub fn save_settings(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    state.config.save()?;
    log::info!("Settings saved successfully");
    Ok(())
}

/// 验证设置是否有效
pub fn validate_settings(state: &AppState) -> Result<(), String> {
    // 检查安装目录
    if !state.config.install_dir.exists() {
        return Err(format!(
            "Installation directory does not exist: {}",
            state.config.install_dir.display()
        ));
    }

    // 检查项目目录
    if !state.config.projects_dir.exists() {
        return Err(format!(
            "Projects directory does not exist: {}",
            state.config.projects_dir.display()
        ));
    }

    Ok(())
}
