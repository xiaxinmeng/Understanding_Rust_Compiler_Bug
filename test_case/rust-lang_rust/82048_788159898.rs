plain
.................................................................................................... 7400/11514
.................................................................................................... 7500/11514
.................................................................................................... 7600/11514
......................i..ii...............................................................ii........ 7700/11514
...................................................................................F..FF....F..F.F.. 7800/11514
i.FF...FFi.F....FFF.F........................................................................i...... 7900/11514
.................................................................................................... 8100/11514
....................................................F............................................... 8200/11514
.....................................................................i.............................. 8300/11514
.................................................................................................... 8400/11514
---
.................................................................................................... 9300/11514
.................................................................................................... 9400/11514
.................................................................i......i........................... 9500/11514
.................................................................................................... 9600/11514
....iiiiiii..iiiiii.i............................................................................... 9700/11514
.................................................................................................... 9900/11514
.................................................................................................... 10000/11514
.................................................................................................... 10100/11514
.................................................................................................... 10200/11514
---

---- [ui] ui/or-patterns/already-bound-name.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/already-bound-name.rs:21:9
+    |
+ LL |     let A(a, a) | B(a) = A(0, 1);
+    |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, a) | B(a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/already-bound-name.rs:24:9
+    |
+ LL |     let B(a) | A(a, a) = A(0, 1);
+    |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(a) | A(a, a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/already-bound-name.rs:32:9
+    |
+ LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/already-bound-name.rs:37:9
+    |
+ LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(_) | A(A(a, _) | B(a), A(a, _) | B(a)))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/already-bound-name.rs:42:9
+    |
+ LL |     let B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)))`
+ 
1 error[E0416]: identifier `a` is bound more than once in the same pattern
2   --> $DIR/already-bound-name.rs:11:13


95               found type `E<{integer}>`
96    = note: a binding must have the same type in all alternatives
- error: aborting due to 15 previous errors
+ error: aborting due to 20 previous errors
99 
100 Some errors have detailed explanations: E0308, E0408, E0416.
100 Some errors have detailed explanations: E0308, E0408, E0416.
101 For more information about an error, try `rustc --explain E0308`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/already-bound-name/already-bound-name.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/already-bound-name.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/already-bound-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/already-bound-name" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/already-bound-name/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:21:9
   |
LL |     let A(a, a) | B(a) = A(0, 1);
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, a) | B(a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:24:9
   |
LL |     let B(a) | A(a, a) = A(0, 1);
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(a) | A(a, a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:32:9
   |
LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:37:9
   |
LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(_) | A(A(a, _) | B(a), A(a, _) | B(a)))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:42:9
   |
LL |     let B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)))`

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:11:13
   |
LL |     let (a, a) = (0, 1); // Standard duplication without an or-pattern.
   |             ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:14:15
   |
LL |     let (a, A(a, _) | B(a)) = (0, A(1, 2));
   |               ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:14:25
   |
LL |     let (a, A(a, _) | B(a)) = (0, A(1, 2));
   |                         ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:18:26
   |
LL |     let (A(a, _) | B(a), a) = (A(0, 1), 2);
   |                          ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:21:14
   |
LL |     let A(a, a) | B(a) = A(0, 1);
   |              ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:24:21
   |
LL |     let B(a) | A(a, a) = A(0, 1);
   |                     ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:28:21
   |
LL |         B(a) | A(a, a) => {} // Let's ensure `match` has no funny business.
   |                     ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:32:36
   |
LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
   |                                    ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:32:46
   |
LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
   |                                              ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:37:36
   |
LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                    ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:37:46
   |
LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                              ^ used in a pattern more than once

error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:37:9
   |
LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |         ^^^^ pattern doesn't bind `a`        - variable not in all patterns

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:42:49
   |
LL |     let B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                                 ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:42:59
   |
LL |     let B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                                           ^ used in a pattern more than once
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:32:31
   |
   |
LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
   |             -                 ^                    ------- this expression has type `E<E<{integer}>>`
   |             |                 expected integer, found enum `E`
   |             |                 expected integer, found enum `E`
   |             first introduced with type `{integer}` here
   |
   = note: expected type `{integer}`
              found type `E<{integer}>`
   = note: a binding must have the same type in all alternatives
error: aborting due to 20 previous errors

Some errors have detailed explanations: E0308, E0408, E0416.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/or-patterns/const-fn.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/const-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/const-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/const-fn.rs:6:9
   |
LL |     let Ok(y) | Err(y) = x;
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(y) | Err(y))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/const-fn.rs:11:9
   |
