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
Diff in /checkout/src/librustdoc/passes/html_tags.rs at line 4:
 use crate::fold::DocFolder;
 use crate::html::markdown::main_body_opts;
 use core::ops::Range;
-use pulldown_cmark::{Event, Parser, Tag};
 use itertools::Itertools;
+use pulldown_cmark::{Event, Parser, Tag};
 use std::str::CharIndices;
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/formats/mod.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/formats/renderer.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/passes/html_tags.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/formats/cache.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
