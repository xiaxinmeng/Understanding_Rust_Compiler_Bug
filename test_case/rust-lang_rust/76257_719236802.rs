
 ---- [ui] ui/issues/issue-75777.rs stdout ----
diff of stderr:

14	   |
15	LL |     Box::new(move |_| fut)
16	   |              ^^^^^^^^^^^^
-	   = note: expected `Pin<Box<dyn Future<Output = A> + Send>>`
-	              found `Pin<Box<(dyn Future<Output = A> + Send + 'a)>>`
+	   = note: expected `(Pin<Box<dyn Future<Output = A> + Send>>,)`
+	              found `(Pin<Box<(dyn Future<Output = A> + Send + 'a)>>,)`
