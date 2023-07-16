
---- [ui] ui/issues/issue-72554.rs stdout ----
diff of stderr:

6	LL |     A(ElemDerived)
7	   |       ----------- recursive without indirection
8	   |
-	   = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `ElemDerived` representable
+	help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `ElemDerived` representable
+	   |
+	LL |     A(Box<ElemDerived>)
+	   |       ^^^^           ^
10	
11	error: aborting due to previous error
12	