LL |     let Ok(y) | Err(y) = x;
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(y) | Err(y))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/const-fn.rs:16:9
   |
LL |     let Ok(y) | Err(y) = x;
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(y) | Err(y))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/const-fn.rs:21:9
   |
LL |     let Ok(y) | Err(y) = x;
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(y) | Err(y))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/const-fn.rs:27:13
   |
LL |         let Ok(y) | Err(y) = x;
   |             ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(y) | Err(y))`
error: aborting due to 5 previous errors


------------------------------------------
------------------------------------------


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
  --> /checkout/src/test/ui/or-patterns/consistent-bindings.rs:11:9
   |
LL |     let Ok(a) | Err(a) = Ok(0);
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(a) | Err(a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/consistent-bindings.rs:12:9
   |
LL |     let Ok(ref a) | Err(ref a) = Ok(0);
   |         ^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(ref a) | Err(ref a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/consistent-bindings.rs:13:9
   |
LL |     let Ok(ref mut a) | Err(ref mut a) = Ok(0);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(ref mut a) | Err(ref mut a))`
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/or-patterns/feature-gate-or_patterns-leading-let.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let | A;
+    |         ^^^ help: wrap the pattern in parentheses: `(A)`
+ 
1 error[E0658]: or-patterns syntax is experimental
3    |

7    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
8    = help: add `#![feature(or_patterns)]` to the crate attributes to enable
---
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/feature-gate-or_patterns-leading-let/feature-gate-or_patterns-leading-let.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/feature-gate-or_patterns-leading-let.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/feature-gate-or_patterns-leading-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/feature-gate-or_patterns-leading-let" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/feature-gate-or_patterns-leading-let/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns-leading-let.rs:7:9
   |
LL |     let | A; //~ ERROR or-patterns syntax is experimental
   |         ^^^ help: wrap the pattern in parentheses: `(A)`

error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns-leading-let.rs:7:9
   |
LL |     let | A; //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable

---

---- [ui] ui/or-patterns/feature-gate-or_patterns.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let | A | B;
+    |         ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A | B;
+    |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
+ 
1 error[E0658]: or-patterns syntax is experimental
3    |

169    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
170    = help: add `#![feature(or_patterns)]` to the crate attributes to enable
---
175 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/feature-gate-or_patterns/feature-gate-or_patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/feature-gate-or_patterns.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/feature-gate-or_patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/feature-gate-or_patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:28:9
   |
LL |     let | A | B; //~ ERROR or-patterns syntax is experimental
   |         ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:29:9
   |
LL |     let A | B; //~ ERROR or-patterns syntax is experimental
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:5:14
   |
LL |         Some(0 | 1 | 2) => {}
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:28:9
   |
LL |     let | A | B; //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:29:9
   |
LL |     let A | B; //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:30:9
   |
LL |     for | A | B in 0 {} //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:31:9
   |
LL |     for A | B in 0 {} //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:32:13
   |
LL |     fn fun((A | B): _) {} //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:33:15
   |
LL |     let _ = |(A | B): u8| (); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:34:10
   |
LL |     let (A | B); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:35:10
   |
LL |     let (A | B,); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:36:11
   |
LL |     let A(B | C); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:37:14
   |
LL |     let E::V(B | C); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:38:17
   |
LL |     let S { f1: B | C, f2 }; //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:39:20
   |
LL |     let E::V { f1: B | C, f2 }; //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:40:10
   |
LL |     let [A | B]; //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:16:14
   |
LL | accept_pat!((p | q)); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:17:14
   |
LL | accept_pat!((p | q,)); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:18:16
   |
LL | accept_pat!(TS(p | q)); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:19:21
   |
LL | accept_pat!(NS { f: p | q }); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> /checkout/src/test/ui/or-patterns/feature-gate-or_patterns.rs:20:14
   |
LL | accept_pat!([p | q]); //~ ERROR or-patterns syntax is experimental
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable

---

---- [ui] ui/or-patterns/issue-69875-should-have-been-expanded-earlier-non-exhaustive.rs stdout ----
diff of stderr:

- error[E0005]: refutable pattern in local binding: `i32::MIN..=-1_i32` and `3_i32..=i32::MAX` not covered
+ error: top-level or-pattern cannot be followed by type annotation
3    |
3    |
4 LL |     let 0 | (1 | 2) = 0;

