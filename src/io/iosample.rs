use std::io::Result;


pub trait IOSamples {
    fn read_samples(&mut self) -> Result<Vec<f32>>;
    fn write_samples(&mut self, output : Vec<f32>) -> Result<()>;
    fn get_sample_rate(&mut self) -> Option<u32>;
}

