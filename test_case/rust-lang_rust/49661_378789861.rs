plain
[00:00:54] configure: rust.quiet-tests     := True
---
[00:02:23] error[E0425]: cannot find function `read_string` in module `fs`
[00:02:23]     --> bootstrap/lib.rs:1152:16
[00:02:23]      |
[00:02:23] 1152 |         t!(fs::read_string(path))
[00:02:23]      |                ^^^^^^^^^^^ did you mean `read_to_string`?
[00:02:23]
[00:02:27] error: aborting due to previous error
[00:02:27]
[00:02:27] For more information about this error, try `rustc --explain E0425`.
[00:02:27] error: Could not compile `bootstrap`.
[00:02:27]
[00:02:27] To learn more, run the command again with --verbose.
[00:02:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:27] Build completed unsuccessfully in 0:01:32
[00:02:27] Makefile:81: recipe for target 'prepare' failed
[00:02:27] make: *** [prepare] Error 1
[00:02:27] Command failed. Attempt 2/5:
[00:02:27]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:29] error[E0425]: cannot find function `read_string` in module `fs`
[00:02:29]     --> bootstrap/lib.rs:1152:16
[00:02:29]      |
[00:02:29] 1152 |         t!(fs::read_string(path))
[00:02:29]      |                ^^^^^^^^^^^ did you mean `read_to_string`?
[00:02:29]
[00:02:33] error: aborting due to previous error
[00:02:33]
[00:02:33] For more information about this error, try `rustc --explain E0425`.
[00:02:33] error: Could not compile `bootstrap`.
[00:02:33]
[00:02:33] To learn more, run the command again with --verbose.
[00:02:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:33] Build completed unsuccessfully in 0:00:06
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Command failed. Attempt 3/5:
[00:02:34]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:35] error[E0425]: cannot find function `read_string` in module `fs`
[00:02:35]     --> bootstrap/lib.rs:1152:16
[00:02:35]      |
[00:02:35] 1152 |         t!(fs::read_string(path))
[00:02:35]      |                ^^^^^^^^^^^ did you mean `read_to_string`?
[00:02:35]
[00:02:39] error: aborting due to previous error
[00:02:39]
[00:02:39] For more information about this error, try `rustc --explain E0425`.
[00:02:39] error: Could not compile `bootstrap`.
[00:02:39]
[00:02:39] To learn more, run the command again with --verbose.
[00:02:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:39] Build completed unsuccessfully in 0:00:05
[00:02:39] Makefile:81: recipe for target 'prepare' failed
[00:02:39] make: *** [prepare] Error 1
[00:02:39] Command failed. Attempt 4/5:
[00:02:39]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:41] error[E0425]: cannot find function `read_string` in module `fs`
[00:02:41]     --> bootstrap/lib.rs:1152:16
[00:02:41]      |
[00:02:41] 1152 |         t!(fs::read_string(path))
[00:02:41]      |                ^^^^^^^^^^^ did you mean `read_to_string`?
[00:02:41]
[00:02:45] error: aborting due to previous error
[00:02:45]
[00:02:45] For more information about this error, try `rustc --explain E0425`.
[00:02:45] error: Could not compile `bootstrap`.
[00:02:45]
[00:02:45] To learn more, run the command again with --verbose.
[00:02:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:45] Build completed unsuccessfully in 0:00:05
[00:02:45] make: *** [prepare] Error 1
[00:02:45] Makefile:81: recipe for target 'prepare' failed
[00:02:45] Command failed. Attempt 5/5:
[00:02:45]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:47] error[E0425]: cannot find function `read_string` in module `fs`
[00:02:47]     --> bootstrap/lib.rs:1152:16
[00:02:47]      |
[00:02:47] 1152 |         t!(fs::read_string(path))
[00:02:47]      |                ^^^^^^^^^^^ did you mean `read_to_string`?
