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

pub fn coin_params() -> SynthParams {
    let mut rng = StdRng::from_os_rng();

    let mut params = SynthParams {
        p_base_freq: 0.4 + rng.random::<f32>() * 0.5,
        p_env_attack: 0.0,
        p_env_sustain: rng.random::<f32>() * 0.1,
        p_env_decay: 0.1 + rng.random::<f32>() * 0.4,
        p_env_punch: 0.3 + rng.random::<f32>() * 0.3,
        ..Default::default()
    };

    if rng.random_bool(0.5) {
        params.p_arp_speed = 0.5 + rng.random::<f32>() * 0.2;
        params.p_arp_mod = 0.2 + rng.random::<f32>() * 0.4;
    }
    params
}
