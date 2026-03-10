use arboard::{Clipboard, ImageData};
use eframe::egui::{self, CentralPanel, Color32, SidePanel, TopBottomPanel, Vec2};
use qrcode::QrCode;
use std::borrow::Cow;

use super::history::{HistoryEntry, load_history, save_history};
use super::qr::{render_qr_image, save_img};
use super::types::{ModuleShape, SaveFormat};

/// Central application state. Holds all user settings, QR code state,
/// history, and UI flags. Persisted partially via `save()` on app close.
pub struct MyApp {
    /// Current text or URL to encode into the QR code.
    pub text: String,
    /// The generated QR code, regenerated every frame from `text` and `ec_level`.
    pub code: QrCode,
    /// Error correction level — controls how much damage the QR can sustain and still scan.
    pub ec_level: qrcode::EcLevel,
    /// Scale factor (0.3–1.0) controlling how much of the central panel the QR fills.
    pub qr_scale: f32,
    /// Selected export format: PNG or SVG.
    pub fmt: SaveFormat,
    /// Visual style of QR modules: square corners or rounded.
    pub module_shape: ModuleShape,
    pub dark_mode: bool,
    /// History entries loaded from disk on startup and saved on close.
    pub history: Vec<HistoryEntry>,
    /// Foreground color — applied to dark QR modules.
    pub fg_color: Color32,
    /// Background color — applied to light QR modules.
    pub bg_color: Color32,
    /// PNG export resolution in pixels (width = height, e.g. 512 → 512×512).
    pub export_size: u32,
    /// Tracks whether PNG is selected (true) or SVG (false), to show the resolution picker.
    pub select: bool,
    pub clipboard: Clipboard,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            code: QrCode::new("").unwrap(),
            text: String::new(),
            ec_level: qrcode::EcLevel::M,
            qr_scale: 0.8,
            fmt: SaveFormat::PNG,
            module_shape: ModuleShape::ROUNDED,
            dark_mode: true,
            history: load_history(),
            fg_color: Color32::BLACK,
            bg_color: Color32::WHITE,
            export_size: 512,
            select: true,
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // Ctrl+S triggers the save dialog without needing to click the button.
        ctx.input(|i| {
            if i.key_pressed(egui::Key::S) && i.modifiers.ctrl {
                save_img(
                    &self.code,
                    &self.fmt,
                    self.export_size,
                    self.fg_color,
                    self.bg_color,
                );
            }
        });

