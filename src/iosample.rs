use std::io::Result;


pub trait IOSamples {
    // filepaths may change to some enum/trait depending on what type of system src/out will be
    fn read_samples(&mut self) -> Result<Vec<i16> >;
    fn write_samples(&mut self, output : Vec<i16>) -> Result<()>;
    fn get_sample_rate(&mut self) -> Option<u32>;
}

