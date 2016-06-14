use lexer::token::Token;
use lexer::matcher::Matcher;

struct Snapshot {
    index: usize,
    line: u32,
    column: u32,
}

pub struct Tokenizer<T> where T: Iterator, <T as Iterator>::Item : Clone {
    index : usize,
    line: u32,
    column: u32,
    items : Vec<<T as Iterator>::Item>,
    snapshots : Vec<Snapshot>,
    matchers : Vec<Box<Matcher<T>>>,
}

impl<T> Iterator for Tokenizer<T> where T: Iterator, <T as Iterator>::Item : Clone {
    type Item = <T as Iterator>::Item;

    fn next (&mut self) -> Option <Self::Item> {
        if self.end () {
            return None
        }
        let val = Some (self.items[self.index].clone ());
        self.index += 1;
        self.column += 1;
        val
    }
}

impl<T> Tokenizer<T> where T: Iterator, <T as Iterator>::Item : Clone {
    pub fn new (items: T) -> Tokenizer<T>  {
        Tokenizer {
            index : 0,
            line : 0,
            column : 0,
            items : items.collect (),
            snapshots : Vec::new (),
            matchers : Vec::new (),
        }
    }

    pub fn end (&mut self) -> bool {
        if self.index >= self.items.len () {
            return true
        }
        false
    }

    pub fn peek (&mut self) -> Option<&<T as Iterator>::Item> {
        if self.end () {
            return None
        }
        Some (&self.items[self.index])
    }

    pub fn read (&mut self) -> Option<&<T as Iterator>::Item> {
        if self.end () {
            return None
        }
        let val = Some (&self.items[self.index]);
        self.index += 1;
        self.column += 1;
        val
    }

    pub fn take_snapshot (&mut self) {
        self.snapshots.push (Snapshot { index : self.index, line : self.line, column : self.column });
    }

    pub fn rollback_snapshot (&mut self) {
        let snapshot = self.snapshots.pop ().unwrap ();
        self.index = snapshot.index;
        self.line = snapshot.line;
        self.column = snapshot.column;
    }

    pub fn commit_snapshot (&mut self) {
        self.snapshots.pop ();
    }
}
