use super::params::SynthParams;

use rand::{Rng, SeedableRng, rngs::StdRng};

enum SoundType {
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

impl SynthPreset {
    pub fn new() -> Self {
        SynthPreset {
            rng: StdRng::from_os_rng(),
        }
    }

    pub fn coin(&mut self) -> SynthParams {
        let mut params = SynthParams {
            p_base_freq: 0.4 + self.rng.random::<f32>() * 0.5,
            p_env_attack: 0.0,
            p_env_sustain: self.rng.random::<f32>() * 0.1,
            p_env_decay: 0.1 + self.rng.random::<f32>() * 0.4,
            p_env_punch: 0.3 + self.rng.random::<f32>() * 0.3,
            ..Default::default()
        };

        if self.rng.random_bool(0.5) {
            params.p_arp_speed = 0.5 + self.rng.random::<f32>() * 0.2;
            params.p_arp_mod = 0.2 + self.rng.random::<f32>() * 0.4;
        }
        params
    }
}
