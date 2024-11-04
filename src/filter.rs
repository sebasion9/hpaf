use std::{io::{Error, ErrorKind, Result}, path::Path, f32::consts::PI, ffi::OsStr};


use crate::{iosample::IOSamples, logger::Log};
pub struct Filter {
    cutoff_freq : u16,
}

impl Filter {
    pub fn new(cutoff_freq : u16) -> Self {
        Self {
            cutoff_freq,
        }
    }

    pub fn audio_format_supported(&self, filepath : &String) -> Result<AudioFormat> {
        if let Some(ext) = Path::new(filepath).extension().and_then(OsStr::to_str) {
            let audio_format = AudioFormat::from(ext);
            if audio_format == AudioFormat::NotSupported {
                return Err(Error::new(ErrorKind::Unsupported, format!("This type of audio is not supported yet: {}", ext)));
            }
            return Ok(audio_format);
        }
        Err(Error::new(ErrorKind::InvalidInput, "Failed to retrieve the file extension"))
    }

    pub fn apply(&mut self, mut io_audio : impl IOSamples, filepath : &String, output_path: &String) -> Result<()> {
        let mut logger = Log::new();
        logger.time_start();

        let mut samples = io_audio.read_samples(filepath)?;

        logger.time_end();
        logger.benchmark(format!("Reading samples succeded"));

        let sample_rate;
        if let Some(rate) = io_audio.get_sample_rate() {
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

        logger.time_start();
        logger.info(format!("Writing to file at: {}", output_path));
        io_audio.write_samples(output_path, samples)?;

        logger.time_end();
        logger.benchmark(format!("Writing samples succeded"));

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
