// Style - 统一样式模块
// 包含所有颜色常量、样式配置和可复用的 UI 组件

use egui::{Color32, RichText, Stroke, Vec2};
use crate::state::Theme;

// ============================================================================
// 颜色常量定义 - 支持深色和浅色主题
// ============================================================================

/// 深色主题颜色
pub mod dark_colors {
    use super::Color32;

    pub const BG_PRIMARY: Color32 = Color32::from_rgb(26, 29, 35);
    pub const BG_SECONDARY: Color32 = Color32::from_rgb(37, 40, 48);
    pub const BG_SIDEBAR: Color32 = Color32::from_rgb(22, 24, 29);
    pub const BG_HOVER: Color32 = Color32::from_rgb(45, 49, 58);
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(71, 140, 191);
    pub const ACCENT_BLUE_LIGHT: Color32 = Color32::from_rgb(92, 143, 184);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(224, 224, 224);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 146, 168);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(107, 114, 128);
    pub const BORDER: Color32 = Color32::from_rgb(58, 63, 75);
    pub const BADGE_BLUE: Color32 = Color32::from_rgb(71, 140, 191);
    pub const BADGE_PURPLE: Color32 = Color32::from_rgb(142, 68, 173);
    pub const BADGE_GREEN: Color32 = Color32::from_rgb(39, 174, 96);
    pub const BADGE_ORANGE: Color32 = Color32::from_rgb(255, 152, 0);
    pub const SUCCESS: Color32 = Color32::from_rgb(46, 139, 87);
    pub const WARNING: Color32 = Color32::from_rgb(255, 165, 0);
    pub const ERROR: Color32 = Color32::from_rgb(220, 53, 69);
}

/// 浅色主题颜色
pub mod light_colors {
    use super::Color32;

    pub const BG_PRIMARY: Color32 = Color32::from_rgb(245, 247, 250);
    pub const BG_SECONDARY: Color32 = Color32::from_rgb(255, 255, 255);
    pub const BG_SIDEBAR: Color32 = Color32::from_rgb(240, 242, 245);
    pub const BG_HOVER: Color32 = Color32::from_rgb(232, 235, 240);
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(41, 98, 255);
    pub const ACCENT_BLUE_LIGHT: Color32 = Color32::from_rgb(66, 133, 244);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(32, 33, 36);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(95, 99, 104);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(128, 134, 139);
    pub const BORDER: Color32 = Color32::from_rgb(218, 220, 224);
    pub const BADGE_BLUE: Color32 = Color32::from_rgb(41, 98, 255);
    pub const BADGE_PURPLE: Color32 = Color32::from_rgb(156, 39, 176);
    pub const BADGE_GREEN: Color32 = Color32::from_rgb(76, 175, 80);
    pub const BADGE_ORANGE: Color32 = Color32::from_rgb(255, 152, 0);
    pub const SUCCESS: Color32 = Color32::from_rgb(76, 175, 80);
    pub const WARNING: Color32 = Color32::from_rgb(255, 193, 7);
    pub const ERROR: Color32 = Color32::from_rgb(244, 67, 54);
}

/// 当前主题的颜色集合
pub struct ThemeColors {
    pub bg_primary: Color32,
    pub bg_secondary: Color32,
    pub bg_sidebar: Color32,
    pub bg_hover: Color32,
    pub accent_blue: Color32,
    pub accent_blue_light: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub border: Color32,
    pub badge_blue: Color32,
    pub badge_purple: Color32,
    pub badge_green: Color32,
    pub badge_orange: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
}

impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            bg_primary: dark_colors::BG_PRIMARY,
            bg_secondary: dark_colors::BG_SECONDARY,
            bg_sidebar: dark_colors::BG_SIDEBAR,
            bg_hover: dark_colors::BG_HOVER,
            accent_blue: dark_colors::ACCENT_BLUE,
            accent_blue_light: dark_colors::ACCENT_BLUE_LIGHT,
            text_primary: dark_colors::TEXT_PRIMARY,
            text_secondary: dark_colors::TEXT_SECONDARY,
            text_muted: dark_colors::TEXT_MUTED,
            border: dark_colors::BORDER,
            badge_blue: dark_colors::BADGE_BLUE,
            badge_purple: dark_colors::BADGE_PURPLE,
            badge_green: dark_colors::BADGE_GREEN,
            badge_orange: dark_colors::BADGE_ORANGE,
            success: dark_colors::SUCCESS,
            warning: dark_colors::WARNING,
            error: dark_colors::ERROR,
        }
    }

    pub fn light() -> Self {
        Self {
            bg_primary: light_colors::BG_PRIMARY,
            bg_secondary: light_colors::BG_SECONDARY,
            bg_sidebar: light_colors::BG_SIDEBAR,
            bg_hover: light_colors::BG_HOVER,
            accent_blue: light_colors::ACCENT_BLUE,
            accent_blue_light: light_colors::ACCENT_BLUE_LIGHT,
            text_primary: light_colors::TEXT_PRIMARY,
            text_secondary: light_colors::TEXT_SECONDARY,
            text_muted: light_colors::TEXT_MUTED,
            border: light_colors::BORDER,
            badge_blue: light_colors::BADGE_BLUE,
            badge_purple: light_colors::BADGE_PURPLE,
            badge_green: light_colors::BADGE_GREEN,
            badge_orange: light_colors::BADGE_ORANGE,
            success: light_colors::SUCCESS,
            warning: light_colors::WARNING,
            error: light_colors::ERROR,
        }
    }

    pub fn from_theme(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Self::dark(),
            Theme::Light => Self::light(),
            Theme::System => {
                // TODO: 检测系统主题
                Self::dark()
            }
        }
    }
}

