diff
diff --cc src/librustdoc/clean/mod.rs
index c992a5388d1,52d693463e4..00000000000
--- a/src/librustdoc/clean/mod.rs
+++ b/src/librustdoc/clean/mod.rs
@@@ -2377,8 -2459,8 +2377,8 @@@ fn clean_extern_crate<'tcx>
      orig_name: Option<Symbol>,
      cx: &mut DocContext<'tcx>,
  ) -> Vec<Item> {
-     // this is the ID of the `extern crate` statement
-     let cnum = cx.tcx.extern_mod_stmt_cnum(krate.owner_id.def_id).unwrap_or(LOCAL_CRATE);
+     // this is the ID of the `extern crate` statement -- should always return CrateNum struct from currently-compiled crate
 -    let cnum = cx.tcx.extern_mod_stmt_cnum(krate.owner_id.def_id).unwrap();
++    let cnum = cx.tcx.extern_mod_stmt_cnum(krate.owner_id.def_id).unwrap_or_else(|| span_bug!(krate.span, "missing extern mod stmt"));
      // this is the ID of the crate itself
      let crate_def_id = cnum.as_def_id();
      let attrs = cx.tcx.hir().attrs(krate.hir_id());
