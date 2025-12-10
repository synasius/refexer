use super::params::{SynthParams, WaveType};

use rand::{Rng, SeedableRng, rngs::StdRng, seq::IndexedRandom};

pub enum SoundType {
    PickupCoin,
    LaserShoot,
    Explosion,
    PowerUp,
    HitHurt,
    Jump,
    BlipSelect,
}

pub struct SynthPreset {
    rng: StdRng,
}

impl Default for SynthPreset {
    fn default() -> Self {
        Self::new()
    }
}

impl SynthPreset {
    pub fn new() -> Self {
        SynthPreset {
            rng: StdRng::from_os_rng(),
        }
    }

    pub fn generate(&mut self, sound_type: SoundType) -> SynthParams {
        match sound_type {
            SoundType::PickupCoin => self.coin(),
            SoundType::LaserShoot => self.shoot(),
            SoundType::Explosion => self.explosion(),
            SoundType::PowerUp => self.powerup(),
            SoundType::HitHurt => self.hit(),
            SoundType::Jump => self.jump(),
            SoundType::BlipSelect => self.blip(),
        }
    }

    fn coin(&mut self) -> SynthParams {
        let mut params = SynthParams {
            p_base_freq: 0.4 + self.frnd(0.5),
            p_env_attack: 0.0,
            p_env_sustain: self.frnd(0.1),
            p_env_decay: 0.1 + self.frnd(0.4),
            p_env_punch: 0.3 + self.frnd(0.3),
            ..Default::default()
        };

        if self.rng.random_bool(0.5) {
            params.p_arp_speed = 0.5 + self.frnd(0.2);
            params.p_arp_mod = 0.2 + self.frnd(0.4);
        }
        params
    }

    fn shoot(&mut self) -> SynthParams {
        let mut params = SynthParams {
            ..Default::default()
        };

        // I converted the old code to a weighted random extraction
        let waves = [
            (WaveType::Square, 5.0 / 12.0),
            (WaveType::Sawtooth, 5.0 / 12.0),
            (WaveType::Sine, 2.0 / 12.0),
        ];
        params.wave_type = waves
            .choose_weighted(&mut self.rng, |item| item.1)
            .unwrap()
            .0;

        if self.rng.random_ratio(1, 3) {
            params.p_base_freq = 0.3 + self.frnd(0.6);
            params.p_freq_limit = self.frnd(0.1);
            params.p_freq_ramp = -0.35 - self.frnd(0.3);
        } else {
            params.p_base_freq = 0.5 + self.frnd(0.5);
            params.p_freq_limit = (params.p_base_freq - 0.2 - self.frnd(0.6)).max(0.2);
            params.p_freq_ramp = -0.15 - self.frnd(0.2);
        }

        if self.rng.random::<bool>() {
            params.p_duty = self.frnd(0.5);
            params.p_duty_ramp = self.frnd(0.2);
        } else {
            params.p_duty = 0.4 + self.frnd(0.5);
            params.p_duty_ramp = -self.frnd(0.7);
        }

        params.p_env_attack = 0.0;
        params.p_env_sustain = 0.1 + self.frnd(0.2);
        params.p_env_decay = -self.frnd(0.4);

        if self.rng.random::<bool>() {
            params.p_env_punch = self.frnd(0.3);
        }

        if self.rng.random_ratio(1, 3) {
            params.p_pha_offset = self.frnd(0.2);
            params.p_pha_ramp = -self.frnd(0.2);
        }

        if self.rng.random::<bool>() {
            params.p_hpf_freq = self.frnd(0.3);
        }

        params
    }

