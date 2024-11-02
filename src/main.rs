use anyhow::Result;
use cpal::traits::*;

pub struct FMSynthesizer {
    carrier_freq: f32,
    modulator_freq: f32,
    modulation_index: f32,
    sample_rate: f32,
    time: f32,
}

impl FMSynthesizer {
    pub fn new(
        carrier_freq: f32,
        modulator_freq: f32,
        modulation_index: f32,
        sample_rate: f32,
    ) -> Self {
        FMSynthesizer {
            carrier_freq,
            modulator_freq,
            modulation_index,
            sample_rate,
            time: 0.0,
        }
    }

    pub fn generate_sample(&mut self) -> f32 {
        self.time += 1.0 / self.sample_rate;
        let carrier = 2.0 * std::f32::consts::PI * self.carrier_freq * self.time;
        let modulator = 2.0 * std::f32::consts::PI * self.modulator_freq * self.time;
        (carrier + self.modulation_index * modulator.sin()).sin()
    }
}

fn main() -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device
        .default_output_config()
        .expect("failed to get default output config");

    match config.sample_format() {
        cpal::SampleFormat::F32 => {
            let mut synthesizer =
                FMSynthesizer::new(440.0, 110.0, 1.0, config.sample_rate().0 as f32);
            let stream = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _| write_data(data, &mut synthesizer),
                move |err| eprintln!("an error occurred on the output audio stream: {}", err),
            )?;
            stream.play()?;
            std::thread::sleep(std::time::Duration::from_secs(5));
            Ok(())
        }
        _ => panic!("unsupported sample format"),
    }
}

fn write_data<T>(output: &mut [T], synthesizer: &mut FMSynthesizer)
where
    T: cpal::Sample,
{
    for sample in output.iter_mut() {
        *sample = cpal::Sample::from(&synthesizer.generate_sample());
    }
}
