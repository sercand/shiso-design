//! The Shiso look for egui.
//!
//! ```no_run
//! # let ctx = egui::Context::default();
//! shiso_theme::egui::install(&ctx, shiso_theme::tokens::COMPACT);
//! ```
//!
//! After `install`, plain egui widgets (buttons, text edits, checkboxes, combo boxes) already look right
//! in both light and dark. Use the helpers here for what egui lacks.

use crate::tokens::{self, Density, Palette};
use egui::epaint::text::{FontInsert, FontPriority, InsertFontFamily};
use egui::{
    Color32, Context, CornerRadius, FontData, FontFamily, FontId, Frame, Margin, Response, Sense, Shadow, Stroke,
    Style, TextStyle, Theme, Ui, Vec2, Widget,
};

const HANKEN_REGULAR: &[u8] = include_bytes!("../../../fonts/HankenGrotesk-Regular.ttf");
const HANKEN_MEDIUM: &[u8] = include_bytes!("../../../fonts/HankenGrotesk-Medium.ttf");
const HANKEN_SEMIBOLD: &[u8] = include_bytes!("../../../fonts/HankenGrotesk-SemiBold.ttf");
const HANKEN_BOLD: &[u8] = include_bytes!("../../../fonts/HankenGrotesk-Bold.ttf");
const JETBRAINS_REGULAR: &[u8] = include_bytes!("../../../fonts/JetBrainsMono-Regular.ttf");

/// egui has no font weights, so each weight is its own family.
pub fn medium() -> FontFamily {
    FontFamily::Name("shiso-medium".into())
}
pub fn semibold() -> FontFamily {
    FontFamily::Name("shiso-semibold".into())
}
pub fn bold() -> FontFamily {
    FontFamily::Name("shiso-bold".into())
}

pub fn color(rgb: [u8; 3]) -> Color32 {
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}

/// The palette matching the ui's current light or dark mode.
pub fn palette(ui: &Ui) -> &'static Palette {
    if ui.visuals().dark_mode {
        &tokens::DARK
    } else {
        &tokens::LIGHT
    }
}

/// Install fonts and both themes. Call once at startup; egui then follows the system theme,
/// or call `ctx.set_theme(..)` to pin one.
pub fn install(ctx: &Context, density: Density) {
    install_fonts(ctx);
    set_density(ctx, density);
}

/// Re-apply the styles at another density (fonts stay).
pub fn set_density(ctx: &Context, density: Density) {
    ctx.set_style_of(Theme::Light, style(&tokens::LIGHT, false, density));
    ctx.set_style_of(Theme::Dark, style(&tokens::DARK, true, density));
}

fn install_fonts(ctx: &Context) {
    let add = |name: &str, data: &'static [u8], family: FontFamily| {
        ctx.add_font(FontInsert::new(
            name,
            FontData::from_static(data),
            vec![InsertFontFamily { family, priority: FontPriority::Highest }],
        ));
    };
    add("HankenGrotesk-Regular", HANKEN_REGULAR, FontFamily::Proportional);
    add("HankenGrotesk-Medium", HANKEN_MEDIUM, medium());
    add("HankenGrotesk-SemiBold", HANKEN_SEMIBOLD, semibold());
    add("HankenGrotesk-Bold", HANKEN_BOLD, bold());
    add("JetBrainsMono-Regular", JETBRAINS_REGULAR, FontFamily::Monospace);
}

