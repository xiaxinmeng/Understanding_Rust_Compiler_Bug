diff
 ---- [ui] tests/ui/consts/const_unsafe_unreachable_ub.rs stdout ----
diff of stderr:

1	error[E0080]: evaluation of constant value failed
-	  --> $SRC_DIR/core/src/hint.rs:LL:COL
+	  --> $SRC_DIR/core/src/panicking.rs:LL:COL
3	   |
-	   = note: entering unreachable code
+	   = note: the evaluated program panicked at 'hint::unreachable_unchecked must never be reached', $SRC_DIR/core/src/panicking.rs:LL:COL
5	   |
