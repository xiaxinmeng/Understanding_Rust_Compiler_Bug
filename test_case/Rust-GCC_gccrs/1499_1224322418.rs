
diff --git a/gcc/rust/backend/rust-constexpr.cc b/gcc/rust/backend/rust-constexpr.cc
index ed0fc2ce345..77e8a059465 100644
--- a/gcc/rust/backend/rust-constexpr.cc
+++ b/gcc/rust/backend/rust-constexpr.cc
@@ -2080,15 +2080,7 @@ eval_constant_expression (const constexpr_ctx *ctx, tree t, bool lval,
                                    non_constant_p, overflow_p);
       break;
 
-    case NOP_EXPR:
-      if (REINTERPRET_CAST_P (t))
-       {
-         if (!ctx->quiet)
-           error_at (loc, "%<reinterpret_cast%> is not a constant expression");
-         *non_constant_p = true;
-         return t;
-       }
-      /* FALLTHROUGH.  */
+
     case MODIFY_EXPR:
       r = eval_store_expression (ctx, t, false, non_constant_p, overflow_p);
       break;
@@ -2347,6 +2339,7 @@ eval_constant_expression (const constexpr_ctx *ctx, tree t, bool lval,
       break;
 
       /* FALLTHROUGH.  */
+    case NOP_EXPR:
     case CONVERT_EXPR:
       case VIEW_CONVERT_EXPR: {
        tree oldop = TREE_OPERAND (t, 0);
