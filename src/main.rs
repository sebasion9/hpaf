mod args;
mod wav;
mod filter;
mod iosample;
mod mp3;
use std::io::Result;
use clap::Parser;
use crate::{args::Args, filter::{Filter, AudioFormat}, wav::WavIO, mp3::Mp3IO};


fn main() -> Result<()> {
    let args = Args::parse();
    let input_path = &args.source;
    let output_path = &args.output;
    let mut filter = Filter::new(args.frequency);

    let audio_format = filter.audio_format_supported(input_path)?;


    match audio_format {
        AudioFormat::NotSupported => {},
        AudioFormat::Wav => {
            let audio_format_str : &str = audio_format.into();
            println!("hpaf :: Applying filter for '{}' audio", audio_format_str);

            let wav_io = WavIO { spec : None };
            filter.apply(wav_io, input_path, output_path)?;
        },
        AudioFormat::Mp3 => {
            let audio_format_str : &str = audio_format.into();
            println!("hpaf :: Applying filter for '{}' audio", audio_format_str);

            let mp3_io = Mp3IO { spec : None };
            filter.apply(mp3_io, input_path, output_path)?;
        }
    }

    println!("hpaf :: Exiting with success");

    Ok(())
}
