use eframe::{egui, NativeOptions};
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0,600.0)),
        ..eframe::NativeOptions::default()
    };

    // Create the main window
    eframe::run_native(
        "Main Viewport",
        native_options,
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default, Clone)]
struct MyApp {
    show_window1: Arc<AtomicBool>,
    show_window2: Arc<AtomicBool>,
    show_window3: Arc<AtomicBool>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Main Viewport");
            ui.label("Click a button to open a corresponding window:");

            if ui.button("Open Window 1").clicked() {
                if !self.show_window1.load(Ordering::Relaxed) {
                    self.show_window1.store(true, Ordering::Relaxed);
                }
            }

            if ui.button("Open Window 2").clicked() {
                if !self.show_window2.load(Ordering::Relaxed) {
                    self.show_window2.store(true, Ordering::Relaxed);
                }
            }

            if ui.button("Open Window 3").clicked() {
                if !self.show_window3.load(Ordering::Relaxed) {
                    self.show_window3.store(true, Ordering::Relaxed);
                }
            }
        });

        // Manage deferred viewports
        if self.show_window1.load(Ordering::Relaxed) {
            let show_window1 = self.show_window1.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId("Window 1".into()),
                egui::ViewportBuilder::default().with_title("Window1").with_inner_size(egui::vec2(600.0, 400.0)),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from deferred viewport");
                        if ctx.input(|i| i.viewport().close_requested()) {
                            show_window1.store(false, Ordering::Relaxed);
                        }
                    });

                },
            );
        }

        if self.show_window2.load(Ordering::Relaxed) {
            let show_window2 = self.show_window2.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId("Window 2".into()),
                egui::ViewportBuilder::default().with_title("Window 2").with_inner_size(egui::vec2(600.0, 400.0)),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from deferred viewport");
                        if ctx.input(|i| i.viewport().close_requested()) {
                            show_window2.store(false, Ordering::Relaxed);
                        }
                    });

                },
            );
        }

        if self.show_window3.load(Ordering::Relaxed) {
            let show_window3 = self.show_window3.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId("Window 3".into()),
                egui::ViewportBuilder::default().with_title("Window 3").with_inner_size(egui::vec2(600.0, 400.0)),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from deferred viewport");
                        ui.label("This is Window 1");
                        if ctx.input(|i| i.viewport().close_requested()) {
                            show_window3.store(false, Ordering::Relaxed);
                        }
                    });

                },
            );
        }
    }
}
