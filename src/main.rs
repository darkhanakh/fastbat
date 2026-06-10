use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: fastbat <filename>");
        return;
    }
    let filename = &args[1];
    match std::fs::read_to_string(filename) {
        Ok(contents) => {
            for line in contents.lines() {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("fastbat: {}: {}", filename, e);
            std::process::exit(1);
        }
    }
}
