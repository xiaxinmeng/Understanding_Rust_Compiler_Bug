rust
use std::cell::Cell;

fn main() {
    let n = &Cell::new(5);
    let c = &Cell::new(0);

    let it = std::iter::from_fn(|| {
        let x = c.get();
        if x == 0 {
            let x = n.get();
            n.set(x - 1);
            c.set(x);
            None
        } else {
            c.set(x - 1);
            Some(x)
        }
    });
    let it = it.cycle();
    it.for_each(|x| {
        dbg!(x);
    });
}
