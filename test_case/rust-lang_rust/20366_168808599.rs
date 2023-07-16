
extern crate alloc;

use std::marker::Send;
use alloc::arc::Arc;

struct Inner(*mut u8);

unsafe impl Send for Inner {}

fn is_send<T:Send>(s: T) {}

fn main() {
    let arc = Arc::new(Inner(0 as *mut u8));
    is_send(arc);
}
