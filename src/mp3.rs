use crate::iosample::IOSamples;


pub struct Mp3IO ();
impl IOSamples for Mp3IO {
    fn read_samples(&mut self, filepath : &String) -> std::io::Result<Vec<i16> > {
        todo!()
    }
    fn write_samples(&mut self, filepath : &String, output : Vec<i16>) -> std::io::Result<()> {
        todo!()
    }
    fn get_sample_rate(&mut self) -> Option<u32> {
        todo!()
    }
}
