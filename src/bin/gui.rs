use anyhow::anyhow;
use cpal::traits::StreamTrait;
use std::sync::{Arc, Mutex};

use eframe::egui;
use refexer::{
    sound::stream_setup,
    synth::{
        Synth,
        params::SynthParams,
        presets::{SoundType, SynthPreset},
    },
};

fn main() -> anyhow::Result<()> {
    // initialize the synth and the audio stream
    let synth = Synth::new(SynthParams::default());
    let synth = Arc::new(Mutex::new(synth));
    let stream = stream_setup(Arc::clone(&synth))?;
    stream.play()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::new(RefexerApp::new(synth)))),
    )
    .map_err(|e| anyhow!("Failed to start eframe: {}", e))
}

struct RefexerApp {
    synth: Arc<Mutex<Synth>>,
    preset: SynthPreset,
}

impl RefexerApp {
    fn new(synth: Arc<Mutex<Synth>>) -> Self {
        RefexerApp {
            synth,
            preset: SynthPreset::new(),
        }
    }
}

impl eframe::App for RefexerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Refexer");
            ui.vertical(|ui| {
                if ui.button("pickup coin").clicked() {
                    let params = self.preset.generate(SoundType::PickupCoin);
                    let mut synth = self.synth.lock().unwrap();
                    synth.set_params(params);
                    synth.play_sample();
                }
                if ui.button("laser / shoot").clicked() {
                    let params = self.preset.generate(SoundType::LaserShoot);
                    let mut synth = self.synth.lock().unwrap();
                    synth.set_params(params);
                    synth.play_sample();
                }
                if ui.button("explosion").clicked() {
                    let params = self.preset.generate(SoundType::Explosion);
                    let mut synth = self.synth.lock().unwrap();
                    synth.set_params(params);
                    synth.play_sample();
                }
                if ui.button("power up").clicked() {
                    let params = self.preset.generate(SoundType::PowerUp);
                    let mut synth = self.synth.lock().unwrap();
                    synth.set_params(params);
                    synth.play_sample();
                }
                if ui.button("hit / hurt").clicked() {
                    let params = self.preset.generate(SoundType::HitHurt);
                    let mut synth = self.synth.lock().unwrap();
                    synth.set_params(params);
                    synth.play_sample();
                }
                if ui.button("jump").clicked() {
                    let params = self.preset.generate(SoundType::Jump);
                    let mut synth = self.synth.lock().unwrap();
                    synth.set_params(params);
                    synth.play_sample();
                }
                if ui.button("blip / select").clicked() {
                    let params = self.preset.generate(SoundType::BlipSelect);
                    let mut synth = self.synth.lock().unwrap();
                    synth.set_params(params);
                    synth.play_sample();
                }
            });
        });
    }
}
