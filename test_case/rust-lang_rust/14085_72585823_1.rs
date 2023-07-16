 rust
/t/triage $ cat foo.rs
#![feature(box_syntax)]

fn main() {
    //box (1 + 2) * 3;

    box (1.0 + 2.0).sqrt()
}
/t/triage $ rustc foo.rs
foo.rs:6:20: 6:21 error: expected expression, found `.`
foo.rs:6     box (1.0 + 2.0).sqrt()
                            ^
foo.rs:6:5: 6:20 help: perhaps you meant `box() (foo)` instead?
foo.rs:6     box (1.0 + 2.0).sqrt()
             ^~~~~~~~~~~~~~~
error: aborting due to previous error
