// cli app
use std::io;
use std::io::Write;
use std::collections::HashMap;

enum Command {
    Help,
    List,
    Quit,
    Search,
    Invalid,
}

fn parse_command(input: &str) -> Command {
    match input {
        "h" | "help" | "killme" => Command::Help,
        "list" => Command::List,
        "exit" | "quit" => Command::Quit,
        "search" => Command::Search,
        _ => Command::Invalid,
    }
}

pub fn main() {
    println!("==== vocabulary app v1.0 ====");

    // hashmap from tuples
    let vocabulary_list: HashMap<&str, &str> = [
        ("sonder", "realization that everyone has a story"),
        ("eloquent", "able to write skillfully"),
    ]
    .into_iter()
    .collect();

    loop {
        print!("> ");

        // flush for skipping buffering to terminal, unwrap for err handling
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");

        let parts: Vec<&str> = user_input.trim().split_whitespace().collect();

        let command = parse_command(parts[0]);

        match command {
            Command::Help => {
                println!("available commands: ");
                println!("list: lists all words in dictionary with meanings\n search <word>: search word in dictionary\n quit/exit: quit\n h/help/killme: help")
            },
            Command::List => {
                // println!("show words in dict!");

                for (word, meaning) in vocabulary_list.iter() {
                    println!("{} : {}", word, meaning);
                }
            },
            Command::Quit => break,
            Command::Search => {

                match parts.len() {
                    1 => println!("please provide a word to search"),
                    2 => {
                        // do the search
                        let target_word = parts[1];

                        if vocabulary_list.contains_key(target_word) {
                            println!("{}: {}", target_word, vocabulary_list[target_word]);
                        } else {
                            println!("'{}' not found, please try again", target_word);
                        }
                    }
                    _ => println!("please provide just one word for searching"),
                }
            },
            Command::Invalid => println!("unknown command, try 'list'?"),
        }
    }
}
