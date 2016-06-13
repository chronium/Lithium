struct Snapshot {
    index: usize
}

struct Tokenizable<T> where T: Iterator, <T as Iterator>::Item : Clone {
    index : usize,
    items : Vec<<T as Iterator>::Item>,
    snapshots : Vec<Snapshot>,
}

impl<T> Iterator for Tokenizable<T> where T: Iterator, <T as Iterator>::Item : Clone {
    type Item = <T as Iterator>::Item;

    fn next (&mut self) -> Option <Self::Item> {
        if self.end () {
            return None
        }
        let val = Some (self.items[self.index].clone ());
        self.index += 1;
        val
    }
}

impl<T> Tokenizable<T> where T: Iterator, <T as Iterator>::Item : Clone {
    fn new (items: T) -> Tokenizable<T>  {
        Tokenizable {
            index : 0,
            items : items.collect (),
            snapshots : Vec::new (),
        }
    }

    fn end (&mut self) -> bool {
        if self.index >= self.items.len () {
            return true
        }
        false
    }

    fn peek (&mut self) -> Option<&<T as Iterator>::Item> {
        if self.end () {
            return None
        }
        Some (&self.items[self.index])
    }

    fn read (&mut self) -> Option<&<T as Iterator>::Item> {
        if self.end () {
            return None
        }
        let val = Some (&self.items[self.index]);
        self.index += 1;
        val
    }

    fn take_snapshot (&mut self) {
        self.snapshots.push (Snapshot { index : self.index });
    }

    fn rollback_snapshot (&mut self) {
        let snapshot = self.snapshots.pop ();
        self.index = snapshot.unwrap ().index;
    }

    fn commit_snapshot (&mut self) {
        self.snapshots.pop ();
    }
}

fn main () {
    let mut data = "Hello".chars ();
    let mut tokenizable = Tokenizable::new (data);
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
