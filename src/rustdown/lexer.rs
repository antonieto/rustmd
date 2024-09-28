use std::fmt::Debug;

use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
    H1(String),
    H2(String),
    H3(String),
    Paragraph(String),
    BR,
}

impl Token {
    pub fn as_tag(&self) -> (String, String) {
        match self {
            Token::H1(v) => (String::from("h1"), v.clone()),
            Token::H2(v) => (String::from("h2"), v.clone()),
            Token::H3(v) => (String::from("h3"), v.clone()),
            Token::Paragraph(v) => (String::from("p"), v.clone()),
            Token::BR => (String::from("br"), String::new()),
        }
    }
}

lexer! {
    fn next_token(text: 'a) -> Token;

    // H3 Tokens
    r#"### ?[^\r\n]*[\r\n]"# => {
        let mut skipped = text.get(3..).expect("Failed to get value");
        skipped = skipped.get(..skipped.len() - 1).expect("Failed to split");
        Token::H3(String::from(skipped))
    }

    // H2 tokens
    r#"## [^\r\n]*[\r\n]"# => {
        let mut skipped = text.get(2..).expect("Failed to get value");
        skipped = skipped.get(..skipped.len() - 1).expect("Failed to split");
        Token::H2(String::from(skipped))
    }

    // H1 tokens
    r#"# [^\r\n]*[\r\n]"# => {
        let mut skipped = text.get(1..).expect("Failed to get value");
        skipped = skipped.get(..skipped.len() - 1).expect("Failed to split");
        Token::H1(String::from(skipped))
    }

    r#"[^\r\n]+[\r\n]"# => {
        let skipped = text.get(..text.len() - 1).expect("Failed to split");
        Token::Paragraph(String::from(skipped))
    }

    r#"."# => Token::BR,
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

// Used to keep track of string slice indices
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };

            return Some((tok, span));
        }
    }
}
