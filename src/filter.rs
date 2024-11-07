use std::{io::{Error, ErrorKind, Result}, path::Path, f32::consts::PI, ffi::OsStr};


use crate::{iosample::IOSamples, logger::Log};
pub struct Filter<T : IOSamples> {
    cutoff_freq : u16,
    io_samples : T
}

impl<T : IOSamples> Filter<T> {
    pub fn new(cutoff_freq : u16, io_samples : T) -> Self {
        Self {
            cutoff_freq,
            io_samples
        }
    }

    pub fn apply(&mut self) -> Result<()> {
        let mut logger = Log::new();

        let mut samples = self.io_samples.read_samples()?;

        let sample_rate;
        if let Some(rate) = self.io_samples.get_sample_rate() {
            sample_rate = rate as f32; 
        }
        else {
            return Err(Error::new(ErrorKind::InvalidData, "Failed to retrieve sample rate"));
        }

        logger.time_start();
        logger.info(format!("Processing samples now"));

        let tan = (PI * self.cutoff_freq as f32  / sample_rate).tan();
        let coef = (tan - 1.0) / (tan + 1.0);

        let mut prev_s = samples[0] as f32;
        let mut prev_output = samples[0] as f32;

        for i in 1..samples.len() - 1 {
            let s = samples[i] as f32;
            let processed_sample = coef * (prev_output + s - prev_s);
            prev_output = processed_sample;
            prev_s = s;
            samples[i] = processed_sample as i16;
        }

        logger.time_end();
        logger.benchmark(format!("Filter applied"));

        self.io_samples.write_samples(samples)?;


        Ok(())
    }
}

#[derive(Eq, PartialEq)]
pub enum AudioFormat {
    Wav,
    Mp3,
    NotSupported
}
impl From<&str> for AudioFormat {
    fn from(value: &str) -> Self {
        match value {
            "wav" => AudioFormat::Wav,
            "mp3" => AudioFormat::Mp3,
            _=> AudioFormat::NotSupported
        }
    }
}
impl Into<&str> for AudioFormat {
    fn into(self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Mp3=> "mp3",
            Self::NotSupported => "audio format not supported"
        }
    }
}

pub fn audio_format_supported(filepath : &String) -> Result<AudioFormat> {
    if let Some(ext) = Path::new(filepath).extension().and_then(OsStr::to_str) {
        let audio_format = AudioFormat::from(ext);
        if audio_format == AudioFormat::NotSupported {
            return Err(Error::new(ErrorKind::Unsupported, format!("This type of audio is not supported yet: {}", ext)));
        }
        return Ok(audio_format);
    }
    Err(Error::new(ErrorKind::InvalidInput, "Failed to retrieve the file extension"))
}
