 diff
diff --git a/src/rustc/back/link.rs b/src/rustc/back/link.rs
index 114f02f..1a20084 100644
--- a/src/rustc/back/link.rs
+++ b/src/rustc/back/link.rs
@@ -13,6 +13,7 @@ import syntax::print::pprust;
 import lib::llvm::{ModuleRef, mk_pass_manager, mk_target_data, True, False};
 import util::filesearch;
 import middle::ast_map::{path, path_mod, path_name};
+import io::writer_util;

 enum output_type {
     output_type_none,
@@ -448,7 +449,6 @@ fn sanitize(s: str) -> str {
     let mut result = "";
     str::chars_iter(s) {|c|
         alt c {
-          '@' { result += "_sbox_"; }
           '~' { result += "_ubox_"; }
           '*' { result += "_ptr_"; }
           '&' { result += "_ref_"; }
@@ -458,7 +458,7 @@ fn sanitize(s: str) -> str {
           'a' to 'z'
           | 'A' to 'Z'
           | '0' to '9'
-          | '_' { str::push_char(result,c); }
+          | '_' | '@' | '.' { str::push_char(result,c); }
           _ {
             if c > 'z' && char::is_XID_continue(c) {
                 str::push_char(result,c);
@@ -485,12 +485,13 @@ fn mangle(ss: path) -> str {
 }

 fn exported_name(path: path, hash: str, vers: str) -> str {
-    ret mangle(path + [path_name(hash)] + [path_name(vers)]);
+    ret mangle(path + [path_name(hash)]) + "@VERS_" + sanitize(vers);
 }

 fn mangle_exported_name(ccx: @crate_ctxt, path: path, t: ty::t) -> str {
     let hash = get_symbol_hash(ccx, t);
-    ret exported_name(path, hash, ccx.link_meta.vers);
+    let exp = exported_name(path, hash, ccx.link_meta.vers);
+    ret exp;
 }

 fn mangle_internal_name_by_type_only(ccx: @crate_ctxt, t: ty::t, name: str) ->
@@ -602,6 +603,18 @@ fn link_binary(sess: session,
     if sess.building_library {
         cc_args += [lib_cmd];

+        if sess.targ_cfg.os != session::os_macos {
+            let version_script = out_filename + ".verscript";
+            #debug["version_script: %s", version_script];
+            alt io::file_writer(version_script, [io::create]) {
+                result::ok(w) {
+                    w.write_line("VERS_" + lm.vers + " { global: *; };");
+                }
+                _ { fail }
+            };
+            cc_args += ["-Wl,--version-script=" + version_script];
+        }
+
         // On mac we need to tell the linker to let this library
         // be rpathed
         if sess.targ_cfg.os == session::os_macos {

