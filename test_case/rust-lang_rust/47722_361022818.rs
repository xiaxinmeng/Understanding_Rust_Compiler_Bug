rust
#![feature(nll)]

struct AutoGCRooter {
    stackTop: *mut *mut AutoGCRooter,
}

impl AutoGCRooter {
    unsafe fn add_to_root_stack(&mut self) {
        *self.stackTop = self as *mut AutoGCRooter;
    }
}

fn main() {}