-    |         ^^^^^^^^^^^ patterns `i32::MIN..=-1_i32` and `3_i32..=i32::MAX` not covered
-    |
-    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
-    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
-    = note: the matched value is of type `i32`
- help: you might want to use `if let` to ignore the variant that isn't matched
-    |
- LL |     if let 0 | (1 | 2) = 0 { /* */ }
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |         ^^^^^^^^^^^ help: wrap the pattern in parentheses: `(0 | (1 | 2))`
14 
- error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `3_i32..=i32::MAX` not covered
-   --> $DIR/issue-69875-should-have-been-expanded-earlier-non-exhaustive.rs:5:11
- LL |     match 0 {
- LL |     match 0 {
-    |           ^ patterns `i32::MIN..=-1_i32` and `3_i32..=i32::MAX` not covered
-    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
-    = note: the matched value is of type `i32`
+ error: aborting due to previous error
23 
---
28 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier-non-exhaustive/issue-69875-should-have-been-expanded-earlier-non-exhaustive.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/issue-69875-should-have-been-expanded-earlier-non-exhaustive.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier-non-exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier-non-exhaustive" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier-non-exhaustive/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier-non-exhaustive.rs:4:9
   |
LL |     let 0 | (1 | 2) = 0; //~ ERROR refutable pattern in local binding
   |         ^^^^^^^^^^^ help: wrap the pattern in parentheses: `(0 | (1 | 2))`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/or-patterns/issue-69875-should-have-been-expanded-earlier.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/issue-69875-should-have-been-expanded-earlier.rs:6:9
   |
LL |     let 0 | (1 | _) = 0;
   |         ^^^^^^^^^^^ help: wrap the pattern in parentheses: `(0 | (1 | _))`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/or-patterns/inconsistent-modes.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok((ref a, b)) | Err((ref mut a, ref b)))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let Ok(Ok(a) | Err(a)) | Err(ref a) = Err(0);
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(Ok(a) | Err(a)) | Err(ref a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let Ok([Ok((Ok(ref a) | Err(a),)) | Err(a)]) | Err(a) = Err(&1);
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok([Ok((Ok(ref a) | Err(a),)) | Err(a)]) | Err(a))`
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
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:14:9
   |
LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok((ref a, b)) | Err((ref mut a, ref b)))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:20:9
   |
LL |     let Ok(Ok(a) | Err(a)) | Err(ref a) = Err(0);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(Ok(a) | Err(a)) | Err(ref a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:24:9
   |
LL |     let Ok([Ok((Ok(ref a) | Err(a),)) | Err(a)]) | Err(a) = Err(&1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok([Ok((Ok(ref a) | Err(a),)) | Err(a)]) | Err(a))`

error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:7:26
   |
LL |     let (Ok(a) | Err(ref a)): Result<&u8, u8> = Ok(&0);
   |             -            ^ bound in different ways
   |             first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:9:30
   |
LL |     let (Ok(ref mut a) | Err(a)): Result<u8, &mut u8> = Ok(0);
   |                     -        ^ bound in different ways
   |                     first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:34
   |
LL |     let (Ok(ref a) | Err(ref mut a)): Result<&u8, &mut u8> = Ok(&0);
   |                 - first binding  ^ bound in different ways

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
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:26
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:26
   |
LL |     let (Ok(ref a) | Err(ref mut a)): Result<&u8, &mut u8> = Ok(&0);
   |             -----        ^^^^^^^^^    -------------------- expected due to this
   |             |            types differ in mutability
   |             |            types differ in mutability
   |             first introduced with type `&&u8` here
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


---- [ui] ui/or-patterns/let-pattern.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/let-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/let-pattern/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/let-pattern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/let-pattern.rs:6:9
   |
LL |     let Ok(y) | Err(y) = x;
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(y) | Err(y))`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/or-patterns/missing-bindings.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let alpha | beta | charlie = alpha;
+    |         ^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(alpha | beta | charlie)`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(a, _) | _ = X;
+    |         ^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, _) | _)`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let _ | B(a) = X;
+    |         ^^^^^^^^ help: wrap the pattern in parentheses: `(_ | B(a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(..) | B(a) = X;
+    |         ^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(..) | B(a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(a, _) | B(_) = X;
+    |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, _) | B(_))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(_, a) | B(_) = X;
+    |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(_, a) | B(_))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(a, b) | B(a) = X;
+    |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, b) | B(a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(A(..) | B(_), _) | B(a) = Y;
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(A(..) | B(_), _) | B(a))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(A(..) | B(a), _) | B(A(a, _) | B(a)) = Y;
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(A(..) | B(a), _) | B(A(a, _) | B(a)))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(A(a, b) | B(c), d) | B(e))`
+ 
1 error[E0408]: variable `beta` is not bound in all patterns
3    |

237 LL |               V3(c),
238    |                  - variable not in all patterns
---
243 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/missing-bindings/missing-bindings.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/missing-bindings.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/missing-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/missing-bindings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/missing-bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:20:9
   |
LL |     let alpha | beta | charlie = alpha; //~  ERROR variable `beta` is not bound in all patterns
   |         ^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(alpha | beta | charlie)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:34:9
   |
LL |     let A(a, _) | _ = X; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, _) | _)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:35:9
   |
LL |     let _ | B(a) = X; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^^^^ help: wrap the pattern in parentheses: `(_ | B(a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:36:9
   |
LL |     let A(..) | B(a) = X; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(..) | B(a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:37:9
   |
LL |     let A(a, _) | B(_) = X; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, _) | B(_))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:38:9
   |
LL |     let A(_, a) | B(_) = X; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(_, a) | B(_))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:39:9
   |
