// SettingsPanel - 设置面板 UI 组件
// 优化版本：卡片式布局、统一设计风格、改进交互体验

use egui::{Color32, RichText, ScrollArea, Stroke, Vec2};

use crate::state::{AppState, Theme};

/// 绘制设置面板
pub fn draw_settings_panel(ui: &mut egui::Ui, state: &mut AppState) {
    // 顶部标题区域
    egui::TopBottomPanel::top("settings_header")
        .frame(egui::Frame::NONE.inner_margin(egui::Margin::same(16)))
        .show_inside(ui, |ui| {
            draw_panel_header(ui, state);
        });

    // 主内容区域
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.add_space(8.0);

            // 目录设置
            draw_directory_settings(ui, state);

            ui.add_space(16.0);

            // 行为设置
            draw_behavior_settings(ui, state);

            ui.add_space(16.0);

            // 关于信息
            draw_about_section(ui);

            ui.add_space(16.0);
        });
}

/// 绘制面板头部
fn draw_panel_header(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Settings");
            ui.label(
                RichText::new("Configure application preferences")
                    .small()
                    .weak()
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 保存按钮
            let save_btn = egui::Button::new("💾 Save")
                .fill(Color32::from_rgb(46, 139, 87));

            let mut response = ui.add(save_btn);
            response = response.on_hover_text("Save current settings");

            if response.clicked() {
                if let Err(e) = state.config.save() {
                    log::error!("Failed to save settings: {}", e);
                } else {
                    log::info!("Settings saved successfully");
                }
            }

            ui.add_space(8.0);

            // 重置按钮
            let reset_btn = egui::Button::new("🔄 Reset")
                .fill(Color32::from_rgb(220, 53, 69));

            let mut response = ui.add(reset_btn);
            response = response.on_hover_text("Reset all settings to default values");

            if response.clicked() {
                state.config = crate::state::AppConfig::default();
                log::info!("Settings reset to default");
            }
        });
    });
}

/// 绘制目录设置部分
fn draw_directory_settings(ui: &mut egui::Ui, state: &mut AppState) {
    draw_settings_card(ui, "📂 Directories", |ui| {
        // 安装目录设置
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Installation Directory")
                        .strong()
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let open_btn = egui::Button::new("📂 Open")
                        .small()
                        .fill(Color32::from_rgba_unmultiplied(128, 128, 128, 30));

                    if ui.add(open_btn).clicked() {
                        open_folder(&state.config.install_dir);
                    }
                });
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                let mut path_str = state.config.install_dir.display().to_string();
                let response = ui.add_sized(
                    [ui.available_width() - 90.0, 24.0],
                    egui::TextEdit::singleline(&mut path_str)
                );

                if response.changed() {
                    state.config.install_dir = std::path::PathBuf::from(path_str);
                }

                if ui.button("Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Select Installation Directory")
                        .pick_folder()
                    {
                        state.config.install_dir = path;
                        log::info!("Selected install directory: {:?}", state.config.install_dir);
                    }
                }
            });

            ui.add_space(4.0);

            ui.label(
                RichText::new("Where Godot versions will be installed")
                    .small()
                    .weak()
            );
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        // 项目目录设置
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Projects Directory")
                        .strong()
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let open_btn = egui::Button::new("📂 Open")
                        .small()
                        .fill(Color32::from_rgba_unmultiplied(128, 128, 128, 30));

                    if ui.add(open_btn).clicked() {
                        open_folder(&state.config.projects_dir);
                    }
                });
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                let mut path_str = state.config.projects_dir.display().to_string();
                let response = ui.add_sized(
                    [ui.available_width() - 90.0, 24.0],
                    egui::TextEdit::singleline(&mut path_str)
                );

                if response.changed() {
                    state.config.projects_dir = std::path::PathBuf::from(path_str);
                }

                if ui.button("Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Select Projects Directory")
                        .pick_folder()
                    {
                        state.config.projects_dir = path;
                        log::info!("Selected projects directory: {:?}", state.config.projects_dir);
                    }
                }
            });

            ui.add_space(4.0);

            ui.label(
                RichText::new("Where your Godot projects are stored")
                    .small()
                    .weak()
            );
        });
    });
}

