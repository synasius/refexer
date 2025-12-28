use std::process;
use std::sync::mpsc;

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

    // create the correct preset for the selected sound type
    let mut preset = SynthPreset::new();
    let params = preset.generate(sound_type);

    // create the synth from params and pass it to the stream
    let mut synth = Synth::new(params);
    synth.play_sample();

    let (tx, rx) = mpsc::channel();
    let stream = stream_setup(rx)?;

    stream.play()?;

    // generate sound data and send
    let mut data = Vec::new();
    while let Some(value) = synth.synth_sample() {
        data.push(value);
    }
    let length = data.len();
    tx.send(data)?;

    let secs = length as f32 / 44100.0;
    // wait for the length of the sound
    std::thread::sleep(std::time::Duration::from_secs_f32(secs + 0.1));

    Ok(())
}
