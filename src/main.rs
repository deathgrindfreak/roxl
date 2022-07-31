extern crate rlox;

use std::io::Result;
use rlox::chunk::{Chunk, OpCode};
use rlox::vm::VM;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result as RLResult};

fn main()  {
    let mut args = std::env::args();
    if args.len() == 1 {
        if repl().is_err() {
            eprintln!("Could not instantiate repl!");
            std::process::exit(74);
        }
    } else if args.len() == 2 {
        let file_name = args.nth(1).unwrap();
        if run_file(&file_name).is_err() {
            eprintln!("Could not run file {}", file_name);
            std::process::exit(74);
        }
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }
}

fn run_file(file_name: &str) -> Result<()> {
    std::fs::File::open(file_name)?;
    Ok(())
}

fn repl() -> RLResult<()> {
    let mut rl = Editor::<()>::new()?;

    println!("Welcome to lox.");

    loop {
        match rl.readline("> ") {
            Ok(l) => {
                rl.add_history_entry(l.as_str());
                println!("{}", l);
            },
            Err(ReadlineError::Eof) => {
                std::process::exit(0);
            },
            Err(ReadlineError::Interrupted) => {
                // TODO Will probably clear a buffer full of input here (multi-line input mode)
            },
            Err(err) => eprintln!("{:?}", err),
        };
    }
}
