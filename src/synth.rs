use rand::prelude::*;

use crate::other::Other;

pub enum WaveType {
    Sine,
    Square,
    Sawtooth,
    Noise,
}

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub struct Synth {
    pub wave_type: WaveType,
    pub p_base_freq: f32,

    p_freq_limit: f32,
    p_freq_ramp: f32,
    p_freq_dramp: f32,
    p_duty: f32,
    p_duty_ramp: f32,

    p_vib_strength: f32,
    p_vib_speed: f32,
    p_vib_delay: f32,

    p_env_attack: f32,
    p_env_sustain: f32,
    p_env_decay: f32,
    p_env_punch: f32,

    filter_on: bool,
    p_lpf_resonance: f32,
    p_lpf_freq: f32,
    p_lpf_ramp: f32,
    p_hpf_freq: f32,
    p_hpf_ramp: f32,

    p_pha_offset: f32,
    p_pha_ramp: f32,

    p_repeat_speed: f32,

    p_arp_speed: f32,
    p_arp_mod: f32,

    master_vol: f32,
    sound_vol: f32,

    rng: StdRng,
    other: Other,
}

impl Synth {
    pub fn new() -> Self {
        Synth {
            wave_type: WaveType::Square,

            p_base_freq: 0.2,
            p_freq_limit: 0.0,
            p_freq_ramp: 0.0,
            p_freq_dramp: 0.0,
            p_duty: 0.0,
            p_duty_ramp: 0.0,

            p_vib_strength: 0.0,
            p_vib_speed: 0.0,
            p_vib_delay: 0.0,

            p_env_attack: 0.0,
            p_env_sustain: 0.3,
            p_env_decay: 0.4,
            p_env_punch: 0.0,

            filter_on: false,
            p_lpf_resonance: 0.0,
            p_lpf_freq: 1.0,
            p_lpf_ramp: 0.0,
            p_hpf_freq: 0.0,
            p_hpf_ramp: 0.0,

            p_pha_offset: 0.0,
            p_pha_ramp: 0.0,

            p_repeat_speed: 0.0,

            p_arp_speed: 0.0,
            p_arp_mod: 0.0,

            master_vol: 0.05,
            sound_vol: 0.5,

            other: Other::default(),

            rng: StdRng::from_os_rng(),
        }
    }

    pub fn reset_params(&mut self) {
        self.wave_type = WaveType::Square;

        self.p_base_freq = 0.3;
        self.p_freq_limit = 0.0;
        self.p_freq_ramp = 0.0;
        self.p_freq_dramp = 0.0;
        self.p_duty = 0.0;
        self.p_duty_ramp = 0.0;

        self.p_vib_strength = 0.0;
        self.p_vib_speed = 0.0;
        self.p_vib_delay = 0.0;

        self.p_env_attack = 0.0;
        self.p_env_sustain = 0.3;
        self.p_env_decay = 0.4;
        self.p_env_punch = 0.0;

        self.filter_on = false;
        self.p_lpf_resonance = 0.0;
        self.p_lpf_freq = 1.0;
        self.p_lpf_ramp = 0.0;
        self.p_hpf_freq = 0.0;
        self.p_hpf_ramp = 0.0;

        self.p_pha_offset = 0.0;
        self.p_pha_ramp = 0.0;

        self.p_repeat_speed = 0.0;

        self.p_arp_speed = 0.0;
        self.p_arp_mod = 0.0;
    }

    pub fn play_sample(&mut self) {
        self.reset_sample(false);
        self.other.playing_sample = true;
    }

    pub fn synth_sample(&mut self, length: usize, buffer: &mut [f32]) {
        for i in 0..length {
            if !self.other.playing_sample {
                break;
            }

            self.other.rep_time += 1;
            if self.other.rep_limit != 0 && self.other.rep_time >= self.other.rep_limit {
                self.other.rep_time = 0;
                self.reset_sample(true);
            }

            // TODO:

            let mut ssample: f32 = 0.0;
            // 8x supersampling
            for _si in 0..8 {
                let mut sample: f32 = 0.0;
                self.other.phase += 1;

                if self.other.phase >= self.other.period {
                    self.other.phase %= self.other.period;
                    if let WaveType::Noise = self.wave_type {
                        for i in 0..32 {
                            // noise_buffer[i]=frnd(2.0f)-1.0f;
                        }
                    }
                }

                // base waveform
                let fp = self.other.phase as f32 / self.other.period as f32;
                match self.wave_type {
                    WaveType::Sine => {
                        sample = (fp * TWO_PI).sin();
                    }
                    WaveType::Square => {
                        if fp < self.other.square_duty {
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
            self.other.phase = 0;
        }
        self.other.fperiod = 100.0 as f64 / (self.p_base_freq * self.p_base_freq + 0.001) as f64;
        self.other.period = self.other.fperiod as i32;
        // TODO: other stuff
        self.other.square_duty = 0.5 - self.p_duty * 0.5;
        // TODO

        if !restart {
            // TODO: other stuff

            self.other.rep_time = 0;
            self.other.rep_limit = ((1.0 - self.p_repeat_speed).powf(2.0) * 20_000.0 + 32.0) as i32;
            if self.p_repeat_speed == 0.0 {
                self.other.rep_limit = 0;
            }
        }
    }
}
