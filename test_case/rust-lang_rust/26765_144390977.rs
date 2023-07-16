
D:\code\rusty>rustc main.rs
main.rs:1:1: 1:1 error: multiple matching crates for `std`
main.rs:1 fn main() {
          ^
note: candidates:
note: path: \\?\D:\coding\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\std-4e7c5e5c.dll
note: path: \\?\D:\coding\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\libstd-4e7c5e5c.rlib
note: crate name: std
note: path: \\?\D:\coding\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\std-198068b3.dll
note: path: \\?\D:\coding\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\libstd-198068b3.rlib
note: crate name: std
main.rs:1:1: 1:1 error: found staticlib `std` instead of rlib or dylib
main.rs:1 fn main() {
          ^
main.rs:1:1: 1:1 help: please recompile this crate using --crate-type lib
main.rs:1:1: 1:1 note: crate `std` path #1: D:\coding\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\libstdc++.a
error: aborting due to 2 previous errors
