plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:04:16] error: attributes are not yet allowed on `if` expressions
[00:04:16]    --> liballoc_system/lib.rs:137:17
[00:04:16]     |
[00:04:16] 137 |                 #[cfg(target_os = "macos")]
[00:04:16]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:16]
[00:04:16] error: aborting due to previous error
[00:04:16]
[00:04:16] error: Could not compile `alloc_system`.
[00:04:16]
[00:04:16] Caused by:
[00:04:16]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc_system liballoc_system/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=ed94ac9e95a953a9 -C extra-filename=-ed94ac9e95a953a9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-811292bee4a6b310.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stageux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu
---
113580 ./obj/build/bootstrap/debug/incremental/bootstrap-1xj36c8o119z7/s-ezq968k9js-1lqr5ph-1qldka3x57wek
