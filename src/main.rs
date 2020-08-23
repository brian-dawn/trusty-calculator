use anyhow::Result;
use std::io;
use std::io::Write;
use trusty_calculator::{parser::parse, walk};

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut input = String::new();

        print!(">");
        stdout.flush().unwrap();

        stdin.read_line(&mut input)?;

        if input.trim() == "quit" {
            break;
        }

        match parse(&input) {
            Ok(exp) => println!("{}", walk(&exp)),
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}