LL |     let A(a, b) | B(a) = X; //~ ERROR variable `b` is not bound in all patterns
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(a, b) | B(a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:43:9
   |
LL |     let A(A(..) | B(_), _) | B(a) = Y; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(A(..) | B(_), _) | B(a))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:44:9
   |
LL |     let A(A(..) | B(a), _) | B(A(a, _) | B(a)) = Y;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(A(..) | B(a), _) | B(A(a, _) | B(a)))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:9
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(A(A(a, b) | B(c), d) | B(e))`

error[E0408]: variable `beta` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:20:9
   |
LL |     let alpha | beta | charlie = alpha; //~  ERROR variable `beta` is not bound in all patterns
   |         ^^^^^   ----   ^^^^^^^ pattern doesn't bind `beta`
   |         |       variable not in all patterns
   |         |       variable not in all patterns
   |         pattern doesn't bind `beta`

error[E0408]: variable `beta` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:22:14
   |
LL |         Some(alpha | beta) => {} //~ ERROR variable `beta` is not bound in all patterns
   |              ^^^^^   ---- variable not in all patterns
   |              |
   |              pattern doesn't bind `beta`

error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:34:19
   |
LL |     let A(a, _) | _ = X; //~ ERROR variable `a` is not bound in all patterns
   |           -       ^ pattern doesn't bind `a`
   |           variable not in all patterns


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:35:9
   |
LL |     let _ | B(a) = X; //~ ERROR variable `a` is not bound in all patterns
   |         ^     - variable not in all patterns
   |         pattern doesn't bind `a`


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:36:9
   |
LL |     let A(..) | B(a) = X; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^     - variable not in all patterns
   |         pattern doesn't bind `a`


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:37:19
   |
LL |     let A(a, _) | B(_) = X; //~ ERROR variable `a` is not bound in all patterns
   |           -       ^^^^ pattern doesn't bind `a`
   |           variable not in all patterns


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:38:19
   |
LL |     let A(_, a) | B(_) = X; //~ ERROR variable `a` is not bound in all patterns
   |              -    ^^^^ pattern doesn't bind `a`
   |              variable not in all patterns


error[E0408]: variable `b` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:39:19
   |
LL |     let A(a, b) | B(a) = X; //~ ERROR variable `b` is not bound in all patterns
   |              -    ^^^^ pattern doesn't bind `b`
   |              variable not in all patterns


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:43:9
   |
LL |     let A(A(..) | B(_), _) | B(a) = Y; //~ ERROR variable `a` is not bound in all patterns
   |         ^^^^^^^^^^^^^^^^^^     - variable not in all patterns
   |         pattern doesn't bind `a`


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:44:11
   |
LL |     let A(A(..) | B(a), _) | B(A(a, _) | B(a)) = Y;
   |           ^^^^^     - variable not in all patterns
   |           pattern doesn't bind `a`


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:21
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |             -       ^^^^ pattern doesn't bind `a`
   |             variable not in all patterns


error[E0408]: variable `b` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:21
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |                -    ^^^^ pattern doesn't bind `b`
   |                variable not in all patterns


error[E0408]: variable `c` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:11
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |           ^^^^^^^     - variable not in all patterns
   |           pattern doesn't bind `c`


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:32
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |             -                  ^^^^ pattern doesn't bind `a`
   |             variable not in all patterns


error[E0408]: variable `b` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:32
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |                -               ^^^^ pattern doesn't bind `b`
   |                variable not in all patterns


error[E0408]: variable `c` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:32
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |                       -        ^^^^ pattern doesn't bind `c`
   |                       variable not in all patterns

error[E0408]: variable `d` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:32
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:32
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |                           -    ^^^^ pattern doesn't bind `d`
   |                           variable not in all patterns


error[E0408]: variable `e` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:46:9
   |
LL |     let A(A(a, b) | B(c), d) | B(e) = Y;
   |         ^^^^^^^^^^^^^^^^^^^^     - variable not in all patterns
   |         pattern doesn't bind `e`


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:62:29
   |
LL |                     Ok(a) | Err(_), //~ ERROR variable `a` is not bound in all patterns
   |                        -    ^^^^^^ pattern doesn't bind `a`
   |                        variable not in all patterns


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:70:21
   |
LL |                     A(_, a) | //~ ERROR variable `b` is not bound in all patterns
   |                          - variable not in all patterns
LL |                     B(b), //~ ERROR variable `a` is not bound in all patterns
   |                     ^^^^ pattern doesn't bind `a`

error[E0408]: variable `b` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:69:21
   |
LL |                     A(_, a) | //~ ERROR variable `b` is not bound in all patterns
   |                     ^^^^^^^ pattern doesn't bind `b`
LL |                     B(b), //~ ERROR variable `a` is not bound in all patterns
   |                       - variable not in all patterns

error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:73:17
   |
LL |                     A(_, a) | //~ ERROR variable `b` is not bound in all patterns
   |                          - variable not in all patterns
LL |                 B(_)
   |                 ^^^^ pattern doesn't bind `a`


error[E0408]: variable `b` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:73:17
   |
LL |                     B(b), //~ ERROR variable `a` is not bound in all patterns
   |                       - variable not in all patterns
LL |                 B(_)
   |                 ^^^^ pattern doesn't bind `b`


error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:77:13
   |
LL |                 B(Ok(a) | Err(a))
   |                               - variable not in all patterns
...
LL |                     A(_, a) | //~ ERROR variable `b` is not bound in all patterns
   |                          - variable not in all patterns
LL |             V3(c),
   |             ^^^^^ pattern doesn't bind `a`


error[E0408]: variable `b` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:58:13
LL | /             V1(
LL | /             V1(
LL | |             //~^ ERROR variable `b` is not bound in all patterns
LL | |             //~| ERROR variable `c` is not bound in all patterns
LL | |                 A(
...  |
LL | |                 B(Ok(a) | Err(a))
LL | |             ) |
   | |_____________^ pattern doesn't bind `b`
...
LL |                       B(b), //~ ERROR variable `a` is not bound in all patterns
   |                         - variable not in all patterns
LL |               V3(c),
   |               ^^^^^ pattern doesn't bind `b`


error[E0408]: variable `c` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/missing-bindings.rs:58:13
LL | /             V1(
LL | /             V1(
LL | |             //~^ ERROR variable `b` is not bound in all patterns
LL | |             //~| ERROR variable `c` is not bound in all patterns
LL | |                 A(
...  |
LL | |                 B(Ok(a) | Err(a))
LL | |             ) |
   | |_____________^ pattern doesn't bind `c`
LL | /             V2(
LL | |                 A(
LL | |                     A(_, a) | //~ ERROR variable `b` is not bound in all patterns
LL | |                     B(b), //~ ERROR variable `a` is not bound in all patterns
...  |
LL | |                 //~| ERROR variable `b` is not bound in all patterns
LL | |             ) |
   | |_____________^ pattern doesn't bind `c`
LL |               V3(c),
   |                  - variable not in all patterns
error: aborting due to 36 previous errors

For more information about this error, try `rustc --explain E0408`.


------------------------------------------


---- [ui] ui/or-patterns/mismatched-bindings-async-fn.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let x | s = String::new();
+    |         ^^^^^ help: wrap the pattern in parentheses: `(x | s)`
+ 
1 error[E0408]: variable `s` is not bound in all patterns
3    |

30    |         |
31    |         variable not in all patterns
---
36 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/mismatched-bindings-async-fn/mismatched-bindings-async-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/mismatched-bindings-async-fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/mismatched-bindings-async-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/mismatched-bindings-async-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/mismatched-bindings-async-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/mismatched-bindings-async-fn.rs:11:9
   |
LL |     let x | s = String::new();
   |         ^^^^^ help: wrap the pattern in parentheses: `(x | s)`

error[E0408]: variable `s` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/mismatched-bindings-async-fn.rs:6:13
   |
LL | async fn a((x | s): String) {}
   |             ^   - variable not in all patterns
   |             pattern doesn't bind `s`


error[E0408]: variable `x` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/mismatched-bindings-async-fn.rs:6:17
   |
LL | async fn a((x | s): String) {}
   |             -   ^ pattern doesn't bind `x`
   |             variable not in all patterns


error[E0408]: variable `s` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/mismatched-bindings-async-fn.rs:11:9
   |
LL |     let x | s = String::new();
   |         ^   - variable not in all patterns
   |         pattern doesn't bind `s`


error[E0408]: variable `x` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/mismatched-bindings-async-fn.rs:11:13
   |
LL |     let x | s = String::new();
   |         -   ^ pattern doesn't bind `x`
   |         variable not in all patterns

error: aborting due to 5 previous errors


For more information about this error, try `rustc --explain E0408`.

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
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:26:9
   |
LL |     let | A | B;
   |         ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:27:9
LL |     let A | B;
LL |     let A | B;
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:29:9
   |
LL |     let A | B = 0;
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:72:9
   |
LL |     let box 0 | 1; // Unstable; we *can* the precedence if we want.
   |         ^^^^^^^^^ help: wrap the pattern in parentheses: `(box 0 | 1)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:73:9
   |
