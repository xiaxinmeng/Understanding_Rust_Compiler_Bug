plain
........................................i...............................................  1584/14871
........................................................................................  1672/14871
..................................................................i.....................  1760/14871
........................................................................................  1848/14871
..........F......F......................................................................  1936/14871
.......................i.....................i...........ii.............................  2112/14871
........................................................................................  2200/14871
........................................................................................  2288/14871
...................................i....................................................  2376/14871
---
..................................................i....i................................  7744/14871
....................................i..............i....................................  7832/14871
...................................i....................................................  7920/14871
............................................................i...........................  8008/14871
..................................................................................F.....  8096/14871
.............................................................F.............F.F.....F....  8184/14871
...F....................................................................................  8272/14871
.....................................i..................................................  8448/14871
........................................................................................  8536/14871
.........................................................ii.............................  8624/14871
........................................................................................  8712/14871
---
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/liveness_unintentional_copy.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closures/2229_closure_analysis/diagnostics/liveness_unintentional_copy.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/liveness_unintentional_copy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/liveness_unintentional_copy/auxiliary" "--edition=2021"
stdout: none
warning: value passed to `a` is never read
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness_unintentional_copy.rs:20:9
   |
LL |         a = s;
LL |         a = s;
   |         ^
   |
   = help: maybe it is overwritten before being read?
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness_unintentional_copy.rs:4:9
   |
LL | #![warn(unused)]
   |         ^^^^^^
---

warning: unused variable: `a`
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness_unintentional_copy.rs:36:9
   |
LL |         a += x; //~ WARN unused variable: `a`
   |
   = help: did you mean to capture by reference instead?

warning: 3 warnings emitted
---
26    |
27 LL |         a += 1;

29    |
30    = help: maybe it is overwritten before being read?
- warning: value assigned to `a` is never read
+ warning: value passed to `a` is never read
33   --> $DIR/liveness.rs:77:13
34    |
34    |
35 LL |             a = Some("d1");
37    |
37    |
38    = help: maybe it is overwritten before being read?
- warning: value assigned to `b` is never read
- warning: value assigned to `b` is never read
+ warning: value passed to `b` is never read
42    |
43 LL |             b = Some("e1");

45    |
45    |
46    = help: maybe it is overwritten before being read?
- warning: value assigned to `b` is never read
- warning: value assigned to `b` is never read
+ warning: value passed to `b` is never read
50    |
51 LL |             b = Some("e2");



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/liveness/liveness.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/liveness.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closures/2229_closure_analysis/diagnostics/liveness.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/liveness" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/liveness/auxiliary" "--edition=2021"
stdout: none
warning: value captured by `a` is never read
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:24:9
   |
   |
LL |         a = 1; //~ WARN value captured by `a` is never read
   |
   = help: did you mean to capture by reference instead?
note: the lint level is defined here
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:5:9
---

warning: unused variable: `a`
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:33:9
   |
