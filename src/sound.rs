use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::mpsc::Receiver;

pub fn stream_setup(
    receiver: Receiver<Vec<f32>>,
) -> anyhow::Result<(cpal::Stream, cpal::SampleRate)> {
    let (device, config) = host_device_setup()?;

    let sample_rate = config.sample_rate();
    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into(), receiver),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into(), receiver),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into(), receiver),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into(), receiver),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into(), receiver),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into(), receiver),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into(), receiver),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into(), receiver),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into(), receiver),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into(), receiver),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }?;

    Ok((stream, sample_rate))
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
    receiver: Receiver<Vec<f32>>,
) -> anyhow::Result<cpal::Stream>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    // let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;
    let mut synth = SynthReceiver {
        receiver,
        data: Vec::new(),
        index: 0,
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| write_data(data, channels, &mut synth),
        |err| eprintln!("an error occurred on stream: {err}"),
        None,
    )?;

    Ok(stream)
}

fn write_data<T>(output: &mut [T], channels: usize, synth: &mut SynthReceiver)
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value = synth.synth_sample();

        let value: T = T::from_sample(value);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

struct SynthReceiver {
    receiver: Receiver<Vec<f32>>,
    data: Vec<f32>,
    index: usize,
}

impl SynthReceiver {
    pub fn synth_sample(&mut self) -> f32 {
        // receive new data if there is
        let result = self.receiver.try_recv();
        if let Ok(result) = result {
            self.index = 0;
            self.data = result;
        }

        // consume the data buffer
        let mut value = 0.0;
        if self.index < self.data.len() {
            value = self.data[self.index];
            self.index += 1;
        }

        value
    }
}
