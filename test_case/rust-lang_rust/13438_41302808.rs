 rust

extern crate collections;
extern crate sync;

use collections::hashmap::HashMap;
use std::hash::Hash;
use std::io::File;

struct Evil {
  ix: uint
}


impl Evil {
    fn new(ix: uint, val: &[u8]) -> Evil {
        let result = Evil { ix: ix };
        result.mutate(val);
        result
    }

    fn mutate(&self, val: &[u8]) {
        let f = File::open(&Path::new(self.ix.to_str()));
        f.write(val);
    }
}

impl Eq for Evil {
    fn eq(&self, other: &Evil) -> bool {
        self.ix == other.ix
    }
}

impl TotalEq for Evil {}

impl<S: Writer> Hash<S> for Evil {
    fn hash(&self, state: &mut S) {
        let _ = state.write_le_uint(File::open(&Path::new(self.ix.to_str()))
            .read_to_end());
    }
}

#[test]
fn test() {
    let mut map = HashMap::new();

    for i in range(0u, 1000u) {
        map.insert(Evil::new(i, bytes!("good")), i);
    }

    // With `Share` you can mutate through immutable references ...
    for k in map.keys() {
        k.mutate(bytes!("evil"));
    }

    // ... leads to problems that Rust promises to prevent :(
    for i in range(0u, 1000u) {
        assert_eq!(map.find_copy(&Evil::new(i,  bytes!("good"))), Some(i));
    }
}
