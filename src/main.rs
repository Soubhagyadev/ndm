// Importing Library
use eframe::egui;

// Restructuring.
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
