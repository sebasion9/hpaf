use std::error::Error;
use hound::{WavSpec,WavReader,WavWriter};
use crate::{bridge::{SampleFormat, Specs, IOSamples}, filter::Filter};

pub struct WavIO ();
impl From<WavSpec> for Specs {
    fn from(value: WavSpec) -> Self {
        Self {
            channels : value.channels,
            sample_rate : value.sample_rate,
            sample_format : value.sample_format.into(),
            bits_per_sample : value.bits_per_sample
        }
    }
}

impl Into<WavSpec> for Specs {
    fn into(self) -> WavSpec {
        WavSpec {
            channels : self.channels,
            sample_rate : self.sample_rate,
            sample_format : self.sample_format.into(),
            bits_per_sample : self.bits_per_sample
        }
    }
}

impl IOSamples for WavIO {
    fn read_samples(&self, filepath : &String, filter : &mut Filter) -> Result<Vec<i16>, Box<dyn Error>> {
        let mut reader = WavReader::open(filepath)?;
        let samples = reader.samples::<i16>();
        let samples_vec : Vec<i16> = samples.map(|s| {
            match s {
                Ok(s) => s,
                Err(e) => panic!("Failed to read sample: {}", e),
            }
        }).collect();
        filter.specs = reader.spec().into();
        Ok(samples_vec)
    }
    fn write_samples(&self, filepath : &String, output : Vec<i16>, filter : &mut Filter) -> Result<(), Box<dyn Error>> {
        let spec; 
        spec = WavSpec {
                channels : filter.specs.channels,
                bits_per_sample : filter.specs.bits_per_sample,
                sample_rate : filter.specs.sample_rate,
                sample_format : SampleFormat::Int.into()
            };

        let mut writer = WavWriter::create(filepath, spec)?;

        for sample in output {
            writer.write_sample(sample)?;
        }
        Ok(())
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
