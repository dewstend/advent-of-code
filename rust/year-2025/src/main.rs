use std::error::Error;

mod days;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    eprintln!("Usage: cargo run <day>");
    std::process::exit(1);
  }

  match args[1].as_str() {
    "day1" => days::day1::solve(),
    _ => {
      eprintln!("Unknown day");
      std::process::exit(1);
    }
  }
}