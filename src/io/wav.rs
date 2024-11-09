use std::io::Error;
use hound::{WavSpec,WavReader,WavWriter};
use crate::io::iosample::IOSamples;

pub struct WavIO {
    spec: Option<WavSpec>,
    input_path : String,
    output_path : String
}
impl WavIO {
    pub fn new(input_path : String, output_path : String, spec : Option<WavSpec>) -> Self {
        Self {
            input_path,
            output_path,
            spec
        }
    }
}

impl IOSamples for WavIO {
    fn read_samples(&mut self) -> Result<Vec<i16>, Error> {
        let mut reader = WavReader::open(&self.input_path).expect("Failed to open '.wav' file");
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
    fn write_samples(&mut self, output : Vec<i16>) -> Result<(), Error> {
        let spec;
        if let Some(sp) = self.spec {
            spec = sp;
        }
        else {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Unitialized spec in .wav"));
        }
        let mut writer = WavWriter::create(&self.output_path, spec).expect("Failed to create '.wav' file");

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