LL |     let &0 | 1;
   |         ^^^^^^ help: wrap the pattern in parentheses: `(&0 | 1)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:74:9
   |
LL |     let &mut 0 | 1;
   |         ^^^^^^^^^^ help: wrap the pattern in parentheses: `(&mut 0 | 1)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:75:9
   |
LL |     let x @ 0 | 1;
   |         ^^^^^^^^^ help: wrap the pattern in parentheses: `(x @ 0 | 1)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:76:9
   |
LL |     let ref x @ 0 | 1;
   |         ^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(ref x @ 0 | 1)`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass.rs:77:9
   |
LL |     let ref mut x @ 0 | 1;
   |         ^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(ref mut x @ 0 | 1)`
error: aborting due to 9 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/or-patterns/or-patterns-binding-type-mismatch.rs stdout ----
diff of stderr:

+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/or-patterns-binding-type-mismatch.rs:55:9
+    |
+ LL |     let Blah::A(_, x, y) | Blah::B(x, y) = Blah::A(1, 1, 2);
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Blah::A(_, x, y) | Blah::B(x, y))`
+ 
+ error: top-level or-pattern cannot be followed by type annotation
+   --> $DIR/or-patterns-binding-type-mismatch.rs:58:9
+    |
+ LL |     let (x, y) | (y, x) = (0u8, 1u16);
+    |         ^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `((x, y) | (y, x))`
1 error[E0308]: mismatched types
2   --> $DIR/or-patterns-binding-type-mismatch.rs:13:39
3    |


