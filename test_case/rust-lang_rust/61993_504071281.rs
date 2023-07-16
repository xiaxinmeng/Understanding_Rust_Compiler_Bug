Rust
let y = {
   let x = opt.unwrap();
   x.as_ptr()
} // <--- x is dropped, y is now dangling!!
