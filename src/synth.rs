pub mod params;
mod state;

use params::{SynthParams, WaveType};
use state::SynthState;

use rand::prelude::*;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub struct Synth {
    params: SynthParams,
    state: SynthState,

    master_vol: f32,
    sound_vol: f32,

    rng: StdRng,
}

impl Synth {
    pub fn new(params: SynthParams) -> Self {
        Synth {
            params,
            state: SynthState::default(),

            master_vol: 0.05,
            sound_vol: 0.5,

            rng: StdRng::from_os_rng(),
        }
    }

    pub fn reset_params(&mut self) {
        self.params = SynthParams::default()
    }

    pub fn play_sample(&mut self) {
        self.reset_sample(false);
        self.state.playing_sample = true;
    }

    pub fn synth_sample(&mut self, length: usize, buffer: &mut [f32]) {
        for i in 0..length {
            if !self.state.playing_sample {
                break;
            }

            self.state.rep_time += 1;
            if self.state.rep_limit != 0 && self.state.rep_time >= self.state.rep_limit {
                self.state.rep_time = 0;
                self.reset_sample(true);
            }

            // TODO:

            let mut ssample: f32 = 0.0;
            // 8x supersampling
            for _si in 0..8 {
                let mut sample: f32 = 0.0;
                self.state.phase += 1;

                if self.state.phase >= self.state.period {
                    self.state.phase %= self.state.period;
                    if let WaveType::Noise = self.params.wave_type {
                        for i in 0..32 {
                            // noise_buffer[i]=frnd(2.0f)-1.0f;
                        }
                    }
                }

                // base waveform
                let fp = self.state.phase as f32 / self.state.period as f32;
                match self.params.wave_type {
                    WaveType::Sine => {
                        sample = (fp * TWO_PI).sin();
                    }
                    WaveType::Square => {
                        if fp < self.state.square_duty {
                            sample = 0.5;
                        } else {
                            sample = -0.5;
                        }
                    }
                    WaveType::Sawtooth => {
                        sample = 1.0 - fp * 2.0;
                    }
                    WaveType::Noise => {
                        // sample=noise_buffer[phase*32/period];
                    }
                }
                // TODO: lp filter
                // TODO: hp filter
                // TODO: phaser

                // TODO: envelop application
                ssample += sample;
            }

            ssample = ssample / 8.0 * self.master_vol;
            ssample *= 2.0 * self.sound_vol;

            if ssample > 1.0 {
                ssample = 1.0;
            }
            if ssample < -1.0 {
                ssample = -1.0
            }

            buffer[i] = ssample;
        }
    }

    fn reset_sample(&mut self, restart: bool) {
        if !restart {
            self.state.phase = 0;
        }
        self.state.fperiod =
            100.0 as f64 / (self.params.p_base_freq * self.params.p_base_freq + 0.001) as f64;
        self.state.period = self.state.fperiod as i32;
        // TODO: other stuff
        self.state.square_duty = 0.5 - self.params.p_duty * 0.5;
        // TODO

        if !restart {
            // TODO: other stuff

            self.state.rep_time = 0;
            self.state.rep_limit =
                ((1.0 - self.params.p_repeat_speed).powf(2.0) * 20_000.0 + 32.0) as i32;
            if self.params.p_repeat_speed == 0.0 {
                self.state.rep_limit = 0;
            }
        }
    }
}
