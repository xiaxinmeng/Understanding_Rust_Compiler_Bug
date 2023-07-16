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
Diff in /checkout/src/librustdoc/passes/non_autolinks.rs at line 4:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/non_autolinks.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 use crate::fold::DocFolder;
 use crate::html::markdown::opts;
 use core::ops::Range;
-use std::lazy::SyncLazy;
-use std::mem;
 use pulldown_cmark::{Event, Parser, Tag};
 use regex::Regex;
 use rustc_errors::Applicability;
Diff in /checkout/src/librustdoc/passes/non_autolinks.rs at line 12:
 use rustc_session::lint;
+use std::lazy::SyncLazy;
+use std::mem;
 crate const CHECK_NON_AUTOLINKS: Pass = Pass {
 crate const CHECK_NON_AUTOLINKS: Pass = Pass {
     name: "check-non-autolinks",
Diff in /checkout/src/librustdoc/passes/non_autolinks.rs at line 17:
     description: "detects URLs that could be linkified",
 
 
-const URL_REGEX: SyncLazy<Regex> = SyncLazy::new(|| Regex::new(concat!(
-    r"https?://",                          // url scheme
-    r"([-a-zA-Z0-9@:%._\+~#=]{2,256}\.)+", // one or more subdomains
-    r"[a-zA-Z]{2,63}",                     // root domain
-    r"\b([-a-zA-Z0-9@:%_\+.~#?&/=]*)"      // optional query or url fragments
-)).expect("failed to build regex"));
+const URL_REGEX: SyncLazy<Regex> = SyncLazy::new(|| {
+    Regex::new(concat!(
+        r"https?://",                          // url scheme
+        r"([-a-zA-Z0-9@:%._\+~#=]{2,256}\.)+", // one or more subdomains
+        r"[a-zA-Z]{2,63}",                     // root domain
+        r"\b([-a-zA-Z0-9@:%_\+.~#?&/=]*)"      // optional query or url fragments
+    ))
+    .expect("failed to build regex")
 
 
 struct NonAutolinksLinter<'a, 'tcx> {
     cx: &'a DocContext<'tcx>,
Diff in /checkout/src/librustdoc/passes/non_autolinks.rs at line 90:
                     Event::Start(tag @ (Tag::CodeBlock(_) | Tag::Link(..))) => {
                         while let Some((event, _)) = p.next() {
                             match event {
-                                Event::End(end) if mem::discriminant(&end) == mem::discriminant(&tag) => break,
+                                Event::End(end)
+                                    if mem::discriminant(&end) == mem::discriminant(&tag) =>
+                                    break;
+                                }
                                 _ => {}
                             }
