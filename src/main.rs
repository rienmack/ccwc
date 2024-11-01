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
        let line_count = data.lines().count();
        println!("{line_count}");
    }

    Ok(())
}