// 为了向后兼容，提供默认的 colors 模块
pub mod colors {


}

// ============================================================================
// 尺寸常量定义
// ============================================================================

pub mod spacing {
    pub const SIDEBAR_WIDTH_COLLAPSED: f32 = 60.0;
    pub const SIDEBAR_WIDTH_EXPANDED: f32 = 220.0;
    pub const CARD_GAP: f32 = 16.0;
    pub const PAGE_PADDING: f32 = 24.0;
    pub const CARD_ROUNDING: f32 = 12.0;
    pub const BUTTON_ROUNDING: f32 = 6.0;
    pub const PILL_ROUNDING: f32 = 12.0;
    pub const BUTTON_HEIGHT: f32 = 32.0;
    pub const BUTTON_HEIGHT_LARGE: f32 = 40.0;
}

// ============================================================================
// 样式配置函数
// ============================================================================

/// 配置 egui 的视觉效果（支持主题切换）
pub fn setup_visuals(ctx: &egui::Context, theme: Theme) {
    let colors = ThemeColors::from_theme(theme);

    let mut visuals = match theme {
        Theme::Light => egui::Visuals::light(),
        Theme::Dark | Theme::System => egui::Visuals::dark(),
    };

    // 窗口/面板背景
    visuals.window_fill = colors.bg_primary;
    visuals.panel_fill = colors.bg_primary;
    visuals.extreme_bg_color = colors.bg_sidebar;

    // 文字
    visuals.override_text_color = Some(colors.text_primary);
    visuals.text_cursor.stroke.color = colors.accent_blue;

    // 组件 - 非激活状态
    visuals.widgets.inactive.weak_bg_fill = colors.bg_secondary;
    visuals.widgets.inactive.bg_fill = colors.bg_secondary;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, colors.border);

    // 组件 - 悬停状态
    visuals.widgets.hovered.weak_bg_fill = colors.bg_hover;
    visuals.widgets.hovered.bg_fill = colors.bg_hover;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, colors.accent_blue);

    // 组件 - 激活状态
    visuals.widgets.active.bg_fill = colors.accent_blue;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, colors.accent_blue_light);

    // 组件 - 打开状态
    visuals.widgets.open.bg_fill = colors.bg_hover;

    // 选择高亮
    visuals.selection.bg_fill = colors.accent_blue;

    // 应用样式
    ctx.set_visuals(visuals);

    // 自定义样式
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.interact_size = egui::vec2(40.0, 20.0);
    style.visuals.button_frame = true;
    style.visuals.collapsing_header_frame = true;
    ctx.set_style(style);
}

/// 获取当前主题的颜色
pub fn get_theme_colors(theme: Theme) -> ThemeColors {
    ThemeColors::from_theme(theme)
}

// ============================================================================
// 可复用 UI 组件
// ============================================================================

/// 绘制状态标签
pub fn badge(ui: &mut egui::Ui, text: &str, color: Color32) {
    ui.label(
        RichText::new(text)
            .size(11.0)
            .color(Color32::WHITE)
            .background_color(color)
    );
}

/// 绘制 Pill 形状的状态标签
pub fn status_pill(ui: &mut egui::Ui, text: &str, color: Color32) {
    egui::Frame::NONE
        .fill(color)
        .corner_radius(spacing::PILL_ROUNDING)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .size(11.0)
                    .color(Color32::WHITE)
                    .strong()
            );
        });
}

/// 绘制卡片容器
pub fn card_frame(theme: Theme) -> egui::Frame {
    let colors = ThemeColors::from_theme(theme);
    egui::Frame::NONE
        .fill(colors.bg_secondary)
        .corner_radius(spacing::CARD_ROUNDING)
        .stroke(Stroke::new(1.0, colors.border))
        .inner_margin(12.0)
}

