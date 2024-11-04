use std::io::Error;
use hound::{WavSpec,WavReader,WavWriter};
use crate::iosample::{SampleFormat, IOSamples};

pub struct WavIO {
    pub spec: Option<WavSpec>
}

impl IOSamples for WavIO {
    fn read_samples(&mut self, filepath : &String) -> Result<Vec<i16>, Error> {
        let mut reader = WavReader::open(filepath).expect("Failed to open '.wav' file");
        let samples = reader.samples::<i16>();
        let samples_vec : Vec<i16> = samples.map(|s| {
            match s {
                Ok(s) => s,
                Err(e) => panic!("Failed to read sample: {}", e),
            }
        }).collect();
        self.spec = Some(reader.spec().into());
        Ok(samples_vec)
    }
    fn write_samples(&mut self, filepath : &String, output : Vec<i16>) -> Result<(), Error> {
        let spec;
        if let Some(sp) = self.spec {
            spec = sp;
        }
        else {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Unitialized spec in .wav"));
        }
        let mut writer = WavWriter::create(filepath, spec).expect("Failed to create '.wav' file");

        for sample in output {
            writer.write_sample(sample).expect("Failed to write sample to output");
        }
        Ok(())
    }
    fn get_sample_rate(&mut self) -> Option<u32> {
        if let Some(spec) = self.spec {
            return Some(spec.sample_rate)
        }
        else {
            return None
        }
    }
}

impl Into<hound::SampleFormat> for SampleFormat {
    fn into(self) -> hound::SampleFormat {
        match self {
            Self::Int => hound::SampleFormat::Int,
            Self::Float => hound::SampleFormat::Float,
        }
    }
}
impl From<hound::SampleFormat> for SampleFormat {
    fn from(value: hound::SampleFormat) -> Self {
        match value {
            hound::SampleFormat::Int => Self::Int,
            hound::SampleFormat::Float => Self::Float,
        }
    }
}
