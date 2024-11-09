use clap::{Parser, Subcommand, Args};

/// Piece of software, that applies high-pass filter for audio files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(name = "hpaf", version)]
pub struct App {
    #[command(subcommand)]
    pub cmd : Commands,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Use to convert from input audio file to output audio file
    Convert(ConvertArgs),
    /// Use to apply filter for pipeline between audio source and sink
    Stream(StreamArgs)
}


#[derive(Debug, Args)]
pub struct ConvertArgs {
    /// Source audio file path
    #[arg(short = 's', long)]
    pub source: String,
    /// Destination audio file path
    #[arg(short = 'o', long)]
    pub output: String,
    /// Cutoff frequency in Hz
    #[arg(short = 'f', long, required = true)]
    pub frequency : u16,
}

#[derive(Debug, Args)]
pub struct StreamArgs {
    /// Input device to use
    #[arg(short, long, value_name = "IN", default_value_t = String::from("default"))]
    pub input_dev: String,
    /// Output device to use
    #[arg(short, long, value_name = "OUT", default_value_t = String::from("default"))]
    pub output_dev : String,
    /// Cutoff frequency in Hz
    #[arg(short = 'f', long, required = true)]
    pub frequency : u16,
    /// Specify the delay between input and output
    #[arg(short, long, value_name = "DELAY_MS")]
    pub latency : f32,
}