252    |
253    = note: a binding must have the same type in all alternatives
- error: aborting due to 22 previous errors
+ error: aborting due to 24 previous errors
256 
257 For more information about this error, try `rustc --explain E0308`.
257 For more information about this error, try `rustc --explain E0308`.
258 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-binding-type-mismatch/or-patterns-binding-type-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/or-patterns-binding-type-mismatch.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-binding-type-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-binding-type-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:55:9
   |
LL |     let Blah::A(_, x, y) | Blah::B(x, y) = Blah::A(1, 1, 2);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Blah::A(_, x, y) | Blah::B(x, y))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:58:9
   |
LL |     let (x, y) | (y, x) = (0u8, 1u16);
   |         ^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `((x, y) | (y, x))`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:13:39
   |
   |
LL |     match Blah::A(1, 1, 2) {
   |           ---------------- this expression has type `Blah`
LL |         Blah::A(_, x, y) | Blah::B(x, y) => {} //~ ERROR mismatched types
   |                       -               ^ expected `usize`, found `isize`
   |                       |
   |                       first introduced with type `usize` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:17:44
   |
   |
LL |     match Some(Blah::A(1, 1, 2)) {
   |           ---------------------- this expression has type `Option<Blah>`
LL |         Some(Blah::A(_, x, y) | Blah::B(x, y)) => {} //~ ERROR mismatched types
   |                            -               ^ expected `usize`, found `isize`
   |                            |
   |                            first introduced with type `usize` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:21:19
   |
LL |     match (0u8, 1u16) {
LL |     match (0u8, 1u16) {
   |           ----------- this expression has type `(u8, u16)`
LL |         (x, y) | (y, x) => {} //~ ERROR mismatched types
   |             -     ^ expected `u16`, found `u8`
   |             |
   |             first introduced with type `u16` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:21:22
   |
LL |     match (0u8, 1u16) {
LL |     match (0u8, 1u16) {
   |           ----------- this expression has type `(u8, u16)`
LL |         (x, y) | (y, x) => {} //~ ERROR mismatched types
   |          -           ^ expected `u8`, found `u16`
   |          |
   |          first introduced with type `u8` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:26:41
   |
   |
LL |     match Some((0u8, Some((1u16, 2u32)))) {
   |           ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
LL |         Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x)))) => {}
   |                        -                ^ expected `u16`, found `u8`
   |                        |
   |                        first introduced with type `u16` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:26:50
   |
   |
LL |     match Some((0u8, Some((1u16, 2u32)))) {
   |           ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
LL |         Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x)))) => {}
   |               -                                  ^ expected `u8`, found `u16`
   |               |
   |               first introduced with type `u8` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:26:59
   |
   |
