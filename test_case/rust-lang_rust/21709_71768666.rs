 rust
use std::ptr::Unique;
use std::thread::Thread;

#[derive(Copy)]
struct ShouldBeSendable {
    x: i32
}

unsafe impl std::marker::Send for ShouldBeSendable { }

fn main() {
    let ptr : *mut ShouldBeSendable = &mut ShouldBeSendable {x: 5};  // this is not `Send`
    let sendablePtr = Unique(ptr);  // but this is!

    let closure = move |:| {
        // `sendablePtr` con be moved inside this closure
        let ptr = sendablePtr.0;  // unpack the raw pointer
        println!("{}", unsafe { (*ptr).x })
    };
    let something = Thread::scoped(closure).join();
}
