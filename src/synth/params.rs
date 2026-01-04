use rand::prelude::*;

#[derive(Default, Clone, Copy)]
pub enum WaveType {
    #[default]
    Square,
    Sawtooth,
    Sine,
    Noise,
}

#[derive(Clone, Copy)]
pub struct SynthParams {
    pub wave_type: WaveType,
    pub base_freq: f32,

    pub freq_limit: f32,
    pub freq_ramp: f32,
    pub freq_dramp: f32,
    pub duty: f32,
    pub duty_ramp: f32,

    pub vib_strength: f32,
    pub vib_speed: f32,

    pub env_attack: f32,
    pub env_sustain: f32,
    pub env_decay: f32,
    pub env_punch: f32,

    pub lpf_resonance: f32,
    pub lpf_freq: f32,
    pub lpf_ramp: f32,
    pub hpf_freq: f32,
    pub hpf_ramp: f32,

    pub pha_offset: f32,
    pub pha_ramp: f32,

    pub repeat_speed: f32,
    pub arp_speed: f32,
    pub arp_mod: f32,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            wave_type: Default::default(),
            base_freq: 0.3,
            freq_limit: Default::default(),
            freq_ramp: Default::default(),
            freq_dramp: Default::default(),
            duty: Default::default(),
            duty_ramp: Default::default(),

            vib_strength: Default::default(),
            vib_speed: Default::default(),

            env_attack: Default::default(),
            env_sustain: 0.3,
            env_decay: 0.4,
            env_punch: Default::default(),

            lpf_resonance: Default::default(),
            lpf_freq: 1.0,
            lpf_ramp: Default::default(),
            hpf_freq: Default::default(),
            hpf_ramp: Default::default(),

            pha_offset: Default::default(),
            pha_ramp: Default::default(),

            repeat_speed: Default::default(),
            arp_speed: Default::default(),
            arp_mod: Default::default(),
        }
    }
}

impl SynthParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mutate(&mut self, rng: &mut impl Rng) {
        if rng.random::<bool>() {
            self.base_freq += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.freq_ramp += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.freq_dramp += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.duty += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.duty_ramp += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.vib_strength += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.vib_speed += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.env_attack += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.env_sustain += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.env_decay += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.env_punch += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.lpf_resonance += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.lpf_freq += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.lpf_ramp += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.hpf_freq += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.hpf_ramp += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.pha_offset += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.pha_ramp += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.repeat_speed += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.arp_speed += frnd(rng, 0.1) - 0.05;
        }
        if rng.random::<bool>() {
            self.arp_mod += frnd(rng, 0.1) - 0.05;
        }
    }
}

fn frnd(rng: &mut impl Rng, range: f32) -> f32 {
    rng.random::<f32>() * range
}
