

---- [ui] ui/target-feature-gate.rs stdout ----


executing "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/ui/target-feature-gate.rs" "--target=s390x-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/ui/target-feature-gate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-L" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/ui/target-feature-gate/auxiliary" "-A" "unused"
------stdout------------------------------

------stderr------------------------------
{"message":"the feature named `avx512bw` is not valid for this target","code":null,"level":"error","spans":[{"file_name":"/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/ui/target-feature-gate.rs","byte_start":990,"byte_end":1009,"line_start":31,"line_end":31,"column_start":18,"column_end":37,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"avx512bw\")]","highlight_start":18,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the feature named `avx512bw` is not valid for this target\n  --> /<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/ui/target-feature-gate.rs:31:18\n   |\nLL | #[target_feature(enable = \"avx512bw\")]\n   |                  ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}

------------------------------------------
diff of stderr:

-	error[E0658]: the target feature `avx512bw` is currently unstable (see issue #44839)
+	error: the feature named `avx512bw` is not valid for this target
2	  --> $DIR/target-feature-gate.rs:31:18
3	   |
4	LL | #[target_feature(enable = "avx512bw")]

5	   |                  ^^^^^^^^^^^^^^^^^^^
-	   |
-	   = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
8	
9	error: aborting due to previous error
10	

-	For more information about this error, try `rustc --explain E0658`.
12
