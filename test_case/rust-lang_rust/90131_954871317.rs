plain
.................................................................................................... 600/12355
...........................................................................i........................ 700/12355
...................................................ii............................................... 800/12355
.................................................................................................... 900/12355
...................F................................F..F............................................ 1000/12355
...............................................F.................................................... 1100/12355
..........F...........F.................................................i........................... 1200/12355
.................................................................................................... 1300/12355
...................................................................................F........F.....F. 1400/12355
...F................................................................................................ 1500/12355
........................................................i...........F............................... 1600/12355
..............................................................i..................................... 1800/12355
.................................................................................................... 1900/12355
.................................................................................................... 2000/12355
.............................i...................................................................... 2100/12355
.............................i...................................................................... 2100/12355
.................................................................................................... 2200/12355
....................................F............................................F.................. 2300/12355
.................................................................................................... 2500/12355
.................................................................................................... 2600/12355
.................................................................................................... 2700/12355
...................................................i..i............................................. 2800/12355
---
.........................................................................................i....i..... 6600/12355
...........F.........................i...............i.............i................................ 6700/12355
................i................................................................................... 6800/12355
.................i.................................................................................. 6900/12355
............................................F.FFi...............................F................... 7000/12355
.........................i.......................................................................... 7200/12355
.................................................................................................... 7300/12355
.................................................................................................... 7400/12355
.................................................................................................... 7500/12355
---
diff of stderr:

3    |
4 LL |     println!("{}", i);
5    |                    ^ use of possibly-uninitialized `i`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args borrowck/borrowck-and-init.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-and-init.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-and-init" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-and-init/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0381]: borrow of possibly-uninitialized variable: `i`
   |
   |
LL |     println!("{}", i); //~ ERROR borrow of possibly-uninitialized variable: `i`
   |                    ^ use of possibly-uninitialized `i`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

3    |
4 LL |     println!("{}", x);
5    |                    ^ use of possibly-uninitialized `x`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args borrowck/borrowck-break-uninit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-break-uninit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-break-uninit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-break-uninit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0381]: borrow of possibly-uninitialized variable: `x`
   |
   |
LL |     println!("{}", x); //~ ERROR borrow of possibly-uninitialized variable: `x`
   |                    ^ use of possibly-uninitialized `x`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

3    |
4 LL |     println!("{}", x);
5    |                    ^ use of possibly-uninitialized `x`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
8 
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-break-uninit-2/borrowck-break-uninit-2.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-break-uninit-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-break-uninit-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-break-uninit-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-break-uninit-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0381]: borrow of possibly-uninitialized variable: `x`
   |
   |
LL |     println!("{}", x); //~ ERROR borrow of possibly-uninitialized variable: `x`
   |                    ^ use of possibly-uninitialized `x`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

3    |
4 LL |     println!("{}", i);
5    |                    ^ use of possibly-uninitialized `i`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args borrowck/borrowck-or-init.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-or-init.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-or-init" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-or-init/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0381]: borrow of possibly-uninitialized variable: `i`
   |
   |
LL |     println!("{}", i); //~ ERROR borrow of possibly-uninitialized variable: `i`
   |                    ^ use of possibly-uninitialized `i`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
---- [ui] ui/borrowck/borrowck-while-break.rs stdout ----
diff of stderr:

3    |
4 LL |     println!("{}", v);
5    |                    ^ use of possibly-uninitialized `v`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args borrowck/borrowck-while-break.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-while-break.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-while-break" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-while-break/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0381]: borrow of possibly-uninitialized variable: `v`
   |
   |
LL |     println!("{}", v); //~ ERROR borrow of possibly-uninitialized variable: `v`
   |                    ^ use of possibly-uninitialized `v`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

3    |
4 LL |     println!("{}", x);
5    |                    ^ use of possibly-uninitialized `x`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
6 
6 
7 error[E0381]: borrow of possibly-uninitialized variable: `x`

