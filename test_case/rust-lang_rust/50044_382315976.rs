plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:02:06] 556 |     if build.config.rust_codegen_polly_by_default {
[00:02:06]     |        ^^^^^ not found in this scope
[00:02:06] help: possible candidates are found in other modules, you can import them into scope
---
[00:02:11] error: Could not compile `bootstrap`.
[00:02:11]
[00:02:11] To learn more, run the command again with --verbose.
[00:02:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:11] Build completed unsuccessfully in 0:01:25
[00:02:11] make: *** [prepare] Error 1
[00:02:11] Makefile:81: recipe for target 'prepare' failed
[00:02:11] Command failed. Attempt 2/5:
[00:02:11]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:13] error[E0425]: cannot find value `build` in this scope
[00:02:13]    --> bootstrap/compile.rs:556:8
[00:02:13]     |
[00:02:13] 556 |     if build.config.rust_codegen_polly_by_default {
[00:02:13]     |        ^^^^^ not found in this scope
[00:02:13] help: possible candidates are found in other modules, you can import them into scope
---
[00:02:17] error: Could not compile `bootstrap`.
[00:02:17]
[00:02:17] To learn more, run the command again with --verbose.
[00:02:17] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:17] Build completed unsuccessfully in 0:00:06
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:81: recipe for target 'prepare' failed
[00:02:17] Command failed. Attempt 3/5:
[00:02:18]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:19] error[E0425]: cannot find value `build` in this scope
[00:02:19]    --> bootstrap/compile.rs:556:8
[00:02:19]     |
[00:02:19] 556 |     if build.config.rust_codegen_polly_by_default {
[00:02:19]     |        ^^^^^ not found in this scope
[00:02:19] help: possible candidates are found in other modules, you can import them into scope
---
[00:02:24] error: Could not compile `bootstrap`.
[00:02:24]
[00:02:24] To learn more, run the command again with --verbose.
[00:02:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:24] Build completed unsuccessfully in 0:00:06
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:81: recipe for target 'prepare' failed
[00:02:24] Command failed. Attempt 4/5:
[00:02:24]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:26] error[E0425]: cannot find value `build` in this scope
[00:02:26]    --> bootstrap/compile.rs:556:8
[00:02:26]     |
[00:02:26] 556 |     if build.config.rust_codegen_polly_by_default {
[00:02:26]     |        ^^^^^ not found in this scope
[00:02:26] help: possible candidates are found in other modules, you can import them into scope
---
[00:02:30] error: Could not compile `bootstrap`.
[00:02:30]
[00:02:30] To learn more, run the command again with --verbose.
[00:02:30] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:30] Build completed unsuccessfully in 0:00:06
[00:02:30] make: *** [prepare] Error 1
[00:02:30] Makefile:81: recipe for target 'prepare' failed
[00:02:30] Command failed. Attempt 5/5:
[00:02:31]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:32] error[E0425]: cannot find value `build` in this scope
[00:02:32]    --> bootstrap/compile.rs:556:8
[00:02:32]     |
[00:02:32] 556 |     if build.config.rust_codegen_polly_by_default {
[00:02:32]     |        ^^^^^ not found in this scope
[00:02:32] help: possible candidates are found in other modules, you can import them into scope
---
[00:02:37] error: Could not compile `bootstrap`.
[00:02:37]
[00:02:37] To learn more, run the command again with --verbose.
[00:02:37] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:37] Build completed unsuccessfully in 0:00:06
[00:02:37] Makefile:81: recipe for target 'prepare' failed
[00:02:37] make: *** [prepare] Error 1
[00:02:37] The command has failed after 5 attempts.
