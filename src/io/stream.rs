use std::error::Error;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device, Host, StreamConfig};
use ringbuf::{HeapRb,
    traits::{Consumer, Producer, Split}
};

use crate::util::args::StreamArgs;

pub struct StreamIO {
    input_dev : String,
    output_dev : String,
    frequency : u16,
    latency : f32,
}
impl StreamIO {
    pub fn from_stream_args(args : StreamArgs) -> Result<Self, Box<dyn Error>> {
        let host = cpal::default_host();
        let input_dev = if args.input_dev == "default" {
            host.default_input_device()
        }
        else {
            host.input_devices()?.find(|x| x.name().map(|y| y == args.input_dev).unwrap_or(false))
        }.expect("Failed to find input device");

        let output_dev = if args.output_dev == "default" {
            host.default_output_device()
        }
        else {
            host.output_devices()?.find(|x| x.name().map(|y| y == args.output_dev).unwrap_or(false))
        }.expect("Failed to find output device");

        let config : StreamConfig = input_dev.default_input_config()?.into();

        let latency_frames = (args.latency / 1000.0) * config.sample_rate.0 as f32;
        let latency_samples = latency_frames as usize * config.channels as usize;

        let ring = HeapRb::<f32>::new(latency_samples * 2);
        let (mut producer, mut consumer) = ring.split();

        for _ in 0..latency_samples {
            producer.try_push(0.0).unwrap();
        }
        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.try_push(sample).is_err() {
                    output_fell_behind = true;
                }
            }
            if output_fell_behind {
                eprintln!("Output stream fell behind: try increasing latency");
            }
        };

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.try_pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                }
            }
            if input_fell_behind {
                eprintln!("Input stream fell behind: try increasing latency");
            }
        };
        println!("Attempting to build both streams with f32 samples and `{:?}`.",config);

        let input_stream = input_dev.build_input_stream(&config, input_data_fn, Self::err_fn, None)?;
        let output_stream = output_dev.build_output_stream(&config, output_data_fn, Self::err_fn, None)?;
        println!("Successfully built streams.");

        // Play the streams.
        println!(
            "Starting the input and output streams with `{}` milliseconds of latency.",
            args.latency
            );
        input_stream.play()?;
        output_stream.play()?;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        drop(input_stream);
        drop(output_stream);

        Ok(Self {
            input_dev : args.input_dev,
            output_dev : args.output_dev,
            frequency : args.frequency,
            latency : args.latency,
        })
    }

    fn err_fn(err: cpal::StreamError) {
        eprintln!("An error occured on stream: {}", err)
    }
}
