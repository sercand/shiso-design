//! Shiso design system demo for egui. Run with `cargo run -p shiso-egui-demo`.
//! Mirrors apps/html so the two can be compared side by side.
//!
//! For visual checks without clicking around:
//! `SHISO_SCREENSHOT=out.png SHISO_THEME=dark SHISO_DENSITY=comfortable cargo run -p shiso-egui-demo`
//! renders a few frames, writes the PNG and exits.

use eframe::egui::{self, Align, Layout, RichText, ThemePreference};
use shiso_theme::egui::{self as shiso, pill, text, Tone};
use shiso_theme::tokens::{self, Density};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1180.0, 820.0]).with_title("Shiso Design: egui demo"),
        ..Default::default()
    };
    eframe::run_native(
        "shiso-egui-demo",
        options,
        Box::new(|cc| {
            let mut demo = Demo::default();
            if std::env::var("SHISO_DENSITY").as_deref() == Ok("comfortable") {
                demo.density = tokens::COMFORTABLE;
            }
            shiso::install(&cc.egui_ctx, demo.density);
            match std::env::var("SHISO_THEME").as_deref() {
                Ok("dark") => cc.egui_ctx.set_theme(ThemePreference::Dark),
                Ok("light") => cc.egui_ctx.set_theme(ThemePreference::Light),
                _ => {}
            }
            demo.screenshot = std::env::var_os("SHISO_SCREENSHOT").map(Into::into);
            Ok(Box::new(demo))
        }),
    )
}

struct Demo {
    density: Density,
    nav: usize,
    tab: usize,
    search: String,
    budget: String,
    tier: usize,
    review: bool,
    notify: bool,
    note: String,
    screenshot: Option<std::path::PathBuf>,
    frames: u32,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            density: tokens::COMPACT,
            nav: 1,
            tab: 0,
            search: String::new(),
            budget: "$60.00".into(),
            tier: 1,
            review: true,
            notify: true,
            note: String::new(),
            screenshot: None,
            frames: 0,
        }
    }
}

const NAV: [&str; 6] = ["Home", "Approvals", "Tickets", "Agents", "Changes", "Drive"];
const TIERS: [&str; 3] = ["Local only", "Local and mid", "Any model"];
const TICKETS: [(&str, &str, &str, &str, Tone, &str); 4] = [
    ("OPS-142", "Reconcile October provider invoices", "Mira", "In review", Tone::Warn, "$3.84"),
    ("ENG-88", "Fix crash on save in the level editor", "Kai", "Coding", Tone::Soft, "$11.02"),
    ("LEG-12", "Draft NDA for Northwind Audio", "Lexi", "Validator failed", Tone::Bad, "$0.61"),
    ("OPS-139", "Roster for week 47", "Deniz", "Done", Tone::Ok, "$0.00"),
];

impl eframe::App for Demo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let d = self.density;
        self.capture(ui.ctx());
        egui::Panel::left("side").resizable(false).exact_size(220.0).show(ui, |ui| self.sidebar(ui));
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = d.gap;
                ui.horizontal(|ui| {
                    ui.label(text::h1("Waiting on you"));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let _ = shiso::primary_button(ui, "New ticket");
                        ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search").desired_width(200.0));
                    });
                });
                ui.columns(3, |cols| {
                    self.change_request(&mut cols[0]);
                    self.agent(&mut cols[1]);
                    self.policy(&mut cols[2]);
                });
                self.tickets(ui);
                self.type_and_color(ui);
            });
        });
    }
}

