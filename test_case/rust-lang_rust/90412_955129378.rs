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
Diff in /checkout/src/librustdoc/lib.rs at line 107:
 mod config;
 mod core;
 mod docfs;
+mod doctest;
 mod doctree;
 mod error;
 mod externalfiles;
Diff in /checkout/src/librustdoc/lib.rs at line 113:
-mod doctest;
 mod fold;
 mod formats;
 // used by the error-index generator, so it needs to be public
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/highlight/fixtures/sample.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/render/templates.rs" "/checkout/src/librustdoc/html/tests.rs"` failed.
Diff in /checkout/src/librustdoc/html/render/context.rs at line 22:
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
     BASIC_KEYWORDS,
 
 
-use crate::{clean, try_err};
 use crate::clean::ExternalCrate;
 use crate::config::RenderOptions;
 use crate::docfs::{DocFS, PathError};
Diff in /checkout/src/librustdoc/html/render/context.rs at line 35:
 use crate::html::markdown::{self, plain_text_summary, ErrorCodes, IdMap};
 use crate::html::{layout, sources};
 use crate::scrape_examples::AllCallLocations;
+use crate::{clean, try_err};
 
 /// Major driving force in all rustdoc rendering. This contains information
 /// about where in the tree-like hierarchy rendering is occurring and controls
