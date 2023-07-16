
   Compiling playground v0.0.1 (file:///playground)
error[E0382]: use of moved value: `src`
  --> src/main.rs:16:5
   |
13 |     while let Some(node) = {src}.next.as_mut().map(|node| &mut *node) {
   |                             --- value moved here
...
16 |     src.next = None;
   |     ^^^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `src` has type `&mut Node`, which does not implement the `Copy` trait
