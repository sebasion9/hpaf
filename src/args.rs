use clap::Parser;

/// Piece of software, that applies high-pass filter for audio files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Source audio file path
    #[arg(short, long)]
    pub source: String,

    /// Cutoff frequency in Hz
    #[arg(short, long, default_value_t = 100)]
    pub frequency: u16,

    /// Destination audio file path
    #[arg(short, long)]
    pub output: String,
}


