plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:05:31] error[E0596]: cannot borrow immutable borrowed content as mutable
[00:05:31]    --> libarena/lib.rs:447:9
[00:05:31]     |
[00:05:31] 447 |         self.lock.lock().clear();
[00:05:31]     |         ^^^^^^^^^^^^^^^^ cannot borrow as mutable
[00:05:31]
[00:05:31] error: aborting due to previous error
[00:05:31]
[00:05:31] For more information about this error, try `rustc --explain E0596`.
[00:05:31] error: Could not compile `arena`.
[00:05:31]
[00:05:31] Caused by:
[00:05:31]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name arena libarena/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=dd80e21b25e89693 -C extra-filename=-dd80e21b25e89693 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so` (exit code: 101)
