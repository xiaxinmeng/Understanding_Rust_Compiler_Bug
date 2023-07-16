patch
From 8178487a239a5649124ad84bfb3e85824d101e18 Mon Sep 17 00:00:00 2001
From: Noah Lev <camelidcamel@gmail.com>
Date: Wed, 29 Sep 2021 13:03:16 -0700
Subject: [PATCH] Feature gate new `non_exhaustive_omitted_patterns` lint

---
 compiler/rustc_feature/src/active.rs                          | 3 +++
 compiler/rustc_lint_defs/src/builtin.rs                       | 2 ++
 compiler/rustc_span/src/symbol.rs                             | 1 +
 .../feature-gate-non_exhaustive_omitted_patterns.rs           | 4 ++++
 4 files changed, 10 insertions(+)
 create mode 100644 src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns.rs

diff --git a/compiler/rustc_feature/src/active.rs b/compiler/rustc_feature/src/active.rs
index f8b865e615c..9c0806dae16 100644
--- a/compiler/rustc_feature/src/active.rs
+++ b/compiler/rustc_feature/src/active.rs
@@ -678,6 +678,9 @@ pub fn set(&self, features: &mut Features, span: Span) {
     /// Allows `#[track_caller]` on closures and generators.
     (active, closure_track_caller, "1.57.0", Some(87417), None),
 
+    /// Allows using the `non_exhaustive_omitted_patterns` lint.
+    (active, non_exhaustive_omitted_patterns, "1.57.0", None, None),
+
     // -------------------------------------------------------------------------
     // feature-group-end: actual feature gates
     // -------------------------------------------------------------------------
diff --git a/compiler/rustc_lint_defs/src/builtin.rs b/compiler/rustc_lint_defs/src/builtin.rs
index 5830ce26fea..8a4d21f5d11 100644
--- a/compiler/rustc_lint_defs/src/builtin.rs
+++ b/compiler/rustc_lint_defs/src/builtin.rs
@@ -6,6 +6,7 @@
 
 use crate::{declare_lint, declare_lint_pass, FutureIncompatibilityReason};
 use rustc_span::edition::Edition;
+use rustc_span::symbol::sym;
 
 declare_lint! {
     /// The `forbidden_lint_groups` lint detects violations of
@@ -3510,4 +3511,5 @@
     pub NON_EXHAUSTIVE_OMITTED_PATTERNS,
     Allow,
     "detect when patterns of types marked `non_exhaustive` are missed",
+    @feature_gate = sym::non_exhaustive_omitted_patterns;
 }
diff --git a/compiler/rustc_span/src/symbol.rs b/compiler/rustc_span/src/symbol.rs
index 7cb4e18398c..6b89cb6b4aa 100644
--- a/compiler/rustc_span/src/symbol.rs
+++ b/compiler/rustc_span/src/symbol.rs
@@ -887,6 +887,7 @@
         nomem,
         non_ascii_idents,
         non_exhaustive,
+        non_exhaustive_omitted_patterns,
         non_modrs_mods,
         none_error,
         nontemporal_store,
diff --git a/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns.rs b/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns.rs
new file mode 100644
index 00000000000..9952b4b3283
--- /dev/null
+++ b/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns.rs
@@ -0,0 +1,4 @@
+#![deny(non_exhaustive_omitted_patterns)] //~ ERROR unstable
+#![allow(non_exhaustive_omitted_patterns)] //~ ERROR unstable
+
+fn main() {}
-- 
2.29.2
