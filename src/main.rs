use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use fundsp::hacker::*;

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()).unwrap(),
    }

    let bpm = 128.0 * 4.0;

    let t0 = 1 as f64 / bpm_hz(bpm);
    let t1 = (1 + 6) as f64 / bpm_hz(bpm);

    let mut sequencer = Sequencer::new(sample_rate, 2);
    sequencer.add64(t0, t1, 0.0, 0.4, Box::new(bassdrum()));
}