LL |         a += 1; //~ WARN unused variable: `a`
   |
   = help: did you mean to capture by reference instead?
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`


warning: value passed to `a` is never read
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:53:9
   |
LL |         a += 1; //~ WARN value assigned to `a` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `a` is never read
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:77:13
   |
   |
LL |             a = Some("d1"); //~ WARN value assigned to `a` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `b` is never read
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:85:13
   |
   |
LL |             b = Some("e1"); //~ WARN value assigned to `b` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `b` is never read
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:87:13
   |
   |
LL |             b = Some("e2"); //~ WARN value assigned to `b` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `b`
  --> fake-test-src-base/closures/2229_closure_analysis/diagnostics/liveness.rs:85:13
   |
   |
LL |             b = Some("e1"); //~ WARN value assigned to `b` is never read
   |
   = help: did you mean to capture by reference instead?

warning: 7 warnings emitted
---
- warning: value assigned to `a` is never read
+ warning: value passed to `a` is never read
2   --> $DIR/warn-unused-duplication.rs:9:6
3    |
4 LL |     (a, a) = (0, 1);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/warn-unused-duplication/warn-unused-duplication.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args destructuring-assignment/warn-unused-duplication.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/destructuring-assignment/warn-unused-duplication.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/warn-unused-duplication/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/warn-unused-duplication/auxiliary"
stdout: none
warning: value passed to `a` is never read
  --> fake-test-src-base/destructuring-assignment/warn-unused-duplication.rs:9:6
   |
   |
LL |     (a, a) = (0, 1); //~ WARN value assigned to `a` is never read
   |
   |
   = help: maybe it is overwritten before being read?
  --> fake-test-src-base/destructuring-assignment/warn-unused-duplication.rs:3:9
   |
LL | #![warn(unused_assignments)]
   |         ^^^^^^^^^^^^^^^^^^
---

---- [ui] tests/ui/issues/issue-11958.rs stdout ----
diff of stderr:

- warning: value assigned to `x` is never read
+ warning: value passed to `x` is never read
3    |
3    |
4 LL |     let _thunk = Box::new(move|| { x = 2; });

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11958/issue-11958.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-11958.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-11958.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11958/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11958/auxiliary"
stdout: none
warning: value passed to `x` is never read
  --> fake-test-src-base/issues/issue-11958.rs:8:36
   |
   |
LL |     let _thunk = Box::new(move|| { x = 2; });
   |
   |
   = help: maybe it is overwritten before being read?

warning: unused variable: `x`
  --> fake-test-src-base/issues/issue-11958.rs:8:36
   |
   |
LL |     let _thunk = Box::new(move|| { x = 2; });
   |
   = help: did you mean to capture by reference instead?
   = note: `#[warn(unused_variables)]` on by default

---
---- [ui] tests/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs stdout ----
diff of stderr:

43    |
44    = note: consider using `_hours_are_suns` instead
45 
- warning: value assigned to `hours_are_suns` is never read
+ warning: value passed to `hours_are_suns` is never read
48    |
48    |
49 LL |         hours_are_suns = false;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern/issue-47390-unused-variable-in-struct-pattern.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unused/issue-47390-unused-variable-in-struct-pattern.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `i_think_continually`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:26:9
   |
LL |     let i_think_continually = 2; //~ WARNING unused variable: `i_think_continually`
   |         ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_i_think_continually`
note: the lint level is defined here
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:5:9
   |
   |
LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`

warning: unused variable: `mut_unused_var`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:13
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:13
   |
LL |     let mut mut_unused_var = 1;
   |             ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_mut_unused_var`

warning: unused variable: `var`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:14
   |
LL |     let (mut var, unused_var) = (1, 2);
   |              ^^^ help: if this is intentional, prefix it with an underscore: `_var`
warning: unused variable: `unused_var`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:19
   |
   |
LL |     let (mut var, unused_var) = (1, 2);
   |                   ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused_var`

warning: unused variable: `corridors_of_light`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:45:26
   |
