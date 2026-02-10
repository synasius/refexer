use anyhow::anyhow;
use cpal::traits::StreamTrait;
use std::sync::mpsc;

use eframe::egui;
use refexer::sound::stream_setup;

mod gui;
mod plot;

fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    // initialize the synth and the audio stream
    let (stream, _) = stream_setup(rx)?;
    stream.play()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Refexer - Retro Sound FX Generator",
        options,
        Box::new(|_cc| Ok(Box::new(gui::RefexerApp::new(tx)))),
    )
    .map_err(|e| anyhow!("Failed to start eframe: {}", e))
}
