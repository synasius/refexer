pub struct SynthState {
    pub playing_sample: bool,
    pub phase: i32,
    pub rep_time: i32,
    pub rep_limit: i32,

    pub arp_time: i32,
    pub arp_limit: i32,
    pub arp_mod: f64,

    pub fperiod: f64,
    pub period: i32,
    pub fmaxperiod: f64,
    pub fslide: f64,
    pub fdslide: f64,

    pub square_duty: f32,
    pub square_slide: f32,

    pub fltp: f32,
    pub fltdp: f32,
    pub fltw: f32,
    pub fltw_d: f32,
    pub fltdmp: f32,
    pub fltphp: f32,
    pub flthp: f32,
    pub flthp_d: f32,

    pub vib_phase: f32,
    pub vib_speed: f32,
    pub vib_amp: f32,

    pub env_vol: f32,
    pub env_stage: i32,
    pub env_time: i32,
    pub env_length: [i32; 3],

    pub fphase: f32,
    pub fdphase: f32,
    pub iphase: i32,
    pub ipp: i32,
    pub phaser_buffer: [f32; 1024],

    pub noise_buffer: [f32; 32],
}

impl Default for SynthState {
    fn default() -> Self {
        Self {
            playing_sample: Default::default(),
            phase: Default::default(),
            rep_time: Default::default(),
            rep_limit: Default::default(),
            arp_time: Default::default(),
            arp_limit: Default::default(),
            arp_mod: Default::default(),
            fperiod: Default::default(),
            period: Default::default(),
            fmaxperiod: Default::default(),
            fslide: Default::default(),
            fdslide: Default::default(),
            square_duty: Default::default(),
            square_slide: Default::default(),
            fltp: Default::default(),
            fltdp: Default::default(),
            fltw: Default::default(),
            fltw_d: Default::default(),
            fltdmp: Default::default(),
            fltphp: Default::default(),
            flthp: Default::default(),
            flthp_d: Default::default(),
            vib_phase: Default::default(),
            vib_speed: Default::default(),
            vib_amp: Default::default(),
            env_vol: Default::default(),
            env_stage: Default::default(),
            env_time: Default::default(),
            env_length: Default::default(),
            fphase: Default::default(),
            fdphase: Default::default(),
            iphase: Default::default(),
            ipp: Default::default(),
            phaser_buffer: [0.0; 1024],
            noise_buffer: Default::default(),
        }
    }
}
