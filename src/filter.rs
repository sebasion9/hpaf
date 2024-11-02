use std::{path::Path, f32::consts::PI, ffi::OsStr, error::Error};


use crate::bridge::{Specs, IOSamples};
pub struct Filter {
    cutoff_freq : u16,
    pub specs : Specs,
}

impl Filter {
    pub fn new(cutoff_freq : u16, specs : Specs) -> Self {
        Self {
            cutoff_freq,
            specs
        }
    }

    pub fn audio_format_supported(&self, filepath : &String) -> Result<AudioFormat, Box <dyn Error>> {
        if let Some(ext) = Path::new(filepath).extension().and_then(OsStr::to_str) {
            let audio_format = AudioFormat::from(ext);
            if audio_format == AudioFormat::NotSupported {
                return Err(Into::into(format!("This type of audio is not supported yet: {}", ext)))
            }
            return Ok(audio_format);
        }
        Err(Into::into("Failed to retrieve the file extension"))
    }
    pub fn apply(&mut self, io_audio : impl IOSamples, filepath : &String, output_path: &String) -> Result<(), Box<dyn Error>> {
        let samples = io_audio.read_samples(filepath, self)?;
        let sample_rate = self.specs.sample_rate as f32;

        let tan = (PI * self.cutoff_freq as f32  / sample_rate).tan();
        let coef = (tan - 1.0) / (tan + 1.0);

        let fs = &samples[0];
        let ls = &samples[samples.len() - 1];
        let mut output : Vec<i16> = vec![*fs];

        let mut prev_s = output[0] as f32;
        let mut prev_output = output[0] as f32;
        for i in 1..samples.len() - 1 {
            let s = samples[i] as f32;
            let processed_sample = coef * (prev_output + s - prev_s);
            prev_output = processed_sample;
            prev_s = s;
            output.push(processed_sample as i16);
        }
        output.push(*ls);

        io_audio.write_samples(output_path, output, self)?;

        Ok(())
    }
}

#[derive(Eq, PartialEq)]
pub enum AudioFormat {
    Wav,
    NotSupported
}
impl From<&str> for AudioFormat {
    fn from(value: &str) -> Self {
        match value {
            "wav" => AudioFormat::Wav,
            _=> AudioFormat::NotSupported
        }
    }
}
