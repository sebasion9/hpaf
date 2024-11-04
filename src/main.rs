mod args;
mod wav;
mod logger;
mod filter;
mod iosample;
mod mp3;
use std::io::Result;
use clap::Parser;
use crate::{args::Args, filter::{Filter, AudioFormat}, wav::WavIO, mp3::Mp3IO, logger::Log};


fn main() -> Result<()> {
    let args = Args::parse();
    let input_path = &args.source;
    let output_path = &args.output;

    let logger = Log::new();

    let mut filter = Filter::new(args.frequency);
    let audio_format = filter.audio_format_supported(input_path)?;


    match audio_format {
        AudioFormat::NotSupported => {},
        AudioFormat::Wav => {
            let audio_format_str : &str = audio_format.into();
            logger.info(format!("Applying filter for '{}' audio", audio_format_str));

            let wav_io = WavIO { spec : None };
            filter.apply(wav_io, input_path, output_path)?;
        },
        AudioFormat::Mp3 => {
            let audio_format_str : &str = audio_format.into();
            logger.info(format!("Applying filter for '{}' audio", audio_format_str));

            let mp3_io = Mp3IO { spec : None };
            filter.apply(mp3_io, input_path, output_path)?;
        }
    }

    logger.info(format!("Exiting with success"));
    Ok(())
}
