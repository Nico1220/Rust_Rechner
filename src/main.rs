 use eframe::{egui::CentralPanel, epi::App, run_nativ}
 use std:env

struct Headlines;

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame &mut eframe::epi::Frame)
    {
        CentralPanel::default().show(ctx, add_contents: |ui: mut Ui| {
            ui.label("article text");
        });
    }
    fn name(&self) -> &str{
        "Headlines"
    }
}

fn main() {
    let app:Headlines = Headlines;
    let win_options = NativeOptins::default();
    run_nativ(app, native_optins)
}

