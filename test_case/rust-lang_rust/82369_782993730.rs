plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/lib.rs at line 276:
 
 // note: does not need to be public
 mod bool;
+mod sealed;
 mod tuple;
 mod unit;
-mod sealed;
 
 #[stable(feature = "core_primitive", since = "1.43.0")]
 pub mod primitive;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/fmt/float.rs" "/checkout/compiler/rustc_lint/src/traits.rs" "/checkout/compiler/rustc_lint/src/lib.rs" "/checkout/library/core/src/lib.rs" "/checkout/library/core/src/task/wake.rs" "/checkout/library/core/src/macros/mod.rs" "/checkout/library/core/src/task/poll.rs" "/checkout/compiler/rustc_lint/src/internal.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
