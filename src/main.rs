use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use trusty_calculator::{parser::parse, walk};
fn main() -> Result<()> {
    let home_dir = dirs::home_dir();
    let history_path = home_dir.map(|mut path| {
        path.push(".trusty-calculator-history.txt");
        path
    });

    let mut rl = Editor::<()>::new();
    if let Some(ref path) = history_path {
        if rl.load_history(&path).is_err() {
            // Do nothing, no history exists yet.
        }
    }

    loop {
        let readline = rl.readline("λ ");
        match readline {
            Ok(input) => {
                if input.trim() == "quit" || input.trim() == "q" {
                    break;
                }

                match parse(&input) {
                    Ok(exp) => {
                        rl.add_history_entry(input.as_str());
                        println!("{}", walk(&exp));
                    }
                    Err(e) => println!("{}", e),
                }
            }

            Err(ReadlineError::Eof) => {
                break;
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }

    if let Some(ref path) = history_path {
        rl.save_history(&path).unwrap();
    }

    Ok(())
}
