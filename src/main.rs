mod args;
mod wav;
mod filter;
mod bridge;
use std::{error::Error, time::Instant};
use clap::Parser;
use crate::{args::Args, bridge::Specs, filter::{Filter, AudioFormat}, wav::WavIO};


fn main() -> Result<(), Box <dyn Error>> {
    let args = Args::parse();
    let input_path = &args.source;
    let output_path = &args.output;
    let mut filter = Filter::new(args.frequency, Specs::default());

    let audio_format = filter.audio_format_supported(input_path)?;


    let now = Instant::now();
    match audio_format {
        AudioFormat::NotSupported => {},
        AudioFormat::Wav => {
            let audio_format_str : &str = audio_format.into();
            println!("hpaf :: Applying filter for '{}' audio", audio_format_str);

            let wav_io = WavIO();
            filter.apply(wav_io, input_path, output_path)?;
        }
    }
    let elapsed = now.elapsed();

    println!("hpaf :: Filter applied in: {:.2?}", elapsed);
    println!("hpaf :: Exiting with success");

    Ok(())
}