9    |
10 LL |     println!("{}", x);
10 LL |     println!("{}", x);
11    |                    ^ use of possibly-uninitialized `x`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error: aborting due to 2 previous errors
14 
---
To only update this specific test, also pass `--test-args borrowck/issue-24267-flow-exit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-24267-flow-exit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-24267-flow-exit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-24267-flow-exit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0381]: borrow of possibly-uninitialized variable: `x`
   |
   |
LL |     println!("{}", x); //~ ERROR borrow of possibly-uninitialized variable: `x`
   |                    ^ use of possibly-uninitialized `x`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0381]: borrow of possibly-uninitialized variable: `x`
   |
   |
LL |     println!("{}", x); //~ ERROR borrow of possibly-uninitialized variable: `x`
   |                    ^ use of possibly-uninitialized `x`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

---
---- [ui] ui/closures/2229_closure_analysis/diagnostics/arrays.rs stdout ----
diff of stderr:

81 ...
82 LL |     c();
83    |     - mutable borrow later used here
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
84 
84 
85 error[E0502]: cannot borrow `arr` as immutable because it is also borrowed as mutable


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/arrays/arrays.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/arrays/arrays.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/arrays.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/arrays.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/arrays" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/arrays/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0503]: cannot use `arr` because it was mutably borrowed
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- borrow of `arr` occurs here
LL |         arr[0] += 10;
   |         --- borrow occurs due to use of `arr` in closure
...
LL |     arr[1] += 10;
   |     ^^^^^^ use of borrowed `arr`
LL |     c();
LL |     c();
   |     - borrow later used here

error[E0503]: cannot use `arr[_]` because it was mutably borrowed
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- borrow of `arr` occurs here
LL |         arr[0] += 10;
   |         --- borrow occurs due to use of `arr` in closure
...
LL |     arr[1] += 10;
   |     ^^^^^^^^^^^^ use of borrowed `arr`
LL |     c();
LL |     c();
   |     - borrow later used here

error[E0506]: cannot assign to `arr[_]` because it is borrowed
   |
LL |     let c = || {
LL |     let c = || {
   |             -- borrow of `arr[_]` occurs here
LL |         println!("{:#?}", &arr[3..4]);
   |                            --- borrow occurs due to use in closure
...
LL |     arr[1] += 10;
   |     ^^^^^^^^^^^^ assignment to borrowed `arr[_]` occurs here
LL |     //~^ ERROR: cannot assign to `arr[_]` because it is borrowed
LL |     c();
   |     - borrow later used here

error[E0506]: cannot assign to `arr[_]` because it is borrowed
   |
LL |     let c = || {
LL |     let c = || {
   |             -- borrow of `arr[_]` occurs here
LL |         println!("{}", arr[3]);
   |                        --- borrow occurs due to use in closure
...
LL |     arr[1] += 10;
   |     ^^^^^^^^^^^^ assignment to borrowed `arr[_]` occurs here
LL |     //~^ ERROR: cannot assign to `arr[_]` because it is borrowed
LL |     c();
   |     - borrow later used here

error[E0503]: cannot use `arr` because it was mutably borrowed
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- borrow of `arr` occurs here
LL |         arr[1] += 10;
   |         --- borrow occurs due to use of `arr` in closure
LL |     println!("{}", arr[3]);
   |                    ^^^^^^ use of borrowed `arr`
...
LL |     c();
LL |     c();
   |     - borrow later used here

error[E0502]: cannot borrow `arr[_]` as immutable because it is also borrowed as mutable
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- mutable borrow occurs here
LL |         arr[1] += 10;
   |         --- first borrow occurs due to use of `arr` in closure
LL |     println!("{}", arr[3]);
LL |     println!("{}", arr[3]);
   |                    ^^^^^^ immutable borrow occurs here
LL |     c();
LL |     c();
   |     - mutable borrow later used here
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0502]: cannot borrow `arr` as immutable because it is also borrowed as mutable
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- mutable borrow occurs here
LL |         arr[1] += 10;
   |         --- first borrow occurs due to use of `arr` in closure
...
LL |     println!("{:#?}", &arr[3..2]);
   |                        ^^^ immutable borrow occurs here
LL |     c();
LL |     c();
   |     - mutable borrow later used here
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0502, E0503, E0506.
For more information about an error, try `rustc --explain E0502`.
---
---- [ui] ui/closures/2229_closure_analysis/diagnostics/box.rs stdout ----
diff of stderr:

25 LL |
26 LL |     c();
27    |     - mutable borrow later used here
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
28 
28 
29 error[E0506]: cannot assign to `e.0.0.m.x` because it is borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/box/box.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/box/box.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/box.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/box" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/box/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0506]: cannot assign to `e.0.0.m.x` because it is borrowed
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- borrow of `e.0.0.m.x` occurs here
LL |         e.0.0.m.x = format!("not-x");
   |         --------- borrow occurs due to use in closure
...
LL |     e.0.0.m.x = format!("not-x");
   |     ^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
LL |     //~^ ERROR: cannot assign to `e.0.0.m.x` because it is borrowed
LL |     c();
   |     - borrow later used here

error[E0502]: cannot borrow `e.0.0.m.x` as immutable because it is also borrowed as mutable
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- mutable borrow occurs here
LL |         e.0.0.m.x = format!("not-x");
   |         --------- first borrow occurs due to use of `e.0.0.m.x` in closure
...
LL |     println!("{}", e.0.0.m.x);
   |                    ^^^^^^^^^ immutable borrow occurs here
LL |     //~^ ERROR: cannot borrow `e.0.0.m.x` as immutable because it is also borrowed as mutable
LL |     c();
   |     - mutable borrow later used here
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0506]: cannot assign to `e.0.0.m.x` because it is borrowed
   |
LL |     let c = || {
LL |     let c = || {
   |             -- borrow of `e.0.0.m.x` occurs here
LL |         println!("{}", e.0.0.m.x);
   |                        --------- borrow occurs due to use in closure
...
LL |     e.0.0.m.x = format!("not-x");
   |     ^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
LL |     //~^ ERROR: cannot assign to `e.0.0.m.x` because it is borrowed
LL |     c();
   |     - borrow later used here
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0502, E0506.
For more information about an error, try `rustc --explain E0502`.
---
diff of stderr:

8    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
9    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
10    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+    = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
12 warning: 1 warning emitted
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed/repr_packed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/repr_packed.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed.rs:24:24
   |
LL |         println!("{}", foo.x);
   |
   = note: `#[warn(unaligned_references)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/closures/2229_closure_analysis/diagnostics/simple-struct-min-capture.rs stdout ----