LL |     match Some((0u8, Some((1u16, 2u32)))) {
   |           ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
LL |         Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x)))) => {}
   |                           -                               ^ expected `u32`, found `u16`
   |                           |
   |                           first introduced with type `u32` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:26:62
   |
   |
LL |     match Some((0u8, Some((1u16, 2u32)))) {
   |           ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
LL |         Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x)))) => {}
   |               - first introduced with type `u8` here         ^ expected `u8`, found `u32`
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:34:42
   |
   |
LL |     if let Blah::A(_, x, y) | Blah::B(x, y) = Blah::A(1, 1, 2) {
   |                          -               ^    ---------------- this expression has type `Blah`
   |                          |               |
   |                          |               expected `usize`, found `isize`
   |                          first introduced with type `usize` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:38:47
   |
   |
LL |     if let Some(Blah::A(_, x, y) | Blah::B(x, y)) = Some(Blah::A(1, 1, 2)) {
   |                               -               ^     ---------------------- this expression has type `Option<Blah>`
   |                               |               |
   |                               |               expected `usize`, found `isize`
   |                               first introduced with type `usize` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:42:22
   |
   |
LL |     if let (x, y) | (y, x) = (0u8, 1u16) {
   |                -     ^       ----------- this expression has type `(u8, u16)`
   |                |     |
   |                |     expected `u16`, found `u8`
   |                first introduced with type `u16` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:42:25
   |
   |
LL |     if let (x, y) | (y, x) = (0u8, 1u16) {
   |             -           ^    ----------- this expression has type `(u8, u16)`
   |             |           |
   |             |           expected `u8`, found `u16`
   |             first introduced with type `u8` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:47:44
   |
   |
LL |     if let Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x))))
   |                           -                ^ expected `u16`, found `u8`
   |                           |
   |                           first introduced with type `u16` here
...
LL |     = Some((0u8, Some((1u16, 2u32))))
   |       ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:47:53
   |
   |
LL |     if let Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x))))
   |                  -                                  ^ expected `u8`, found `u16`
   |                  |
   |                  first introduced with type `u8` here
...
LL |     = Some((0u8, Some((1u16, 2u32))))
   |       ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:47:62
   |
   |
LL |     if let Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x))))
   |                              -                               ^ expected `u32`, found `u16`
   |                              |
   |                              first introduced with type `u32` here
...
LL |     = Some((0u8, Some((1u16, 2u32))))
   |       ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:47:65
   |
   |
LL |     if let Some((x, Some((y, z)))) | Some((y, Some((x, z) | (z, x))))
   |                  - first introduced with type `u8` here         ^ expected `u8`, found `u32`
...
LL |     = Some((0u8, Some((1u16, 2u32))))
   |       ------------------------------- this expression has type `Option<(u8, Option<(u16, u32)>)>`
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:55:39
   |
   |
LL |     let Blah::A(_, x, y) | Blah::B(x, y) = Blah::A(1, 1, 2);
   |                       -               ^    ---------------- this expression has type `Blah`
   |                       |               |
   |                       |               expected `usize`, found `isize`
   |                       first introduced with type `usize` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:58:19
   |
   |
LL |     let (x, y) | (y, x) = (0u8, 1u16);
   |             -     ^       ----------- this expression has type `(u8, u16)`
   |             |     |
   |             |     expected `u16`, found `u8`
   |             first introduced with type `u16` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:58:22
   |
   |
LL |     let (x, y) | (y, x) = (0u8, 1u16);
   |          -           ^    ----------- this expression has type `(u8, u16)`
   |          |           |
   |          |           expected `u8`, found `u16`
   |          first introduced with type `u8` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:62:42
   |
   |
LL |     fn f1((Blah::A(_, x, y) | Blah::B(x, y)): Blah) {}
   |                          -               ^    ---- expected due to this
   |                          |               |
   |                          |               expected `usize`, found `isize`
   |                          first introduced with type `usize` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:65:22
   |
   |
LL |     fn f2(((x, y) | (y, x)): (u8, u16)) {}
   |                -     ^       --------- expected due to this
   |                |     |
   |                |     expected `u16`, found `u8`
   |                first introduced with type `u16` here
   |
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-binding-type-mismatch.rs:65:25
   |
   |
