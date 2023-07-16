`rust
extern "C" {
    static _dispatch_queue_attr_concurrent: [u8; 0];
}

fn main() {
let x: &[u8; 0] =
    unsafe { &_dispatch_queue_attr_concurrent };
}
