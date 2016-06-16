use lexer::tokenizer::Tokenizer;
use lexer::matcher::Matcher;
use lexer::token::{Token, TokenType};

pub struct Lexer {
    tokenizer: Tokenizer,
    pub matchers: Vec<Box<Matcher>>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut tok = self.match_token().unwrap();

        match tok.tok_type {
            TokenType::EOF => None,
            TokenType::WhiteSpace => {
                tok = self.next().unwrap();
                Some(tok)
            },
            _ => Some(tok),
        }
    }
}

impl Lexer {
    #[allow(dead_code)]
    pub fn new(tok: Tokenizer) -> Lexer {
        Lexer {
            tokenizer: tok,
            matchers: Vec::new(),
        }
    }

    pub fn match_token(&mut self) -> Option<Token> {
        for matcher in &mut self.matchers {
            match self.tokenizer.try_match_token(matcher.as_ref()) {
                Some(tok) => return Some(tok),
                None => continue,
            }
        }
        None
    }
}
