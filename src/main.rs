use std::{borrow::Cow, iter::FromIterator};

use eframe::{egui::{ Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Seperator, Vec2}, epi::Storage};

struct Headlines{
    articles: Vec<NewsCardData>
}

impl Headlines {
    fn new() -> Headlines {
        let iter = (0..20).map(|a|NewsCardData{
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a)
        });
        Headlines{
            articles: Vec::from_iter(iter)
        }
    }
    fn configure_fonts(&Self { articles }: Self)
}

struct NewsCardData{
    title: String,
    desc: String,
    url: String
}

impl App for Headlines {
    fn  setup(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>, storage: Option<&dyn eframe::epi::Storage>) {
        self.configure_fonts();
    }  

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui|{
            ScrollArea::auto_sized().show(ui, |ui: |{
                for a in &self.articles{
                                ui.label(a.title);
                                ui.label(a.url);
                                ui.label(a.desc);
                            }
            })            
        });
    }
    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app = Headlines::new();
    let mut win_options = NativeOptins::default();
    win_options.initial_window_size = Some(Vec2::new(540, 960))
    run_nativ(Box::new(app), win_options);
}