/// 绘制主要按钮
pub fn primary_button(text: &str, theme: Theme) -> egui::Button<'_> {
    let colors = ThemeColors::from_theme(theme);
    egui::Button::new(
        RichText::new(text)
            .strong()
            .color(Color32::WHITE)
    )
    .fill(colors.accent_blue)
    .min_size(Vec2::new(120.0, spacing::BUTTON_HEIGHT))
}

/// 绘制次要按钮
pub fn secondary_button(text: &str, theme: Theme) -> egui::Button<'_> {
    let colors = ThemeColors::from_theme(theme);
    egui::Button::new(RichText::new(text).color(colors.text_primary))
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::new(1.0, colors.border))
        .min_size(Vec2::new(120.0, spacing::BUTTON_HEIGHT))
}

/// 绘制危险操作按钮
pub fn danger_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(
        RichText::new(text)
            .color(Color32::WHITE)
    )
    .fill(dark_colors::ERROR)
    .min_size(Vec2::new(120.0, spacing::BUTTON_HEIGHT))
}

/// 绘制成功/运行按钮
pub fn success_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(
        RichText::new(text)
            .color(Color32::WHITE)
    )
    .fill(dark_colors::SUCCESS)
    .min_size(Vec2::new(64.0, spacing::BUTTON_HEIGHT))
}

/// 绘制空状态组件
pub fn empty_state(
    ui: &mut egui::Ui,
    theme: Theme,
    icon: &str,
    title: &str,
    description: &str,
    action_text: Option<&str>,
    action: Option<&mut dyn FnMut()>,
) {
    let colors = ThemeColors::from_theme(theme);
    egui::Frame::group(ui.style())
        .inner_margin(24.0)
        .outer_margin(0.0)
        .corner_radius(8.0)
        .fill(colors.bg_secondary)
        .stroke(Stroke::new(1.0, colors.border))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.0);
                ui.label(RichText::new(icon).size(48.0).color(colors.text_muted));
                ui.add_space(12.0);
                ui.label(RichText::new(title).size(16.0).strong().color(colors.text_primary));
                ui.add_space(8.0);
                ui.label(RichText::new(description).color(colors.text_secondary));

                if let (Some(text), Some(callback)) = (action_text, action) {
                    ui.add_space(16.0);
                    let btn = primary_button(text, theme);
                    if ui.add(btn).clicked() {
                        callback();
                    }
                }

                ui.add_space(8.0);
            });
        });
}

/// 绘制统计卡片
pub fn stat_card(ui: &mut egui::Ui, _theme: Theme, label: &str, value: &str, icon: &str, color: Color32) {
    egui::Frame::group(ui.style())
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(6.0)
        .fill(color)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(icon).size(20.0));
                ui.vertical(|ui| {
                    ui.label(RichText::new(value).size(18.0).strong());
                    ui.label(RichText::new(label).small().weak());
                });
            });
        });
}

/// 绘制分隔标题
pub fn section_header(ui: &mut egui::Ui, theme: Theme, icon: &str, text: &str, count: Option<usize>) {
    let colors = ThemeColors::from_theme(theme);
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("{} {}", icon, text))
                .size(16.0)
                .strong()
                .color(colors.text_primary)
        );

        if let Some(n) = count {
            ui.add_space(8.0);
            ui.label(
                RichText::new(format!("({})", n))
                    .small()
                    .color(colors.text_secondary)
            );
        }
    });
}

/// 绘制面板头部
pub fn panel_header(ui: &mut egui::Ui, theme: Theme, title: &str, description: &str) {
    let colors = ThemeColors::from_theme(theme);
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading(
                RichText::new(title)
                    .size(20.0)
                    .strong()
                    .color(colors.text_primary)
            );
            ui.label(
                RichText::new(description)
                    .small()
                    .color(colors.text_secondary)
            );
        });
    });
}

/// 截断长文本并添加悬停提示
pub fn truncate_text(text: &str, max_len: usize) -> (String, bool) {
    if text.len() > max_len {
        let truncated = format!("...{}", &text[text.len().saturating_sub(max_len - 3)..]);
        (truncated, true)
    } else {
        (text.to_string(), false)
    }
}

/// 绘制带悬停提示的路径显示
pub fn path_label(ui: &mut egui::Ui, theme: Theme, path: &str, max_len: usize) {
    let colors = ThemeColors::from_theme(theme);
    let (display_path, is_truncated) = truncate_text(path, max_len);

    let label = ui.label(
        RichText::new(format!("📂 {}", display_path))
            .small()
            .color(colors.text_secondary)
            .code()
    );

    if is_truncated {
        label.on_hover_text(path);
    }
}