diff of stderr:

13 LL |
14 LL |     c();
15    |     - mutable borrow later used here
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
16 
17 error: aborting due to previous error
18 
---
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/simple-struct-min-capture.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/simple-struct-min-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/simple-struct-min-capture" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/simple-struct-min-capture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0502]: cannot borrow `p` as immutable because it is also borrowed as mutable
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- mutable borrow occurs here
LL |         p.x += 10;
   |         --- capture is mutable because of use here
LL |         println!("{:?}", p);
   |                          - first borrow occurs due to use of `p` in closure
LL |     println!("{:?}", p);
LL |     println!("{:?}", p);
   |                      ^ immutable borrow occurs here
LL |     //~^ ERROR: cannot borrow `p` as immutable because it is also borrowed as mutable
LL |     c();
   |     - mutable borrow later used here
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
19 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab_3/tab_3.stderr
To only update this specific test, also pass `--test-args codemap_tests/tab_3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/tab_3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab_3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab_3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `some_vec`
   |
   |
LL |     let some_vec = vec!["hi"];
   |         -------- move occurs because `some_vec` has type `Vec<&str>`, which does not implement the `Copy` trait
LL |     some_vec.into_iter();
   |              ----------- `some_vec` moved due to this method call
LL |     {
LL |         println!("{:?}", some_vec); //~ ERROR borrow of moved
   |                          ^^^^^^^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `some_vec`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
---
To only update this specific test, also pass `--test-args consts/const-eval/conditional_array_execution.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:7:19
   |
