plain
[00:06:04]    Compiling backtrace v0.3.9
[00:06:05]    Compiling crossbeam-deque v0.2.0
[00:06:08]    Compiling rustc-rayon v0.1.1
[00:06:14]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:14] error[E0252]: the name `Path` is defined multiple times
[00:06:14]   --> librustc_data_structures/flock.rs:27:13
[00:06:14] 21 | use std::path::Path;
[00:06:14] 21 | use std::path::Path;
[00:06:14]    |     --------------- previous import of the type `Path` here
[00:06:14] 27 |         use std::path::Path;
[00:06:14] 27 |         use std::path::Path;
[00:06:14]    |             ^^^^^^^^^^^^^^^ `Path` reimported here
[00:06:14]    |
[00:06:14]    = note: `Path` must be defined only once in the type namespace of this module
[00:06:14] help: You can use `as` to change the binding name of the import
[00:06:14] 27 |         use std::path::Path as OtherPath;
[00:06:14]    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:06:14] 
[00:06:14] error: unused import: `std::path::Path`
---
[00:06:16] For more information about this error, try `rustc --explain E0252`.
[00:06:16] error: Could not compile `rustc_data_structures`.
[00:06:16] 
[00:06:16] Caused by:
[00:06:16]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_data_structures librustc_data_structures/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=05301c67193a930e -C extra-filename=-05301c67193a930e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-c76d7635015dbd91.rlib --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libena-78f18b1831cf7d50.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-87308d43fa547c79.rlib --extern parking_lot_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-f33344b1613f3387.rlib --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-b1d18f68284bbb3d.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-f58fc2695e5bd48d.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-4928a7a7d96a17f8.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-b3ce1c0970d50596.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cb741677cd0e0351.rlib --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-712b2a476a36a5ef.rlib` (exit code: 1)
