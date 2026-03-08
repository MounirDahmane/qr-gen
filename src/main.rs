use egui::CentralPanel;
use image::Luma;
use qrcode::QrCode;
use eframe::{NativeOptions, egui};

fn generate(data: &str) {
    // Encode some data into bits.
    let code = QrCode::new(data.as_bytes()).unwrap();

    // Render the bits into an image.
    let img = code.render::<Luma<u8>>().build();

    // Save the image.
    img.save("./qrcode.png").unwrap();
}


struct MyApp {
    name: &'static str,
    age: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { name:"mounir", age: 18 }
    }
}

impl eframe::App for MyApp {
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("text");
        });
    }    
}

fn main() -> eframe::Result {
    //    generate("https://github.com/MounirDahmane");
    let app = MyApp::default();
    let options = NativeOptions::default();

    eframe::run_native("QrGen", options, Box::new(|cc| {

            Ok(Box::<MyApp>::default())
        }))
}

