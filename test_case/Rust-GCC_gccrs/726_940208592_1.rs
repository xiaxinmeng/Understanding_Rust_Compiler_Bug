
$ diff -Nru rust-hir-expr.h <(clang-format rust-hir-expr.h )
--- rust-hir-expr.h     2021-10-11 19:12:09.956572139 +0200
+++ /dev/fd/63  2021-10-11 19:12:33.541069345 +0200
@@ -759,7 +759,7 @@
 // Value array elements
 class ArrayElemsValues : public ArrayElems
 {
-  std::vector<     std::unique_ptr<   Expr> >     values;
+  std::vector<std::unique_ptr<Expr> > values;

   // TODO: should this store location data?
