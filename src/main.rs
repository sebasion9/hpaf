mod io;
mod util;
mod filter;

use std::io::Result;
use clap::Parser;

use filter::{audio_format_supported, AudioFormat, Filter};
use io::{mp3::Mp3IO, wav::WavIO};
use util::{args::{App, Commands}, logger::Log};

fn main() -> Result<()> {
    let app = App::parse();
    let logger = Log::new();
    match app.cmd {
        Commands::Convert(args) => {
            let input_path = args.source;
            let output_path = args.output;

            let audio_format = audio_format_supported(&input_path)?;
            let audio_format_str : &str = audio_format.into();
            logger.info(format!("Convert mode"));
            logger.info(format!("Applying filter for '{}' audio", audio_format_str));

            match audio_format {
                AudioFormat::NotSupported => {},
                AudioFormat::Wav => {

                    let wav_io = WavIO::new(input_path, output_path, None);
                    let mut filter = Filter::new(args.frequency, wav_io);
                    filter.convert()?;
                },
                AudioFormat::Mp3 => {

                    let mp3_io = Mp3IO::new(input_path, output_path, None);
                    let mut filter = Filter::new(args.frequency, mp3_io);
                    filter.convert()?;
                }
            }
        },
        Commands::Stream(_args) => {
            todo!()
        }
    }

    logger.info(format!("Exiting with success"));
    Ok(())
}
