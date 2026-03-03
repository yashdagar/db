use std::io::{Write, stdin, stdout};
use clearscreen::clear;

fn main() {
    let _ = clear();
    println!("Simple Rust DB (v0.1.0)");
    println!("");
    println!("Type 'help' or '\\h' for help. Type 'clear' or '\\c' to clear the current input statement.");
    println!("");

    loop {
        print!("rdb> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input.");
            continue;
        }

        let input = input.trim();

        match input {
            "\\q" | "\\e" | "quit" | "exit" | "bye" => {
                println!("Bye!");
                break;
            }
            "\\?" | "?" | "\\h" | "help" => {
                println!("");
                println!("List of all RDB commands:");
                println!("?     (\\?) - Synonym for 'help'.");
                println!("clear (\\c) - Clear the screen.");
                println!("exit  (\\e) - Exit RDB.");
                println!("help  (\\h) - Show this help message.");
                println!("quit  (\\q) - Quit RDB, same as exit.");
                println!("");
            }
            "\\c" | "clear" => {
                let _ = clear();
            }
            _ => {}
        }

        // just make a dbms here now :)
    }
}