/// 绘制行为设置部分
fn draw_behavior_settings(ui: &mut egui::Ui, state: &mut AppState) {
    draw_settings_card(ui, "⚙️ Behavior", |ui| {
        // 启动时检查更新
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let mut check_updates = state.config.check_updates_on_start;
                if ui.checkbox(&mut check_updates, "Check for updates on startup").clicked() {
                    state.config.check_updates_on_start = check_updates;
                }
            });

            ui.add_space(4.0);

            ui.label(
                RichText::new("Automatically check for new Godot versions when the app starts")
                    .small()
                    .weak()
            );
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        // 自动启动 Godot（占位）
        ui.vertical(|ui| {
            let mut auto_launch = false;
            if ui.checkbox(&mut auto_launch, "Launch Godot with default project").clicked() {
                // TODO: 实现自动启动功能
                log::info!("Auto launch setting changed");
            }

            ui.add_space(4.0);

            ui.label(
                RichText::new("Automatically open a project when launching Godot")
                    .small()
                    .weak()
            );
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        // 主题选择
        ui.vertical(|ui| {
            ui.label(
                RichText::new("Theme")
                    .strong()
            );

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                // Dark 主题按钮
                let is_dark = state.config.theme == Theme::Dark;
                let dark_btn = egui::Button::new("🌙 Dark")
                    .fill(if is_dark {
                        Color32::from_rgb(70, 130, 180)
                    } else {
                        Color32::from_rgba_unmultiplied(128, 128, 128, 30)
                    })
                    .min_size(Vec2::new(80.0, 28.0));

                let mut response = ui.add(dark_btn);
                response = response.on_hover_text("Use dark theme");

                if response.clicked() {
                    state.config.theme = Theme::Dark;
                    if let Err(e) = state.config.save() {
                        log::error!("Failed to save theme setting: {}", e);
                    }
                    log::info!("Theme changed to Dark");
                }

                ui.add_space(8.0);

                // Light 主题按钮
                let is_light = state.config.theme == Theme::Light;
                let light_btn = egui::Button::new("☀️ Light")
                    .fill(if is_light {
                        Color32::from_rgb(70, 130, 180)
                    } else {
                        Color32::from_rgba_unmultiplied(128, 128, 128, 30)
                    })
                    .min_size(Vec2::new(80.0, 28.0));

                let mut response = ui.add(light_btn);
                response = response.on_hover_text("Use light theme");

                if response.clicked() {
                    state.config.theme = Theme::Light;
                    if let Err(e) = state.config.save() {
                        log::error!("Failed to save theme setting: {}", e);
                    }
                    log::info!("Theme changed to Light");
                }

                ui.add_space(8.0);

                // System 主题按钮
                let is_system = state.config.theme == Theme::System;
                let system_btn = egui::Button::new("💻 System")
                    .fill(if is_system {
                        Color32::from_rgb(70, 130, 180)
                    } else {
                        Color32::from_rgba_unmultiplied(128, 128, 128, 30)
                    })
                    .min_size(Vec2::new(80.0, 28.0));

                let mut response = ui.add(system_btn);
                response = response.on_hover_text("Follow system theme");

                if response.clicked() {
                    state.config.theme = Theme::System;
                    if let Err(e) = state.config.save() {
                        log::error!("Failed to save theme setting: {}", e);
                    }
                    log::info!("Theme changed to System");
                }
            });

            ui.add_space(4.0);

            ui.label(
                RichText::new("Choose your preferred color theme")
                    .small()
                    .weak()
            );
        });
    });
}

/// 绘制关于部分
fn draw_about_section(ui: &mut egui::Ui) {
    draw_settings_card(ui, "ℹ️ About", |ui| {
        ui.vertical(|ui| {
            // 应用信息
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🎮")
                        .size(32.0)
                );

                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Godot Hub")
                            .size(18.0)
                            .strong()
                    );

                    ui.label(
                        RichText::new("Version 0.1.0")
                            .small()
                            .weak()
                    );
                });
            });

            ui.add_space(12.0);

            ui.label(
                RichText::new("A cross-platform Godot engine manager built with Rust and egui")
                    .weak()
            );

            ui.add_space(16.0);

            // 链接
            ui.horizontal(|ui| {
                ui.hyperlink_to(
                    "🌐 Godot Engine",
                    "https://godotengine.org/"
                );

                ui.add_space(16.0);

                ui.hyperlink_to(
                    "📦 GitHub Repository",
                    "https://github.com/"
                );
            });

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            // 技术栈信息
            ui.label(
                RichText::new("Built with:")
                    .strong()
            );

            ui.add_space(8.0);

            ui.horizontal_wrapped(|ui| {
                for tech in &["Rust", "egui", "eframe", "tokio", "serde"] {
                    ui.label(
                        RichText::new(format!(" {} ", tech))
                            .small()
                            .background_color(Color32::from_rgba_unmultiplied(70, 130, 180, 50))
                            .color(Color32::from_rgb(70, 130, 180))
                    );
                    ui.add_space(4.0);
                }
            });

            ui.add_space(16.0);

            // 许可证信息
            ui.label(
                RichText::new("MIT License")
                    .small()
                    .weak()
            );
        });
    });
}

/// 绘制设置卡片
fn draw_settings_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::group(ui.style())
        .inner_margin(16.0)
        .outer_margin(0.0)
        .corner_radius(8.0)
        .stroke(Stroke::new(
            1.0,
            ui.style().visuals.widgets.noninteractive.bg_stroke.color
        ))
        .show(ui, |ui| {
            // 卡片标题
            ui.label(
                RichText::new(title)
                    .size(16.0)
                    .strong()
            );

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            // 卡片内容
            content(ui);
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

/// 保存设置（供外部调用）
pub fn save_settings(state: &mut AppState) -> Result<(), String> {
    // 保存配置到文件
    state
        .config
        .save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;

    log::info!("Settings saved successfully");
    Ok(())
}

/// 验证设置是否有效
pub fn validate_settings(state: &AppState) -> Result<(), String> {
    // 验证安装目录
    if state.config.install_dir.to_string_lossy().is_empty() {
        return Err("Installation directory cannot be empty".to_string());
    }

    // 验证项目目录
    if state.config.projects_dir.to_string_lossy().is_empty() {
        return Err("Projects directory cannot be empty".to_string());
    }

    // 验证目录权限（尝试创建目录）
    if let Err(e) = state.config.ensure_directories() {
        return Err(format!("Cannot create required directories: {}", e));
    }

    Ok(())
}
