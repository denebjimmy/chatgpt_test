use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: reversecat <file>");
            std::process::exit(1);
        }
    };

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()?;

    for line in lines.iter().rev() {
        println!("{}", line);
    }

    Ok(())
}
