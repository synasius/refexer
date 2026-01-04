use super::params::{SynthParams, WaveType};

use rand::{Rng, SeedableRng, rngs::StdRng, seq::IndexedRandom};

#[derive(Default, Copy, Clone)]
pub enum SoundType {
    #[default]
    PickupCoin,
    LaserShoot,
    Explosion,
    PowerUp,
    HitHurt,
    Jump,
    BlipSelect,
    Randomize,
}

impl TryFrom<&str> for SoundType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "coin" => Ok(Self::PickupCoin),
            "pickup" => Ok(Self::PickupCoin),
            "shoot" => Ok(Self::LaserShoot),
            "laser" => Ok(Self::LaserShoot),
            "explosion" => Ok(Self::Explosion),
            "powerup" => Ok(Self::PowerUp),
            "hit" => Ok(Self::HitHurt),
            "hurt" => Ok(Self::HitHurt),
            "jump" => Ok(Self::Jump),
            "select" => Ok(Self::BlipSelect),
            "blip" => Ok(Self::BlipSelect),
            "randomize" => Ok(Self::Randomize),
            _ => Err(format!("Unknown sound type: {}", value)),
        }
    }
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
            SoundType::Randomize => self.randomize(),
        }
    }

    fn coin(&mut self) -> SynthParams {
        let mut params = SynthParams::new();
        params.base_freq = 0.4 + self.frnd(0.5);
        params.env_attack = 0.0;
        params.env_sustain = self.frnd(0.1);
        params.env_decay = 0.1 + self.frnd(0.4);
        params.env_punch = 0.3 + self.frnd(0.3);

        if self.rng.random_bool(0.5) {
            params.arp_speed = 0.5 + self.frnd(0.2);
            params.arp_mod = 0.2 + self.frnd(0.4);
        }
        params
    }

    fn shoot(&mut self) -> SynthParams {
        let mut params = SynthParams::new();

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
            params.base_freq = 0.3 + self.frnd(0.6);
            params.freq_limit = self.frnd(0.1);
            params.freq_ramp = -0.35 - self.frnd(0.3);
        } else {
            params.base_freq = 0.5 + self.frnd(0.5);
            params.freq_limit = (params.base_freq - 0.2 - self.frnd(0.6)).max(0.2);
            params.freq_ramp = -0.15 - self.frnd(0.2);
        }

        if self.rng.random::<bool>() {
            params.duty = self.frnd(0.5);
            params.duty_ramp = self.frnd(0.2);
        } else {
            params.duty = 0.4 + self.frnd(0.5);
            params.duty_ramp = -self.frnd(0.7);
        }

        params.env_attack = 0.0;
        params.env_sustain = 0.1 + self.frnd(0.2);
        params.env_decay = -self.frnd(0.4);

        if self.rng.random::<bool>() {
            params.env_punch = self.frnd(0.3);
        }

        if self.rng.random_ratio(1, 3) {
            params.pha_offset = self.frnd(0.2);
            params.pha_ramp = -self.frnd(0.2);
        }

        if self.rng.random::<bool>() {
            params.hpf_freq = self.frnd(0.3);
        }

        params
    }

    fn explosion(&mut self) -> SynthParams {
        let mut params = SynthParams::new();
        params.wave_type = WaveType::Noise;

        if self.rng.random::<bool>() {
            params.base_freq = 0.1 + self.frnd(0.4);
            params.freq_ramp = -0.1 + self.frnd(0.4);
        } else {
            params.base_freq = 0.2 + self.frnd(0.7);
            params.freq_ramp = -0.2 - self.frnd(0.2);
        }
        params.base_freq = params.base_freq.powf(2.0);

        if self.rng.random_ratio(1, 4) {
            params.freq_ramp = 0.0;
        }

        if self.rng.random_ratio(1, 3) {
            params.repeat_speed = 0.3 + self.frnd(0.5);
        }

        params.env_attack = 0.0;
        params.env_sustain = 0.1 + self.frnd(0.3);
        params.env_decay = self.frnd(0.5);

        if self.rng.random::<bool>() {
            params.pha_offset = -0.3 + self.frnd(0.9);
            params.pha_ramp = -self.frnd(0.3);
        }

        params.env_punch = 0.2 + self.frnd(0.6);

        if self.rng.random::<bool>() {
            params.vib_strength = self.frnd(0.7);
            params.vib_speed = self.frnd(0.6);
        }

        if self.rng.random_ratio(1, 3) {
            params.arp_speed = 0.6 + self.frnd(0.3);
            params.arp_mod = 0.8 - self.frnd(1.6);
        }

        params
    }

    fn powerup(&mut self) -> SynthParams {
        let mut params = SynthParams::new();
        params.env_attack = 0.0;
        params.env_sustain = self.frnd(0.4);
        params.env_decay = 0.1 + self.frnd(0.4);

        if self.rng.random::<bool>() {
            params.wave_type = WaveType::Sawtooth;
        } else {
            params.duty = self.frnd(0.6);
        }

        if self.rng.random::<bool>() {
            params.base_freq = 0.2 + self.frnd(0.3);
            params.freq_ramp = 0.1 + self.frnd(0.4);
            params.repeat_speed = 0.4 + self.frnd(0.4);
        } else {
            params.base_freq = 0.2 + self.frnd(0.3);
            params.freq_ramp = 0.05 + self.frnd(0.2);
            if self.rng.random::<bool>() {
                params.vib_strength = self.frnd(0.7);
                params.vib_speed = self.frnd(0.6);
            }
        }

        params
    }

    fn hit(&mut self) -> SynthParams {
        let waves = [WaveType::Square, WaveType::Sawtooth, WaveType::Noise];
        let mut params = SynthParams::new();
        params.wave_type = *waves.choose(&mut self.rng).unwrap();
        params.base_freq = 0.2 + self.frnd(0.6);
        params.freq_ramp = -0.3 - self.frnd(0.4);
        params.env_attack = 0.0;
        params.env_sustain = self.frnd(0.1);
        params.env_decay = 0.1 + self.frnd(0.2);

        if matches!(params.wave_type, WaveType::Square) {
            params.duty = self.frnd(0.6);
        }

        if self.rng.random::<bool>() {
            params.hpf_freq = self.frnd(0.3);
        }

        params
    }

    fn jump(&mut self) -> SynthParams {
        let mut params = SynthParams::new();
        params.wave_type = WaveType::Square;
        params.duty = self.frnd(0.6);
        params.base_freq = 0.3 + self.frnd(0.3);
        params.freq_ramp = 0.1 + self.frnd(0.2);
        params.env_attack = 0.0;
        params.env_sustain = 0.1 + self.frnd(0.3);
        params.env_decay = 0.1 + self.frnd(0.2);

        if self.rng.random::<bool>() {
            params.hpf_freq = self.frnd(0.3);
        }

        if self.rng.random::<bool>() {
            params.lpf_freq = 1.0 - self.frnd(0.6);
        }

        params
    }

    fn blip(&mut self) -> SynthParams {
        let waves = [WaveType::Square, WaveType::Sawtooth];
        let mut params = SynthParams::new();
        params.wave_type = *waves.choose(&mut self.rng).unwrap();
        params.base_freq = 0.2 + self.frnd(0.4);
        params.env_attack = 0.0;
        params.env_sustain = 0.1 + self.frnd(0.1);
        params.env_decay = self.frnd(0.2);
        params.hpf_freq = 0.1;

        if matches!(params.wave_type, WaveType::Square) {
            params.duty = self.frnd(0.6);
        }

        params
    }

    fn randomize(&mut self) -> SynthParams {
        let waves = [
            WaveType::Square,
            WaveType::Sawtooth,
            WaveType::Sine,
            WaveType::Noise,
        ];
        let mut params = SynthParams::new();
        params.wave_type = *waves.choose(&mut self.rng).unwrap();

        params.base_freq = (self.frnd(2.0) - 1.0).powf(2.0);
        if self.rng.random::<bool>() {
            params.base_freq = (self.frnd(2.0) - 1.0).powf(3.0) + 0.5;
        }

        params.freq_limit = 0.0;
        params.freq_ramp = (self.frnd(2.0) - 1.0).powf(5.0);

        if params.base_freq > 0.7 && params.freq_ramp > 0.2 {
            params.freq_ramp = -params.freq_ramp;
        }
        if params.base_freq < 0.2 && params.freq_ramp < -0.05 {
            params.freq_ramp = -params.freq_ramp;
        }
        params.freq_dramp = (self.frnd(2.0) - 1.0).powf(3.0);

        params.duty = self.frnd(2.0) - 1.0;
        params.duty_ramp = (self.frnd(2.0) - 1.0).powf(3.0);

        params.vib_strength = (self.frnd(2.0) - 1.0).powf(3.0);
        params.vib_speed = self.frnd(2.0) - 1.0;

        params.env_attack = (self.frnd(2.0) - 1.0).powf(3.0);
        params.env_sustain = (self.frnd(2.0) - 1.0).powf(2.0);
        params.env_decay = self.frnd(2.0) - 1.0;
        params.env_punch = self.frnd(0.8).powf(2.0);

        if params.env_attack + params.env_sustain + params.env_decay < 0.2 {
            params.env_sustain += 0.2 + self.frnd(0.3);
            params.env_decay += 0.2 + self.frnd(0.3);
        }

        params.lpf_resonance = self.frnd(2.0) - 1.0;
        params.lpf_freq = 1.0 - self.frnd(1.0).powf(3.0);
        params.lpf_ramp = (self.frnd(2.0) - 1.0).powf(3.0);

        if params.lpf_freq < 0.1 && params.lpf_ramp < -0.05 {
            params.lpf_ramp = -params.lpf_ramp;
        }

        params.hpf_freq = self.frnd(1.0).powf(5.0);
        params.hpf_ramp = (self.frnd(2.0) - 1.0).powf(5.0);

        params.pha_offset = (self.frnd(2.0) - 1.0).powf(3.0);
        params.pha_ramp = (self.frnd(2.0) - 1.0).powf(3.0);

        params.repeat_speed = self.frnd(2.0) - 1.0;
        params.arp_speed = self.frnd(2.0) - 1.0;
        params.arp_mod = self.frnd(2.0) - 1.0;

        params
    }

    fn frnd(&mut self, range: f32) -> f32 {
        self.rng.random::<f32>() * range
    }
}
