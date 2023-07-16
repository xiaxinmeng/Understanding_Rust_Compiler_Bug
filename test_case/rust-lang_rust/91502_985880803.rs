rust
use once_cell::sync::OnceCell;
struct Bar {}

static BAR: OnceCell<Bar> = OnceCell::new();

fn main() {
    let b = Bar {
        ..BAR.get().unwrap()
    };
}
