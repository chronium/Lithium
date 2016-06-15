#[derive (Debug, PartialEq)]
pub enum TokenType {
    IntLiteral(u64),
    Symbol(String),
    WhiteSpace,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub tok_type: TokenType,
    pub line: u32,
    pub col: u32,
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.tok_type == other.tok_type
    }

    fn ne(&self, other: &Token) -> bool {
        self.tok_type != other.tok_type
    }
}
