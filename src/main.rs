mod lexer;

fn main () {
    let mut data = "Hello".chars ();
    let mut tokenizable = lexer::Tokenizable::new (data);
    tokenizable.take_snapshot ();
    println!("{:?}", tokenizable.next ().unwrap ());
    println!("{:?}", tokenizable.next ().unwrap ());
    tokenizable.rollback_snapshot();
    println!("{:?}", tokenizable.next ().unwrap ());
    println!("{:?}", tokenizable.next ().unwrap ());
    tokenizable.take_snapshot ();
    println!("{:?}", tokenizable.next ().unwrap ());
    println!("{:?}", tokenizable.next ().unwrap ());
    tokenizable.commit_snapshot();
}
