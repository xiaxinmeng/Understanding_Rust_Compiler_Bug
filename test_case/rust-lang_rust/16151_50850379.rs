 rust
use std::mem;

struct Fragment {
    dummy: int
}

impl Fragment {
    fn new(d: int) -> Fragment {
        Fragment {
            dummy: d,
        }
    }
}

impl Drop for Fragment {
    fn drop(&mut self) {
        println!("drop {}", self.dummy);
    }
}

fn main() {
    let mut fragments = vec!();
    fragments.push(Fragment::new(1));
    fragments.push(Fragment::new(2));

    let new_fragments: Vec<Fragment> = mem::replace(&mut fragments, vec![])
        .move_iter()
        .skip_while(|fragment| {
            println!("Skip {}", fragment.dummy);
            true
        }).collect();
    std::io::println("End");
}
