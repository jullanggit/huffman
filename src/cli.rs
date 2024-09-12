#![feature(cow_is_borrowed)]

use std::fs;
use std::io::stdin;

use clap::Parser;
use clap::Subcommand;
use decode::decode;
use encode::encode;

mod bitvec;
mod decode;
mod encode;
mod traits;

#[derive(Parser, Debug)]
#[command(name = "huffman")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        #[arg(short, long)]
        input_file: Option<String>,

        #[arg(short, long)]
        output_file: Option<String>,

        #[arg(short = 'O', long)]
        orig_symbol_size: Option<u8>,
    },
    Decode {
        #[arg(short, long)]
        input_file: Option<String>,

        #[arg(short, long)]
        output_file: Option<String>,
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
            let input_data = read_input(input_file);
            let output_data = encode(input_data, orig_symbol_size.unwrap_or(8), true);

            write_output(output_file, output_data);
        }
        Commands::Decode {
            input_file,
            output_file,
        } => {
            let input_data = read_input(input_file);
            let output_data = decode(input_data);
            write_output(output_file, output_data);
        }
    }
}

fn write_output(output_file: Option<String>, output_data: Vec<u8>) {
    if let Some(output_file) = output_file {
        fs::write(output_file, output_data).unwrap();
    } else {
        let output_string = String::from_utf8_lossy(&output_data);
        println!("{output_string}");

        if output_string.is_owned() {
            println!("NOTE: Some of the encoded data can't be displayed, consider writing to a file instead!");
        }
    }
}

fn read_input(input_file: Option<String>) -> Vec<u8> {
    if let Some(input_file) = input_file {
        fs::read(input_file).unwrap()
    } else {
        let mut buf = String::new();
        stdin().read_line(&mut buf);
        buf.as_bytes().to_vec()
    }
}
