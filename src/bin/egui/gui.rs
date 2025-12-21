//! Refexer GUI Application
//!
//! A gui for generating retro-style sound effects used in old
//! video games.
use anyhow::anyhow;
use cpal::traits::StreamTrait;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};
use std::sync::mpsc::{self, Sender};

use eframe::egui::{self, Response, RichText};
use refexer::{
    sound::stream_setup,
    synth::{
        Synth,
        params::SynthParams,
        presets::{SoundType, SynthPreset},
    },
};

fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    // initialize the synth and the audio stream
    let synth = Synth::new(SynthParams::default());
    let stream = stream_setup(rx)?;
    stream.play()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Refexer - Retro Sound FX Generator",
        options,
        Box::new(|_cc| Ok(Box::new(RefexerApp::new(synth, tx)))),
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
    synth: Synth,
    /// TODO:
    sender: Sender<Vec<f32>>,
    /// Random preset generator.
    preset: SynthPreset,
    /// inner plot data
    pub inner: BorrowPointsExample,
}

impl RefexerApp {
    fn new(synth: Synth, sender: Sender<Vec<f32>>) -> Self {
        RefexerApp {
            synth,
            sender,
            preset: SynthPreset::new(),
            inner: Default::default(),
        }
    }

    /// Plays a sound effect for the given type.
    fn play_sound(&mut self, sound_type: SoundType) {
        let params = self.preset.generate(sound_type);
        self.synth.set_params(params);
        self.synth.play_sample();

        // add sound generation and tx.send
        let mut data = Vec::new();
        while let Some(value) = self.synth.synth_sample() {
            data.push(value);
        }

        self.inner.points.clear();
        // copy the data to the inner plot buffer
        for (i, &v) in data.iter().enumerate() {
            self.inner.points.push(PlotPoint::new(i as f64, v as f64));
        }

        self.sender.send(data).unwrap();
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

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    for &(label, sound_type) in SOUND_BUTTONS {
                        self.sound_button(ui, label, sound_type);
                    }
                });
                ui.vertical(|ui| {
                    self.inner.show_plot(ui);
                })
            });
        });
    }
}

pub struct BorrowPointsExample {
    points: Vec<PlotPoint>,
}

impl Default for BorrowPointsExample {
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}

impl BorrowPointsExample {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        Plot::new("My Plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("curve", PlotPoints::Borrowed(&self.points)).name("curve"));
            })
            .response
    }
}
