mod utility;

use eframe::{NativeOptions, egui};
use utility::app::MyApp;

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // Prevent the window from becoming too small to use.
            .with_min_inner_size([600.0, 400.0])
            .with_max_inner_size([1200.0, 900.0])
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "QrGen",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
