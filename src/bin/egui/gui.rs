///! Refexer GUI Application
///!
///! A gui for generating retro-style sound effects used in old
///! video games.
use anyhow::anyhow;
use cpal::traits::StreamTrait;
use std::sync::{Arc, Mutex};

use eframe::egui::{self, RichText};
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
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Refexer - Retro Sound FX Generator",
        options,
        Box::new(|_cc| Ok(Box::new(RefexerApp::new(synth)))),
    )
    .map_err(|e| anyhow!("Failed to start eframe: {}", e))
}

/// Sound button configuration
const SOUND_BUTTONS: &[(&str, SoundType)] = &[
    ("Pickup Coin", SoundType::PickupCoin),
    ("Laser / Shoot", SoundType::LaserShoot),
    ("Explosion", SoundType::Explosion),
    ("Power Up", SoundType::PowerUp),
    ("Hit / Hurt", SoundType::HitHurt),
    ("Jump", SoundType::Jump),
    ("Blip / Select", SoundType::BlipSelect),
];

/// Main application state.
struct RefexerApp {
    /// Thread-safe handle to the audio synth.
    synth: Arc<Mutex<Synth>>,
    /// Random preset generator.
    preset: SynthPreset,
}

impl RefexerApp {
    fn new(synth: Arc<Mutex<Synth>>) -> Self {
        RefexerApp {
            synth,
            preset: SynthPreset::new(),
        }
    }

    /// Plays a sound effect for the given type.
    fn play_sound(&mut self, sound_type: SoundType) {
        let params = self.preset.generate(sound_type);
        match self.synth.lock() {
            Ok(mut synth) => {
                synth.set_params(params);
                synth.play_sample();
            }
            Err(e) => eprintln!("Failed to acquire synth lock: {}", e),
        }
    }

    /// Renders a sound button and handles the click by playing
    /// the corresponding sound type effect.
    fn sound_button(&mut self, ui: &mut egui::Ui, label: &str, sound_type: SoundType) {
        if ui
            .add_sized([100.0, 30.0], egui::Button::new(label))
            .clicked()
        {
            self.play_sound(sound_type);
        }
    }
}

impl eframe::App for RefexerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Refexer").size(20.0));
            ui.separator();

            ui.vertical(|ui| {
                for &(label, sound_type) in SOUND_BUTTONS {
                    self.sound_button(ui, label, sound_type);
                }
            });
        });
    }
}
