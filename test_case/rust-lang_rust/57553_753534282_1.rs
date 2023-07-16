rust
error[E0382]: borrow of moved value: `blah`
  --> src/lib.rs:8:29
   |
3  |         let blah: Option<String>;
   |             ---- move occurs because `blah` has type `Option<String>`, which does not implement the `Copy` trait
...
8  |         if let Some(blah) = blah.as_ref() {
   |                             ^^^^ value borrowed here after move
...
11 |     }
   |     - value moved here, in previous iteration of loop
