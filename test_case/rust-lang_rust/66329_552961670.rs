rust
fn loop_forever() { loop {} }

pub enum Empty {}

pub unsafe fn foo(x: bool, bomb: *const Empty) {
    if x {
        loop_forever()
    }
    match *bomb {}
}

