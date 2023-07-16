
diff --git a/compiler/rustc_mir/src/dataflow/framework/graphviz.rs b/compiler/rustc_mir/src/dataflow/framework/graphviz.rs
index 179c471cf48..83cd360b0ec 100644
--- a/compiler/rustc_mir/src/dataflow/framework/graphviz.rs
+++ b/compiler/rustc_mir/src/dataflow/framework/graphviz.rs
@@ -1,6 +1,7 @@
 //! A helpful diagram for debugging dataflow problems.
 
 use std::borrow::Cow;
+use std::lazy::SyncOnceCell;
 use std::{io, ops, str};
 
 use regex::Regex;
@@ -570,6 +571,13 @@ where
     }
 }
 
+macro_rules! regex {
+    ($re:literal $(,)?) => {{
+        static RE: SyncOnceCell<regex::Regex> = SyncOnceCell::new();
+        RE.get_or_init(|| Regex::new($re).unwrap())
+    }};
+}
+
 fn diff_pretty<T, C>(new: T, old: T, ctxt: &C) -> String
 where
     T: DebugWithContext<C>,
@@ -578,7 +586,7 @@ where
         return String::new();
     }
 
-    let re = Regex::new("\u{001f}([+-])").unwrap();
+    let re = regex!("\u{001f}([+-])");
 
     let raw_diff = format!("{:#?}", DebugDiffWithAdapter { new, old, ctxt });
 
diff --git a/compiler/rustc_mir/src/lib.rs b/compiler/rustc_mir/src/lib.rs
index 42717f27384..8d2253b4d82 100644
--- a/compiler/rustc_mir/src/lib.rs
+++ b/compiler/rustc_mir/src/lib.rs
@@ -27,6 +27,7 @@ Rust MIR: a lowered representation of Rust.
 #![feature(trait_alias)]
 #![feature(option_expect_none)]
 #![feature(or_patterns)]
+#![feature(once_cell)]
 #![recursion_limit = "256"]
 
 #[macro_use]