    fn explosion(&mut self) -> SynthParams {
        let mut params = SynthParams {
            wave_type: WaveType::Noise,
            ..Default::default()
        };

        if self.rng.random::<bool>() {
            params.p_base_freq = 0.1 + self.frnd(0.4);
            params.p_freq_ramp = -0.1 + self.frnd(0.4);
        } else {
            params.p_base_freq = 0.2 + self.frnd(0.7);
            params.p_freq_ramp = -0.2 - self.frnd(0.2);
        }
        params.p_base_freq = params.p_base_freq.powf(2.0);

        if self.rng.random_ratio(1, 4) {
            params.p_freq_ramp = 0.0;
        }

        if self.rng.random_ratio(1, 3) {
            params.p_repeat_speed = 0.3 + self.frnd(0.5);
        }

        params.p_env_attack = 0.0;
        params.p_env_sustain = 0.1 + self.frnd(0.3);
        params.p_env_decay = self.frnd(0.5);

        if self.rng.random::<bool>() {
            params.p_pha_offset = -0.3 + self.frnd(0.9);
            params.p_pha_ramp = -self.frnd(0.3);
        }

        params.p_env_punch = 0.2 + self.frnd(0.6);

        if self.rng.random::<bool>() {
            params.p_vib_strength = self.frnd(0.7);
            params.p_vib_speed = self.frnd(0.6);
        }

        if self.rng.random_ratio(1, 3) {
            params.p_arp_speed = 0.6 + self.frnd(0.3);
            params.p_arp_mod = 0.8 - self.frnd(1.6);
        }

        params
    }

    fn powerup(&mut self) -> SynthParams {
        let mut params = SynthParams {
            p_env_attack: 0.0,
            p_env_sustain: self.frnd(0.4),
            p_env_decay: 0.1 + self.frnd(0.4),
            ..Default::default()
        };

        if self.rng.random::<bool>() {
            params.wave_type = WaveType::Sawtooth;
        } else {
            params.p_duty = self.frnd(0.6);
        }

        if self.rng.random::<bool>() {
            params.p_base_freq = 0.2 + self.frnd(0.3);
            params.p_freq_ramp = 0.1 + self.frnd(0.4);
            params.p_repeat_speed = 0.4 + self.frnd(0.4);
        } else {
            params.p_base_freq = 0.2 + self.frnd(0.3);
            params.p_freq_ramp = 0.05 + self.frnd(0.2);
            if self.rng.random::<bool>() {
                params.p_vib_strength = self.frnd(0.7);
                params.p_vib_speed = self.frnd(0.6);
            }
        }

        params
    }

    fn hit(&mut self) -> SynthParams {
        let waves = [WaveType::Square, WaveType::Sawtooth, WaveType::Noise];
        let mut params = SynthParams {
            wave_type: *waves.choose(&mut self.rng).unwrap(),
            p_base_freq: 0.2 + self.frnd(0.6),
            p_freq_ramp: -0.3 - self.frnd(0.4),
            p_env_attack: 0.0,
            p_env_sustain: self.frnd(0.1),
            p_env_decay: 0.1 + self.frnd(0.2),
            ..Default::default()
        };

        if let WaveType::Square = params.wave_type {
            params.p_duty = self.frnd(0.6);
        }

        if self.rng.random::<bool>() {
            params.p_hpf_freq = self.frnd(0.3);
        }

        params
    }

    fn jump(&mut self) -> SynthParams {
        let mut params = SynthParams {
            wave_type: WaveType::Square,
            p_duty: self.frnd(0.6),
            p_base_freq: 0.3 + self.frnd(0.3),
            p_freq_ramp: 0.1 + self.frnd(0.2),
            p_env_attack: 0.0,
            p_env_sustain: 0.1 + self.frnd(0.3),
            p_env_decay: 0.1 + self.frnd(0.2),
            ..Default::default()
        };

        if self.rng.random::<bool>() {
            params.p_hpf_freq = self.frnd(0.3);
        }

        if self.rng.random::<bool>() {
            params.p_lpf_freq = 1.0 - self.frnd(0.6);
        }

        params
    }

    fn blip(&mut self) -> SynthParams {
        let waves = [WaveType::Square, WaveType::Sawtooth];
        let mut params = SynthParams {
            wave_type: *waves.choose(&mut self.rng).unwrap(),
            p_base_freq: 0.2 + self.frnd(0.4),
            p_env_attack: 0.0,
            p_env_sustain: 0.1 + self.frnd(0.1),
            p_env_decay: self.frnd(0.2),
            p_hpf_freq: 0.1,
            ..Default::default()
        };

        if let WaveType::Square = params.wave_type {
            params.p_duty = self.frnd(0.6);
        }

        params
    }

    fn frnd(&mut self, range: f32) -> f32 {
        self.rng.random::<f32>() * range
    }
}
