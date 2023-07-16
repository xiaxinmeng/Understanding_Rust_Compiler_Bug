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
Diff in /checkout/src/librustdoc/html/render/print_item.rs at line 9:
 use rustc_middle::bug;
 use rustc_middle::middle::stability;
 use rustc_middle::ty::layout::LayoutError;
-use rustc_middle::ty::{TyCtxt, Adt};
+use rustc_middle::ty::{Adt, TyCtxt};
 use rustc_span::hygiene::MacroKind;
 use rustc_span::symbol::{kw, sym, Symbol};
 use rustc_target::abi::Variants;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/layout.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/static_files.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
