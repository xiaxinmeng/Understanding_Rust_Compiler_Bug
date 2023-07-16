diff
--- a/compiler/rustc_codegen_ssa/src/back/link.rs
+++ b/compiler/rustc_codegen_ssa/src/back/link.rs
@@ -2157,6 +2157,7 @@ fn add_upstream_native_libraries(
             last = if (lib.kind, lib.name) == last { continue } else { (lib.kind, lib.name) };
 
             let verbatim = lib.verbatim.unwrap_or(false);
+            info!("link native lib: {:?} from crate #{}", lib, codegen_results.crate_info.crate_name[&cnum]);
             match lib.kind {
                 NativeLibKind::Dylib { as_needed } => {
                     cmd.link_dylib(name, verbatim, as_needed.unwrap_or(true))
