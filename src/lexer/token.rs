#[derive (PartialEq)]
enum TokenType {
    WhiteSpace,
    EOF,
}

pub struct Token {
    tok_type: TokenType,
    line: u32,
    col: u32,
}

impl PartialEq for Token {
    fn eq (&self, other: &Token) -> bool {
        self.tok_type == other.tok_type
    }

    fn ne (&self, other: &Token) -> bool {
        self.tok_type != other.tok_type
    }
}
