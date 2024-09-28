use lib::element::{self, Element, TextElement};
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
pub fn cheese() {
    println!("Cheese!");
}

fn main() {
    let mut readme = String::new();
    let mut output_path: Option<String> = None;
    let args: Vec<String> = env::args().collect();
    let mut root = element::Element::Complex(element::ElementWithChildren::new("body"));

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

    for (tok, _) in lexer {
        let (tag, content) = tok.as_tag();
        let text_element = TextElement::new(tag, content);
        let element = Element::Text(text_element);

        root.add_child(element);
    }

    match &output_path {
        Some(path) => {
            let mut data_file = fs::OpenOptions::new()
                .write(true)
                .open(path)
                .expect("Did not work!");

            let _ = data_file.write(root.render().as_bytes());
        }
        _ => {
            println!("{}", root.render());
        }
    };
}
