use lib::lexer::Lexer;
use std::fs;
use std::io::Write;
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
    let mut readme = String::new();
    let mut output_path: Option<String> = None;
    let mut output_s = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        std::io::stdin()
            .read_to_string(&mut readme)
            .expect("Failed to read from stdin");
    } else {
        readme = read_from_file(&args[1]);

        if args.len() > 2 {
            output_path = Some(args[2].clone());
        }
    }

    let lexer = Lexer::new(readme.as_str());

    let data_file = match &output_path {
        Some(path) => Some(
            fs::OpenOptions::new()
                .write(true)
                .open(path)
                .expect("Did not work!"),
        ),
        None => None,
    };

    for (tok, _) in lexer {
        let (tag, content) = tok.as_tag();
        let line = format!("<{}>{}<{}>\n", tag, content, tag);
        output_s.push_str(&line);
    }

    match data_file {
        Some(mut f) => {
            let _ = f.write(output_s.as_bytes());
        },
        None => {
            println!("{}", output_s);
        }
    }
}
