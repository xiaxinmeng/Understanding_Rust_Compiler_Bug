rust
use std::sync::Once;
use std::panic::catch_unwind;

static o: Once = Once::new();

fn main() {
    catch_unwind(|| o.call_once(|| panic!("foo")));
}
