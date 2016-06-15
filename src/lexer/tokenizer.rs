use lexer::token::{Token, TokenType};
use lexer::matcher::Matcher;

#[derive(Clone)]
struct Snapshot {
    index: usize,
    line: u32,
    column: u32,
}

#[derive(Clone)]
pub struct Tokenizer {
    index: usize,
    pub line: u32,
    pub column: u32,
    items: Vec<char>,
    snapshots: Vec<Snapshot>,
}

impl Iterator for Tokenizer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}

impl Tokenizer {
    #[allow(dead_code)]
    pub fn new(items: &mut Iterator<Item = char>) -> Tokenizer {
        Tokenizer {
            index: 0,
            line: 0,
            column: 0,
            items: items.collect(),
            snapshots: Vec::new(),
        }
    }

    pub fn end(&mut self) -> bool {
        if self.index >= self.items.len() {
            return true;
        }
        false
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.end() {
            return None;
        }
        Some(self.items[self.index].clone())
    }

    pub fn read(&mut self) -> Option<char> {
        if self.end() {
            return None;
        }
        let val = Some(self.items[self.index].clone());
        self.index += 1;
        self.column += 1;
        val
    }

    pub fn advance(&mut self, amm: usize) {
        self.index += amm;
        self.column += amm as u32;
    }

    pub fn take_snapshot(&mut self) {
        self.snapshots.push(Snapshot {
            index: self.index,
            line: self.line,
            column: self.column,
        });
    }

    pub fn rollback_snapshot(&mut self) {
        let snapshot = self.snapshots.pop().unwrap();
        self.index = snapshot.index;
        self.line = snapshot.line;
        self.column = snapshot.column;
    }

    pub fn commit_snapshot(&mut self) {
        self.snapshots.pop();
    }

    pub fn try_match_token(&mut self, matcher: &Matcher) -> Option<Token> {
        if self.end() {
            return Some(Token {
                tok_type: TokenType::EOF,
                line: self.line,
                col: self.column,
            });
        }

        self.take_snapshot();
        match matcher.try_match(self) {
            Some(tok) => {
                self.commit_snapshot();
                Some(tok)
            }
            None => {
                self.rollback_snapshot();
                None
            }
        }
    }
}
