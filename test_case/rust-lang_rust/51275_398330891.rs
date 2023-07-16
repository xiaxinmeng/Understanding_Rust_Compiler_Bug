
---- [ui (nll)] ui\asm-out-assign-imm.rs stdout ----
diff of stderr:
1	error[E0384]: cannot assign twice to immutable variable `x`
-	  --> $DIR/asm-out-assign-imm.rs:31:9
+	  --> $DIR/asm-out-assign-imm.rs:33:9
3	   |
4	LL |     let x: isize;
5	   |         - consider changing this to `mut x`
