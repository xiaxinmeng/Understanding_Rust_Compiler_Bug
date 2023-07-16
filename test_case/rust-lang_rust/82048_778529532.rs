plain
.................................................................................................... 7400/11440
.................................................................................................... 7500/11440
........................................................................i..ii....................... 7600/11440
........................................ii.......................................................... 7700/11440
...................................F.................i....F..i....F.F.......F....................... 7800/11440
......................................i............................................................. 8000/11440
.................................................................................................... 8100/11440
.................................................................................................... 8200/11440
..................i................................................................................. 8300/11440
---
failures:

---- [ui] ui/or-patterns/consistent-bindings.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/consistent-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/consistent-bindings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/consistent-bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/consistent-bindings.rs:23:9
   |
LL |     let Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b))): Result<_, Result<_, _>> =
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b))))`

error: top-level or-pattern cannot be followed by type annotation
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/or-patterns/consistent-bindings.rs:26:9
   |
LL |     let Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b))): Result<
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b))))`
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/or-patterns/inconsistent-modes.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let Ok(a) | Err(ref a): Result<&u8, u8> = Ok(&0);
+    |         ^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(a) | Err(ref a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let Ok(ref mut a) | Err(a): Result<u8, &mut u8> = Ok(0);
+    |         ^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(ref mut a) | Err(a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let Ok(ref a) | Err(ref mut a): Result<&u8, &mut u8> = Ok(&0);
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(ref a) | Err(ref mut a))`
+ 
1 error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
3    |

74               found type `&mut _`
74               found type `&mut _`
75    = note: a binding must have the same type in all alternatives
- error: aborting due to 9 previous errors
+ error: aborting due to 12 previous errors
78 
79 Some errors have detailed explanations: E0308, E0409.
79 Some errors have detailed explanations: E0308, E0409.
80 For more information about an error, try `rustc --explain E0308`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/inconsistent-modes/inconsistent-modes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/inconsistent-modes.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/inconsistent-modes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/inconsistent-modes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/inconsistent-modes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:7:9
   |
LL |     let Ok(a) | Err(ref a): Result<&u8, u8> = Ok(&0);
   |         ^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(a) | Err(ref a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:9:9
   |
LL |     let Ok(ref mut a) | Err(a): Result<u8, &mut u8> = Ok(0);
   |         ^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(ref mut a) | Err(a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:9
   |
LL |     let Ok(ref a) | Err(ref mut a): Result<&u8, &mut u8> = Ok(&0);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(ref a) | Err(ref mut a))`

error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:7:25
   |
LL |     let Ok(a) | Err(ref a): Result<&u8, u8> = Ok(&0);
   |            -            ^ bound in different ways
   |            first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:9:29
   |
LL |     let Ok(ref mut a) | Err(a): Result<u8, &mut u8> = Ok(0);
   |                    -        ^ bound in different ways
   |                    first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:33
   |
LL |     let Ok(ref a) | Err(ref mut a): Result<&u8, &mut u8> = Ok(&0);
   |                - first binding  ^ bound in different ways

error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:14:39
   |
LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
   |                 - first binding       ^ bound in different ways

error[E0409]: variable `b` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:14:46
   |
LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
   |                    - first binding           ^ bound in different ways

error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:20:38
   |
LL |     let Ok(Ok(a) | Err(a)) | Err(ref a) = Err(0);
   |                        -             ^ bound in different ways
   |                        first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:24:33
   |
LL |     let Ok([Ok((Ok(ref a) | Err(a),)) | Err(a)]) | Err(a) = Err(&1);
   |                        -        ^ bound in different ways
   |                        first binding

error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:25
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:25
   |
LL |     let Ok(ref a) | Err(ref mut a): Result<&u8, &mut u8> = Ok(&0);
   |            -----        ^^^^^^^^^   -------------------- expected due to this
   |            |            types differ in mutability
   |            |            types differ in mutability
   |            first introduced with type `&&u8` here
   = note: expected type `&&u8`
              found type `&mut &mut u8`
              found type `&mut &mut u8`
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:14:31
   |
   |
LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
   |             -----             ^^^^^^^^^            ----------- this expression has type `Result<({integer}, &{integer}), (_, _)>`
   |             |                 types differ in mutability
   |             |                 types differ in mutability
   |             first introduced with type `&{integer}` here
   |
   = note: expected type `&{integer}`
              found type `&mut _`
   = note: a binding must have the same type in all alternatives
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0308, E0409.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/or-patterns/or-patterns-syntactic-pass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-pass" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-pass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:28:9
   |
LL |     let A | B: u8;
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:30:9
   |
LL |     let A | B: u8 = 0;
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
error: aborting due to 2 previous errors


------------------------------------------
---

180    |         |
181    |         while parsing this or-pattern starting here
182 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let a | : u8 = 0;
+    |         ^^^ help: wrap the pattern in parentheses: `(a)`
+ 
183 error: a trailing `|` is not allowed in an or-pattern
185    |

196    |         |
197    |         while parsing this or-pattern starting here
---
201 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/remove-leading-vert/remove-leading-vert.stderr
thread '[ui] ui/or-patterns/remove-leading-vert.rs' panicked at 'failed to apply suggestions for "/checkout/src/test/ui/or-patterns/remove-leading-vert.rs" with rustfix', src/tools/compiletest/src/runtest.rs:3216:17
---- [ui] ui/or-patterns/or-patterns-syntactic-fail.rs stdout ----
diff of stderr:


16 LL |     fn fun2(| A | B: E) {}
17    |               ^^^^^ help: wrap the pattern in parenthesis: `(A | B)`
18 
+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/or-patterns-syntactic-fail.rs:25:9
+    |
+ LL |     let A | B: E = A;
+    |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/or-patterns-syntactic-fail.rs:28:11
+    |
+ LL |     let | A | B: E = A;
+    |           ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
+ 
19 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:30:11
+   --> $DIR/or-patterns-syntactic-fail.rs:40:11
21    |
22 LL |     let ( | A | B) = E::A;
23    |           ^ help: remove the `|`
24 
24 
25 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:31:11
+   --> $DIR/or-patterns-syntactic-fail.rs:41:11
27    |
28 LL |     let ( | A | B,) = (E::B,);
29    |           ^ help: remove the `|`
30 
30 
31 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:32:11
+   --> $DIR/or-patterns-syntactic-fail.rs:42:11
33    |
34 LL |     let [ | A | B ] = [E::A];
35    |           ^ help: remove the `|`
36 
36 
37 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:33:13
+   --> $DIR/or-patterns-syntactic-fail.rs:43:13
39    |
40 LL |     let TS( | A | B );
41    |             ^ help: remove the `|`
42 
42 
43 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:34:17
+   --> $DIR/or-patterns-syntactic-fail.rs:44:17
45    |
46 LL |     let NS { f: | A | B };
47    |                 ^ help: remove the `|`
48 
48 
49 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:36:11
+   --> $DIR/or-patterns-syntactic-fail.rs:46:11
51    |
52 LL |     let ( || A | B) = E::A;
53    |           ^^ help: remove the `||`

55    = note: alternatives in or-patterns are separated with `|`, not `||`
56 
57 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:37:11
+   --> $DIR/or-patterns-syntactic-fail.rs:47:11
59    |
60 LL |     let [ || A | B ] = [E::A];
61    |           ^^ help: remove the `||`

63    = note: alternatives in or-patterns are separated with `|`, not `||`
64 
65 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:38:13
+   --> $DIR/or-patterns-syntactic-fail.rs:48:13
67    |
68 LL |     let TS( || A | B );
69    |             ^^ help: remove the `||`

71    = note: alternatives in or-patterns are separated with `|`, not `||`
72 
73 error: a leading `|` is only allowed in a top-level pattern
-   --> $DIR/or-patterns-syntactic-fail.rs:39:17
+   --> $DIR/or-patterns-syntactic-fail.rs:49:17
75    |
76 LL |     let NS { f: || A | B };
77    |                 ^^ help: remove the `||`

89    = note: an implementation of `std::ops::BitOr` might be missing for `E`
91 error[E0308]: mismatched types
-   --> $DIR/or-patterns-syntactic-fail.rs:41:36
+   --> $DIR/or-patterns-syntactic-fail.rs:51:36
93    |
93    |
94 LL |     let recovery_witness: String = 0;
95    |                           ------   ^

98    |                           |        help: try using a conversion method: `0.to_string()`
100 
- error: aborting due to 14 previous errors
+ error: aborting due to 16 previous errors
102 
102 
103 Some errors have detailed explanations: E0308, E0369.
104 For more information about an error, try `rustc --explain E0308`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/or-patterns-syntactic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/or-patterns-syntactic-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: an or-pattern parameter must be wrapped in parenthesis
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:17:13
   |
LL |     fn fun1(A | B: E) {} //~ ERROR an or-pattern parameter must be wrapped in parenthesis
   |             ^^^^^ help: wrap the pattern in parenthesis: `(A | B)`

error: a leading `|` is not allowed in a parameter pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:19:13
   |
LL |     fn fun2(| A | B: E) {}
   |             ^ help: remove the `|`

error: an or-pattern parameter must be wrapped in parenthesis
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:19:15
   |
LL |     fn fun2(| A | B: E) {}
   |               ^^^^^ help: wrap the pattern in parenthesis: `(A | B)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:25:9
   |
LL |     let A | B: E = A;
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:28:11
   |
LL |     let | A | B: E = A;
   |           ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:40:11
   |
LL |     let ( | A | B) = E::A; //~ ERROR a leading `|` is only allowed in a top-level pattern
   |           ^ help: remove the `|`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:41:11
   |
LL |     let ( | A | B,) = (E::B,); //~ ERROR a leading `|` is only allowed in a top-level pattern
   |           ^ help: remove the `|`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:42:11
   |
LL |     let [ | A | B ] = [E::A]; //~ ERROR a leading `|` is only allowed in a top-level pattern
   |           ^ help: remove the `|`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:43:13
   |
LL |     let TS( | A | B ); //~ ERROR a leading `|` is only allowed in a top-level pattern
   |             ^ help: remove the `|`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:44:17
   |
LL |     let NS { f: | A | B }; //~ ERROR a leading `|` is only allowed in a top-level pattern
   |                 ^ help: remove the `|`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:46:11
   |
LL |     let ( || A | B) = E::A; //~ ERROR a leading `|` is only allowed in a top-level pattern
   |           ^^ help: remove the `||`
   |
   = note: alternatives in or-patterns are separated with `|`, not `||`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:47:11
   |
LL |     let [ || A | B ] = [E::A]; //~ ERROR a leading `|` is only allowed in a top-level pattern
   |           ^^ help: remove the `||`
   |
   = note: alternatives in or-patterns are separated with `|`, not `||`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:48:13
   |
LL |     let TS( || A | B ); //~ ERROR a leading `|` is only allowed in a top-level pattern
   |             ^^ help: remove the `||`
   |
   = note: alternatives in or-patterns are separated with `|`, not `||`

error: a leading `|` is only allowed in a top-level pattern
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:49:17
   |
LL |     let NS { f: || A | B }; //~ ERROR a leading `|` is only allowed in a top-level pattern
   |                 ^^ help: remove the `||`
   |
   = note: alternatives in or-patterns are separated with `|`, not `||`

error[E0369]: no implementation for `E | ()`
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:13:22
   |
LL |     let _ = |A | B: E| (); //~ ERROR no implementation for `E | ()`
   |                  ----^ -- ()
   |                  E
   |
   |
   = note: an implementation of `std::ops::BitOr` might be missing for `E`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:51:36
   |
   |
LL |     let recovery_witness: String = 0; //~ ERROR mismatched types
   |                           ------   ^
   |                           |        expected struct `String`, found integer
   |                           |        expected struct `String`, found integer
   |                           |        help: try using a conversion method: `0.to_string()`

error: aborting due to 16 previous errors

Some errors have detailed explanations: E0308, E0369.
---
test result: FAILED. 11343 passed; 5 failed; 92 ignored; 0 measured; 0 filtered out; finished in 136.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:54
