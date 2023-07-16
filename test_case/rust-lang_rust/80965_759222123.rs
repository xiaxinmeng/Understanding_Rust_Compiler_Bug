
---- [ui] ui/panic-handler/weak-lang-item.rs stdout ----
diff of stderr:

10	LL | extern crate core as other_core;
11	   |
12	
-	error: language item required, but not found: `eh_personality`
-	
15	error: `#[panic_handler]` function required, but not found
+	
+	error: language item required, but not found: `eh_personality`
16	
17	error: aborting due to 3 previous errors
18	