LL |     if let SoulHistory { corridors_of_light, //~ WARNING unused variable: `corridors_of_light`
   |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`

warning: variable `hours_are_suns` is assigned to, but never used
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:46:30
   |
LL |                          mut hours_are_suns, //~ WARNING `hours_are_suns` is assigned to, but
   |
   |
   = note: consider using `_hours_are_suns` instead

warning: value passed to `hours_are_suns` is never read
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:48:9
   |
LL |         hours_are_suns = false; //~ WARNING unused_assignments
   |
   |
   = help: maybe it is overwritten before being read?

warning: unused variable: `fire`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:52:32
   |
   |
LL |     let LovelyAmbition { lips, fire } = the_spirit; //~ WARNING unused variable: `fire`
   |                                ^^^^ help: try ignoring the field: `fire: _`
warning: unused variable: `case`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:61:23
   |
   |
LL |         Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                       ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:66:24
   |
   |
LL |         &Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:71:27
   |
   |
LL |         box Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                           ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:76:24
   |
   |
LL |         (Large::Suit { case },) => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:81:24
   |
   |
LL |         [Large::Suit { case }] => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:86:29
   |
   |
LL |         Tuple(Large::Suit { case }, ()) => {} //~ WARNING unused variable: `case`
   |                             ^^^^ help: try ignoring the field: `case: _`
warning: variable does not need to be mutable
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:9
   |
LL |     let mut mut_unused_var = 1;
---

warning: variable does not need to be mutable
  --> fake-test-src-base/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:10
   |
LL |     let (mut var, unused_var) = (1, 2);
   |          |
   |          help: remove this `mut`

warning: 16 warnings emitted
warning: 16 warnings emitted
------------------------------------------


---- [ui] tests/ui/liveness/liveness-asm.rs stdout ----
diff of stderr:

- warning: value assigned to `src` is never read
+ warning: value passed to `src` is never read
3    |
3    |
4 LL |     asm!("/*{0}*/", inout(reg) src);
11 LL | #![warn(unused_assignments)]
12    |         ^^^^^^^^^^^^^^^^^^
13 
13 
- warning: value assigned to `src` is never read
+ warning: value passed to `src` is never read
16    |
16    |
17 LL |     asm!("/*{0}*/", inout(reg) src => src);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-asm/liveness-asm.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args liveness/liveness-asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/liveness/liveness-asm.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-asm" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-asm/auxiliary"
stdout: none
--- stderr -------------------------------
warning: value passed to `src` is never read
  --> fake-test-src-base/liveness/liveness-asm.rs:14:32
   |
LL |     asm!("/*{0}*/", inout(reg) src); //~ WARN value assigned to `src` is never read
   |
   |
   = help: maybe it is overwritten before being read?
  --> fake-test-src-base/liveness/liveness-asm.rs:7:9
   |
LL | #![warn(unused_assignments)]
   |         ^^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^^

warning: value passed to `src` is never read
  --> fake-test-src-base/liveness/liveness-asm.rs:24:39
   |
LL |     asm!("/*{0}*/", inout(reg) src => src); //~ WARN value assigned to `src` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: 2 warnings emitted
------------------------------------------



---- [ui] tests/ui/liveness/liveness-dead.rs stdout ----
diff of stderr:

- error: value assigned to `x` is never read
+ error: value passed to `x` is never read
3    |
4 LL |     let mut x: isize = 3;

11 LL | #![deny(unused_assignments)]
11 LL | #![deny(unused_assignments)]
12    |         ^^^^^^^^^^^^^^^^^^
13 
- error: value assigned to `x` is never read
+ error: value passed to `x` is never read
16    |
17 LL |     x = 4;

19    |
19    |
20    = help: maybe it is overwritten before being read?
21 
- error: value passed to `x` is never read
+ error: value assigned to `x` is never read
24    |
25 LL | fn f4(mut x: i32) {

27    |
27    |
28    = help: maybe it is overwritten before being read?
29 
- error: value assigned to `x` is never read
+ error: value passed to `x` is never read
32    |
33 LL |     x = 4;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead/liveness-dead.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args liveness/liveness-dead.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/liveness/liveness-dead.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead/auxiliary"
stdout: none
--- stderr -------------------------------
error: value passed to `x` is never read
  --> fake-test-src-base/liveness/liveness-dead.rs:9:13
   |
LL |     let mut x: isize = 3; //~ ERROR: value assigned to `x` is never read
   |
   |
   = help: maybe it is overwritten before being read?
  --> fake-test-src-base/liveness/liveness-dead.rs:2:9
   |
LL | #![deny(unused_assignments)]
   |         ^^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^^

error: value passed to `x` is never read
  --> fake-test-src-base/liveness/liveness-dead.rs:17:5
   |
LL |     x = 4; //~ ERROR: value assigned to `x` is never read
   |
   |
   = help: maybe it is overwritten before being read?

error: value assigned to `x` is never read
  --> fake-test-src-base/liveness/liveness-dead.rs:20:11
   |
LL | fn f4(mut x: i32) { //~ ERROR: value passed to `x` is never read
   |
   |
   = help: maybe it is overwritten before being read?

error: value passed to `x` is never read
  --> fake-test-src-base/liveness/liveness-dead.rs:27:5
   |
LL |     x = 4; //~ ERROR: value assigned to `x` is never read
   |
   |
   = help: maybe it is overwritten before being read?
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] tests/ui/liveness/liveness-consts.rs stdout ----
diff of stderr:

37    |
38    = note: consider using `_a` instead
- warning: value assigned to `b` is never read
- warning: value assigned to `b` is never read
+ warning: value passed to `b` is never read
42    |
43 LL |     b += 1;


46    = help: maybe it is overwritten before being read?
48 
- warning: value assigned to `t` is never read
+ warning: value passed to `t` is never read
50   --> $DIR/liveness-consts.rs:42:9
50   --> $DIR/liveness-consts.rs:42:9
51    |
52 LL |         t = t + t;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-consts/liveness-consts.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args liveness/liveness-consts.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/liveness/liveness-consts.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-consts" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-consts/auxiliary"
stdout: none
warning: unused variable: `e`
  --> fake-test-src-base/liveness/liveness-consts.rs:24:13
   |
   |
LL |         let e = 1; //~ WARN unused variable: `e`
   |             ^ help: if this is intentional, prefix it with an underscore: `_e`
note: the lint level is defined here
  --> fake-test-src-base/liveness/liveness-consts.rs:2:9
   |
LL | #![warn(unused)]
LL | #![warn(unused)]
   |         ^^^^^^
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`

warning: unused variable: `s`
  --> fake-test-src-base/liveness/liveness-consts.rs:33:24
   |
LL | pub fn f(x: [u8; { let s = 17; 100 }]) -> [u8;  { let z = 18; 100 }] {
   |                        ^ help: if this is intentional, prefix it with an underscore: `_s`
warning: unused variable: `z`
  --> fake-test-src-base/liveness/liveness-consts.rs:33:55
   |
   |
LL | pub fn f(x: [u8; { let s = 17; 100 }]) -> [u8;  { let z = 18; 100 }] {
   |                                                       ^ help: if this is intentional, prefix it with an underscore: `_z`
warning: unused variable: `z`
  --> fake-test-src-base/liveness/liveness-consts.rs:60:13
   |
   |
LL |         let z = 42; //~ WARN unused variable: `z`
   |             ^ help: if this is intentional, prefix it with an underscore: `_z`

warning: variable `a` is assigned to, but never used
  --> fake-test-src-base/liveness/liveness-consts.rs:7:13
   |
LL |     let mut a = 0; //~ WARN variable `a` is assigned to, but never used
   |
   |
   = note: consider using `_a` instead
warning: value passed to `b` is never read
  --> fake-test-src-base/liveness/liveness-consts.rs:17:5
   |
   |
LL |     b += 1; //~ WARN value assigned to `b` is never read
   |
   |
   = help: maybe it is overwritten before being read?

warning: value passed to `t` is never read
  --> fake-test-src-base/liveness/liveness-consts.rs:42:9
   |
   |
LL |         t = t + t; //~ WARN value assigned to `t` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `w`
  --> fake-test-src-base/liveness/liveness-consts.rs:49:13
   |
   |
LL |         let w = 10; //~ WARN unused variable: `w`
   |             ^ help: if this is intentional, prefix it with an underscore: `_w`
warning: 8 warnings emitted
------------------------------------------



---- [ui] tests/ui/liveness/liveness-unused.rs stdout ----
diff of stderr:

51    |
52    = note: consider using `_x` instead
53 
- error: value assigned to `x` is never read
+ error: value passed to `x` is never read
56    |
57 LL |     x += 4;

104    |
104    |
105    = note: consider using `_x` instead
106 
- error: value assigned to `x` is never read
+ error: value passed to `x` is never read
109    |
110 LL |         x = 0;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused/liveness-unused.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args liveness/liveness-unused.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/liveness/liveness-unused.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused/auxiliary"
stdout: none
warning: unreachable statement
  --> fake-test-src-base/liveness/liveness-unused.rs:92:9
   |
LL |         continue;
LL |         continue;
   |         -------- any code following this expression is unreachable
LL |         drop(*x as i32); //~ WARNING unreachable statement
   |         ^^^^^^^^^^^^^^^^ unreachable statement
note: the lint level is defined here
  --> fake-test-src-base/liveness/liveness-unused.rs:1:9
   |
LL | #![warn(unused)]
LL | #![warn(unused)]
   |         ^^^^^^
   = note: `#[warn(unreachable_code)]` implied by `#[warn(unused)]`

error: unused variable: `x`
  --> fake-test-src-base/liveness/liveness-unused.rs:8:7
   |
LL | fn f1(x: isize) {
   |       ^ help: if this is intentional, prefix it with an underscore: `_x`
note: the lint level is defined here
  --> fake-test-src-base/liveness/liveness-unused.rs:2:9
   |
LL | #![deny(unused_variables)]
LL | #![deny(unused_variables)]
   |         ^^^^^^^^^^^^^^^^

error: unused variable: `x`
  --> fake-test-src-base/liveness/liveness-unused.rs:12:8
   |
LL | fn f1b(x: &mut isize) {
   |        ^ help: if this is intentional, prefix it with an underscore: `_x`
error: unused variable: `x`
  --> fake-test-src-base/liveness/liveness-unused.rs:20:9
   |
LL |     let x: isize;
---
   |
LL |     let x = 3;
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`

error: variable `x` is assigned to, but never used
  --> fake-test-src-base/liveness/liveness-unused.rs:30:13
LL |     let mut x = 3;
   |             ^
   |
   |
   = note: consider using `_x` instead

error: value passed to `x` is never read
  --> fake-test-src-base/liveness/liveness-unused.rs:32:5
LL |     x += 4;
   |     ^
   |
   |
   = help: maybe it is overwritten before being read?
  --> fake-test-src-base/liveness/liveness-unused.rs:3:9
   |
LL | #![deny(unused_assignments)]
   |         ^^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^^

error: variable `z` is assigned to, but never used
  --> fake-test-src-base/liveness/liveness-unused.rs:37:13
   |
LL |     let mut z = 3;
   |
   |
   = note: consider using `_z` instead
error: unused variable: `i`
  --> fake-test-src-base/liveness/liveness-unused.rs:59:12
   |
LL |       Some(i) => {
---

error: unused variable: `x`
  --> fake-test-src-base/liveness/liveness-unused.rs:84:10
   |
LL |     for (x, _) in [1, 2, 3].iter().enumerate() { }
   |          ^ help: if this is intentional, prefix it with an underscore: `_x`
error: unused variable: `x`
  --> fake-test-src-base/liveness/liveness-unused.rs:89:13
   |
   |
LL |     for (_, x) in [1, 2, 3].iter().enumerate() {
   |             ^ help: if this is intentional, prefix it with an underscore: `_x`

error: variable `x` is assigned to, but never used
  --> fake-test-src-base/liveness/liveness-unused.rs:112:9
LL |     let x;
   |         ^
   |
   |
   = note: consider using `_x` instead

error: value passed to `x` is never read
  --> fake-test-src-base/liveness/liveness-unused.rs:116:9
   |
LL |         x = 0;  //~ ERROR value assigned to `x` is never read
   |
   |
   = help: maybe it is overwritten before being read?
error: aborting due to 13 previous errors; 1 warning emitted
------------------------------------------



---- [ui] tests/ui/liveness/liveness-upvars.rs stdout ----
diff of stderr:

- warning: value assigned to `last` is never read
+ warning: value passed to `last` is never read
3    |
4 LL |         last = Some(s);

53    |
---
74    |
75 LL |         c += 1;

77    |
78    = help: maybe it is overwritten before being read?
- warning: value assigned to `c` is never read
+ warning: value passed to `c` is never read
81   --> $DIR/liveness-upvars.rs:64:9
82    |
82    |
83 LL |         c += 1;

85    |
86    = help: maybe it is overwritten before being read?
- warning: value assigned to `d` is never read
+ warning: value passed to `d` is never read
89   --> $DIR/liveness-upvars.rs:73:13
90    |
90    |
91 LL |             d = Some("d1");
93    |
93    |
94    = help: maybe it is overwritten before being read?
- warning: value assigned to `e` is never read
- warning: value assigned to `e` is never read
+ warning: value passed to `e` is never read
98    |
98    |
99 LL |             e = Some("e1");
101    |
101    |
102    = help: maybe it is overwritten before being read?
- warning: value assigned to `e` is never read
- warning: value assigned to `e` is never read
+ warning: value passed to `e` is never read
106    |
106    |
107 LL |             e = Some("e2");
117    |
118    = help: did you mean to capture by reference instead?
119 
119 
- warning: value assigned to `v` is never read
+ warning: value passed to `v` is never read
122    |
123 LL |             v = T::default();

125    |
125    |
126    = help: maybe it is overwritten before being read?
127 
- warning: value assigned to `z` is never read
+ warning: value passed to `z` is never read
130    |
130    |
131 LL |                 z = T::default();
141    |
142    = help: did you mean to capture by reference instead?
143 
- warning: value assigned to `state` is never read
- warning: value assigned to `state` is never read
+ warning: value passed to `state` is never read
145   --> $DIR/liveness-upvars.rs:125:9
146    |
147 LL |         state = 4;

149    |
150    = help: maybe it is overwritten before being read?
- warning: value assigned to `state` is never read
+ warning: value passed to `state` is never read
153   --> $DIR/liveness-upvars.rs:128:9
154    |
154    |
155 LL |         state = 5;

165    |
166    = help: did you mean to capture by reference instead?
167 
- warning: value assigned to `s` is never read
+ warning: value passed to `s` is never read
170    |
171 LL |         s = 1;

173    |
173    |
174    = help: maybe it is overwritten before being read?
- warning: value assigned to `s` is never read
- warning: value assigned to `s` is never read
+ warning: value passed to `s` is never read
178    |
178    |
179 LL |         s = yield ();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-upvars/liveness-upvars.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args liveness/liveness-upvars.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/liveness/liveness-upvars.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-upvars" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-upvars/auxiliary" "--edition=2018"
stdout: none
warning: value passed to `last` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:10:9
   |
   |
LL |         last = Some(s); //~  WARN value assigned to `last` is never read
   |
   |
   = help: maybe it is overwritten before being read?
  --> fake-test-src-base/liveness/liveness-upvars.rs:4:9
   |
LL | #![warn(unused)]
   |         ^^^^^^
   |         ^^^^^^
   = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`

warning: unused variable: `last`
  --> fake-test-src-base/liveness/liveness-upvars.rs:10:9
   |
LL |         last = Some(s); //~  WARN value assigned to `last` is never read
   |
   = help: did you mean to capture by reference instead?
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`


warning: unused variable: `sum`
  --> fake-test-src-base/liveness/liveness-upvars.rs:22:9
   |
LL |         sum += x; //~ WARN unused variable: `sum`
   |
   = help: did you mean to capture by reference instead?

warning: value captured by `c` is never read
warning: value captured by `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:32:9
   |
LL |         c = 1; //~ WARN value captured by `c` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value captured by `c` is never read
warning: value captured by `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:36:9
   |
LL |         c = 1; //~ WARN value captured by `c` is never read
   |
   = help: did you mean to capture by reference instead?

warning: unused variable: `c`
warning: unused variable: `c`
  --> fake-test-src-base/liveness/liveness-upvars.rs:42:9
   |
LL |         c += 1; //~ WARN unused variable: `c`
   |
   = help: did you mean to capture by reference instead?

warning: value passed to `c` is never read
warning: value passed to `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:45:9
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `c`
  --> fake-test-src-base/liveness/liveness-upvars.rs:45:9
   |
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value passed to `c` is never read
warning: value passed to `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:58:9
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `c` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:64:9
   |
   |
LL |         c += 1; //~  WARN value assigned to `c` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `d` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:73:13
   |
   |
LL |             d = Some("d1"); //~ WARN value assigned to `d` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `e` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:77:13
   |
   |
LL |             e = Some("e1"); //~  WARN value assigned to `e` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `e` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:79:13
   |
   |
LL |             e = Some("e2"); //~  WARN value assigned to `e` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `e`
  --> fake-test-src-base/liveness/liveness-upvars.rs:77:13
   |
   |
LL |             e = Some("e1"); //~  WARN value assigned to `e` is never read
   |
   = help: did you mean to capture by reference instead?


warning: value passed to `v` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:87:13
   |
LL |             v = T::default(); //~ WARN value assigned to `v` is never read
   |
   |
   = help: maybe it is overwritten before being read?

warning: value passed to `z` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:99:17
   |
LL |                 z = T::default(); //~  WARN value assigned to `z` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `z`
  --> fake-test-src-base/liveness/liveness-upvars.rs:99:17
   |
   |
LL |                 z = T::default(); //~  WARN value assigned to `z` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value passed to `state` is never read
warning: value passed to `state` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:125:9
   |
LL |         state = 4;  //~  WARN value assigned to `state` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `state` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:128:9
   |
   |
LL |         state = 5;  //~ WARN value assigned to `state` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: unused variable: `state`
  --> fake-test-src-base/liveness/liveness-upvars.rs:125:9
   |
   |
LL |         state = 4;  //~  WARN value assigned to `state` is never read
   |
   = help: did you mean to capture by reference instead?

warning: value passed to `s` is never read
warning: value passed to `s` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:137:9
   |
LL |         s = 1; //~ WARN value assigned to `s` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: value passed to `s` is never read
  --> fake-test-src-base/liveness/liveness-upvars.rs:139:9
   |
   |
LL |         s = yield (); //~ WARN value assigned to `s` is never read
   |
   |
   = help: maybe it is overwritten before being read?
warning: 22 warnings emitted
------------------------------------------


---
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-counter-not-moved.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unboxed-closures/unboxed-closures-counter-not-moved.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-counter-not-moved/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-counter-not-moved/auxiliary"
stdout: none
warning: unused variable: `item`
  --> fake-test-src-base/unboxed-closures/unboxed-closures-counter-not-moved.rs:15:13
   |
   |
LL |         for item in y { //~ WARN unused variable: `item`
   |             ^^^^ help: if this is intentional, prefix it with an underscore: `_item`
   = note: `#[warn(unused_variables)]` on by default

warning: value passed to `counter` is never read
  --> fake-test-src-base/unboxed-closures/unboxed-closures-counter-not-moved.rs:24:9
  --> fake-test-src-base/unboxed-closures/unboxed-closures-counter-not-moved.rs:24:9
   |
LL |         counter += 1; //~  WARN value assigned to `counter` is never read
   |
   |
   = help: maybe it is overwritten before being read?

warning: unused variable: `counter`
  --> fake-test-src-base/unboxed-closures/unboxed-closures-counter-not-moved.rs:24:9
   |
   |
LL |         counter += 1; //~  WARN value assigned to `counter` is never read
   |
   = help: did you mean to capture by reference instead?

warning: 3 warnings emitted
