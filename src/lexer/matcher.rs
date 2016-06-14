use lexer::tokenizer::Tokenizer;
use lexer::token::Token;

pub trait Matcher<T> where T: Iterator, <T as Iterator>::Item : Clone  {
    fn is_match (&self, mut tok: Tokenizer<T>) -> Token;
}
