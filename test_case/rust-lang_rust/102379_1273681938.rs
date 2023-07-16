
---- [ui] src/test/ui/consts/const-eval/ub-enum.rs stdout ----
diff of 64bit.stderr:
6	   |
7	   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
8	   = note: the raw bytes of the constant (size: 8, align: 8) {
-	               01 00 00 00 00 00 00 00                         │ ........
+	               00 00 00 00 00 00 00 01                         │ ........
10	           }
11	
12	error: any use of this value will cause an error
117	   |
118	   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
119	   = note: the raw bytes of the constant (size: 8, align: 4) {
-	               78 00 00 00 ff ff ff ff                         │ x.......
+	               00 00 00 78 ff ff ff ff                         │ ...x....
121	           }
