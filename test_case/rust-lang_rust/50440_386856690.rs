
---- [ui] ui\single-use-lifetime\one-use-in-fn-argument-in-band.rs stdout ----
	diff of stderr:
-	error: lifetime parameter `'b` only used once
-	  --> $DIR/one-use-in-fn-argument-in-band.rs:19:22
+	error: lifetime parameter `'a` only used once
+	  --> $DIR/one-use-in-fn-argument-in-band.rs:19:10
3	   |
4	LL | fn a(x: &'a u32, y: &'b u32) {
-	   |                      ^^
+	   |          ^^
6	   |
7	note: lint level defined here
8	  --> $DIR/one-use-in-fn-argument-in-band.rs:12:9
10	LL | #![deny(single_use_lifetime)]
11	   |         ^^^^^^^^^^^^^^^^^^^
12	
-	error: lifetime parameter `'a` only used once
-	  --> $DIR/one-use-in-fn-argument-in-band.rs:19:10
+	error: lifetime parameter `'b` only used once
+	  --> $DIR/one-use-in-fn-argument-in-band.rs:19:22
15	   |
16	LL | fn a(x: &'a u32, y: &'b u32) {
-	   |          ^^
+	   |                      ^^
18	
19	error: aborting due to 2 previous errors
20	
