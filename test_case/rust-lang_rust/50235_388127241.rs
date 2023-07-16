plain
######################################################################## 100.0%
[00:00:59] extracting /checkout/obj/build/cache/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:02]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:19]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:20] error: failed to load source for a dependency on `rustc-rayon`
[00:01:20] Caused by:
[00:01:20] Caused by:
[00:01:20]   Unable to update https://github.com/Zoxc/rayon.git?branch=rustc#7874a154
[00:01:20] Caused by:
[00:01:20] Caused by:
[00:01:20]   revspec '7874a15417fe45294192d7377c0be1a81a67d6f0' not found; class=Reference (4); code=NotFound (-3)
[00:01:20] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:20] Build completed unsuccessfully in 0:00:34
[00:01:20] Makefile:81: recipe for target 'prepare' failed
[00:01:20] make: *** [prepare] Error 1
[00:01:20]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:20]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:26] error: failed to load source for a dependency on `rustc-rayon`
[00:01:26] Caused by:
[00:01:26] Caused by:
[00:01:26]   Unable to update https://github.com/Zoxc/rayon.git?branch=rustc#7874a154
[00:01:26] Caused by:
[00:01:26] Caused by:
[00:01:26]   revspec '7874a15417fe45294192d7377c0be1a81a67d6f0' not found; class=Reference (4); code=NotFound (-3)
[00:01:26] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:26] Build completed unsuccessfully in 0:00:05
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] make: *** [prepare] Error 1
[00:01:26]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:26]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:31] error: failed to load source for a dependency on `rustc-rayon`
[00:01:31] Caused by:
[00:01:31] Caused by:
[00:01:31]   Unable to update https://github.com/Zoxc/rayon.git?branch=rustc#7874a154
[00:01:31] Caused by:
[00:01:31] Caused by:
[00:01:31]   revspec '7874a15417fe45294192d7377c0be1a81a67d6f0' not found; class=Reference (4); code=NotFound (-3)
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:05
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:31] make: *** [prepare] Error 1
[00:01:31]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:31]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:37] error: failed to load source for a dependency on `rustc-rayon`
[00:01:37] Caused by:
[00:01:37] Caused by:
[00:01:37]   Unable to update https://github.com/Zoxc/rayon.git?branch=rustc#7874a154
[00:01:37] Caused by:
[00:01:37] Caused by:
[00:01:37]   revspec '7874a15417fe45294192d7377c0be1a81a67d6f0' not found; class=Reference (4); code=NotFound (-3)
[00:01:37] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:37] Build completed unsuccessfully in 0:00:05
[00:01:37] make: *** [prepare] Error 1
[00:01:37] Makefile:81: recipe for target 'prepare' failed
[00:01:37]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:37]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:42] error: failed to load source for a dependency on `rustc-rayon`
[00:01:42] Caused by:
[00:01:42] Caused by:
[00:01:42]   Unable to update https://github.com/Zoxc/rayon.git?branch=rustc#7874a154
[00:01:42] Caused by:
[00:01:42] Caused by:
[00:01:42]   revspec '7874a15417fe45294192d7377c0be1a81a67d6f0' not found; class=Reference (4); code=NotFound (-3)
[00:01:42] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:42] Build completed unsuccessfully in 0:00:04
[00:01:42] make: *** [prepare] Error 1
[00:01:42] Makefile:81: recipe for target 'prepare' failed
[00:01:42] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:001b362c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
