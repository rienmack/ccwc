use clap::Parser;
use std::io;
use std::{fs::File, io::BufRead, io::BufReader, io::Read, path::PathBuf};

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

    if args.count {
        let count = data.by_ref().bytes().count();
        println!("{count}");
    }

    if args.length {
        let line_count = data.by_ref().lines().count();
        println!("{line_count}");
    }

    if args.word {
        let mut word_count = 0;
        for line in data.lines() {
            word_count += line?.split_whitespace().count();
        }
        println!("{word_count}")
    }

    Ok(())
}