impl Demo {
    /// Screenshot mode: ask for a frame once layout has settled, save it, close.
    fn capture(&mut self, ctx: &egui::Context) {
        let Some(path) = self.screenshot.clone() else { return };
        self.frames += 1;
        if self.frames == 5 {
            self.screenshot = None;
            let ctx2 = ctx.clone();
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::ScreenshotCallback::new(move |image| {
                let [w, h] = image.size;
                if let Err(err) = image::save_buffer(&path, image.as_raw(), w as u32, h as u32, image::ColorType::Rgba8) {
                    eprintln!("could not write {}: {err}", path.display());
                }
                ctx2.send_viewport_cmd(egui::ViewportCommand::Close);
            })));
        }
        ctx.request_repaint();
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(tokens::SPACE_2);
        ui.label(text::h3("Kumo Games"));
        ui.add_space(tokens::SPACE_2);
        ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
            for (i, name) in NAV.iter().enumerate() {
                let label = if i == 1 { format!("{name}   3") } else { (*name).to_owned() };
                if ui.add(egui::Button::selectable(self.nav == i, label)).clicked() {
                    self.nav = i;
                }
            }
        });
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            ui.add_space(tokens::SPACE_2);
            ui.horizontal(|ui| {
                for (label, value) in [("Compact", tokens::COMPACT), ("Comfortable", tokens::COMFORTABLE)] {
                    if ui.selectable_label(self.density == value, label).clicked() {
                        self.density = value;
                        shiso::set_density(ui.ctx(), value);
                    }
                }
            });
            ui.horizontal(|ui| {
                let mut pref = ui.ctx().options(|o| o.theme_preference);
                let before = pref;
                ui.selectable_value(&mut pref, ThemePreference::System, "System");
                ui.selectable_value(&mut pref, ThemePreference::Light, "Light");
                ui.selectable_value(&mut pref, ThemePreference::Dark, "Dark");
                if pref != before {
                    ui.ctx().set_theme(pref);
                }
            });
        });
    }

    fn change_request(&mut self, ui: &mut egui::Ui) {
        let d = self.density;
        shiso::card(ui, d, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(pill("In review", Tone::Warn));
                ui.add(pill("2 of 3 parts approved", Tone::Neutral));
            });
            ui.label(text::h3("Raise contractor day rates for 2027"));
            ui.label(RichText::new("Proposed by Mira, finance agent. Applies when approved.").weak());
            shiso::inset(ui, |ui| {
                for (field, before, after) in
                    [("Senior artist", "₺9.500", "₺10.400"), ("QA tester", "₺5.200", "₺5.600"), ("Effective from", "", "1 Jan 2027")]
                {
                    ui.horizontal(|ui| {
                        ui.label(field);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(text::strong(after, d.text));
                            if !before.is_empty() {
                                ui.label(RichText::new(before).weak().strikethrough());
                            }
                        });
                    });
                }
            });
            ui.horizontal_wrapped(|ui| {
                let _ = shiso::primary_button(ui, "Approve");
                let _ = ui.button("Request changes");
                let _ = shiso::ghost_button(ui, "Open");
            });
        });
    }

    fn agent(&mut self, ui: &mut egui::Ui) {
        let d = self.density;
        shiso::card(ui, d, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(text::h3("Mira"));
                    ui.label(RichText::new("Finance agent, owned by Deniz").weak());
                });
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| ui.add(pill("Running", Tone::Live)));
            });
            ui.horizontal(|ui| {
                for (i, name) in ["Overview", "Spend", "Policy"].iter().enumerate() {
                    ui.selectable_value(&mut self.tab, i, *name);
                }
            });
            kv(ui, "Budget this month", "$41.20 of $60.00", d.text, true);
            shiso::meter(ui, 41.20 / 60.0);
            kv(ui, "Model tier", "Local and mid", d.text, false);
            kv(ui, "Tickets accepted", "18", d.text, false);
            kv(ui, "Probation ends", "26 Nov", d.text, false);
        });
    }

    fn policy(&mut self, ui: &mut egui::Ui) {
        let d = self.density;
        shiso::card(ui, d, |ui| {
            ui.label(text::h3("Agent policy"));
            ui.label(text::label("Monthly budget", d.text));
            ui.add(egui::TextEdit::singleline(&mut self.budget).desired_width(f32::INFINITY));
            ui.label(text::label("Model tier", d.text));
            egui::ComboBox::from_id_salt("tier").width(ui.available_width()).selected_text(TIERS[self.tier]).show_ui(ui, |ui| {
                for (i, t) in TIERS.iter().enumerate() {
                    ui.selectable_value(&mut self.tier, i, *t);
                }
            });
            ui.checkbox(&mut self.review, "Review before marking done");
            ui.checkbox(&mut self.notify, "Tell me at 75% of budget");
            ui.add(egui::TextEdit::multiline(&mut self.note).hint_text("Why this budget?").desired_rows(2).desired_width(f32::INFINITY));
            ui.horizontal_wrapped(|ui| {
                let _ = shiso::primary_button(ui, "Save changes");
                let _ = ui.button("Cancel");
            });
        });
    }

    fn tickets(&mut self, ui: &mut egui::Ui) {
        let d = self.density;
        shiso::card(ui, d, |ui| {
            ui.label(text::h3("Open tickets"));
            egui::Grid::new("tickets").num_columns(5).spacing([24.0, d.pad * 0.6]).min_col_width(70.0).show(ui, |ui| {
                for h in ["Key", "Ticket", "Assignee", "Status", "Cost"] {
                    ui.label(RichText::new(h).weak());
                }
                ui.end_row();
                for (key, title, who, status, tone, cost) in TICKETS {
                    ui.label(RichText::new(key).monospace().weak());
                    ui.label(text::label(title, d.text));
                    ui.label(who);
                    ui.add(pill(status, tone));
                    ui.label(cost);
                    ui.end_row();
                }
            });
        });
    }

    fn type_and_color(&mut self, ui: &mut egui::Ui) {
        let d = self.density;
        shiso::card(ui, d, |ui| {
            ui.label(text::h3("Type and colors"));
            ui.label(text::h1("Every agent has an owner"));
            ui.label(text::h2("Değişiklik isteği İstanbul şubesinde onay bekliyor"));
            ui.horizontal_wrapped(|ui| {
                ui.label("Body text at the base size.");
                ui.label(RichText::new("Secondary text sits one step back.").weak());
            });
            ui.label("0123456789 ₺10.400,00 $41.20 68.7%");
            ui.label(RichText::new("open_change_request{title} → cr_8f3a2c").monospace());
            let p = shiso::palette(ui);
            let swatches = [
                ("canvas", p.canvas), ("card", p.card), ("fill", p.fill), ("line", p.line), ("text", p.text), ("text-2", p.text_2),
                ("primary", p.primary), ("soft", p.soft), ("live", p.live), ("ok", p.ok), ("warn", p.warn), ("bad", p.bad),
            ];
            ui.horizontal(|ui| {
                for (name, rgb) in swatches {
                    ui.vertical(|ui| {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(58.0, 36.0), egui::Sense::hover());
                        ui.painter().rect(rect, tokens::RADIUS_SM as u8, shiso::color(rgb), egui::Stroke::new(1.0, shiso::color(p.line)), egui::StrokeKind::Inside);
                        ui.label(RichText::new(name).small().weak());
                    });
                }
            });
        });
    }
}

fn kv(ui: &mut egui::Ui, key: &str, value: &str, size: f32, strong: bool) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(key).weak());
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if strong {
                ui.label(text::strong(value, size));
            } else {
                ui.label(value);
            }
        });
    });
}
