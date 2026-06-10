use std::io::{BufWriter, Write};
use std::{env, io, process};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: fastbat <filename>");
        process::exit(1);
    }
    let filename = &args[1];

    let mut stream = BufWriter::new(io::stdout().lock());

    match std::fs::read_to_string(filename) {
        Ok(contents) => {
            for (line_number, line) in contents.lines().enumerate() {
                writeln!(stream, "{:>4}: {}", line_number + 1, line)?;
            }
            stream.flush()?;
            Ok(())
        }
        Err(e) => {
            eprintln!("fastbat: {}: {}", filename, e);
            Err(e)
        }
    }
}
