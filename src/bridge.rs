use crate::filter::Filter;

use std::error::Error;

pub trait IOSamples {
    fn read_samples(&self, filepath : &String, filter: &mut Filter) -> Result<Vec<i16>, Box<dyn Error>>;
    fn write_samples(&self, filepath : &String, output : Vec<i16>, filter : &mut Filter) -> Result<(), Box<dyn Error>>;
}

pub enum SampleFormat {
    Int,
    Float
}

pub struct Specs {
    pub channels : u16,
    pub sample_rate : u32,
    pub bits_per_sample : u16,
    pub sample_format : SampleFormat
}

impl Default for Specs {
    fn default() -> Self {
        Self {
            bits_per_sample : 0,
            channels : 1,
            sample_format : SampleFormat::Int,
            sample_rate : 44100
        }
    }
}
