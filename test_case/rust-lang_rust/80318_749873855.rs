rust
// Doesn't compile with `wasm-pack` (but otherwise compiles fine)
let my_size = (u16::max as usize) + 1;
let my_size = (u16::max as usize) - 1;
let my_size = (u16::max as usize) + 2;
let my_size = (u16::max as usize) - 2;

// Compiles Fine with `wasm-pack`:
let my_size = (u16::max as usize) * 1;
let my_size = (u16::max as usize) / 1;
let my_size = (u16::max as usize) * 2;
let my_size = (u16::max as usize) / 2;
