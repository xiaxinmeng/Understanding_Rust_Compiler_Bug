 rust
use std::collections::HashMap;

pub struct Map {
    map: HashMap<&'static str, &'static str>
}

fn produce(_: &Map) -> &'static str { "goodbye" }

impl Map {
    fn get(&mut self, k: &'static str) -> Option<&&'static str> {
        {
            if let Some(x) = self.map.get(&k) {
                let sneak_out : *const &'static str = x;
                return Some(unsafe {&*sneak_out});
            }
            // Immutable borrow ENDS here as usual
            // because the borrow checker is not aware
            // of any references escaping this block.
        }

        let tmp = produce(self);
        self.map.insert(k, tmp);
        self.get(k)
    }
}

fn main() {
  let mut m = Map { map: HashMap::new() };
  m.get("hello");
}
