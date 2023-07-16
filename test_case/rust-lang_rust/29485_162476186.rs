 patch
diff --git a/src/librustc_trans/trans/attributes.rs b/src/librustc_trans/trans/attributes.rs
index 28dfa4e..32dd31d 100644
--- a/src/librustc_trans/trans/attributes.rs
+++ b/src/librustc_trans/trans/attributes.rs
@@ -10,7 +10,7 @@
 //! Set and unset common attributes on LLVM values.

 use libc::{c_uint, c_ulonglong};
-use llvm::{self, ValueRef, AttrHelper};
+use llvm::{self, ValueRef};
 use middle::ty;
 use middle::infer;
 use session::config::NoDebugInfo;
@@ -117,7 +117,6 @@ pub fn from_fn_attrs(ccx: &CrateContext, attrs: &[ast::Attribute], llfn: ValueRe
                                                llvm::ColdAttribute as u64)
             }
         } else if attr.check_name("allocator") {
-            llvm::Attribute::NoAlias.apply_llfn(llvm::ReturnIndex as c_uint, llfn);
         } else if attr.check_name("unwind") {
             unwind(llfn, true);
         }
@@ -190,24 +189,12 @@ pub fn from_fn_type<'a, 'tcx>(ccx: &CrateContext<'a, 'tcx>, fn_type: ty::Ty<'tcx
             // invisible to the program. We also know it's nonnull as well
             // as how many bytes we can dereference
             attrs.arg(1, llvm::Attribute::StructRet)
-                 .arg(1, llvm::Attribute::NoAlias)
                  .arg(1, llvm::Attribute::NoCapture)
                  .arg(1, llvm::DereferenceableAttribute(llret_sz));

             // Add one more since there's an outptr
             idx += 1;
         } else {
-            // The `noalias` attribute on the return value is useful to a
-            // function ptr caller.
-            match ret_ty.sty {
-                // `Box` pointer return values never alias because ownership
-                // is transferred
-                ty::TyBox(it) if common::type_is_sized(ccx.tcx(), it) => {
-                    attrs.ret(llvm::Attribute::NoAlias);
-                }
-                _ => {}
-            }
-
             // We can also mark the return value as `dereferenceable` in certain cases
             match ret_ty.sty {
                 // These are not really pointers but pairs, (pointer, len)
@@ -233,8 +220,7 @@ pub fn from_fn_type<'a, 'tcx>(ccx: &CrateContext<'a, 'tcx>, fn_type: ty::Ty<'tcx
                 // For non-immediate arguments the callee gets its own copy of
                 // the value on the stack, so there are no aliases. It's also
                 // program-invisible so can't possibly capture
-                attrs.arg(idx, llvm::Attribute::NoAlias)
-                     .arg(idx, llvm::Attribute::NoCapture)
+                attrs.arg(idx, llvm::Attribute::NoCapture)
                      .arg(idx, llvm::DereferenceableAttribute(llarg_sz));
             }

@@ -244,8 +230,6 @@ pub fn from_fn_type<'a, 'tcx>(ccx: &CrateContext<'a, 'tcx>, fn_type: ty::Ty<'tcx

             // `Box` pointer parameters never alias because ownership is transferred
             ty::TyBox(inner) => {
-                attrs.arg(idx, llvm::Attribute::NoAlias);
-
                 if common::type_is_sized(ccx.tcx(), inner) {
                     let llsz = machine::llsize_of_real(ccx, type_of::type_of(ccx, inner));
                     attrs.arg(idx, llvm::DereferenceableAttribute(llsz));
@@ -265,10 +249,6 @@ pub fn from_fn_type<'a, 'tcx>(ccx: &CrateContext<'a, 'tcx>, fn_type: ty::Ty<'tcx
                 // on memory dependencies rather than pointer equality
                 let interior_unsafe = mt.ty.type_contents(ccx.tcx()).interior_unsafe();

-                if mt.mutbl == hir::MutMutable || !interior_unsafe {
-                    attrs.arg(idx, llvm::Attribute::NoAlias);
-                }
-
                 if mt.mutbl == hir::MutImmutable && !interior_unsafe {
                     attrs.arg(idx, llvm::Attribute::ReadOnly);
                 }
diff --git a/src/librustc_trans/trans/foreign.rs b/src/librustc_trans/trans/foreign.rs
index 95e9e85..39858e9 100644
--- a/src/librustc_trans/trans/foreign.rs
+++ b/src/librustc_trans/trans/foreign.rs
@@ -363,8 +363,7 @@ pub fn trans_native_call<'blk, 'tcx>(bcx: Block<'blk, 'tcx>,
         // The outptr can be noalias and nocapture because it's entirely
         // invisible to the program. We also know it's nonnull as well
         // as how many bytes we can dereference
-        attrs.arg(1, llvm::Attribute::NoAlias)
-             .arg(1, llvm::Attribute::NoCapture)
+        attrs.arg(1, llvm::Attribute::NoCapture)
              .arg(1, llvm::DereferenceableAttribute(llret_sz));
     };