fn style(p: &Palette, dark: bool, d: Density) -> Style {
    let mut s = Style::default();
    s.visuals = if dark { egui::Visuals::dark() } else { egui::Visuals::light() };
    let control = CornerRadius::same(tokens::RADIUS_CONTROL as u8);

    s.text_styles = [
        (TextStyle::Small, FontId::new(tokens::TEXT_SM, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(d.text, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(d.text, medium())),
        (TextStyle::Heading, FontId::new(tokens::TEXT_XL, semibold())),
        (TextStyle::Monospace, FontId::new(d.text - 1.0, FontFamily::Monospace)),
    ]
    .into();

    let sp = &mut s.spacing;
    sp.item_spacing = Vec2::new(tokens::SPACE_2, tokens::SPACE_2);
    sp.interact_size = Vec2::new(d.control, d.control);
    sp.button_padding = Vec2::new((d.control * 0.45).round(), ((d.control - d.text * 1.4) / 2.0).max(2.0).floor());
    sp.window_margin = Margin::same(d.pad as i8);
    sp.menu_margin = Margin::same(tokens::SPACE_2 as i8);
    sp.icon_width = 16.0;
    sp.icon_width_inner = 10.0;

    let v = &mut s.visuals;
    v.panel_fill = color(p.canvas);
    v.window_fill = color(p.card);
    v.extreme_bg_color = color(p.fill);
    v.text_edit_bg_color = Some(color(p.fill));
    v.faint_bg_color = color(p.fill);
    v.code_bg_color = color(p.fill);
    v.hyperlink_color = color(p.primary);
    v.warn_fg_color = color(p.warn);
    v.error_fg_color = color(p.bad);
    v.weak_text_color = Some(color(p.text_2));
    v.selection.bg_fill = color(p.soft);
    v.selection.stroke = Stroke::new(1.0, color(p.soft_text));
    v.window_corner_radius = CornerRadius::same(tokens::RADIUS_CARD as u8);
    v.menu_corner_radius = control;
    v.window_stroke = if dark { Stroke::new(1.0, color(p.line)) } else { Stroke::NONE };
    v.window_shadow = card_shadow(dark);
    v.popup_shadow = card_shadow(dark);

    let w = &mut v.widgets;
    for wv in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
        wv.corner_radius = control;
        wv.bg_stroke = Stroke::NONE;
        wv.fg_stroke = Stroke::new(1.0, color(p.text));
        wv.expansion = 0.0;
    }
    w.noninteractive.bg_fill = color(p.canvas);
    w.noninteractive.weak_bg_fill = color(p.canvas);
    w.noninteractive.bg_stroke = Stroke::new(1.0, color(p.line));
    w.inactive.bg_fill = color(p.fill);
    w.inactive.weak_bg_fill = color(p.fill);
    w.hovered.bg_fill = color(p.fill_hover);
    w.hovered.weak_bg_fill = color(p.fill_hover);
    w.active.bg_fill = color(p.fill_hover);
    w.active.weak_bg_fill = color(p.fill_hover);
    w.active.bg_stroke = Stroke::new(1.5, color(p.primary));
    w.open.bg_fill = color(p.fill);
    w.open.weak_bg_fill = color(p.fill);
    s
}

fn card_shadow(dark: bool) -> Shadow {
    if dark {
        Shadow::NONE
    } else {
        let t = tokens::LIGHT.text;
        Shadow { offset: [0, 6], blur: 20, spread: 0, color: Color32::from_rgba_unmultiplied(t[0], t[1], t[2], 22) }
    }
}

/// The frame of a card: the one lifted surface. Do not nest cards.
pub fn card_frame(ui: &Ui, density: Density) -> Frame {
    let p = palette(ui);
    let dark = ui.visuals().dark_mode;
    Frame::new()
        .fill(color(p.card))
        .corner_radius(CornerRadius::same(tokens::RADIUS_CARD as u8))
        .inner_margin(Margin::same(density.pad as i8))
        .shadow(card_shadow(dark))
        .stroke(if dark { Stroke::new(1.0, Color32::from_white_alpha(13)) } else { Stroke::NONE })
}

/// A card filling the available width.
pub fn card<R>(ui: &mut Ui, density: Density, add: impl FnOnce(&mut Ui) -> R) -> R {
    card_frame(ui, density)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = density.gap * 0.7;
            add(ui)
        })
        .inner
}

/// A filled area inside a card (diffs, code, grouped rows).
pub fn inset<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let p = palette(ui);
    Frame::new()
        .fill(color(p.fill))
        .corner_radius(CornerRadius::same(tokens::RADIUS_INNER as u8))
        .inner_margin(Margin::symmetric(tokens::SPACE_3 as i8, tokens::SPACE_2 as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add(ui)
        })
        .inner
}

