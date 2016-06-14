mod lexer;

fn main () {
    let mut data = "Hello".chars ();
    let mut tok = lexer::Tokenizer::new (data);
    tok.take_snapshot ();
    println!("{:?}", tok.next ().unwrap ());
    println!("{:?}", tok.next ().unwrap ());
    tok.rollback_snapshot();
    println!("{:?}", tok.next ().unwrap ());
    println!("{:?}", tok.next ().unwrap ());
    tok.take_snapshot ();
    println!("{:?}", tok.next ().unwrap ());
    println!("{:?}", tok.next ().unwrap ());
    tok.commit_snapshot();
}
