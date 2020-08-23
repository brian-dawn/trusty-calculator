use anyhow::Result;
use std::io;
use std::io::Write;
use trusty_calculator::{parser::parse, walk};

fn main() -> Result<()> {
    let stdin = io::stdin();
    loop {
        let mut buf = String::new();
        print!(">");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut buf)?;

        if buf.trim() == "quit" {
            break;
        }

        if let Ok(exp) = parse(&buf) {
            println!("{}", walk(&exp))
        } else {
            println!("What?")
        }
    }

    Ok(())
}