/// The primary action. One per view.
pub fn primary_button(ui: &mut Ui, text: &str) -> Response {
    let p = palette(ui);
    ui.scope(|ui| {
        let w = &mut ui.visuals_mut().widgets;
        w.inactive.weak_bg_fill = color(p.primary);
        w.hovered.weak_bg_fill = color(p.primary_hover);
        w.active.weak_bg_fill = color(p.primary_hover);
        for wv in [&mut w.inactive, &mut w.hovered, &mut w.active] {
            wv.fg_stroke = Stroke::new(1.0, color(p.primary_text));
        }
        ui.button(text)
    })
    .inner
}

/// A button with no fill until hovered.
pub fn ghost_button(ui: &mut Ui, text: &str) -> Response {
    let p = palette(ui);
    ui.scope(|ui| {
        let w = &mut ui.visuals_mut().widgets;
        w.inactive.weak_bg_fill = Color32::TRANSPARENT;
        w.inactive.fg_stroke = Stroke::new(1.0, color(p.text_2));
        ui.button(text)
    })
    .inner
}

/// What a pill says about its subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Ok,
    Warn,
    Bad,
    Soft,
    /// An agent is working right now.
    Live,
}

/// A status pill: a dot and a word.
pub struct Pill<'a> {
    text: &'a str,
    tone: Tone,
    dot: bool,
}

pub fn pill(text: &str, tone: Tone) -> Pill<'_> {
    Pill { text, tone, dot: tone != Tone::Neutral }
}

impl Widget for Pill<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = palette(ui);
        let (bg, fg, dot) = match self.tone {
            Tone::Neutral => (p.fill, p.text_2, p.text_2),
            Tone::Ok => (p.ok_bg, p.ok, p.ok),
            Tone::Warn => (p.warn_bg, p.warn, p.warn),
            Tone::Bad => (p.bad_bg, p.bad, p.bad),
            Tone::Soft => (p.soft, p.soft_text, p.soft_text),
            Tone::Live => (p.fill, p.text, p.live),
        };
        let size = TextStyle::Body.resolve(ui.style()).size - 1.0;
        let galley = ui.painter().layout_no_wrap(self.text.to_owned(), FontId::new(size, medium()), color(fg));
        let dot_w = if self.dot { 12.0 } else { 0.0 };
        let desired = Vec2::new(galley.size().x + dot_w + 18.0, galley.size().y + 4.0);
        let (rect, response) = ui.allocate_exact_size(desired, Sense::hover());
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect_filled(rect, CornerRadius::same(255), color(bg));
            let mut x = rect.left() + 9.0;
            if self.dot {
                painter.circle_filled(egui::pos2(x + 3.0, rect.center().y), 3.0, color(dot));
                x += dot_w;
            }
            painter.galley(egui::pos2(x, rect.center().y - galley.size().y / 2.0), galley, color(fg));
        }
        response
    }
}

/// A progress meter. `fraction` is clamped to 0..=1.
pub fn meter(ui: &mut Ui, fraction: f32) -> Response {
    let p = palette(ui);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 8.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let r = CornerRadius::same(4);
        ui.painter().rect_filled(rect, r, color(p.fill));
        let mut fill = rect;
        fill.set_width(rect.width() * fraction.clamp(0.0, 1.0));
        ui.painter().rect_filled(fill, r, color(p.primary));
    }
    response
}

/// Text helpers so sizes and weights come from the tokens.
pub mod text {
    use super::{bold, medium, semibold, tokens};
    use egui::{FontId, RichText};

    pub fn h1(s: impl Into<String>) -> RichText {
        RichText::new(s).font(FontId::new(tokens::TEXT_2XL, semibold()))
    }
    pub fn h2(s: impl Into<String>) -> RichText {
        RichText::new(s).font(FontId::new(tokens::TEXT_XL, semibold()))
    }
    pub fn h3(s: impl Into<String>) -> RichText {
        RichText::new(s).font(FontId::new(tokens::TEXT_LG, semibold()))
    }
    /// Emphasis inside body text.
    pub fn strong(s: impl Into<String>, size: f32) -> RichText {
        RichText::new(s).font(FontId::new(size, bold()))
    }
    pub fn label(s: impl Into<String>, size: f32) -> RichText {
        RichText::new(s).font(FontId::new(size, medium()))
    }
}
