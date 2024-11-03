use clap::Parser;
use std::io;
use std::{fs::File, io::BufReader, io::Read, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file: PathBuf,

    #[arg(short, long, default_value_t = false)]
    count: bool,

    #[arg(short, long, default_value_t = false)]
    length: bool,

    #[arg(short, long, default_value_t = false)]
    word: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.file)?;
    let mut data = BufReader::new(file);

    let mut content = String::new();
    data.read_to_string(&mut content)?;

    let byte_count = content.len();
    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();

    if !args.count && !args.length && !args.word {
        println!("{line_count} {word_count} {byte_count}");
    } else {
        if args.count {
            println!("{byte_count}");
        }

        if args.length {
            println!("{line_count}");
        }

        if args.word {
            println!("{word_count}")
        }
    }
    Ok(())
}