LL |     fn f2(((x, y) | (y, x)): (u8, u16)) {}
   |             -           ^    --------- expected due to this
   |             |           |
   |             |           expected `u8`, found `u16`
   |             first introduced with type `u8` here
   |
   = note: a binding must have the same type in all alternatives
error: aborting due to 24 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/or-patterns/or-patterns-default-binding-modes.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-default-binding-modes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-default-binding-modes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-default-binding-modes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-default-binding-modes.rs:40:9
   |
LL |     let Ok(mut x) | &Err(mut x) = res;
   |         ^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(mut x) | &Err(mut x))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-default-binding-modes.rs:44:9
   |
LL |     let Ok(x) | Err(x) = res;
   |         ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Ok(x) | Err(x))`

error: top-level or-pattern cannot be followed by type annotation
  --> /checkout/src/test/ui/or-patterns/or-patterns-default-binding-modes.rs:122:9
   |
LL |       let Tri::A(Ok(mut x) | Err(mut x))
   |  _________^
LL | |     | Tri::B(&Ok(mut x) | Err(mut x))
LL | |     | &Tri::C(Ok(mut x) | Err(mut x)) = tri;
   |
   |
help: wrap the pattern in parentheses
   |
LL |     let (Tri::A(Ok(mut x) | Err(mut x)) | Tri::B(&Ok(mut x) | Err(mut x)) |
LL | &Tri::C(Ok(mut x) | Err(mut x))) = tri;

error: aborting due to 3 previous errors


---

150    |         |
151    |         while parsing this or-pattern starting here
152 
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+    |
+ LL |     let a | = 0;
+    |         ^^^ help: wrap the pattern in parentheses: `(a)`
+ 
153 error: a trailing `|` is not allowed in an or-pattern
155    |

158    |         |
159    |         while parsing this or-pattern starting here
159    |         while parsing this or-pattern starting here
160 
- error: aborting due to 21 previous errors
+ error: top-level or-pattern cannot be followed by type annotation
+    |
+ LL |     let a | ;
+ LL |     let a | ;
+    |         ^^^ help: wrap the pattern in parentheses: `(a)`
+ error: aborting due to 23 previous errors
162 
163 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/remove-leading-vert/remove-leading-vert.stderr
thread '[ui] ui/or-patterns/remove-leading-vert.rs' panicked at 'failed to apply suggestions for "/checkout/src/test/ui/or-patterns/remove-leading-vert.rs" with rustfix', src/tools/compiletest/src/runtest.rs:3217:17
---- [ui] ui/parser/issue-63135.rs stdout ----
diff of stderr:

39    |      |
39    |      |
40    |      while parsing the fields for this pattern
41 
- error: expected one of `:` or `|`, found `)`
-   --> $DIR/issue-63135.rs:3:16
-    |
- LL | fn i(n{...,f #
-    |                ^ expected one of `:` or `|`
- 
- error: expected one of `->`, `;`, `where`, or `{`, found `<eof>`
-   --> $DIR/issue-63135.rs:3:16
-    |
- LL | fn i(n{...,f #
-    |                ^ expected one of `->`, `;`, `where`, or `{`
- error: aborting due to 7 previous errors
+ error: aborting due to 5 previous errors
55 
56 
56 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63135/issue-63135.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-63135.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-63135.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63135" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63135/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/parser/issue-63135.rs:3:16
   |
LL | fn i(n{...,f #
   |     - -        ^
   |     | unclosed delimiter
   |     unclosed delimiter

error: this file contains an unclosed delimiter
error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/parser/issue-63135.rs:3:16
   |
LL | fn i(n{...,f #
   |     - -        ^
   |     | unclosed delimiter
   |     unclosed delimiter


error: expected field pattern, found `...`
   |
   |
LL | fn i(n{...,f #
   |        ^^^ help: to omit remaining fields, use one fewer `.`: `..`

error: expected `}`, found `,`
   |
   |
LL | fn i(n{...,f #
   |        ---^
   |        |  expected `}`
   |        |  expected `}`
   |        `..` must be at the end and cannot have a trailing comma

error: expected one of `!` or `[`, found `}`
   |
   |
LL | fn i(n{...,f #
   |      -         ^ expected one of `!` or `[`
   |      while parsing the fields for this pattern

error: aborting due to 5 previous errors

---
test result: FAILED. 11405 passed; 16 failed; 93 ignored; 0 measured; 0 filtered out; finished in 141.34s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:18
