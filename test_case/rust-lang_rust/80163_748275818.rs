plain
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_8d135248-9ae8-4247-95c0-d4838be317fa
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=binder-refactor-part-3
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_8d135248-9ae8-4247-95c0-d4838be317fa
GITHUB_REF=refs/pull/80163/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=431148521
GITHUB_RUN_NUMBER=21631
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-e0dz7fz9/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmp48g0xexbpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
.................................................................................................... 6800/11178
.................................................................................................... 6900/11178
...............................................................................ii................i.i 7000/11178
..ii................................................................................................ 7100/11178
..................................................FF....FFFFF...F.FF.F...F.......................... 7200/11178
................................................F................................................... 7400/11178
.....................i....ii..........................................................ii............ 7500/11178
.................................................................................................... 7600/11178
.................................................................................................... 7700/11178
---
.................................................................................................... 9000/11178
.................................................................................................... 9100/11178
......................................................................i......i...................... 9200/11178
.................................................................................................... 9300/11178
.........iiiiii..iiiiii.i........................................................................... 9400/11178
.................................................................................................... 9600/11178
.................................................................................................... 9700/11178
.................................................................................................... 9800/11178
.................................................................................................... 9900/11178
---
---- [ui] ui/nll/closure-requirements/escape-argument.rs stdout ----
diff of stderr:

6    |
7    = note: defining type: test::{closure#0} with closure substs [
8                i16,
-                for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) mut &ReLateBound(DebruijnIndex(0), BrNamed('s)) i32, &ReLateBound(DebruijnIndex(0), BrNamed('s)) i32)),
+                for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) mut &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32)),
10                (),
12 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument/escape-argument.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/escape-argument.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/escape-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument.rs:26:38
   |
LL |         let mut closure = expect_sig(|p, y| *p = y);
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) mut &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32)),
               (),

note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument.rs:20:1
   |
   |
LL | / fn test() {
LL | |     let x = 44;
LL | |     let mut p = &x;
LL | |
...  |
LL | |     deref(p);
LL | | }
   |
   = note: defining type: test


error[E0597]: `y` does not live long enough
   |
   |
LL |         closure(&mut p, &y);
   |                         ^^ borrowed value does not live long enough
LL |         //~^ ERROR `y` does not live long enough [E0597]
LL |     }
   |     - `y` dropped here while still borrowed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     deref(p);
LL |     deref(p);
   |           - borrow later used here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/nll/closure-requirements/escape-argument-callee.rs stdout ----
diff of stderr:

6    |
7    = note: defining type: test::{closure#0} with closure substs [
8                i16,
-                for<'r, 's, 't0> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) mut &ReLateBound(DebruijnIndex(0), BrNamed('s)) i32, &ReLateBound(DebruijnIndex(0), BrNamed('t0)) i32)),
+                for<'r, 's, 't0> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) mut &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) i32)),
10                (),
12 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/escape-argument-callee.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/escape-argument-callee.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:26:38
   |
LL |         let mut closure = expect_sig(|p, y| *p = y);
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) mut &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) i32)),
               (),


error: lifetime may not live long enough
   |
   |
LL |         let mut closure = expect_sig(|p, y| *p = y);
   |                                       -  -  ^^^^^^ assignment requires that `'1` must outlive `'2`
   |                                       |  |
   |                                       |  has type `&'1 i32`
   |                                       has type `&'_#2r mut &'2 i32`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:20:1
   |
LL | / fn test() {
LL | / fn test() {
LL | |     let x = 44;
LL | |     let mut p = &x;
LL | |
...  |
LL | |     deref(p);
LL | | }
   |
   = note: defining type: test

error: aborting due to previous error
---
---- [ui] ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs stdout ----
diff of stderr:

10    |
11    = note: defining type: supply::{closure#0} with closure substs [
12                i16,
-                for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>, std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) &'_#3r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>)),
+                for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#3r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
14                (),
15            ]
16    = note: late-bound region is '_#4r

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-fail-no-postdom.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:43:9
   |
