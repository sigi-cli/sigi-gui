use eframe::egui::*;
use eframe::epi;
use gag::BufferRedirect;
use sigi::{effects::*, output::*};
use std::io::BufReader;

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
            ui.indent("version", |ui| {
                ui.label(format!("Version: {}", RELEASE_VERSION))
            });

            // HACKIN

            let mut rdr = list_all("sigi");

            for result in rdr.records() {
                let record = result.unwrap();
                ui.label(record.get(1).unwrap());
            }

            // DONE HACKIN
        });
    }
}

pub fn run() {
    let app = SigiApp::default();
    let native_options = epi::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);
}

type SigiReader = csv::Reader<BufReader<gag::Buffer>>;

pub fn list_all(stack: &str) -> SigiReader {
    captured(|| ListAll::from(stack).run(OutputFormat::Tsv))
}

pub fn captured<F>(tsv_printer: F) -> SigiReader
where
    F: FnOnce(),
{
    let buf = BufferRedirect::stdout().unwrap();

    tsv_printer();

    csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(BufReader::new(buf.into_inner()))
}
