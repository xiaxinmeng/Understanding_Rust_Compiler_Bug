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
Diff in /checkout/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs at line 22:
 use rustc_span::source_map::{Span, Spanned};
 
+use itertools::Itertools;
 use rustc_data_structures::sync::Lrc;
 use smallvec::SmallVec;
 use smallvec::SmallVec;
 use std::any::Any;
Diff in /checkout/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs at line 28:
 
 macro_rules! provide {
 macro_rules! provide {
     (<$lt:tt> $tcx:ident, $def_id:ident, $other:ident, $cdata:ident,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_metadata/src/rmeta/decoder.rs" "/checkout/compiler/rustc_attr/src/builtin.rs" "/checkout/compiler/rustc_metadata/src/rmeta/table.rs" "/checkout/compiler/rustc_metadata/src/rmeta/encoder.rs" "/checkout/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs" "/checkout/compiler/rustc_arena/src/lib.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_attr/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
