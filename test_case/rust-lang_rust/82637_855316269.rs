diff
diff --git a/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.stderr b/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.stderr
index 919b18632e6..ba5391dd803 100644
--- a/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.stderr
+++ b/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.stderr
@@ -1,13 +1,8 @@
 error[E0277]: `<<Self as Case1>::A as Iterator>::Item` doesn't implement `Debug`
   --> $DIR/bounds-on-assoc-in-trait.rs:18:28
    |
-LL |     type A: Iterator<Item: Debug>;
-   |                            ^^^^^ `<<Self as Case1>::A as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-   |
-  ::: $SRC_DIR/core/src/fmt/mod.rs:LL:COL
-   |
-LL | pub trait Debug {
-   | --------------- required by this bound in `Debug`
+LL |       type A: Iterator<Item: Debug>;
+   |                              ^^^^^ `<<Self as Case1>::A as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
    |
    = help: the trait `Debug` is not implemented for `<<Self as Case1>::A as Iterator>::Item`
 help: consider further restricting the associated type
