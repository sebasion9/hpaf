use mp3lame_encoder::{Builder, DualPcm,FlushNoGap};
use symphonia::core::{audio::SampleBuffer, codecs::{DecoderOptions, CODEC_TYPE_NULL}, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint};
use symphonia::core::errors::Error;
use crate::{iosample::IOSamples, logger::Log};
use std::{fs::File, io::Write};

pub struct Mp3Spec {
    channels : u32,
    sample_rate : u32,
}

pub struct Mp3IO {
    spec : Option<Mp3Spec>,
    input_path : String,
    output_path : String
}
impl Mp3IO {
    pub fn new(input_path : String, output_path : String, spec : Option<Mp3Spec>) -> Self {
        Self {
            input_path,
            output_path,
            spec
        }
    }
}

#[allow(unused_variables)]
impl IOSamples for Mp3IO {
    // reference https://github.com/pdeljanov/Symphonia/blob/master/symphonia/examples/getting-started.rs
    fn read_samples(&mut self) -> std::io::Result<Vec<i16> > {
        let logger = Log::new();
        let mut samples : Vec<i16> = vec![];

        logger.info(format!("Prepare for decoding mp3"));

        let src = File::open(&self.input_path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("mp3");

        let meta_opts = MetadataOptions::default();
        let fmt_opts = FormatOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("Unsupported format");

        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("No supported audio tracks");

        self.spec = Some(Mp3Spec { 
            channels : track.codec_params.channels.unwrap().count() as u32,
            sample_rate : track.codec_params.sample_rate.unwrap_or(44100),
        });

        let dec_opts = DecoderOptions::default();

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("Unsupported codec");

        let track_id = track.id;

        let mut sample_count = 0;
        let mut sample_buf = None;
        logger.info(format!("Decoding mp3 audio"));

        // decode loop
        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(err) => {
                    // unrecoverable error, halt decoding
                    break;
                }
            };
            while !format.metadata().is_latest() {
                format.metadata().pop();
            }
            if packet.track_id() != track_id {
                continue;
            }
            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    if sample_buf.is_none() {
                        let spec = *audio_buf.spec();
                        let dur = audio_buf.capacity() as u64;
                        sample_buf = Some(SampleBuffer::<i16>::new(dur, spec));
                    }
                    if let Some(buf) = &mut sample_buf {
                        buf.copy_interleaved_ref(audio_buf);

                        sample_count += buf.samples().len();
                        let mut buf_samples = buf.samples().to_vec();
                        samples.append(&mut buf_samples);
                    }

                }
                Err(Error::IoError(_)) => {
                    logger.error(format!("Failed to decode packet due to an IO error, skipping the packet"));
                    continue;
                }
                Err(Error::DecodeError(_)) => {
                    logger.error(format!("Failed to decode packet due to invalid data, skipping the packet"));
                    continue;
                }
                Err(e) => {
                    // unrecoverable error, halt decoding
                    logger.error(format!("Ran into unrecoverable error: {}",e));
                    break;
                }
            }
        };
        return Ok(samples);
    }

    // reference https://github.com/DoumanAsh/mp3lame-encoder?tab=readme-ov-file#example
    fn write_samples(&mut self, output : Vec<i16>) -> std::io::Result<()> {
        let mp3_spec;
        if let Some(spec) = &self.spec {
            mp3_spec = spec;
        }
        else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unitialized spec in .mp3"));
        }
        let mut mp3_encoder = Builder::new().expect("Failed to create LAME builder");
        mp3_encoder.set_num_channels(mp3_spec.channels as u8).expect("Failed to set channels");
        mp3_encoder.set_sample_rate(mp3_spec.sample_rate).expect("Failed to set sample rate");
        mp3_encoder.set_brate(mp3lame_encoder::Birtate::Kbps192).expect("Failed to set sample rate");
        mp3_encoder.set_quality(mp3lame_encoder::Quality::Best).expect("Failed to set sample rate");
        let mut mp3_encoder = mp3_encoder.build().expect("Failed to initialize LAME encoder");

        let left_channel: Vec<i16> = output.iter().step_by(2).cloned().collect();
        let right_channel: Vec<i16> = output.iter().skip(1).step_by(2).cloned().collect();

        let input = DualPcm {
            left : &left_channel,
            right : &right_channel
        };

        let mut mp3_out_buffer = Vec::new();
        mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(input.left.len()));
        let encoded_size = mp3_encoder.encode(input, mp3_out_buffer.spare_capacity_mut()).expect("Failed to encode mp3");
        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }
        let encoded_size = mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut()).expect("Failed to flush");

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        let mut file = File::create(&self.output_path)?;
        file.write_all(&mp3_out_buffer)?;

        Ok(())
    }
    fn get_sample_rate(&mut self) -> Option<u32> {
        if let Some(spec) = &self.spec {
            return Some(spec.sample_rate)
        }
        else {
            return None
        }
    }
}

