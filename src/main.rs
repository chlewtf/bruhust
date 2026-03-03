mod interpreter;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("bruhust v0.1.0 - no cap fr fr");
        eprintln!("Usage: bruhust <file.bruh>");
        eprintln!("       bruhust --repl");
        eprintln!();
        eprintln!("Keyword cheat sheet:");
        eprintln!("  yeet <val>              => print");
        eprintln!("  no_cap x be 42          => let x = 42");
        eprintln!("  lowkey x be 0           => var x = 0 (mutable)");
        eprintln!("  hits_diff x be 10       => x = 10 (reassign)");
        eprintln!("  fr_fr <cond> {{ }}       => if");
        eprintln!("  nah {{ }}                => else");
        eprintln!("  slay <cond> {{ }}        => while");
        eprintln!("  sus fn(a,b) {{ bet a }}  => function def");
        eprintln!("  rizz fn(1,2)            => call function");
        eprintln!("  bussin / mid            => true / false");
        eprintln!("  ghosted                 => break");
        eprintln!("  periodt                 => continue");
        eprintln!("  sheesh(0,5)             => [0,1,2,3,4]");
        eprintln!("  ratio                   => read input");
        process::exit(1);
    }

    if args[1] == "--repl" {
        run_repl();
        return;
    }

    let path = &args[1];
    if !path.ends_with(".bruh") {
        eprintln!("bestie that ain't a .bruh file 💀");
        process::exit(1);
    }

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't read {}: {}", path, e);
            process::exit(1);
        }
    };

    run_source(&source, path);
}

fn run_source(source: &str, filename: &str) {
    let tokens = match lexer::tokenize(source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[bruhust lexer L] {}: {}", filename, e);
            process::exit(1);
        }
    };

    let ast = match parser::parse(tokens) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[bruhust parser 💀] {}: {}", filename, e);
            process::exit(1);
        }
    };

    let mut interp = interpreter::Interpreter::new();
    if let Err(e) = interp.run(&ast) {
        eprintln!("[bruhust runtime 🚨] {}: {}", filename, e);
        process::exit(1);
    }
}

fn run_repl() {
    use std::io::{self, BufRead, Write};
    println!("bruhust REPL v0.1.0 - type 'gg' to exit 🔥");
    let stdin = io::stdin();
    let mut interp = interpreter::Interpreter::new();
    loop {
        print!("bruh> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() || line.trim() == "gg" {
            println!("gg no re 👋");
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        let tokens = match lexer::tokenize(&line) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("lexer L: {}", e);
                continue;
            }
        };
        let ast = match parser::parse(tokens) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("parser 💀: {}", e);
                continue;
            }
        };
        if let Err(e) = interp.run(&ast) {
            eprintln!("runtime 🚨: {}", e);
        }
    }
}
