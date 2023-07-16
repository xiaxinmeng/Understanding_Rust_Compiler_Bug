
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader falling back to a load
INFO rustc_metadata::locator rlib reading metadata from: /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_64
-unknown-linux-gnu/release/deps/libcore-999232a2b2133b8a.rlib
INFO rustc_metadata::creader register crate `core` (cnum = 1. private_dep = false)
INFO rustc_metadata::creader resolving dep crate core hash: `34298ac765912203` extra filename: `-999232a2b2133b8a`
INFO rustc_metadata::creader resolving crate `core`
INFO rustc_metadata::creader falling back to a load
INFO rustc_metadata::locator lib candidate: /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linu
x-gnu/release/deps/libcore-999232a2b2133b8a.rlib
INFO rustc_metadata::locator lib candidate: /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linu
x-gnu/release/deps/libcore-999232a2b2133b8a.rmeta
INFO rustc_metadata::locator lib candidate: /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86
_64-unknown-linux-gnu/lib/libcore-999232a2b2133b8a.rlib
INFO rustc_metadata::locator rmeta reading metadata from: /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_6
4-unknown-linux-gnu/release/deps/libcore-999232a2b2133b8a.rmeta
INFO rustc_metadata::locator Rejecting via hash: expected 34298ac765912203 got 028056f371feafa5
INFO rustc_metadata::locator metadata mismatch
INFO rustc_metadata::locator rlib reading metadata from: /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-sysroot/li
b/rustlib/x86_64-unknown-linux-gnu/lib/libcore-999232a2b2133b8a.rlib
INFO rustc_metadata::locator rlib reading metadata from: /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_64
-unknown-linux-gnu/release/deps/libcore-999232a2b2133b8a.rlib
INFO rustc_metadata::locator Rejecting via hash: expected 34298ac765912203 got 028056f371feafa5
INFO rustc_metadata::locator metadata mismatch
INFO rustc_metadata::creader register crate `core` (cnum = 5. private_dep = false)
error: internal compiler error: compiler/rustc_metadata/src/creader.rs:352:17: Previously returned E0523 here. See https://github.com/rust-lang/rust/pull/100599 for additional discussion.root.name() = core.
