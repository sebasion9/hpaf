mod args;
mod wav;
mod logger;
mod filter;
mod iosample;
mod mp3;
use std::io::Result;
use clap::Parser;
use filter::audio_format_supported;
use crate::{args::{App, Commands}, filter::{Filter, AudioFormat}, wav::WavIO, mp3::Mp3IO, logger::Log};

fn main() -> Result<()> {
    let app = App::parse();
    let logger = Log::new();
    match app.cmd {
        Commands::Convert(args) => {
            let input_path = args.source;
            let output_path = args.output;


            let audio_format = audio_format_supported(&input_path)?;

            match audio_format {
                AudioFormat::NotSupported => {},
                AudioFormat::Wav => {

                    let audio_format_str : &str = audio_format.into();
                    logger.info(format!("Applying filter for '{}' audio", audio_format_str));

                    let wav_io = WavIO::new(input_path, output_path, None);
                    let mut filter = Filter::new(args.frequency, wav_io);
                    filter.apply()?;
                },
                AudioFormat::Mp3 => {
                    let audio_format_str : &str = audio_format.into();
                    logger.info(format!("Applying filter for '{}' audio", audio_format_str));

                    let mp3_io = Mp3IO::new(input_path, output_path, None);
                    let mut filter = Filter::new(args.frequency, mp3_io);
                    filter.apply()?;
                }
            }
        },
        Commands::Stream(args) => {

        }
    }

    logger.info(format!("Exiting with success"));
    Ok(())
}
