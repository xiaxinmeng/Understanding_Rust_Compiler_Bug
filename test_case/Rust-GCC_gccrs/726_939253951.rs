
$ gcc/rust/hir/tree$ git reset --hard 99c28309d3553346d4f0337dbae49f4a8e48da01
$ gcc/rust/hir/tree$ clang-format -i *
$ git status 
<nothing changed>
$ git reset --hard dkm/pr/dfaust/remove_lambda
$ gcc/rust/hir/tree$ clang-format -i *
$ git diff
 diff --git a/gcc/rust/hir/tree/rust-hir-expr.h b/gcc/rust/hir/tree/rust-hir-expr.h
index d9958a153be..5457ed7d993 100644
--- a/gcc/rust/hir/tree/rust-hir-expr.h
+++ b/gcc/rust/hir/tree/rust-hir-expr.h
@@ -759,12 +759,12 @@ protected:
 // Value array elements
 class ArrayElemsValues : public ArrayElems
 {
-  std::vector<std::unique_ptr<Expr> > values;
+  std::vector<std::unique_ptr<Expr>> values;

   // TODO: should this store location data?

 public:
-  ArrayElemsValues (std::vector<std::unique_ptr<Expr> > elems)
+  ArrayElemsValues (std::vector<std::unique_ptr<Expr>> elems)
     : values (std::move (elems))
   {}
....
