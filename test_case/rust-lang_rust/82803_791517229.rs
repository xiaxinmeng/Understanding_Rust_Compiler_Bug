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
Diff in /checkout/src/librustdoc/html/render/write_shared.rs at line 3:
 use std::fs::{self, File};
 use std::io::prelude::*;
 use std::io::{self, BufReader};
-use std::path::{Component, Path, PathBuf};
 use std::lazy::SyncLazy as Lazy;
+use std::path::{Component, Path, PathBuf};
 use itertools::Itertools;
 use rustc_data_structures::flock;
 use rustc_data_structures::flock;
Diff in /checkout/src/librustdoc/html/render/write_shared.rs at line 19:
 use crate::formats::FormatRenderer;
 use crate::html::{layout, static_files};
 
-crate static FILES_UNVERSIONED: Lazy<FxHashMap<&str, &[u8]>> = Lazy::new(|| map! {
-    "FiraSans-Regular.woff2" => static_files::fira_sans::REGULAR2,
-    "FiraSans-Medium.woff2" => static_files::fira_sans::MEDIUM2,
-    "FiraSans-Regular.woff" => static_files::fira_sans::REGULAR,
-    "FiraSans-Medium.woff" => static_files::fira_sans::MEDIUM,
-    "FiraSans-LICENSE.txt" => static_files::fira_sans::LICENSE,
-    "SourceSerifPro-Regular.ttf.woff" => static_files::source_serif_pro::REGULAR,
-    "SourceSerifPro-Bold.ttf.woff" => static_files::source_serif_pro::BOLD,
-    "SourceSerifPro-It.ttf.woff" => static_files::source_serif_pro::ITALIC,
-    "SourceSerifPro-LICENSE.md" => static_files::source_serif_pro::LICENSE,
-    "SourceCodePro-Regular.woff" => static_files::source_code_pro::REGULAR,
-    "SourceCodePro-Semibold.woff" => static_files::source_code_pro::SEMIBOLD,
-    "SourceCodePro-LICENSE.txt" => static_files::source_code_pro::LICENSE,
-    "LICENSE-MIT.txt" => static_files::LICENSE_MIT,
-    "LICENSE-APACHE.txt" => static_files::LICENSE_APACHE,
-    "COPYRIGHT.txt" => static_files::COPYRIGHT,
+crate static FILES_UNVERSIONED: Lazy<FxHashMap<&str, &[u8]>> = Lazy::new(|| {
+    map! {
+        "FiraSans-Regular.woff2" => static_files::fira_sans::REGULAR2,
+        "FiraSans-Medium.woff2" => static_files::fira_sans::MEDIUM2,
+        "FiraSans-Regular.woff" => static_files::fira_sans::REGULAR,
+        "FiraSans-Medium.woff" => static_files::fira_sans::MEDIUM,
+        "FiraSans-LICENSE.txt" => static_files::fira_sans::LICENSE,
+        "SourceSerifPro-Regular.ttf.woff" => static_files::source_serif_pro::REGULAR,
+        "SourceSerifPro-Bold.ttf.woff" => static_files::source_serif_pro::BOLD,
+        "SourceSerifPro-It.ttf.woff" => static_files::source_serif_pro::ITALIC,
+        "SourceSerifPro-LICENSE.md" => static_files::source_serif_pro::LICENSE,
+        "SourceCodePro-Regular.woff" => static_files::source_code_pro::REGULAR,
+        "SourceCodePro-Semibold.woff" => static_files::source_code_pro::SEMIBOLD,
+        "SourceCodePro-LICENSE.txt" => static_files::source_code_pro::LICENSE,
+        "LICENSE-MIT.txt" => static_files::LICENSE_MIT,
+        "LICENSE-APACHE.txt" => static_files::LICENSE_APACHE,
+        "COPYRIGHT.txt" => static_files::COPYRIGHT,
 });
 
 
 pub(super) fn write_shared(
Diff in /checkout/src/librustdoc/lib.rs at line 524:
                 "Generate JSON file at the top level instead of generating HTML redirection files",
         }),
         }),
-        unstable("print", |o| o.optmulti("", "print", "Rustdoc information to print on stdout", "[unversioned-files]")),
+        unstable("print", |o| {
+            o.optmulti("", "print", "Rustdoc information to print on stdout", "[unversioned-files]")
+        }),
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/strip_private.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/passes/propagate_doc_cfg.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/passes/unindent_comments.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/clean/auto_trait.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:12
