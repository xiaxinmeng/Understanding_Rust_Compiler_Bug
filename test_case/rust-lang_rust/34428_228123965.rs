 rust
tmp.rs:5:18: 5:24 error: mismatched types [E0308]
tmp.rs:5     swap(&mut a, &mut b);
                          ^~~~~~
tmp.rs:5:18: 5:24 help: run `rustc --explain E0308` to see a detailed explanation
tmp.rs:5:18: 5:24 note: expected type `&mut &mut std::vec::Vec<T>`
tmp.rs:5:18: 5:24 note:    found type `&mut std::vec::Vec<_>`
error: aborting due to previous error
