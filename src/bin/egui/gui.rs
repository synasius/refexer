//! Refexer GUI Application
//!
//! A gui for generating retro-style sound effects used in old
//! video games.

use rand::prelude::*;
use std::sync::mpsc::Sender;

use eframe::egui::{self, Layout, Response, RichText, Slider, vec2};
use refexer::synth::{
    Synth,
    params::SynthParams,
    presets::{SoundType, SynthPreset},
};

use super::plot;

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
pub struct RefexerApp {
    /// Audio synth.
    synth: Synth,
    /// Current parameters
    params: SynthParams,
    /// Channel sender for streaming audio data to the playback thread.
    sender: Sender<Vec<f32>>,
    /// Random preset generator.
    preset: SynthPreset,
    /// inner plot data
    waveform_plot: plot::WaveformPlot,
    /// Random generator
    rng: StdRng,
}

impl RefexerApp {
    pub fn new(sender: Sender<Vec<f32>>) -> Self {
        let params = SynthParams::default();
        let synth = Synth::new(params);

        RefexerApp {
            params,
            sender,
            synth,
            preset: SynthPreset::new(),
            waveform_plot: Default::default(),
            rng: StdRng::from_os_rng(),
        }
    }

    /// Plays a sound effect for the given type.
    fn play_sound(&mut self, sound_type: SoundType) {
        self.params = self.preset.generate(sound_type);

        self.play();
    }

    /// Plays the current sound effect with params slightly mutated
    fn mutate_sound(&mut self) {
        self.params.mutate(&mut self.rng);

        self.play();
    }

    fn play(&mut self) {
        self.synth.set_params(self.params);
        self.synth.play_sample();

        // add sound generation and tx.send
        let mut data = Vec::new();
        while let Some(value) = self.synth.synth_sample() {
            data.push(value);
        }

        self.waveform_plot.set_data(&data);

        if let Err(e) = self.sender.send(data) {
            eprintln!("Failed to send audio data: {}", e);
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

    fn envelope(&mut self, ui: &mut egui::Ui) {
        ui.label("Envelope");
        if slider(ui, "Attack time", &mut self.params.env_attack, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Sustain time", &mut self.params.env_sustain, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Sustain punch", &mut self.params.env_punch, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Decay time", &mut self.params.env_decay, 0.0, 1.0).changed() {
            self.play();
        }
    }

    fn frequency(&mut self, ui: &mut egui::Ui) {
        ui.label("Frequency");
        if slider(ui, "Start frequency", &mut self.params.base_freq, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Min frequency", &mut self.params.freq_limit, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Slide", &mut self.params.freq_ramp, -1.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Delta slide", &mut self.params.freq_dramp, -1.0, 1.0).changed() {
            self.play();
        }
    }

    fn vibrato(&mut self, ui: &mut egui::Ui) {
        ui.label("Vibrato");
        if slider(ui, "Depth", &mut self.params.vib_strength, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Speed", &mut self.params.vib_speed, 0.0, 1.0).changed() {
            self.play();
        }
    }

    fn arpeggios(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("Change");
            if slider(ui, "Amount", &mut self.params.arp_mod, -1.0, 1.0).changed() {
                self.play();
            }
            if slider(ui, "Speed", &mut self.params.arp_speed, 0.0, 1.0).changed() {
                self.play();
            }
        });
    }

    fn duty(&mut self, ui: &mut egui::Ui) {
        ui.label("Duty");
        if slider(ui, "Cycle", &mut self.params.duty, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Sweep", &mut self.params.duty_ramp, -1.0, 1.0).changed() {
            self.play();
        }
    }

    fn repeat(&mut self, ui: &mut egui::Ui) {
        ui.label("Repeat");
        if slider(ui, "Speed", &mut self.params.repeat_speed, 0.0, 1.0).changed() {
            self.play();
        }
    }

    fn phaser(&mut self, ui: &mut egui::Ui) {
        ui.label("Phaser");
        if slider(ui, "Offset", &mut self.params.pha_offset, -1.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Sweep", &mut self.params.pha_ramp, -1.0, 1.0).changed() {
            self.play();
        }
    }

    fn low_pass(&mut self, ui: &mut egui::Ui) {
        ui.label("Low-Pass Filter");
        if slider(ui, "Cutoff", &mut self.params.lpf_freq, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Cutoff Sweep", &mut self.params.lpf_ramp, -1.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Resonance", &mut self.params.lpf_resonance, 0.0, 1.0).changed() {
            self.play();
        }
    }

    fn high_pass(&mut self, ui: &mut egui::Ui) {
        ui.label("High-Pass Filter");
        if slider(ui, "Cutoff", &mut self.params.hpf_freq, 0.0, 1.0).changed() {
            self.play();
        }
        if slider(ui, "Cutoff Sweep", &mut self.params.hpf_ramp, -1.0, 1.0).changed() {
            self.play();
        }
    }
}

fn slider(ui: &mut egui::Ui, label: &str, value: &mut f32, min: f32, max: f32) -> Response {
    ui.add(Slider::new(value, min..=max).text(label))
}

impl eframe::App for RefexerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Refexer").size(20.0));
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Generators");
                            for &(label, sound_type) in SOUND_BUTTONS {
                                self.sound_button(ui, label, sound_type);
                            }
                            ui.add_space(48.0);

                            if ui
                                .add_sized([100.0, 30.0], egui::Button::new("Mutate"))
                                .clicked()
                            {
                                self.mutate_sound();
                            }
                            self.sound_button(ui, "Randomize", SoundType::Randomize);
                        });
                        ui.allocate_ui(vec2(200.0, ui.available_size_before_wrap().y), |ui| {
                            ui.with_layout(
                                Layout::top_down(egui::Align::Min).with_main_wrap(true),
                                |ui| {
                                    self.envelope(ui);
                                    ui.add_space(24.0);
                                    self.frequency(ui);
                                    ui.add_space(24.0);
                                    self.vibrato(ui);
                                    ui.add_space(24.0);
                                    self.arpeggios(ui);
                                    ui.add_space(24.0);
                                    self.duty(ui);
                                    ui.add_space(24.0);
                                    self.repeat(ui);
                                    ui.add_space(24.0);
                                    self.phaser(ui);
                                    ui.add_space(24.0);
                                    self.low_pass(ui);
                                    ui.add_space(24.0);
                                    self.high_pass(ui);
                                    ui.add_space(24.0);
                                },
                            )
                        });
                    });
                    // ui.set_min_height(200.0);
                    self.waveform_plot.show_plot(ui);
                })
            });
        });
    }
}
