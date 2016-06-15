mod lexer;

use lexer::matcher::{MatchWhitespace, MatchNumber, MatchSymbol};

fn main() {
    let mut data = "2 + 6 == 8".chars();
    let tok = lexer::Tokenizer::new(&mut data);
    let mut lexer = lexer::Lexer::new(tok);
    let whitespace = MatchWhitespace {};
    let num = MatchNumber {};
    let symbols = MatchSymbol::new(vec![String::from("=="),
                                        String::from("+"),
                                        String::from("-"),
                                        String::from("*"),
                                        String::from("/")]);
    lexer.matchers.push(Box::new(whitespace));
    lexer.matchers.push(Box::new(num));
    lexer.matchers.push(Box::new(symbols));
    for tok in lexer {
        println!("{:?}", tok);
    }
}
