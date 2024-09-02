use lib::lexer::Lexer;
use std::fs;
use std::{env, io::Read};

fn read_from_file(path: &String) -> String {
    let contents = fs::read_to_string(path);
    match contents {
        Err(_) => {
            panic!("Could not read file at path: {}", path)
        }
        Ok(content) => content,
    }
}

fn main() {
    // TODO: read input and output from command line arguments

    let mut readme = String::new();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            std::io::stdin()
                .read_to_string(&mut readme)
                .expect("Failed to read from stdin");
        }
        2 => {
            readme = read_from_file(&args[1]);
        }
        // This might cause an error
        _ => {}
    }

    let some_lexer = Lexer::new(readme.as_str());

    for (tok, _) in some_lexer {
        let (tag, content) = tok.as_tag();
        println!("<{}>{}</{}>", tag, content, tag);
    }
}