LL | /         |_outlives1, _outlives2, _outlives3, x, y| {
LL | |             // Only works if 'x: 'y:
LL | |             let p = x.get();
LL | |             demand_y(x, y, p) //~ ERROR
LL | |         },
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#3r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#5r
   = note: late-bound region is '_#6r

error: lifetime may not live long enough
   |
   |
LL |         |_outlives1, _outlives2, _outlives3, x, y| {
   |          ----------              ---------- has type `Cell<&'2 &'_#3r u32>`
   |          |
   |          has type `Cell<&'_#1r &'1 u32>`
...
LL |             demand_y(x, y, p) //~ ERROR
   |             ^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:38:1
   |
   |
LL | / fn supply<'a, 'b, 'c>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>, cell_c: Cell<&'c u32>) {
LL | |     establish_relationships(
LL | |         cell_a,
LL | |         cell_b,
LL | |     );
LL | | }
   | |_^
   |
---
---- [ui] ui/nll/closure-requirements/propagate-approximated-ref.rs stdout ----
diff of stderr:

11    |
12    = note: defining type: supply::{closure#0} with closure substs [
13                i16,
-                for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('t1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('t1)) u32>)),
+                for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>)),
15                (),
16            ]
17    = note: late-bound region is '_#3r

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/propagate-approximated-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-ref.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:43:47
   |
LL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |  _______________________________________________^
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:42:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
LL | | }
   |
   = note: defining type: supply


error: lifetime may not live long enough
   |
   |
LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |           --  -- lifetime `'b` defined here
   |           |
   |           lifetime `'a` defined here
...
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs stdout ----
diff of stderr:

10    |
11    = note: defining type: case1::{closure#0} with closure substs [
12                i32,
-                for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>)),
+                for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>)),
14                (),
16 

49    |
49    |
50    = note: defining type: case2::{closure#0} with closure substs [
51                i32,
-                for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>)),
+                for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>)),
53                (),
54            ]
55    = note: number of external vids: 2

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/propagate-approximated-shorter-to-static-comparing-against-free.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/propagate-approximated-shorter-to-static-comparing-against-free.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:21:15
   |
LL |       foo(cell, |cell_a, cell_x| {
   |  _______________^
LL | |         cell_a.set(cell_x.get()); // forces 'x: 'a, error in closure
LL | |         //~^ ERROR
LL | |     })
   |
   |
   = note: defining type: case1::{closure#0} with closure substs [
               i32,
               for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>)),
               (),


error[E0521]: borrowed data escapes outside of closure
   |
   |
LL |     foo(cell, |cell_a, cell_x| {
   |                ------  ------ `cell_x` is a reference that is only valid in the closure body
   |                |
   |                `cell_a` declared here, outside of the closure body
LL |         cell_a.set(cell_x.get()); // forces 'x: 'a, error in closure
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ `cell_x` escapes the closure body here
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:18:1
   |
   |
LL | / fn case1() {
LL | |     let a = 0;
LL | |     let cell = Cell::new(&a);
LL | |     foo(cell, |cell_a, cell_x| {
LL | |     })
LL | | }
   | |_^
   |
   |
   = note: defining type: case1

note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:35:15
   |
LL |       foo(cell, |cell_a, cell_x| {
   |  _______________^
LL | |         cell_x.set(cell_a.get()); // forces 'a: 'x, implies 'a = 'static -> borrow error
LL | |     })
   |
   |
   = note: defining type: case2::{closure#0} with closure substs [
               i32,
               for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>)),
               (),
           ]
   = note: number of external vids: 2
   = note: where '_#1r: '_#0r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:28:1
   |
   |
LL | / fn case2() {
LL | |     let a = 0;
LL | |     let cell = Cell::new(&a);
LL | |     //~^ ERROR `a` does not live long enough
LL | |     })
LL | | }
   | |_^
   |
   |
   = note: defining type: case2

error[E0597]: `a` does not live long enough
   |
   |
LL |     let cell = Cell::new(&a);
   |                ----------^^-
   |                |         |
   |                |         borrowed value does not live long enough
   |                argument requires that `a` is borrowed for `'static`
LL | }
LL | }
   | - `a` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs stdout ----
diff of stderr:

