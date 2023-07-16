 diff
diff --git a/src/libsyntax/print/pprust.rs b/src/libsyntax/print/pprust.rs
index 67817ee..685f1e9 100644
--- a/src/libsyntax/print/pprust.rs
+++ b/src/libsyntax/print/pprust.rs
@@ -425,7 +425,7 @@ pub fn variant_to_string(var: &ast::Variant) -> String {
 }

 pub fn arg_to_string(arg: &ast::Arg) -> String {
-    to_string(|s| s.print_arg(arg))
+    to_string(|s| s.print_arg(arg, false))
 }

 pub fn mac_to_string(arg: &ast::Mac) -> String {
@@ -2672,7 +2672,7 @@ impl<'a> State<'a> {
     }

     pub fn print_fn_args(&mut self, decl: &ast::FnDecl,
-                         opt_explicit_self: Option<&ast::ExplicitSelf_>)
+                         opt_explicit_self: Option<&ast::ExplicitSelf_>, is_closure: bool)
         -> io::Result<()> {
         // It is unfortunate to duplicate the commasep logic, but we want the
         // self type and the args all in the same box.
@@ -2698,7 +2698,7 @@ impl<'a> State<'a> {

         for arg in args {
             if first { first = false; } else { try!(self.word_space(",")); }
-            try!(self.print_arg(arg));
+            try!(self.print_arg(arg, is_closure));
         }

         self.end()
@@ -2708,7 +2708,7 @@ impl<'a> State<'a> {
                                  opt_explicit_self: Option<&ast::ExplicitSelf_>)
         -> io::Result<()> {
         try!(self.popen());
-        try!(self.print_fn_args(decl, opt_explicit_self));
+        try!(self.print_fn_args(decl, opt_explicit_self, true));
         if decl.variadic {
             try!(word(&mut self.s, ", ..."));
         }
@@ -2722,7 +2722,7 @@ impl<'a> State<'a> {
             decl: &ast::FnDecl)
             -> io::Result<()> {
         try!(word(&mut self.s, "|"));
-        try!(self.print_fn_args(decl, None));
+        try!(self.print_fn_args(decl, None, false));
         try!(word(&mut self.s, "|"));

         if let ast::DefaultReturn(..) = decl.output {
@@ -2967,10 +2967,10 @@ impl<'a> State<'a> {
         self.print_type(&*mt.ty)
     }

-    pub fn print_arg(&mut self, input: &ast::Arg) -> io::Result<()> {
+    pub fn print_arg(&mut self, input: &ast::Arg, is_closure: bool) -> io::Result<()> {
         try!(self.ibox(INDENT_UNIT));
         match input.ty.node {
-            ast::TyInfer => try!(self.print_pat(&*input.pat)),
+            ast::TyInfer if is_closure == true => try!(self.print_pat(&*input.pat)),
             _ => {
                 match input.pat.node {
                     ast::PatIdent(_, ref path1, _) if
