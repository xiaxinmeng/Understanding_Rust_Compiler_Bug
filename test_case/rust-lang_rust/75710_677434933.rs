
---- [ui] ui/consts/const-eval/const-eval-query-stack.rs stdout ----
diff of stderr:

8	   |
9	   = note: `#[deny(const_err)]` on by default
10	
-	query stack during panic:
-	#0 [const_eval_raw] const-evaluating `main::promoted[1]`
-	#1 [const_eval_validated] const-evaluating + checking `main::promoted[1]`
-	#2 [const_eval_validated] const-evaluating + checking `main::promoted[1]`
-	#3 [normalize_generic_arg_after_erasing_regions] normalizing `main::promoted[1]`
-	#4 [optimized_mir] optimizing MIR for `main`
-	#5 [collect_and_partition_mono_items] collect_and_partition_mono_items
-	end of query stack
+	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
19	
