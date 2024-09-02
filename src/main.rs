use std::io::Read;

use lib::lexer::Lexer;

fn main() {
    let mut buffer = String::new();

    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    let some_lexer = Lexer::new(buffer.as_str());

    for (tok, _) in some_lexer {
        let (tag, content) = tok.as_tag();
        println!("<{}>{}</{}>", tag, content, tag);
    }
}
