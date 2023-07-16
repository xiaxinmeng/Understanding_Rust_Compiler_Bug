rust
extern crate rayon;

use std::sync::Arc;
use rayon::join;

fn main() {
    let a1 = Arc::new(0);
    let mut a2 = a1.clone();
    join(
        || {
            let _ : u32 = *a1;
            drop(a1);
        },
        || {
            loop {
                match Arc::get_mut(&mut a2) {
                    None => {}
                    Some(m) => {
                        *m = 1u32;
                        return;
                    }
                }
            }
        }
    );
}
