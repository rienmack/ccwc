use clap::Parser;
use std::io;
use std::{fs::File, io::BufReader, io::Read, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file: PathBuf,

    #[arg(short, long, default_value_t = false)]
    count: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let data = BufReader::new(File::open(&args.file)?);

    if args.count {
        let mut count = 0;
        for _ in data.bytes() {
            count += 1
        }
        println!("{count}");
    }

    Ok(())
}
