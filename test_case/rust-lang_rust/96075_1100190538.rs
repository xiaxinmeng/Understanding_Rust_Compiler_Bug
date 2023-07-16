plain
[RUSTC-TIMING] object test:false 6.757
[RUSTC-TIMING] gimli test:false 6.911
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-emscripten`

error: unused import: `cvt_nz`
    |
    |
282 |         use crate::sys::{self, cvt_nz, cvt_r};
    |
    |
    = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] std test:false 4.659
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:13:49
