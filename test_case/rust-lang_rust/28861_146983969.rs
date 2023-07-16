 Rust
use std::cell::Cell;
struct C<'a> { val: Cell<Option<&'a C<'a>>> }

fn main() {
    'd: {
        let t: (C<'a>, C<'a>);
        'a: {
             t = (C { val: Cell::new(None) }, C { val: Cell::new(None) });
            Cell::set(&t.0.val, Some(&t.1));
            Cell::set(&t.1.val, Some(&t.0));
        }
        del t;
    }
}
