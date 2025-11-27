mod synth;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use synth::Synth;
use synth::presets::coin_params;

fn main() -> anyhow::Result<()> {
    let stream = stream_setup()?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(4000));
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

    let params = coin_params();
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
    let length = output.len() / channels;
    let mut buffer = vec![0.0f32; length];

    synth.synth_sample(length, &mut buffer);

    for (i, frame) in output.chunks_mut(channels).enumerate() {
        let value = buffer[i];
        println!("value {value}");

        let value: T = T::from_sample(buffer[i]);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
