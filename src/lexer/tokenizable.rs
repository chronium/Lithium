struct Snapshot {
    index: usize
}

pub struct Tokenizable<T> where T: Iterator, <T as Iterator>::Item : Clone {
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
    pub fn new (items: T) -> Tokenizable<T>  {
        Tokenizable {
            index : 0,
            items : items.collect (),
            snapshots : Vec::new (),
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
        val
    }

    pub fn take_snapshot (&mut self) {
        self.snapshots.push (Snapshot { index : self.index });
    }

    pub fn rollback_snapshot (&mut self) {
        let snapshot = self.snapshots.pop ();
        self.index = snapshot.unwrap ().index;
    }

    pub fn commit_snapshot (&mut self) {
        self.snapshots.pop ();
    }
}
