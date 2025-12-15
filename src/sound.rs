use crate::synth::Synth;
use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::{Arc, Mutex};

pub fn stream_setup(synth: Arc<Mutex<Synth>>) -> anyhow::Result<cpal::Stream> {
    let (device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into(), synth),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into(), synth),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into(), synth),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into(), synth),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into(), synth),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into(), synth),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into(), synth),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into(), synth),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into(), synth),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into(), synth),
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
    synth: Arc<Mutex<Synth>>,
) -> anyhow::Result<cpal::Stream>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    // let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    synth.lock().unwrap().play_sample();

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, synth.clone())
        },
        |err| eprintln!("an error occurred on stream: {err}"),
        None,
    )?;

    Ok(stream)
}

fn write_data<T>(output: &mut [T], channels: usize, synth: Arc<Mutex<Synth>>)
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value = synth.lock().unwrap().synth_sample().unwrap_or(0.0);

        let value: T = T::from_sample(value);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
