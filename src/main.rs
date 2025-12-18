use std::process;
use std::sync::{Arc, Mutex};

use cpal::traits::StreamTrait;

use refexer::sound::stream_setup;
use refexer::synth::Synth;
use refexer::synth::presets::{SoundType, SynthPreset};

fn main() -> anyhow::Result<()> {
    // parse the command line and get the sound type
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments");
        process::exit(1);
    }

    let sound_type = SoundType::try_from(args[1].as_str()).unwrap_or_default();

    // create the correct prese for the selected sound type
    let mut preset = SynthPreset::new();
    let params = preset.generate(sound_type);

    // create the synth from params and pass it to the stream
    let mut synth = Synth::new(params);
    synth.play_sample();

    let synth_mutex = Arc::new(Mutex::new(synth));
    let stream = stream_setup(Arc::clone(&synth_mutex))?;

    stream.play()?;

    while synth_mutex.lock().unwrap().is_playing() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
