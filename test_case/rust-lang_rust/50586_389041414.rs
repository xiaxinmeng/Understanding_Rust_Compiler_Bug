rust
extern crate thread_local;

use thread_local::ThreadLocal;

fn main() {
    let tls: ThreadLocal<u32> = ThreadLocal::new();
    assert_eq!(tls.get(), None);
}
