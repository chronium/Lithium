mod lexer;

use lexer::matcher::{ MatchWhitespace, MatchNumber };

fn main () {
    let mut data = "128 256".chars ();
    let mut tok = lexer::Tokenizer::new (&mut data);
    let mut lexer = lexer::Lexer::new (tok);
    let whitespace = MatchWhitespace { };
    let num = MatchNumber { };
    lexer.matchers.push (Box::new (whitespace));
    lexer.matchers.push (Box::new (num));
    for tok in lexer {
        println!("{:?}", tok);
    }
}
