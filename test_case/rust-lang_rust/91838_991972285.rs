plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/tests/array.rs at line 620:
     let array3b: [i32; 3] = [3, 2, 1];
     let array4: [i32; 4] = [1, 2, 3, 4];
 
-    let slice3: &[i32] = &{array3};
-    let slice3b: &[i32] = &{array3b};
-    let slice4: &[i32] = &{array4};
+    let slice3: &[i32] = &{ array3 };
+    let slice3b: &[i32] = &{ array3b };
+    let slice4: &[i32] = &{ array4 };
     assert!(array3 == slice3);
     assert!(array3 != slice3b);
     assert!(array3 != slice4);
Diff in /checkout/library/core/tests/array.rs at line 630:
     assert!(slice3b != array3);
     assert!(slice4 != array3);
 
-    let mut3: &mut [i32] = &mut {array3};
-    let mut3b: &mut [i32] = &mut {array3b};
-    let mut4: &mut [i32] = &mut {array4};
+    let mut3: &mut [i32] = &mut { array3 };
+    let mut3b: &mut [i32] = &mut { array3b };
+    let mut4: &mut [i32] = &mut { array4 };
     assert!(array3 == mut3);
     assert!(array3 != mut3b);
     assert!(array3 != mut4);
Diff in /checkout/library/core/tests/array.rs at line 645:
 fn array_mixed_equality_nans() {
     let array3: [f32; 3] = [1.0, std::f32::NAN, 3.0];
 
-    let slice3: &[f32] = &{array3};
+    let slice3: &[f32] = &{ array3 };
     assert!(!(array3 == slice3));
     assert!(array3 != slice3);
     assert!(!(slice3 == array3));
Diff in /checkout/library/core/tests/array.rs at line 652:
     assert!(slice3 != array3);
 
-    let mut3: &mut [f32] = &mut {array3};
+    let mut3: &mut [f32] = &mut { array3 };
     assert!(!(array3 == mut3));
     assert!(array3 != mut3);
     assert!(!(mut3 == array3));
Build completed unsuccessfully in 0:00:13
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/num/nan.rs" "/checkout/library/core/tests/num/u128.rs" "/checkout/library/core/tests/array.rs" "/checkout/library/core/tests/num/i16.rs" "/checkout/library/core/tests/ops.rs" "/checkout/library/core/tests/num/i128.rs" "/checkout/library/core/tests/task.rs" "/checkout/library/core/tests/const_ptr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
