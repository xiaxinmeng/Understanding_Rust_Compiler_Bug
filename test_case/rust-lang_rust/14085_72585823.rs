 rust
/t/triage $ cat foo.rs
#![feature(box_syntax)]

fn main() {
    box (1 + 2) * 3;

    //box (1.0 + 2.0).sqrt()
}
/t/triage $ rustc foo.rs
foo.rs:4:17: 4:20 error: type `_` cannot be dereferenced
foo.rs:4     box (1 + 2) * 3;
                         ^~~
foo.rs:4:5: 4:20 error: only the managed heap and exchange heap are currently supported [E0066]
foo.rs:4     box (1 + 2) * 3;
             ^~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
