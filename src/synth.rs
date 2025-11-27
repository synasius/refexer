pub mod params;
pub mod presets;
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
        for item in buffer.iter_mut().take(length) {
            if !self.state.playing_sample {
                break;
            }

            self.state.rep_time += 1;
            if self.state.rep_limit != 0 && self.state.rep_time >= self.state.rep_limit {
                self.state.rep_time = 0;
                self.reset_sample(true);
            }

            // frequency envelopes/arpeggios
            self.state.arp_time += 1;
            if self.state.arp_limit != 0 && self.state.arp_time >= self.state.arp_limit {
                self.state.arp_limit = 0;
                self.state.fperiod *= self.state.arp_mod;
            }

            self.state.fslide += self.state.fdslide;
            self.state.fperiod *= self.state.fslide;
            if self.state.fperiod > self.state.fmaxperiod {
                self.state.fperiod = self.state.fmaxperiod;

                if self.params.p_freq_limit > 0.0 {
                    self.state.playing_sample = false;
                }
            }
            let mut rfperiod = self.state.fperiod as f32;
            if self.state.vib_amp > 0.0 {
                self.state.vib_phase += self.state.vib_speed;
                rfperiod = self.state.fperiod as f32
                    * (1.0 + self.state.vib_phase.sin() * self.state.vib_amp);
            }

            self.state.period = rfperiod as i32;
            if self.state.period < 8 {
                self.state.period = 8;
            }
            self.state.square_duty += self.state.square_slide;
            self.state.square_duty = self.state.square_duty.clamp(0.0, 0.5);

            // volume envelope
            self.state.env_time += 1;
            if self.state.env_time > self.state.env_length[self.state.env_stage as usize] {
                self.state.env_time = 0;
                self.state.env_stage += 1;
                if self.state.env_stage == 3 {
                    self.state.playing_sample = false;
                }
            }

            match self.state.env_stage {
                0 => {
                    self.state.env_vol =
                        self.state.env_time as f32 / self.state.env_length[0] as f32;
                }
                1 => {
                    self.state.env_vol = 1.0
                        + (1.0 - self.state.env_time as f32 / self.state.env_length[1] as f32)
                            * 2.0
                            * self.params.p_env_punch
                }
                2 => {
                    self.state.env_vol =
                        1.0 - self.state.env_time as f32 / self.state.env_length[2] as f32
                }
                _ => {}
            }

            // phaser step
            self.state.fphase += self.state.fdphase;
            self.state.iphase = (self.state.fphase as i32).abs();
            if self.state.iphase > 1023 {
                self.state.iphase = 1023
            }

            if self.state.flthp_d != 0.0 {
                self.state.flthp *= self.state.flthp_d;

                self.state.flthp = self.state.flthp.clamp(0.00001, 0.1);
            }

            let mut ssample: f32 = 0.0;
            // 8x supersampling
            for _si in 0..8 {
                // let mut sample: f32 = 0.0;
                self.state.phase += 1;

                if self.state.phase >= self.state.period {
                    self.state.phase %= self.state.period;
                    if let WaveType::Noise = self.params.wave_type {
                        for i in 0..32 {
                            self.state.noise_buffer[i] = self.rng.random::<f32>() * 2.0 - 1.0;
                        }
                    }
                }

                // base waveform
                let fp = self.state.phase as f32 / self.state.period as f32;
                let mut sample = match self.params.wave_type {
                    WaveType::Sine => (fp * TWO_PI).sin(),
                    WaveType::Square => {
                        if fp < self.state.square_duty {
                            0.5
                        } else {
                            -0.5
                        }
                    }
                    WaveType::Sawtooth => 1.0 - fp * 2.0,
                    WaveType::Noise => {
                        let index = self.state.phase * 32 / self.state.period;
                        self.state.noise_buffer[index as usize]
                    }
                };

                // lp filter
                let pp = self.state.fltp;
                self.state.fltw *= self.state.fltw_d;
                if self.state.fltw < 0.0 {
                    self.state.fltw = 0.0;
                }
                if self.state.fltw > 0.1 {
                    self.state.fltw = 0.1;
                };
                self.state.fltw = self.state.fltw.clamp(0.0, 1.0);

                if self.params.p_lpf_freq != 1.0 {
                    self.state.fltdp += (sample - self.state.fltp) * self.state.fltw;
                    self.state.fltdp -= self.state.fltdp * self.state.fltdmp;
                } else {
                    self.state.fltp = sample;
                    self.state.fltdp = 0.0;
                }
                self.state.fltp += self.state.fltdp;

                // hp filter

                self.state.fltphp += self.state.fltp - pp;
                self.state.fltphp -= self.state.fltphp * self.state.flthp;
                sample = self.state.fltphp;

                // phaser
                let index = (self.state.ipp & 1023) as usize;
                self.state.phaser_buffer[index] = sample;

                let index = ((self.state.ipp - self.state.iphase + 1024) & 1023) as usize;
                sample += self.state.phaser_buffer[index];

                self.state.ipp = (self.state.ipp + 1) & 1023;

                // envelop application
                ssample += sample * self.state.env_vol;
            }

            ssample = ssample / 8.0 * self.master_vol;
            ssample *= 2.0 * self.sound_vol;
            ssample = ssample.clamp(-1.0, 1.0);

            // assign the computed sample
            *item = ssample;
        }
    }

    fn reset_sample(&mut self, restart: bool) {
        if !restart {
            self.state.phase = 0;
        }

        self.state.fperiod = 100.0 / ((self.params.p_base_freq as f64).powf(2.0) + 0.001);
        self.state.period = self.state.fperiod as i32;

        self.state.fmaxperiod = 100.0 / ((self.params.p_freq_limit as f64).powf(2.0) + 0.001);
        self.state.fslide = 1.0 - (self.params.p_freq_ramp as f64).powf(3.0) * 0.01;
        self.state.fdslide = -(self.params.p_freq_ramp as f64).powf(3.0) * 0.000001;

        self.state.square_duty = 0.5 - self.params.p_duty * 0.5;
        self.state.square_slide = -self.params.p_duty_ramp * 0.00005;

        if self.params.p_arp_mod >= 0.0 {
            self.state.arp_mod = 1.0 - (self.params.p_arp_mod as f64).powf(2.0) * 0.9;
        } else {
            self.state.arp_mod = 1.0 + (self.params.p_arp_mod as f64).powf(2.0) * 10.0;
        }

        self.state.arp_time = 0;
        self.state.arp_limit = ((1.0 - self.params.p_arp_speed).powf(2.0) * 20_000.0 + 32.0) as i32;
        if self.params.p_arp_speed == 1.0 {
            self.state.arp_limit = 0;
        }

        if !restart {
            self.state.fltp = 0.0;
            self.state.fltdp = 0.0;
            self.state.fltw = self.params.p_lpf_freq.powf(3.0) * 0.1;
            self.state.fltw_d = 1.0 + self.params.p_lpf_ramp * 0.0001;
            self.state.fltdmp = 5.0 / (1.0 + self.params.p_lpf_resonance.powf(2.0) * 20.0)
                * (0.01 + self.state.fltw);
            if self.state.fltdmp > 0.8 {
                self.state.fltdmp = 0.8;
            }

            self.state.fltphp = 0.0;
            self.state.flthp = self.params.p_hpf_freq.powf(2.0) * 0.1;
            self.state.flthp_d = 1.0 + self.params.p_hpf_ramp * 0.0003;

            // reset vibrato
            self.state.vib_phase = 0.0;
            self.state.vib_speed = self.params.p_vib_speed.powf(2.0) * 0.01;
            self.state.vib_amp = self.params.p_vib_strength * 0.5;

            // reset envelope
            self.state.env_vol = 0.0;
            self.state.env_stage = 0;
            self.state.env_time = 0;
            self.state.env_length[0] = (self.params.p_env_attack.powf(2.0) * 100000.0) as i32;
            self.state.env_length[1] = (self.params.p_env_sustain.powf(2.0) * 100000.0) as i32;
            self.state.env_length[2] = (self.params.p_env_decay.powf(2.0) * 100000.0) as i32;

            self.state.fphase = self.params.p_pha_offset.powf(2.0) * 1020.0;
            if self.params.p_pha_offset < 0.0 {
                self.state.fphase = -self.state.fphase;
            }
            self.state.fdphase = self.params.p_pha_ramp.powf(2.0) * 1.0;
            if self.params.p_pha_ramp < 0.0 {
                self.state.fdphase = -self.state.fdphase;
            }
            self.state.iphase = self.state.fphase.abs() as i32;
            self.state.ipp = 0;
            for i in 0..1024 {
                self.state.phaser_buffer[i] = 0.0;
            }

            for i in 0..32 {
                self.state.noise_buffer[i] = self.rng.random::<f32>() * 2.0 - 1.0;
            }

            self.state.rep_time = 0;
            self.state.rep_limit =
                ((1.0 - self.params.p_repeat_speed).powf(2.0) * 20_000.0 + 32.0) as i32;
            if self.params.p_repeat_speed == 0.0 {
                self.state.rep_limit = 0;
            }
        }
    }
}
