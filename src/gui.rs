use eframe::epi;
use eframe::egui::*;
use sigi::{output::*, effects::*};
use gag::BufferRedirect;
use std::io::{BufReader};

pub const RELEASE_VERSION: &'static str = std::env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct SigiApp {}

impl epi::App for SigiApp {
    fn name(&self) -> &str {
        "Sigi"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.indent("version", |ui|
                ui.label(format!("Version: {}", RELEASE_VERSION))
            );
            

            // HACKIN

            let buf = BufferRedirect::stdout().unwrap();
            ListAll::from("sigi").run(OutputFormat::Tsv);
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .from_reader(BufReader::new(buf.into_inner()));

            for result in rdr.records() {
                let record = result.unwrap();
                ui.label(record.get(1).unwrap());
            }
            // DONE HACKIN
        });
    }
}

pub fn run(){
    let app = SigiApp::default();
    let native_options = epi::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);
}