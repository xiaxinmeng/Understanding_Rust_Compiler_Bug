rust
use std::process;

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("This will never be printed!");
    }
}

fn main() {
    let _x = HasDrop;
    process::abort();
    // the destructor implemented for HasDrop will never get run
}
