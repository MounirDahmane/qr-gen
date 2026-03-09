use std::borrow::Cow;

use arboard::{Clipboard, ImageData};
use eframe::{
    NativeOptions,
    egui::{self, CentralPanel, Color32, SidePanel, TopBottomPanel, Vec2},
};
use qrcode::QrCode;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone)]
enum SaveFormat {
    PNG,
    SVG,
}

#[derive(Debug, PartialEq)]
enum ModuleShape {
    SQUARE,
    ROUNDED,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct HistoryEntry {
    text: String,
    ec_level: String,
}

fn save_history(history: &Vec<HistoryEntry>) {
    let path = dirs::data_dir()
        .unwrap_or_default()
        .join("qrgen_history.json");
    let json = serde_json::to_string(history).unwrap();
    std::fs::write(path, json).unwrap();
}

fn load_history() -> Vec<HistoryEntry> {
    let path = dirs::data_dir()
        .unwrap_or_default()
        .join("qrgen_history.json");
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

struct MyApp {
    text: String,
    code: QrCode,
    ec_level: qrcode::EcLevel,
    qr_scale: f32,
    fmt: SaveFormat,
    module_shape: ModuleShape,
    dark_mode: bool,
    history: Vec<HistoryEntry>,
    fg_color: Color32,
    bg_color: Color32,
    export_size: u32,
    select: bool,
    clipboard: Clipboard,
    img: image::RgbaImage,
}
impl MyApp {
    pub fn render_qr_image(&self) -> image::RgbaImage {
        let matrix = self.code.to_colors();
        let qr_width = self.code.width();
        let cell_size = self.export_size / qr_width as u32;
        let actual_size = cell_size * qr_width as u32;
        let mut img = image::RgbaImage::new(actual_size, actual_size);

        for (i, color) in matrix.iter().enumerate() {
            let col = (i % qr_width) as u32;
            let row = (i / qr_width) as u32;
            let fill = if *color == qrcode::Color::Dark {
                image::Rgba([
                    self.fg_color.r(),
                    self.fg_color.g(),
                    self.fg_color.b(),
                    self.fg_color.a(),
                ])
            } else {
                image::Rgba([
                    self.bg_color.r(),
                    self.bg_color.g(),
                    self.bg_color.b(),
                    self.bg_color.a(),
                ])
            };
            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    img.put_pixel(col * cell_size + dx, row * cell_size + dy, fill);
                }
            }
        }
        img
    }

    pub fn save_img(&self) {
        let extension = match self.fmt {
            SaveFormat::PNG => "png",
            SaveFormat::SVG => "svg",
        };
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(extension, &[extension])
            .set_file_name(format!("qrcode.{}", extension))
            .save_file()
        {
            match self.fmt {
                SaveFormat::PNG => {
                    let img = self.render_qr_image();
                    img.save(path).unwrap();
                }
                SaveFormat::SVG => {
                    let svg = self.code.render::<qrcode::render::svg::Color>().build();
                    std::fs::write(path, svg).unwrap();
                }
            }
        }
    }
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
            img: image::RgbaImage::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
        ctx.input(|i| {
            if i.key_pressed(egui::Key::S) && i.modifiers.ctrl {
                self.save_img();
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
            .min_width(160.0)
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

                let mut restore: Option<HistoryEntry> = None;
                let mut delete: Option<usize> = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, entry) in self.history.iter().enumerate() {
                        ui.horizontal(|ui| {
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
                    }
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
            .min_width(200.0)
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

                    // Input
                    ui.label(egui::RichText::new("TEXT / URL").strong());
                    ui.text_edit_singleline(&mut self.text);
                    ui.add_space(6.0);

                    // Error correction
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
                    // Size
                    ui.label(egui::RichText::new("Size").strong());
                    ui.add(egui::Slider::new(&mut self.qr_scale, 0.3..=1.0).text("Scale"));
                    ui.add_space(6.0);

                    // Colors
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

                    // Export

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
                            if self.select {
                                ui.label(egui::RichText::new("PNG Resolution").strong());
                                egui::ComboBox::from_id_salt("PNG Resolution")
                                    .selected_text(format!("{:?}", self.export_size))
                                    .show_ui(ui, |ui| {
                                        ui.vertical(|ui| {
                                            ui.selectable_value(
                                                &mut self.export_size,
                                                256,
                                                "256px",
                                            );
                                            ui.selectable_value(
                                                &mut self.export_size,
                                                512,
                                                "512px",
                                            );
                                            ui.selectable_value(
                                                &mut self.export_size,
                                                1024,
                                                "1024px",
                                            );
                                            ui.selectable_value(
                                                &mut self.export_size,
                                                2048,
                                                "2048px",
                                            );
                                        });
                                    });
                            }
                        });
                    });

                    ui.add_space(6.0);

                    ui.scope(|ui| {
                        ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 80);
                        ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(137, 180, 250);
                        ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(100, 140, 210);
                        ui.visuals_mut().widgets.hovered.fg_stroke.color = Color32::BLACK;
                        ui.visuals_mut().widgets.inactive.fg_stroke.color = match self.dark_mode {
                            true => Color32::YELLOW,
                            _ => Color32::DARK_BLUE,
                        };

                        let save_btn = egui::Button::new(egui::RichText::new("💾 Save").size(14.0))
                            .min_size(Vec2::new(ui.available_width(), 28.0));

                        if ui.add(save_btn).clicked() {
                            self.save_img();
                        }
                    });

                    ui.add_space(6.0);
                    /////////////////////
                    ui.scope(|ui| {
                        ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 80);
                        ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(137, 180, 250);
                        ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(100, 140, 210);
                        ui.visuals_mut().widgets.hovered.fg_stroke.color = Color32::BLACK;
                        ui.visuals_mut().widgets.inactive.fg_stroke.color = match self.dark_mode {
                            true => Color32::YELLOW,
                            _ => Color32::DARK_BLUE,
                        };

                        let copy_btn = egui::Button::new(egui::RichText::new("📋 copy").size(14.0))
                            .min_size(Vec2::new(ui.available_width(), 28.0));

                        if ui.add(copy_btn).clicked() {
                            let img = self.render_qr_image();

                            let cpy_img = ImageData {
                                width: img.width() as usize,
                                height: img.height() as usize,
                                bytes: Cow::Owned(img.as_raw().clone()),
                            };

                            self.clipboard.set_image(cpy_img).unwrap();
                        }
                    });
                    
                    ui.add_space(6.0);
                    ////////////////////
                    // Generate button (adds to history)
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
                })
            });

        // ── CENTER PANEL: QR PREVIEW ──────────────────────────────────────────
        CentralPanel::default()
            .frame(egui::Frame::default().fill(if self.dark_mode {
                Color32::from_rgb(30, 30, 46)
            } else {
                Color32::from_rgb(245, 245, 255)
            }))
            .show(ctx, |ui| {
                self.code = QrCode::with_error_correction_level(&self.text, self.ec_level).unwrap();
                let matrix = self.code.to_colors();

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
                    let rounding = match self.module_shape {
                        ModuleShape::SQUARE => 0.0,
                        ModuleShape::ROUNDED => cell_size * 0.35,
                    };
                    ui.painter().rect_filled(rect, rounding, fill);
                }
            });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        save_history(&self.history);
    }
}

fn main() -> eframe::Result {
    let options = NativeOptions::default();
    eframe::run_native(
        "QrGen",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
