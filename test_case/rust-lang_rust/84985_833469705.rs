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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_attr/src/lib.rs at line 4:
 //! The goal is to move the definition of `MetaItem` and things that don't need to be in `syntax`
 //! to this crate.
-
 #[macro_use]
 extern crate rustc_macros;
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_attr/src/lib.rs" "/checkout/compiler/rustc_attr/src/builtin.rs" "/checkout/compiler/rustc_session/src/options.rs" "/checkout/compiler/rustc_session/src/lib.rs" "/checkout/compiler/rustc_session/src/filesearch.rs" "/checkout/compiler/rustc_session/src/config.rs" "/checkout/compiler/rustc_session/src/code_stats.rs" "/checkout/compiler/rustc_parse_format/src/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
