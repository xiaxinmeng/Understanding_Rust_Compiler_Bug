patch
diff --git a/src/librustc/traits/project.rs b/src/librustc/traits/project.rs
index 76bead9..24731e8 100644
--- a/src/librustc/traits/project.rs
+++ b/src/librustc/traits/project.rs
@@ -1215,8 +1215,8 @@ fn confirm_closure_candidate<'cx, 'gcx, 'tcx>(
                                obligation,
                                &closure_type.sig,
                                util::TupleArgumentsFlag::No)
-        .with_addl_obligations(obligations)
         .with_addl_obligations(vtable.nested)
+        .with_addl_obligations(obligations)
 }
 
 fn confirm_callable_candidate<'cx, 'gcx, 'tcx>(
