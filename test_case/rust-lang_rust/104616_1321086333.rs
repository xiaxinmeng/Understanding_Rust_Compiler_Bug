
diff of 64bit.stderr:

164	   |
165	   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
166	   = note: the raw bytes of the constant (size: 16, align: 8) {
-	               ╾───────ALLOC_ID───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........
+	               ╾──────ALLOC_ID───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........
168	           }
169	
170	error[E0080]: it is undefined behavior to use this value