12    |
13    = note: defining type: supply::{closure#0} with closure substs [
14                i16,
-                for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t1)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('t2)) u32>)),
+                for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) u32>)),
16                (),
17            ]
18    = note: late-bound region is '_#2r

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/propagate-approximated-shorter-to-static-no-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/propagate-approximated-shorter-to-static-no-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:32:47
   |
LL |       establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |  _______________________________________________^
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |     });
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where '_#1r: '_#0r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:31:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |     });
LL | | }
   | |_^
   |
   |
   = note: defining type: supply

error[E0521]: borrowed data escapes outside of function
   |
   |
LL |   fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |                     ------ `cell_a` is a reference that is only valid in the function body
LL | /     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |     });
   | |______^ `cell_a` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs stdout ----
diff of stderr:

12    |
13    = note: defining type: supply::{closure#0} with closure substs [
14                i16,
-                for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t0)) std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BrNamed('t1)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('t1)) u32>)),
+                for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>)),
16                (),
17            ]
18    = note: late-bound region is '_#3r

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/propagate-approximated-shorter-to-static-wrong-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/propagate-approximated-shorter-to-static-wrong-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:35:47
   |
LL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |  _______________________________________________^
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |     });
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#0r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:34:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |     });
LL | | }
   | |_^
   |
   |
   = note: defining type: supply

error[E0521]: borrowed data escapes outside of function
   |
   |
LL |   fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |                     ------ `cell_a` is a reference that is only valid in the function body
LL | /     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |     });
   | |______^ `cell_a` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs stdout ----
diff of stderr:

11    |
12    = note: defining type: supply::{closure#0} with closure substs [
13                i16,
-                for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('t1)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>)),
+                for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
15                (),
16            ]
17    = note: late-bound region is '_#2r

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:35:47
   |
LL |       establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |  _______________________________________________^
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#3r

error: lifetime may not live long enough
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |                                                ---------  - has type `&'_#7r Cell<&'1 u32>`
   |                                                |
   |                                                has type `&'_#5r Cell<&'2 &'_#1r u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:34:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
---
---- [ui] ui/nll/closure-requirements/propagate-despite-same-free-region.rs stdout ----
diff of stderr:

10    |
11    = note: defining type: supply::{closure#0} with closure substs [
12                i16,
-                for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>)),
+                for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
14                (),
15            ]
16    = note: late-bound region is '_#3r

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-despite-same-free-region/propagate-despite-same-free-region.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-despite-same-free-region/propagate-despite-same-free-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-despite-same-free-region.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-despite-same-free-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-despite-same-free-region" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-despite-same-free-region/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-despite-same-free-region.rs:42:9
   |
LL | /         |_outlives1, _outlives2, x, y| {
LL | |             // Only works if 'x: 'y:
LL | |             let p = x.get();
LL | |             demand_y(x, y, p)
LL | |         },
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-despite-same-free-region.rs:39:1
   |
   |
LL | / fn supply<'a>(cell_a: Cell<&'a u32>) {
LL | |     establish_relationships(
LL | |         cell_a,
LL | |         |_outlives1, _outlives2, x, y| {
LL | |     );
LL | | }
   | |_^
   |
---
---- [ui] ui/nll/closure-requirements/propagate-approximated-val.rs stdout ----
diff of stderr:

11    |
12    = note: defining type: test::{closure#0} with closure substs [
13                i16,
-                for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>)),
+                for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
15                (),
16            ]
17    = note: late-bound region is '_#3r

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-val.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:36:45
   |
LL |       establish_relationships(cell_a, cell_b, |outlives1, outlives2, x, y| {
   |  _____________________________________________^
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(outlives1, outlives2, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:35:1
   |
   |
LL | / fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(cell_a, cell_b, |outlives1, outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(outlives1, outlives2, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
LL | | }
   |
   = note: defining type: test


error: lifetime may not live long enough
   |
   |
LL | fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |         demand_y(outlives1, outlives2, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs stdout ----
diff of stderr:

11    |
12    = note: defining type: supply::{closure#0} with closure substs [
13                i16,
-                for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('t1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('s)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed('t3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed('t1)) u32>)),
+                for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>)),
15                (),
16            ]
17    = note: late-bound region is '_#3r

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/propagate-fail-to-approximate-longer-wrong-bounds.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/propagate-fail-to-approximate-longer-wrong-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:39:47
   |
LL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |  _______________________________________________^
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('t1) }) u32>)),
               (),
           ]
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r

error: lifetime may not live long enough
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |                                                ----------  ---------- has type `&'_#8r Cell<&'2 &'_#2r u32>`
   |                                                |
   |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:38:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
---
---- [ui] ui/nll/closure-requirements/return-wrong-bound-region.rs stdout ----
diff of stderr:

6    |
7    = note: defining type: test::{closure#0} with closure substs [
8                i16,
-                for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed('r)) i32, &ReLateBound(DebruijnIndex(0), BrNamed('s)) i32)) -> &ReLateBound(DebruijnIndex(0), BrNamed('r)) i32,
+                for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32)) -> &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) i32,
10                (),
12 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/return-wrong-bound-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/return-wrong-bound-region.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:11:16
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) i32)) -> &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) i32,
               (),


