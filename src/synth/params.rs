#[derive(Default)]
pub enum WaveType {
    #[default]
    Square,
    Sine,
    Sawtooth,
    Noise,
}

pub struct SynthParams {
    pub wave_type: WaveType,
    pub p_base_freq: f32,

    pub p_freq_limit: f32,
    pub p_freq_ramp: f32,
    pub p_freq_dramp: f32,
    pub p_duty: f32,
    pub p_duty_ramp: f32,

    pub p_vib_strength: f32,
    pub p_vib_speed: f32,
    pub p_vib_delay: f32,

    pub p_env_attack: f32,
    pub p_env_sustain: f32,
    pub p_env_decay: f32,
    pub p_env_punch: f32,

    pub filter_on: bool,
    pub p_lpf_resonance: f32,
    pub p_lpf_freq: f32,
    pub p_lpf_ramp: f32,
    pub p_hpf_freq: f32,
    pub p_hpf_ramp: f32,

    pub p_pha_offset: f32,
    pub p_pha_ramp: f32,

    pub p_repeat_speed: f32,
    pub p_arp_speed: f32,
    pub p_arp_mod: f32,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            wave_type: Default::default(),
            p_base_freq: 0.3,
            p_freq_limit: Default::default(),
            p_freq_ramp: Default::default(),
            p_freq_dramp: Default::default(),
            p_duty: Default::default(),
            p_duty_ramp: Default::default(),

            p_vib_strength: Default::default(),
            p_vib_speed: Default::default(),
            p_vib_delay: Default::default(),

            p_env_attack: Default::default(),
            p_env_sustain: 0.3,
            p_env_decay: 0.4,
            p_env_punch: Default::default(),

            filter_on: Default::default(),
            p_lpf_resonance: Default::default(),
            p_lpf_freq: 1.0,
            p_lpf_ramp: Default::default(),
            p_hpf_freq: Default::default(),
            p_hpf_ramp: Default::default(),

            p_pha_offset: Default::default(),
            p_pha_ramp: Default::default(),

            p_repeat_speed: Default::default(),
            p_arp_speed: Default::default(),
            p_arp_mod: Default::default(),
        }
    }
}