LL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];
   |                   |
   |                   |
   |                   attempt to compute `5_u32 - 6_u32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:3:9
   |
   |
LL | #![warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:12:20
   |
LL |     println!("{}", FOO);
   |                    ^^^ referenced constant has errors
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:12:20
   |
LL |     println!("{}", FOO);
LL |     println!("{}", FOO);
   |                    ^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

---
To only update this specific test, also pass `--test-args consts/const-eval/issue-43197.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-43197.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:10:20
   |
LL |     const X: u32 = 0 - 1;
   |                    |
   |                    |
   |                    attempt to compute `0_u32 - 1_u32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:3:9
   |
   |
LL | #![warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: any use of this value will cause an error
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:13:24
   |
LL |     const Y: u32 = foo(0 - 1);
   |                        |
   |                        |
   |                        attempt to compute `0_u32 - 1_u32`, which would overflow
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:23
   |
LL |     println!("{} {}", X, Y);
   |                       ^ referenced constant has errors
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:23
   |
   |
LL |     println!("{} {}", X, Y);
   |                       ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:26
   |
LL |     println!("{} {}", X, Y);
   |                          ^ referenced constant has errors
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:26
   |
   |
LL |     println!("{} {}", X, Y);
   |                          ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

---

---- [ui] ui/generator/yield-while-ref-reborrowed.rs stdout ----
diff of stderr:

10    |                    ^ second borrow occurs here
11 LL |     Pin::new(&mut b).resume(());
12    |              ------ first borrow later used here
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
13 
14 error: aborting due to previous error
15 
---
To only update this specific test, also pass `--test-args generator/yield-while-ref-reborrowed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-while-ref-reborrowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-ref-reborrowed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-ref-reborrowed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0501]: cannot borrow `x` as immutable because previous closure requires unique access
   |
