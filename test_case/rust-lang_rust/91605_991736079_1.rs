rust
// src/lib.rs
pub fn heyo(){
  cfg_if::cfg_if! { if #[cfg(unix)] {} else {} }
}
