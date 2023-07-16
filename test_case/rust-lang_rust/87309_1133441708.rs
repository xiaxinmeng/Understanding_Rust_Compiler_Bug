diff
diff --git a/src/test/ui/async-await/issue-70935-complex-spans.rs b/src/test/ui/async-await/issue-70935-complex-spans.rs
index 2965a7e0654..be0c09bcb92 100644
--- a/src/test/ui/async-await/issue-70935-complex-spans.rs
+++ b/src/test/ui/async-await/issue-70935-complex-spans.rs
@@ -1,4 +1,5 @@
 // edition:2018
+// -Zdrop-tracking
 // #70935: Check if we do not emit snippet
 // with newlines which lead complex diagnostics.
 
diff --git a/src/test/ui/async-await/issue-70935-complex-spans.stderr b/src/test/ui/async-await/issue-70935-complex-spans.stderr
index db309938119..db6e17f3f5d 100644
--- a/src/test/ui/async-await/issue-70935-complex-spans.stderr
+++ b/src/test/ui/async-await/issue-70935-complex-spans.stderr
@@ -1,12 +1,12 @@
 error: future cannot be sent between threads safely
-  --> $DIR/issue-70935-complex-spans.rs:10:45
+  --> $DIR/issue-70935-complex-spans.rs:11:45
    |
 LL | fn foo(tx: std::sync::mpsc::Sender<i32>) -> impl Future + Send {
    |                                             ^^^^^^^^^^^^^^^^^^ future created by async block is not `Send`
    |
    = help: the trait `Sync` is not implemented for `Sender<i32>`
 note: future is not `Send` as this value is used across an await
-  --> $DIR/issue-70935-complex-spans.rs:15:11
+  --> $DIR/issue-70935-complex-spans.rs:16:11
    |
 LL |           baz(|| async{
    |  _____________-
@@ -14,9 +14,9 @@ LL | |             foo(tx.clone());
 LL | |         }).await;
    | |         - ^^^^^^ await occurs here, with the value maybe used later
    | |_________|
-   |           has type `[closure@$DIR/issue-70935-complex-spans.rs:13:13: 15:10]` which is not `Send`
+   |           has type `[closure@$DIR/issue-70935-complex-spans.rs:14:13: 16:10]` which is not `Send`
 note: the value is later dropped here
-  --> $DIR/issue-70935-complex-spans.rs:15:17
+  --> $DIR/issue-70935-complex-spans.rs:16:17
    |
 LL |         }).await;
    |                 ^