LL |     let mut b = || {
   |                 -- generator construction occurs here
LL |         let a = &mut *x;
LL |         let a = &mut *x;
   |                      -- first borrow occurs due to use of `x` in generator
...
LL |     println!("{}", x); //~ ERROR
   |                    ^ second borrow occurs here
LL |     Pin::new(&mut b).resume(());
   |              ------ first borrow later used here
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

8 ...
9 LL |     println!("{}", s);
10    |                    ^ value borrowed here after move
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
12 error: aborting due to previous error
13 
---
To only update this specific test, also pass `--test-args issues/issue-42796.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42796.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `s`
   |
   |
LL |     let s = "Hello!".to_owned();
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
LL |     let mut s_copy = s;
   |                      - value moved here
...
LL |     println!("{}", s); //~ ERROR borrow of moved value
   |                    ^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

12 ...
13 LL |     };
14    |      - ... and the mutable borrow might be used here, when that temporary is dropped and runs the destructor for type `(Option<PeekMut<'_, i32>>, ())`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
15 
16 error: aborting due to previous error
17 
---
To only update this specific test, also pass `--test-args issues/issue-47646.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47646.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47646" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47646/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0502]: cannot borrow `heap` as immutable because it is also borrowed as mutable
   |
   |
LL |     let borrow = heap.peek_mut();
   |                  --------------- mutable borrow occurs here
LL | 
LL |     match (borrow, ()) {
   |           ------------ a temporary with access to the mutable borrow is created here ...
LL |         (Some(_), ()) => {
LL |             println!("{:?}", heap); //~ ERROR cannot borrow `heap` as immutable
   |                              ^^^^ immutable borrow occurs here
LL |     };
LL |     };
   |      - ... and the mutable borrow might be used here, when that temporary is dropped and runs the destructor for type `(Option<PeekMut<'_, i32>>, ())`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---

---- [ui] ui/limits/issue-55878.rs stdout ----
diff of stderr:

18    = note: `#[deny(const_err)]` on by default
20    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
21 
22 error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args limits/issue-55878.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/limits/issue-55878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: values of the type `[u8; 18446744073709551615]` are too big for the current architecture
   |
LL |     intrinsics::size_of::<T>()
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ inside `std::mem::size_of::<[u8; 18446744073709551615]>` at /checkout/library/core/src/mem/mod.rs:304:5
  ::: /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());

error: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---

---- [ui] ui/liveness/liveness-move-in-while.rs stdout ----
diff of stderr:

28    |                        ^ value borrowed here after move
29 LL |         while true { while true { while true { x = y; x.clone(); } } }
30    |                                                    - value moved here, in previous iteration of loop
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
31 
32 error: aborting due to previous error; 3 warnings emitted
33 
---
To only update this specific test, also pass `--test-args liveness/liveness-move-in-while.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-move-in-while.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-in-while" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-in-while/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |         ^^^^^^^^^^ help: use `loop`
   |
   = note: `#[warn(while_true)]` on by default

warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |                      ^^^^^^^^^^ help: use `loop`

warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |                                   ^^^^^^^^^^ help: use `loop`

error[E0382]: borrow of moved value: `y`
   |
   |
LL |     let y: Box<isize> = 42.into();
   |         - move occurs because `y` has type `Box<isize>`, which does not implement the `Copy` trait
...
LL |         println!("{}", y); //~ ERROR borrow of moved value: `y`
   |                        ^ value borrowed here after move
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |                                                    - value moved here, in previous iteration of loop
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error; 3 warnings emitted

---
---- [ui] ui/liveness/liveness-use-after-move.rs stdout ----
diff of stderr:

8 LL | 
9 LL |     println!("{}", *x);
10    |                    ^^ value borrowed here after move
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
12 error: aborting due to previous error
13 
---
To only update this specific test, also pass `--test-args liveness/liveness-use-after-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-use-after-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-move/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `x`
   |
   |
LL |     let x: Box<_> = 5.into();
   |         - move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
LL |     let y = x;
   |             - value moved here
LL | 
LL |     println!("{}", *x); //~ ERROR borrow of moved value: `x`
   |                    ^^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

7    |              ------- value moved here
8 LL |     println!("{}", message);
9    |                    ^^^^^^^ value borrowed here after move
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
10 
11 error: aborting due to previous error
12 
---
To only update this specific test, also pass `--test-args liveness/liveness-use-after-send.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-use-after-send.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-send" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-send/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `message`
   |
   |
LL | fn test00_start(ch: Chan<Box<isize>>, message: Box<isize>, _count: Box<isize>) {
   |                                       ------- move occurs because `message` has type `Box<isize>`, which does not implement the `Copy` trait
LL |     send(ch, message);
   |              ------- value moved here
LL |     println!("{}", message); //~ ERROR borrow of moved value: `message`
   |                    ^^^^^^^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

3    |
4 LL |     println!("{:?}", x);
5    |                      ^ use of possibly-uninitialized `x`
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args loops/loop-proper-liveness.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loops/loop-proper-liveness.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-proper-liveness" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-proper-liveness/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0381]: borrow of possibly-uninitialized variable: `x`
   |
   |
LL |     println!("{:?}", x); //~ ERROR borrow of possibly-uninitialized variable
   |                      ^ use of possibly-uninitialized `x`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

10 LL |     });
11 LL |     println!("{}", x);
12    |                    ^ value borrowed here after move
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
13 
14 error: aborting due to previous error
15 
---
To only update this specific test, also pass `--test-args moves/moves-based-on-type-capture-clause-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/moves-based-on-type-capture-clause-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-capture-clause-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-capture-clause-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `x`
   |
   |
LL |     let x = "Hello world!".to_string();
   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     thread::spawn(move|| {
   |                   ------ value moved into closure here
   |                        - variable moved due to use in closure
LL |     });
LL |     });
LL |     println!("{}", x); //~ ERROR borrow of moved value
   |                    ^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

21 LL |         };
22 LL |         println!("{}", x);
23    |                        ^ value borrowed here after move
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
24 
24 
25 error[E0506]: cannot assign to `i` because it is borrowed
26   --> $DIR/try-block-maybe-bad-lifetime.rs:40:9

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-maybe-bad-lifetime/try-block-maybe-bad-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-block/try-block-maybe-bad-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-maybe-bad-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-maybe-bad-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-maybe-bad-lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0506]: cannot assign to `i` because it is borrowed
   |
LL |             &i
LL |             &i
   |             -- borrow of `i` occurs here
LL |         };
LL |         i = 0; //~ ERROR cannot assign to `i` because it is borrowed
   |         ^^^^^ assignment to borrowed `i` occurs here
LL |         let _ = i;
LL |         do_something_with(x);
   |                           - borrow later used here

error[E0382]: borrow of moved value: `x`
   |
LL |         let x = String::new();
LL |         let x = String::new();
   |             - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |             ::std::mem::drop(x);
   |                              - value moved here
LL |         };
LL |         };
LL |         println!("{}", x); //~ ERROR borrow of moved value: `x`
   |                        ^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0506]: cannot assign to `i` because it is borrowed
   |
   |
LL |             j = &i;
   |                 -- borrow of `i` occurs here
LL |         };
LL |         i = 0; //~ ERROR cannot assign to `i` because it is borrowed
   |         ^^^^^ assignment to borrowed `i` occurs here
LL |         let _ = i;
LL |         do_something_with(j);
   |                           - borrow later used here
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0382, E0506.
For more information about an error, try `rustc --explain E0382`.
---
diff of stderr:

7    |              - value moved here
8 LL |     println!("{}", x);
9    |                    ^ value borrowed here after move
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
10 
11 error: aborting due to previous error
12 
---
To only update this specific test, also pass `--test-args use/use-after-move-based-on-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-after-move-based-on-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-based-on-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-based-on-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `x`
   |
   |
LL |     let x = "Hello!".to_string();
   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     let _y = x;
   |              - value moved here
LL |     println!("{}", x); //~ ERROR borrow of moved value
   |                    ^ value borrowed here after move
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
---- [ui] ui/walk-struct-literal-with.rs stdout ----
diff of stderr:

13    |
14 LL |     fn make_string_bar(mut self) -> Mine{
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
16 
17 error: aborting due to previous error
18 
18 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/walk-struct-literal-with/walk-struct-literal-with.stderr
To only update this specific test, also pass `--test-args walk-struct-literal-with.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/walk-struct-literal-with.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/walk-struct-literal-with" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/walk-struct-literal-with/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `start`
  --> /checkout/src/test/ui/walk-struct-literal-with.rs:16:20
   |
LL |     let start = Mine{test:"Foo".to_string(), other_val:0};
   |         ----- move occurs because `start` has type `Mine`, which does not implement the `Copy` trait
LL |     let end = Mine{other_val:1, ..start.make_string_bar()};
   |                                         ----------------- `start` moved due to this method call
LL |     println!("{}", start.test); //~ ERROR borrow of moved value: `start`
   |                    ^^^^^^^^^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `start`
  --> /checkout/src/test/ui/walk-struct-literal-with.rs:7:28
   |
LL |     fn make_string_bar(mut self) -> Mine{
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
test result: FAILED. 12220 passed; 25 failed; 110 ignored; 0 measured; 0 filtered out; finished in 134.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:18
