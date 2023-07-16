plain
[RUSTC-TIMING] gimli test:false 4.861
[RUSTC-TIMING] object test:false 5.051
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/sys/wasm/../unix/path.rs:4:16
4 | use crate::os::unix::ffi::OsStrExt;
4 | use crate::os::unix::ffi::OsStrExt;
  |                ^^^^ could not find `unix` in `os`

error[E0599]: no method named `as_bytes` found for reference `&OsStr` in the current scope
  --> library/std/src/sys/wasm/../unix/path.rs:33:36
   |
33 |     let path_os = path.as_os_str().as_bytes();
   |                                    ^^^^^^^^ method not found in `&OsStr`
Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
[RUSTC-TIMING] std test:false 1.768
warning: `std` (lib) generated 1 warning
