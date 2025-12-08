use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use refexer::synth::Synth;
use refexer::synth::presets::{SoundType, SynthPreset};

fn main() -> anyhow::Result<()> {
    let stream = stream_setup()?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(3000));
    Ok(())
}

fn stream_setup() -> anyhow::Result<cpal::Stream> {
    let (device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into()),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into()),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into()),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}

fn host_device_setup() -> Result<(cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {config:?}");

    Ok((device, config))
}

fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> anyhow::Result<cpal::Stream>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    // let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut preset = SynthPreset::new();
    let params = preset.generate(SoundType::Explosion);

    let mut synth = Synth::new(params);
    synth.play_sample();

    let err_fn = |err| eprintln!("an error occurred on stream: {err}");

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| write_data(data, channels, &mut synth),
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn write_data<T>(output: &mut [T], channels: usize, synth: &mut Synth)
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value = synth.synth_sample().unwrap_or(0.0);

        let value: T = T::from_sample(value);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
