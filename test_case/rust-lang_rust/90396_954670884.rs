plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/test2.rs at line 8:
 struct Range<T: PartialOrd, const MIN: T, const MAX: T>(T)
 where
     ConstAssert<{ MIN <= MAX }>: True;
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_index/src/lib.rs" "/checkout/compiler/rustc_index/src/bit_set.rs" "/checkout/compiler/rustc_index/src/vec/tests.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/test2.rs" "/checkout/compiler/rustc_incremental/src/assert_dep_graph.rs" "/checkout/compiler/rustc_macros/src/lib.rs" "/checkout/compiler/rustc_index/src/vec.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
