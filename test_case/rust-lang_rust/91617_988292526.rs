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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/cast.rs" "/checkout/compiler/rustc_middle/src/ty/generics.rs" "/checkout/compiler/rustc_middle/src/ty/consts.rs" "/checkout/compiler/rustc_middle/src/ty/util.rs" "/checkout/compiler/rustc_middle/src/ty/layout.rs" "/checkout/compiler/rustc_middle/src/ty/structural_impls.rs" "/checkout/compiler/rustc_middle/src/ty/sty.rs" "/checkout/compiler/rustc_middle/src/ty/list.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/compiler/rustc_middle/src/ty/list.rs at line 47:
     /// Returns a reference to the (unique, static) empty list.
     #[inline(always)]
     pub fn empty<'a>() -> &'a List<T> {
-
         #[repr(align(64))]
         struct MaxAlign;
Build completed unsuccessfully in 0:00:13
