use std::fs;

use clap::Parser;
use clap::Subcommand;
use decode::decode;
use encode::encode;

mod bitvec;
mod decode;
mod encode;

#[derive(Parser, Debug)]
#[command(name = "files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Encode {
        input_file: String,
        output_file: String,
        orig_symbol_size: Option<u8>,
    },
    #[command(arg_required_else_help = true)]
    Decode {
        input_file: String,
        output_file: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Encode {
            input_file,
            output_file,
            orig_symbol_size,
        } => {
            let input_data = fs::read(input_file).unwrap();
            let output_data = encode(input_data, orig_symbol_size.unwrap_or(8), true);
            fs::write(output_file, output_data).unwrap();
        }
        Commands::Decode {
            input_file,
            output_file,
        } => {
            let input_data = fs::read(input_file).unwrap();
            let output_data = decode(input_data);
            fs::write(output_file, output_data).unwrap();
        }
    }
}
