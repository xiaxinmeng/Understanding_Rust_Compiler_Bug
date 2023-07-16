
 ---- [ui] ui/lint/lint-exceeding-bitshifts.rs#opt stdout ----
diff of stderr:

140	  --> $DIR/lint-exceeding-bitshifts.rs:77:15
141	   |
142	LL |       let n = 1_isize << BITS;
-	   |               ^^^^^^^^^^^^^^^ attempt to shift left by `64_usize`, which would overflow
+	   |               ^^^^^^^^^^^^^^^ attempt to shift left by `32_usize`, which would overflow
144	
145	warning: this arithmetic operation will overflow
146	  --> $DIR/lint-exceeding-bitshifts.rs:78:15

147	   |
148	LL |       let n = 1_usize << BITS;
-	   |               ^^^^^^^^^^^^^^^ attempt to shift left by `64_usize`, which would overflow
+	   |               ^^^^^^^^^^^^^^^ attempt to shift left by `32_usize`, which would overflow
150	
151	warning: 24 warnings emitted
