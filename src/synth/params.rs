#[derive(Default, Clone, Copy)]
pub enum WaveType {
    #[default]
    Square,
    Sawtooth,
    Sine,
    Noise,
}

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
