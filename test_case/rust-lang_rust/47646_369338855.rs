diff
diff --git a/src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs b/src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs
index 948c1ac0b136..9919a6e9b54f 100644
--- a/src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs
+++ b/src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs
@@ -50,15 +50,23 @@ impl<'cx, 'gcx, 'tcx> MirBorrowckCtxt<'cx, 'gcx, 'tcx> {
                     Cause::DropVar(local, location) => {
                         match find_drop_use(&mir, regioncx, borrow, location, local) {
                             Some(p) => {
-                                let local_name = &mir.local_decls[local].name.unwrap();
-
-                                err.span_label(
-                                    mir.source_info(p).span,
-                                    format!(
-                                        "borrow later used here, when `{}` is dropped",
-                                        local_name
-                                    ),
-                                );
+                                match &mir.local_decls[local].name {
+                                    Some(local_name) => {
+                                        err.span_label(
+                                            mir.source_info(p).span,
+                                            format!(
+                                                "borrow later used here, when `{}` is dropped",
+                                                local_name
+                                            ),
+                                        );
+                                    }
+                                    None => {
+                                        err.span_label(
+                                            mir.source_info(p).span,
+                                            "borrow later used here, when binding is dropped"
+                                        );
+                                    }
+                                }
                             }
 
                             None => {
diff --git a/src/test/ui/issue-47646.rs b/src/test/ui/issue-47646.rs
new file mode 100644
index 000000000000..040e56d200ec
--- /dev/null
+++ b/src/test/ui/issue-47646.rs
@@ -0,0 +1,29 @@
+// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
+// file at the top-level directory of this distribution and at
+// http://rust-lang.org/COPYRIGHT.
+//
+// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
+// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
+// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
+// option. This file may not be copied, modified, or distributed
+// except according to those terms.
+
+//compile-flags: -Znll-dump-cause
+
+
+#![allow(warnings)]
+#![feature(nll)]
+
+use std::collections::BinaryHeap;
+
+fn main() {
+    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
+    let borrow = heap.peek_mut();
+
+    match (borrow, ()) {
+        (Some(_), ()) => {
+            println!("{:?}", heap); //~ ERROR cannot borrow `heap` as immutable
+        },
+        _ => {},
+    };
+}
diff --git a/src/test/ui/issue-47646.stderr b/src/test/ui/issue-47646.stderr
new file mode 100644
index 000000000000..a58d30f032bd
--- /dev/null
+++ b/src/test/ui/issue-47646.stderr
@@ -0,0 +1,14 @@
+error[E0502]: cannot borrow `heap` as immutable because it is also borrowed as mutable
+  --> $DIR/issue-47646.rs:25:30
+   |
+21 |     let borrow = heap.peek_mut();
+   |                  ---- mutable borrow occurs here
+...
+25 |             println!("{:?}", heap); //~ ERROR cannot borrow `heap` as immutable
+   |                              ^^^^ immutable borrow occurs here
+...
+28 |     };
+   |      - borrow later used here, when binding is dropped
+
+error: aborting due to previous error
+