error: lifetime may not live long enough
   |
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |                 |  |
   |                 |  has type `&'1 i32`
   |                 has type `&'2 i32`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:10:1
   |
LL | / fn test() {
LL | / fn test() {
LL | |     expect_sig(|a, b| b); // ought to return `a`
LL | |     //~^ ERROR
LL | | }
   |
   = note: defining type: test

error: aborting due to previous error
---
---- [ui] ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs stdout ----
diff of stderr:

6    |
7    = note: defining type: generic::<T>::{closure#0} with closure substs [
8                i16,
-                for<'r, 's> extern "rust-call" fn((std::option::Option<std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('r)) ()>>, &ReLateBound(DebruijnIndex(0), BrNamed('s)) T)),
+                for<'r, 's> extern "rust-call" fn((std::option::Option<std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) ()>>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) T)),
10                (),
11            ]
12    = note: number of external vids: 2
31    |
31    |
32    = note: defining type: generic_fail::<T>::{closure#0} with closure substs [
33                i16,
-                for<'r, 's> extern "rust-call" fn((std::option::Option<std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed('r)) ()>>, &ReLateBound(DebruijnIndex(0), BrNamed('s)) T)),
+                for<'r, 's> extern "rust-call" fn((std::option::Option<std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) ()>>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) T)),
35                (),
36            ]
37    = note: late-bound region is '_#2r

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound/ty-param-closure-approximate-lower-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs:24:24
   |
LL |     twice(cell, value, |a, b| invoke(a, b));
   |
   |
   = note: defining type: generic::<T>::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::option::Option<std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) ()>>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) T)),
               (),
           ]
   = note: number of external vids: 2
   = note: where T: '_#1r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs:22:1
   |
   |
LL | / fn generic<T>(value: T) {
LL | |     let cell = Cell::new(&());
LL | |     twice(cell, value, |a, b| invoke(a, b));
LL | | }
   |
   |
   = note: defining type: generic::<T>
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs:29:24
   |
   |
LL |     twice(cell, value, |a, b| invoke(a, b));
   |
   |
   = note: defining type: generic_fail::<T>::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::option::Option<std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('r) }) ()>>, &ReLateBound(DebruijnIndex(0), BoundRegion { kind: BrNamed('s) }) T)),
               (),
           ]
   = note: late-bound region is '_#2r
   = note: number of external vids: 3
   = note: where T: '_#1r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs:28:1
   |
   |
LL | / fn generic_fail<'a, T>(cell: Cell<&'a ()>, value: T) {
LL | |     twice(cell, value, |a, b| invoke(a, b));
LL | |     //~^ ERROR the parameter type `T` may not live long enough
LL | | }
   |
   |
   = note: defining type: generic_fail::<T>

error[E0309]: the parameter type `T` may not live long enough
   |
   |
LL |     twice(cell, value, |a, b| invoke(a, b));
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0309`.

---
test result: FAILED. 11083 passed; 13 failed; 82 ignored; 0 measured; 0 filtered out; finished in 149.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:07
