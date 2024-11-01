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
    let mut data = BufReader::new(File::open(&args.file)?);

    if args.count {
        let mut count = 0;
        for _ in data.by_ref().bytes() {
            count += 1
        }
        println!("{count}");
    }

    if args.length {
        let line_count = data.by_ref().lines().count();
        println!("{line_count}");
    }

    Ok(())
}
