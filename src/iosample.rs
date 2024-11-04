use std::io::Result;


pub trait IOSamples {
    fn read_samples(&mut self, filepath : &String) -> Result<Vec<i16> >;
    fn write_samples(&mut self, filepath : &String, output : Vec<i16>) -> Result<()>;
    fn get_sample_rate(&mut self) -> Option<u32>;
}
