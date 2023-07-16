
tmp.rs:2:12: 2:25 error: constant evaluation error: can't do this op on a usize and isize [E0080]
tmp.rs:2     Bar = (1usize & 0xff) << 1,
                    ^~~~~~~~~~~~~
tmp.rs:2:12: 2:25 help: run `rustc --explain E0080` to see a detailed explanation
error: aborting due to previous error
