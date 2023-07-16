rust
#![feature(nll)]

struct AutoGCRooter {
    stackTop: *mut AutoGCRooter,
}

impl AutoGCRooter {
    unsafe fn add_to_root_stack(&mut self) {
        self.stackTop = self;
    }
}

fn main() {}
