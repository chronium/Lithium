mod lexer;

use lexer::matcher::{MatchWhitespace, MatchNumber, MatchIdentifier, MatchSymbol};

fn main() {
    let mut data = r#"
a = (1 + 2) == 3
a = ()"#
        .chars();
    let tok = lexer::Tokenizer::new(&mut data);
    let mut lexer = lexer::Lexer::new(tok);
    let whitespace = MatchWhitespace {};
    let num = MatchNumber {};
    let symbols = MatchSymbol::new(vec![String::from("=="),
                                        String::from("()"),
                                        String::from("("),
                                        String::from(")"),
                                        String::from("="),
                                        String::from("+"),
                                        String::from("-"),
                                        String::from("*"),
                                        String::from("/")]);
    let identifiers = MatchIdentifier {};
    lexer.matchers.push(Box::new(whitespace));
    lexer.matchers.push(Box::new(num));
    lexer.matchers.push(Box::new(symbols));
    lexer.matchers.push(Box::new(identifiers));
    for tok in lexer {
        println!("{:?}", tok);
    }
}
