diff
Unstaged changes (1)
modified   src/lib.rs
@@ -201,10 +201,10 @@ pub mod python;
 mod err;
 mod conversion;
 mod instance;
+mod noargs;
 mod object;
 mod objects;
 mod objectprotocol;
-mod noargs;
 mod pythonrun;
 #[doc(hidden)]
 pub mod callback;
