use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Boiler",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Boiler Panel");
                ui.add_space(10.0);

                if ui.button("Start").clicked() {
                    println!("Started");
                }
                if ui.button("Stop").clicked() {
                    println!("Stopped");
                }
                if ui.button("Reset").clicked() {
                    println!("Reset");
                }
                if ui.button("Settings").clicked() {
                    println!("Settings");
                }
            });
        });
    }
}
