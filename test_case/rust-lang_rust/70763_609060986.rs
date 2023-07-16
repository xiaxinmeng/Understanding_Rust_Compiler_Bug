rust
fn main() {
   let _ = || {
       #[cfg(any(target_arch == "arm"))]
       let _ = ();
   };
}
