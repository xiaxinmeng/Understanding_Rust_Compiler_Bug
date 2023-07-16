diff
diff --git a/src/librustc_trans/mir/mod.rs b/src/librustc_trans/mir/mod.rs
index b367eb6548d0..56e545cf8c3a 100644
--- a/src/librustc_trans/mir/mod.rs
+++ b/src/librustc_trans/mir/mod.rs
@@ -492,7 +492,7 @@ fn arg_local_refs<'a, 'tcx>(bx: &Builder<'a, 'tcx>,
                 };

                 if let PassMode::Indirect(ref attrs) = arg.mode {
-                    if !attrs.contains(ArgAttribute::ByVal) {
+                    if !attrs.contains(ArgAttribute::ByVal) && false {
                         variable_access = VariableAccess::IndirectVariable {
                             alloca: place.llval,
                             address_operations: &deref_op,