        // ── TOP BAR ──────────────────────────────────────────────────────────
        TopBottomPanel::top("top_panel")
            .frame(
                egui::Frame::default()
                    .fill(if self.dark_mode {
                        Color32::from_rgb(20, 20, 35)
                    } else {
                        Color32::from_rgb(220, 230, 255)
                    })
                    .inner_margin(egui::Margin::symmetric(20, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⬛ QrGen").size(20.0).strong().color(
                        if self.dark_mode {
                            Color32::WHITE
                        } else {
                            Color32::BLACK
                        },
                    ));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let theme_label = if self.dark_mode {
                            "☀ Light"
                        } else {
                            "🌙 Dark"
                        };
                        if ui.button(theme_label).clicked() {
                            self.dark_mode = !self.dark_mode;
                        }
                    });
                });
            });

        // ── LEFT SIDEBAR: HISTORY ─────────────────────────────────────────────
        SidePanel::left("history_panel")
            .min_width(200.0)
            .max_width(260.0)
            .frame(
                egui::Frame::default()
                    .fill(if self.dark_mode {
                        Color32::from_rgb(25, 25, 40)
                    } else {
                        Color32::from_rgb(230, 235, 255)
                    })
                    .inner_margin(egui::Margin::same(8)),
            )
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("History").strong().size(15.0));
                ui.separator();

                if self.history.is_empty() {
                    ui.label(
                        egui::RichText::new("No history yet.")
                            .italics()
                            .color(Color32::GRAY),
                    );
                }

                // Deferred actions — we can't mutate self.history while iterating it,
                // so we collect the intended action and apply it after the loop.
                let mut restore: Option<HistoryEntry> = None;
                let mut delete: Option<usize> = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, entry) in self.history.iter().enumerate() {
                        ui.horizontal(|ui| {
                            // Truncate long labels to prevent sidebar overflow.
                            let label = if entry.text.len() > 16 {
                                format!("{}…", &entry.text[..16])
                            } else {
                                entry.text.clone()
                            };
                            if ui.button(&label).on_hover_text(&entry.text).clicked() {
                                restore = Some(entry.clone());
                            }
                            if ui.small_button("✕").clicked() {
                                delete = Some(i);
                            }
                        });
                    }
                });

                if let Some(entry) = restore {
                    self.text = entry.text.clone();
                    self.ec_level = match entry.ec_level.as_str() {
                        "L" => qrcode::EcLevel::L,
                        "M" => qrcode::EcLevel::M,
                        "Q" => qrcode::EcLevel::Q,
                        _ => qrcode::EcLevel::H,
                    };
                }
                if let Some(i) = delete {
                    self.history.remove(i);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    if ui.small_button("Clear all").clicked() {
                        self.history.clear();
                    }
                });
            });

        // ── RIGHT PANEL: SETTINGS ─────────────────────────────────────────────
        SidePanel::right("settings_panel")
            .min_width(240.0)
            .max_width(300.0)
            .frame(
                egui::Frame::default()
                    .fill(if self.dark_mode {
                        Color32::from_rgb(25, 25, 40)
                    } else {
                        Color32::from_rgb(230, 235, 255)
                    })
                    .inner_margin(egui::Margin::same(10)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(egui::RichText::new("Settings").strong().size(15.0));
                    ui.separator();

                    ui.label(egui::RichText::new("TEXT / URL").strong());
                    ui.text_edit_singleline(&mut self.text);
                    ui.add_space(6.0);

                    ui.label(egui::RichText::new("Error Correction").strong());
                    egui::ComboBox::from_id_salt("ec_combo")
                        .selected_text(format!("{:?}", self.ec_level))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.ec_level,
                                qrcode::EcLevel::L,
                                "L — Low (7%)",
                            );
                            ui.selectable_value(
                                &mut self.ec_level,
                                qrcode::EcLevel::M,
                                "M — Medium (15%)",
                            );
                            ui.selectable_value(
                                &mut self.ec_level,
                                qrcode::EcLevel::Q,
                                "Q — Quartile (25%)",
                            );
                            ui.selectable_value(
                                &mut self.ec_level,
                                qrcode::EcLevel::H,
                                "H — High (30%)",
                            );
                        });
                    ui.add_space(6.0);

                    ui.label(egui::RichText::new("Module Shape").strong());
                    egui::ComboBox::from_id_salt("ModuleShape_combo")
                        .selected_text(format!("{:?}", self.module_shape))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.module_shape,
                                ModuleShape::SQUARE,
                                "Squared",
                            );
                            ui.selectable_value(
                                &mut self.module_shape,
                                ModuleShape::ROUNDED,
                                "Rounded",
                            );
                        });
                    ui.add_space(6.0);

                    ui.label(egui::RichText::new("Size").strong());
                    ui.add(egui::Slider::new(&mut self.qr_scale, 0.3..=1.0).text("Scale"));
                    ui.add_space(6.0);

                    ui.label(egui::RichText::new("Colors").strong());
                    ui.horizontal(|ui| {
                        ui.label("Foreground:");
                        ui.color_edit_button_srgba(&mut self.fg_color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Background:");
                        ui.color_edit_button_srgba(&mut self.bg_color);
                    });
                    ui.add_space(6.0);
                    ui.separator();

                    ui.label(egui::RichText::new("Export").strong());
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_value(&mut self.fmt, SaveFormat::SVG, "SVG")
                            .clicked()
                        {
                            self.select = false;
                        }
                        ui.vertical(|ui| {
                            if ui
                                .selectable_value(&mut self.fmt, SaveFormat::PNG, "PNG")
                                .clicked()
                            {
                                self.select = true;
                            }
                            // Resolution picker is only relevant for PNG exports.
                            if self.select {
                                ui.label(egui::RichText::new("PNG Resolution").strong());
                                egui::ComboBox::from_id_salt("PNG Resolution")
                                    .selected_text(format!("{:?}", self.export_size))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.export_size, 256, "256px");
                                        ui.selectable_value(&mut self.export_size, 512, "512px");
                                        ui.selectable_value(&mut self.export_size, 1024, "1024px");
                                        ui.selectable_value(&mut self.export_size, 2048, "2048px");
                                    });
                            }
                        });
                    });
                    ui.add_space(6.0);

                    // Reusable button style — applied inside ui.scope() to avoid
                    // bleeding styles into surrounding widgets.
                    let style_btn = |ui: &mut egui::Ui, dark: bool| {
                        ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 80);
                        ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(137, 180, 250);
                        ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(100, 140, 210);
                        ui.visuals_mut().widgets.hovered.fg_stroke.color = Color32::BLACK;
                        ui.visuals_mut().widgets.inactive.fg_stroke.color = if dark {
                            Color32::YELLOW
                        } else {
                            Color32::DARK_BLUE
                        };
                    };

                    ui.scope(|ui| {
                        style_btn(ui, self.dark_mode);
                        let save_btn = egui::Button::new(egui::RichText::new("💾 Save").size(14.0))
                            .min_size(Vec2::new(ui.available_width(), 28.0));
                        if ui.add(save_btn).clicked() {
                            save_img(
                                &self.code,
                                &self.fmt,
                                self.export_size,
                                self.fg_color,
                                self.bg_color,
                            );
                        }
                    });

                    ui.add_space(6.0);

                    ui.scope(|ui| {
                        style_btn(ui, self.dark_mode);
                        let copy_btn = egui::Button::new(egui::RichText::new("📋 Copy").size(14.0))
                            .min_size(Vec2::new(ui.available_width(), 28.0));
                        if ui.add(copy_btn).clicked() {
                            let img = render_qr_image(
                                &self.code,
                                self.export_size,
                                self.fg_color,
                                self.bg_color,
                            );
                            // arboard expects raw RGBA bytes — RgbaImage::as_raw() provides exactly that.
                            let cpy_img = ImageData {
                                width: img.width() as usize,
                                height: img.height() as usize,
                                bytes: Cow::Owned(img.as_raw().clone()),
                            };
                            self.clipboard.set_image(cpy_img).unwrap();
                        }
                    });

                    ui.add_space(6.0);

                    if ui.button("⚡ Generate & Save to History").clicked() {
                        if !self.text.is_empty() {
                            let level = match self.ec_level {
                                qrcode::EcLevel::L => "L",
                                qrcode::EcLevel::M => "M",
                                qrcode::EcLevel::Q => "Q",
                                qrcode::EcLevel::H => "H",
                            };
                            self.history.push(HistoryEntry {
                                text: self.text.clone(),
                                ec_level: level.to_string(),
                            });
                        }
                    }
                });
            });

        // ── CENTER PANEL: QR PREVIEW ──────────────────────────────────────────
        CentralPanel::default()
            .frame(egui::Frame::default().fill(if self.dark_mode {
                Color32::from_rgb(30, 30, 46)
            } else {
                Color32::from_rgb(245, 245, 255)
            }))
            .show(ctx, |ui| {
                // Regenerate the QR every frame — cheap enough and keeps preview in sync.
                self.code = QrCode::with_error_correction_level(&self.text, self.ec_level).unwrap();
                let matrix = self.code.to_colors();

                // Scale the QR to fit the panel while respecting qr_scale,
                // then center it by offsetting by half the remaining space.
                let panel_size = ui.available_size().x.min(ui.available_size().y) * self.qr_scale;
                let cell_size = panel_size / self.code.width() as f32;
                let qr_pixel_size = cell_size * self.code.width() as f32;
                let available = ui.available_size();
                let offset = (available - Vec2::new(qr_pixel_size, qr_pixel_size)) / 2.0;
                let pos = ui.min_rect().min + offset;

                for (i, color) in matrix.iter().enumerate() {
                    let x = (i % self.code.width()) as f32 * cell_size;
                    let y = (i / self.code.width()) as f32 * cell_size;
                    let rect = egui::Rect::from_min_size(
                        pos + egui::vec2(x, y),
                        egui::vec2(cell_size, cell_size),
                    );
                    let fill = if *color == qrcode::Color::Dark {
                        self.fg_color
                    } else {
                        self.bg_color
                    };
                    // Rounding of 0.0 = sharp squares; 0.45 * cell_size ≈ near-circular dots.
                    let rounding = match self.module_shape {
                        ModuleShape::SQUARE => 0.0,
                        ModuleShape::ROUNDED => cell_size * 0.45,
                    };
                    ui.painter().rect_filled(rect, rounding, fill);
                }
            });
    }

    /// Called by eframe on app close — persists history to disk.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        save_history(&self.history);
    }
}
