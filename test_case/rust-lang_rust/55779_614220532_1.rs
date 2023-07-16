
diff --git a/src/test/ui/privacy/private-inferred-type-3.stderr b/src/test/ui/privacy/private-inferred-type-3.stderr
index 39ef6472526..a54fb8f7a19 100644
--- a/src/test/ui/privacy/private-inferred-type-3.stderr
+++ b/src/test/ui/privacy/private-inferred-type-3.stderr
@@ -1,4 +1,4 @@
-error: type `fn() {ext::priv_fn}` is private
+error: type `fn() {private_inferred_type::priv_fn}` is private
   --> $DIR/private-inferred-type-3.rs:16:5
    |
 LL |     ext::m!();
@@ -14,7 +14,7 @@ LL |     ext::m!();
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

-error: type `ext::PrivEnum` is private
+error: type `private_inferred_type::PrivEnum` is private
   --> $DIR/private-inferred-type-3.rs:16:5
    |
 LL |     ext::m!();
@@ -22,7 +22,7 @@ LL |     ext::m!();
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
