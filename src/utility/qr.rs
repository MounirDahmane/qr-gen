use super::types::SaveFormat;
use eframe::egui::Color32;
use qrcode::QrCode;

pub fn render_qr_image(
    code: &QrCode,
    export_size: u32,
    fg_color: Color32,
    bg_color: Color32,
) -> image::RgbaImage {
    let matrix = code.to_colors();
    let qr_width = code.width();
    let cell_size = export_size / qr_width as u32;
    let actual_size = cell_size * qr_width as u32;
    let mut img = image::RgbaImage::new(actual_size, actual_size);

    for (i, color) in matrix.iter().enumerate() {
        let col = (i % qr_width) as u32;
        let row = (i / qr_width) as u32;
        let fill = if *color == qrcode::Color::Dark {
            image::Rgba([fg_color.r(), fg_color.g(), fg_color.b(), fg_color.a()])
        } else {
            image::Rgba([bg_color.r(), bg_color.g(), bg_color.b(), bg_color.a()])
        };
        for dy in 0..cell_size {
            for dx in 0..cell_size {
                img.put_pixel(col * cell_size + dx, row * cell_size + dy, fill);
            }
        }
    }
    img
}

pub fn save_img(
    code: &QrCode,
    fmt: &SaveFormat,
    export_size: u32,
    fg_color: Color32,
    bg_color: Color32,
) {
    let extension = match fmt {
        SaveFormat::PNG => "png",
        SaveFormat::SVG => "svg",
    };
    if let Some(path) = rfd::FileDialog::new()
        .add_filter(extension, &[extension])
        .set_file_name(format!("qrcode.{}", extension))
        .save_file()
    {
        match fmt {
            SaveFormat::PNG => {
                let img = render_qr_image(code, export_size, fg_color, bg_color);
                img.save(path).unwrap();
            }
            SaveFormat::SVG => {
                let svg = code.render::<qrcode::render::svg::Color>().build();
                std::fs::write(path, svg).unwrap();
            }
        }
    }
}
