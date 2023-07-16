plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/lib.rs at line 584:
 #[stable(feature = "rust1", since = "1.0.0")]
 #[allow(deprecated, deprecated_in_future)]
 pub use core::{
-    assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, matches, r#try, todo,
+    assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, matches, todo, r#try,
     unimplemented, unreachable, write, writeln,
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sync/barrier/tests.rs" "/checkout/library/std/src/ascii.rs" "/checkout/library/std/src/lib.rs" "/checkout/library/std/src/keyword_docs.rs" "/checkout/library/std/tests/thread.rs" "/checkout/library/std/src/env/tests.rs" "/checkout/library/std/benches/hash/map.rs" "/checkout/library/std/src/collections/hash/map/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
tx.send(entry.into_path()) failed with sending on a closed channelthread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
', format.rs:166:17
