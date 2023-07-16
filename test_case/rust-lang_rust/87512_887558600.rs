plain
    Finished release [optimized] target(s) in 20.99s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12177 tests
.................................................F......F...............................F........... 100/12177
................................F............ii...........ii........................................ 200/12177
.................................................................................................... 300/12177
...........................................F............F........................................... 400/12177
.................................................................................................... 600/12177
....................................................i............................................... 700/12177
..FF.....................ii......................................................................... 800/12177
.................................................................F.................................. 900/12177
.................................................................F.................................. 900/12177
.........................F.......................................................................... 1000/12177
.................................................................................................... 1100/12177
........................F.................i......................................................... 1200/12177
...............................................................................F.................... 1300/12177
................F........F..F................F...................................................... 1400/12177
i................................................................................................... 1600/12177
................................................................................................i... 1700/12177
...................F................................................................................ 1800/12177
.................................................................................................... 1900/12177
.................................................................................................... 1900/12177
.................................................................................................... 2000/12177
.............................................i...................................................... 2100/12177
.................................................................................................... 2200/12177
.................................................................................................... 2300/12177
.................................................................................................... 2400/12177
.................................................................................................... 2500/12177
.................................................................................................... 2600/12177
......F............................................................................................. 2700/12177
............................................................F............i..i....F.................. 2800/12177
.........F....................................F..................F.................................. 2900/12177
.................................................................................................... 3100/12177
.F.................................................................................................. 3200/12177
.............................................................F...................................... 3300/12177
.................................................................................................... 3400/12177
.................................................................................................... 3400/12177
.................................................................................................... 3500/12177
....................................i........i.........i............F............................... 3600/12177
.................................................................................................... 3700/12177
...ii............................................................................................... 3800/12177
.....................i............................................i.....F........................... 3900/12177
.................................................................................................... 4000/12177
.................................................................................................... 4100/12177
..................................F....................................................F............ 4200/12177
..............................F.F................................................................... 4300/12177
.................................................................................................... 4500/12177
..............................................................................ii.................... 4600/12177
...........................F...............................................i........................ 4700/12177
...........................F...............................................i........................ 4700/12177
.........F.....F....F.F.......................................................................F..... 4800/12177
..........F..........................................F.......F..F..........................F........ 4900/12177
F....................F.............................................................................. 5000/12177
............................................F....................................................... 5200/12177
........................................................................................F........... 5300/12177
........................................................................................F........... 5300/12177
.........F...FF.......F.....................F........................................i..i........... 5400/12177
....................F.....F.F....................................................................... 5500/12177
............................................................F.F..................................... 5600/12177
..............................................F..................................................... 5700/12177
.................................F......................................................F........... 5800/12177
....................................................F............................................... 5900/12177
............................................................................................Fi...... 6000/12177
.....F..................................................................................F........... 6100/12177
...............................................................F...............................i.... 6200/12177
F....F..............................................................i............................... 6300/12177
.................F...........ii.ii.......i...i...................................................... 6500/12177
............................................................................i....i.................. 6600/12177
......................i..............i............i................................................. 6700/12177
.............................................................................F..................i... 6800/12177
---
..............F..............................ii................i..i..ii............................. 7500/12177
...........................................................F........................................ 7600/12177
.......F............................................................................................ 7700/12177
.................................................................................................... 7800/12177
.....F...........................................................................................i.. 7900/12177
ii..............F.................F.............................ii.................................. 8000/12177
.................................................................................................... 8100/12177
............F.....F.................F.....F.........................i............................... 8200/12177
.................................................................................................... 8400/12177
i................................................................................................... 8500/12177
......................................................................i............................. 8600/12177
.................................................................................................... 8700/12177
.................................................................................................... 8700/12177
.................................................................................................... 8800/12177
.................................................................................................... 8900/12177
.................................................................................................... 9000/12177
........................................................................iiii.iiiii.................. 9100/12177
...............................................ii...............i................................... 9200/12177
.................................................................................................... 9300/12177
.................F.................................................................................. 9400/12177
................F................................................................................... 9500/12177
.......F............................................................................................ 9600/12177
.................................................F.F................................................ 9700/12177
................ii.ii........i...................................................................... 9900/12177
..............................................................................................iiiiii 10000/12177
.i..iiiiii.i.......................................................................................F 10100/12177
.................................................................................................... 10200/12177
.................................................................................................... 10200/12177
................................F................................................................... 10300/12177
.................................................................................................... 10400/12177
.....................................................F...........F.................................. 10500/12177
............................F..........F.................F.......................................... 10600/12177
..........................................................................F......................... 10700/12177
.................................................................................................... 10800/12177
...ii..............................i.................................................F.............F 10900/12177
...............................F.............................................................F...... 11000/12177
.................................................................................................... 11200/12177
.................................................................................................... 11300/12177
......................................................................................ii............ 11400/12177
.............................................................................F...................... 11500/12177
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/alias-uninit-value.rs:17:31
   |
LL |     return RawT {struct_: st, cname: cname, hash: 0};
   |                               ^^^^^^^^^^^^ help: replace it with: `cname`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args alias-uninit-value.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/alias-uninit-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alias-uninit-value/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alias-uninit-value/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/alias-uninit-value.rs:17:31
   |
LL |     return RawT {struct_: st, cname: cname, hash: 0};
   |                               ^^^^^^^^^^^^ help: replace it with: `cname`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alignment-gep-tup-like-1/alignment-gep-tup-like-1.stderr
To only update this specific test, also pass `--test-args alignment-gep-tup-like-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/alignment-gep-tup-like-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alignment-gep-tup-like-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alignment-gep-tup-like-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/destructure-array-1.rs:11:28
   |
LL |     fn d(x: u8) -> D { D { x: x } }
   |                            ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/destructure-array-1/destructure-array-1.stderr
To only update this specific test, also pass `--test-args array-slice-vec/destructure-array-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/destructure-array-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/destructure-array-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/destructure-array-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/array-slice-vec/destructure-array-1.rs:11:28
   |
LL |     fn d(x: u8) -> D { D { x: x } }
   |                            ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---

+ warning: redundant field names in struct initialization
+   --> $DIR/vec-res-add.rs:6:26
+    |
+ LL | fn r(i:isize) -> R { R { i: i } }
+    |                          ^^^^ help: replace it with: `i`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0369]: cannot add `Vec<R>` to `Vec<R>`
3    |

6    |             |
7    |             Vec<R>
---
To only update this specific test, also pass `--test-args array-slice-vec/vec-res-add.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/vec-res-add.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-res-add" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-res-add/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/array-slice-vec/vec-res-add.rs:6:26
   |
LL | fn r(i:isize) -> R { R { i: i } }
   |                          ^^^^ help: replace it with: `i`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0369]: cannot add `Vec<R>` to `Vec<R>`
   |
   |
LL |     let k = i + j;
   |             - ^ - Vec<R>
   |             Vec<R>

error: aborting due to previous error; 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/associated-types-normalize-unifield-struct.rs:20:80
   |
LL | pub fn from_utc<Off: Offset>(offset: Off::State) -> DateTime<Off> { DateTime { offset: offset } }
   |                                                                                ^^^^^^^^^^^^^^ help: replace it with: `offset`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args associated-types/associated-types-normalize-unifield-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-normalize-unifield-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-normalize-unifield-struct/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-normalize-unifield-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/associated-types/associated-types-normalize-unifield-struct.rs:20:80
   |
LL | pub fn from_utc<Off: Offset>(offset: Off::State) -> DateTime<Off> { DateTime { offset: offset } }
   |                                                                                ^^^^^^^^^^^^^^ help: replace it with: `offset`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/associated-types-ref-from-struct.rs:19:22
   |
LL |         TesterPair { tester: tester, value: value }
   |                      ^^^^^^^^^^^^^^ help: replace it with: `tester`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/associated-types-ref-from-struct.rs:19:38
  --> $DIR/associated-types-ref-from-struct.rs:19:38
   |
LL |         TesterPair { tester: tester, value: value }
   |                                      ^^^^^^^^^^^^ help: replace it with: `value`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args associated-types/associated-types-ref-from-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-ref-from-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-ref-from-struct/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-ref-from-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/associated-types/associated-types-ref-from-struct.rs:19:22
   |
LL |         TesterPair { tester: tester, value: value }
   |                      ^^^^^^^^^^^^^^ help: replace it with: `tester`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/associated-types/associated-types-ref-from-struct.rs:19:38
  --> /checkout/src/test/ui/associated-types/associated-types-ref-from-struct.rs:19:38
   |
LL |         TesterPair { tester: tester, value: value }
   |                                      ^^^^^^^^^^^^ help: replace it with: `value`
warning: 2 warnings emitted


------------------------------------------
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/class-attributes-2/class-attributes-2.stderr
To only update this specific test, also pass `--test-args attributes/class-attributes-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/class-attributes-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/class-attributes-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/class-attributes-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/class-attributes-1.rs:17:35
   |
LL | fn cat(name: String) -> Cat { Cat{name: name,} }
   |                                   ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/class-attributes-1/class-attributes-1.stderr
To only update this specific test, also pass `--test-args attributes/class-attributes-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/class-attributes-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/class-attributes-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/class-attributes-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/attributes/class-attributes-1.rs:17:35
   |
LL | fn cat(name: String) -> Cat { Cat{name: name,} }
   |                                   ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args binop/binops.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binops/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
+    |         ^^^^ help: replace it with: `x`
+    |
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
1 error[E0716]: temporary value dropped while borrowed
3    |


11    = note: consider using a `let` binding to create a longer lived value
12    = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
15 
16 For more information about this error, try `rustc --explain E0716`.
---
To only update this specific test, also pass `--test-args borrowck/borrowck-borrowed-uniq-rvalue-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |         ^^^^ help: replace it with: `x`
   |
   = note: `#[warn(redundant_field_names)]` on by default

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let x = defer(&vec!["Goodbye", "world!"]); //~ ERROR temporary value dropped while borrowed
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                    |
   |                    creates a temporary which is freed while still in use
LL |     x.x[0];
   |     ------ borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0716`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/fsu-moves-and-copies.rs:11:39
   |
LL | fn ncint(v: isize) -> ncint { ncint { v: v } }
   |                                       ^^^^ help: replace it with: `v`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args borrowck/fsu-moves-and-copies.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/fsu-moves-and-copies.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/fsu-moves-and-copies/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/fsu-moves-and-copies/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/borrowck/fsu-moves-and-copies.rs:11:39
   |
LL | fn ncint(v: isize) -> ncint { ncint { v: v } }
   |                                       ^^^^ help: replace it with: `v`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/conditional-compile.rs:57:9
   |
LL |     r { i: i }
   |         ^^^^ help: replace it with: `i`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args cfg/conditional-compile.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg/conditional-compile.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/conditional-compile/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/conditional-compile/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/cfg/conditional-compile.rs:57:9
   |
LL |     r { i: i }
   |         ^^^^ help: replace it with: `i`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---

+ warning: redundant field names in struct initialization
+   --> $DIR/cleanup-rvalue-scopes-cf.rs:12:16
+    |
+ LL |     AddFlags { bits: bits }
+    |                ^^^^^^^^^^ help: replace it with: `bits`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0716]: temporary value dropped while borrowed
3    |

89    |
89    |
90    = note: consider using a `let` binding to create a longer lived value
- error: aborting due to 7 previous errors
+ error: aborting due to 7 previous errors; 1 warning emitted
93 
94 For more information about this error, try `rustc --explain E0716`.
---
To only update this specific test, also pass `--test-args cleanup-rvalue-scopes-cf.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-scopes-cf" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-scopes-cf/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:12:16
   |
LL |     AddFlags { bits: bits }
   |                ^^^^^^^^^^ help: replace it with: `bits`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let x1 = arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                   ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                   |
   |                   creates a temporary which is freed while still in use
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |      -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let x2 = AddFlags(1).get(); //~ ERROR temporary value dropped while borrowed
   |              ^^^^^^^^^^^      - temporary value is freed at the end of this statement
   |              |
   |              creates a temporary which is freed while still in use
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |          -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let x3 = &*arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                     ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                     |
   |                     creates a temporary which is freed while still in use
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |              -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let ref x4 = *arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                        ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                        |
   |                        creates a temporary which is freed while still in use
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |                  -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let &ref x5 = arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                        ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                        |
   |                        creates a temporary which is freed while still in use
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |                      -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let x6 = AddFlags(1).get(); //~ ERROR temporary value dropped while borrowed
   |              ^^^^^^^^^^^      - temporary value is freed at the end of this statement
   |              |
   |              creates a temporary which is freed while still in use
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |                          -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let StackBox { f: x7 } = StackBox { f: AddFlags(1).get() };
   |                                            ^^^^^^^^^^^        - temporary value is freed at the end of this statement
   |                                            |
   |                                            creates a temporary which is freed while still in use
LL |     //~^ ERROR temporary value dropped while borrowed
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |                              -- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value
error: aborting due to 7 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0716`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/cleanup-rvalue-for-scope.rs:17:16
   |
LL |     AddFlags { bits: bits }
   |                ^^^^^^^^^^ help: replace it with: `bits`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args cleanup-rvalue-for-scope.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cleanup-rvalue-for-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-for-scope/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-for-scope/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/cleanup-rvalue-for-scope.rs:17:16
   |
LL |     AddFlags { bits: bits }
   |                ^^^^^^^^^^ help: replace it with: `bits`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args close-over-big-then-small-data.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/close-over-big-then-small-data.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/close-over-big-then-small-data/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/close-over-big-then-small-data/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/cleanup-rvalue-scopes.rs:20:16
   |
LL |     AddFlags { bits: bits }
   |                ^^^^^^^^^^ help: replace it with: `bits`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args cleanup-rvalue-scopes.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cleanup-rvalue-scopes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-scopes/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-scopes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/cleanup-rvalue-scopes.rs:20:16
   |
LL |     AddFlags { bits: bits }
   |                ^^^^^^^^^^ help: replace it with: `bits`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---

+ warning: redundant field names in struct initialization
+   --> $DIR/private-field.rs:9:19
+    |
+ LL |             Dog { age: age, dog_age: age * 7 }
+    |                   ^^^^^^^^ help: replace it with: `age`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0599]: no method named `dog_age` found for struct `Dog` in the current scope
3    |


7 LL |     let dog_age = dog.dog_age();
8    |                       ^^^^^^^ private field, not a method
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
11 
12 For more information about this error, try `rustc --explain E0599`.
---
To only update this specific test, also pass `--test-args confuse-field-and-method/private-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/confuse-field-and-method/private-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/private-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/private-field/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/confuse-field-and-method/private-field.rs:9:19
   |
LL |             Dog { age: age, dog_age: age * 7 }
   |                   ^^^^^^^^ help: replace it with: `age`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0599]: no method named `dog_age` found for struct `Dog` in the current scope
   |
LL |     pub struct Dog {
LL |     pub struct Dog {
   |     -------------- method `dog_age` not found for this
...
LL |     let dog_age = dog.dog_age(); //~ ERROR no method
   |                       ^^^^^^^ private field, not a method
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0599`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-46553.rs:12:13
   |
LL |             function: function,
   |             ^^^^^^^^^^^^^^^^^^ help: replace it with: `function`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args consts/issue-46553.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-46553.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-46553/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-46553/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/consts/issue-46553.rs:12:13
   |
LL |             function: function,
   |             ^^^^^^^^^^^^^^^^^^ help: replace it with: `function`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
1 error[E0599]: no method named `clone` found for struct `Foo` in the current scope
2   --> $DIR/copy-a-resource.rs:18:16
3    |

11    = note: the following trait defines an item `clone`, perhaps you need to implement it:
12            candidate #1: `Clone`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
15 
16 For more information about this error, try `rustc --explain E0599`.
---
To only update this specific test, also pass `--test-args copy-a-resource.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/copy-a-resource.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/copy-a-resource" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/copy-a-resource/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0599]: no method named `clone` found for struct `Foo` in the current scope
  --> /checkout/src/test/ui/copy-a-resource.rs:18:16
   |
LL | struct Foo {
   | ---------- method `clone` not found for this
...
LL |     let _y = x.clone();
   |                ^^^^^ method not found in `Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0599`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/xcrate-trait-lifetime-param.rs:15:18
   |
LL |         Reader { b : b }
   |                  ^^^^^ help: replace it with: `b`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-crate/xcrate-trait-lifetime-param/xcrate-trait-lifetime-param.stderr
To only update this specific test, also pass `--test-args cross-crate/xcrate-trait-lifetime-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cross-crate/xcrate-trait-lifetime-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-crate/xcrate-trait-lifetime-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-crate/xcrate-trait-lifetime-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/cross-crate/xcrate-trait-lifetime-param.rs:15:18
   |
LL |         Reader { b : b }
   |                  ^^^^^ help: replace it with: `b`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---

+ warning: redundant field names in struct initialization
+   --> $DIR/dep-graph-struct-signature.rs:42:22
+    |
+ LL |         WillChange { x: x, y: y }
+    |                      ^^^^ help: replace it with: `x`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/dep-graph-struct-signature.rs:42:28
+   --> $DIR/dep-graph-struct-signature.rs:42:28
+    |
+ LL |         WillChange { x: x, y: y }
+    |                            ^^^^ help: replace it with: `y`
+ 
1 error: no path from `WillChange` to `type_of`
3    |


130 LL |     #[rustc_then_this_would_need(typeck)]
132 
- error: aborting due to 22 previous errors
+ error: aborting due to 22 previous errors; 2 warnings emitted
134 
---
To only update this specific test, also pass `--test-args dep-graph/dep-graph-struct-signature.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "incremental=tmp/dep-graph-struct-signature" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:42:22
   |
LL |         WillChange { x: x, y: y }
   |                      ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:42:28
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:42:28
   |
LL |         WillChange { x: x, y: y }
   |                            ^^^^ help: replace it with: `y`

error: no path from `WillChange` to `type_of`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:27:5
   |
LL |     #[rustc_then_this_would_need(type_of)] //~ ERROR no path


error: no path from `WillChange` to `associated_item`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:28:5
   |
LL |     #[rustc_then_this_would_need(associated_item)] //~ ERROR no path


error: no path from `WillChange` to `trait_def`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:29:5
   |
LL |     #[rustc_then_this_would_need(trait_def)] //~ ERROR no path

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:31:9
   |
   |
LL |         #[rustc_then_this_would_need(fn_sig)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:35:5
   |
   |
LL |     #[rustc_then_this_would_need(fn_sig)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:36:5
   |
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:39:5
   |
   |
LL |     #[rustc_then_this_would_need(fn_sig)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:40:5
   |
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:45:5
   |
   |
LL |     #[rustc_then_this_would_need(type_of)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:47:9
   |
   |
LL |         #[rustc_then_this_would_need(fn_sig)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:48:9
   |
   |
LL |         #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:52:5
   |
   |
LL |     #[rustc_then_this_would_need(type_of)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:54:9
   |
   |
LL |         #[rustc_then_this_would_need(fn_sig)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:55:9
   |
   |
LL |         #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:60:9
   |
   |
LL |         #[rustc_then_this_would_need(type_of)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:62:9
   |
   |
LL |         #[rustc_then_this_would_need(type_of)] //~ ERROR OK


error: no path from `WillChange` to `type_of`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:67:5
   |
LL |     #[rustc_then_this_would_need(type_of)] //~ ERROR no path


error: no path from `WillChange` to `type_of`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:74:5
   |
LL |     #[rustc_then_this_would_need(type_of)] //~ ERROR no path


error: no path from `WillChange` to `fn_sig`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:76:9
   |
LL |         #[rustc_then_this_would_need(fn_sig)] //~ ERROR no path


error: no path from `WillChange` to `fn_sig`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:80:5
   |
LL |     #[rustc_then_this_would_need(fn_sig)] //~ ERROR no path


error: no path from `WillChange` to `fn_sig`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:83:5
   |
LL |     #[rustc_then_this_would_need(fn_sig)] //~ ERROR no path from `WillChange`


error: no path from `WillChange` to `typeck`
  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:84:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR no path from `WillChange`

error: aborting due to 22 previous errors; 2 warnings emitted


---
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1/issue-31997-1.stderr
To only update this specific test, also pass `--test-args derived-errors/issue-31997-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derived-errors/issue-31997-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

+ warning: redundant field names in struct initialization
+   --> $DIR/deref-suggestion.rs:41:17
+    |
+ LL |     let s = S { u: u };
+    |                 ^^^^ help: replace it with: `u`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/deref-suggestion.rs:46:17
+   --> $DIR/deref-suggestion.rs:46:17
+    |
+ LL |     let r = R { i: i };
+    |                 ^^^^ help: replace it with: `i`
1 error[E0308]: mismatched types
2   --> $DIR/deref-suggestion.rs:8:9
3    |


126    | |______`if` and `else` have incompatible types
127    |        expected `i32`, found `&{integer}`
- error: aborting due to 13 previous errors
+ error: aborting due to 13 previous errors; 2 warnings emitted
130 
131 For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args deref-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deref-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-suggestion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/deref-suggestion.rs:41:17
   |
LL |     let s = S { u: u };
   |                 ^^^^ help: replace it with: `u`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/deref-suggestion.rs:46:17
  --> /checkout/src/test/ui/deref-suggestion.rs:46:17
   |
LL |     let r = R { i: i };
   |                 ^^^^ help: replace it with: `i`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:8:9
   |
LL |     foo(s);
LL |     foo(s);
   |         ^
   |         |
   |         expected struct `String`, found `&String`
   |         help: try using a conversion method: `s.to_string()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:14:10
   |
LL |     foo3(u);
LL |     foo3(u);
   |          ^
   |          |
   |          expected `u32`, found `&u32`
   |          help: consider dereferencing the borrow: `*u`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:30:9
   |
   |
LL |     foo(&"aaa".to_owned());
   |         |
   |         |
   |         expected struct `String`, found `&String`
   |         help: consider removing the borrow: `"aaa".to_owned()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:32:9
   |
   |
LL |     foo(&mut "aaa".to_owned());
   |         |
   |         expected struct `String`, found `&mut String`
   |         expected struct `String`, found `&mut String`
   |         help: consider removing the borrow: `"aaa".to_owned()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:2:20
   |
   |
LL |     ($x:expr) => { &$x } //~ ERROR mismatched types
   |                    ^^^ expected `u32`, found `&{integer}`
...
LL |     foo3(borrow!(0));
   |
   |
   = note: this error originates in the macro `borrow` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:36:5
   |
   |
LL |     assert_eq!(3i32, &3i32);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `&i32`
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:39:17
   |
   |
LL |     let s = S { u };
   |                 |
   |                 |
   |                 expected `&u32`, found integer
   |                 help: consider borrowing here: `u: &u`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:41:20
   |
   |
LL |     let s = S { u: u };
   |                    |
   |                    |
   |                    expected `&u32`, found integer
   |                    help: consider borrowing here: `&u`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:44:17
   |
   |
LL |     let r = R { i };
   |                 |
   |                 |
   |                 expected `u32`, found `&{integer}`
   |                 help: consider dereferencing the borrow: `i: *i`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:46:20
   |
   |
LL |     let r = R { i: i };
   |                    |
   |                    |
   |                    expected `u32`, found `&{integer}`
   |                    help: consider dereferencing the borrow: `*i`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:55:9
   |
LL |         b
LL |         b
   |         ^
   |         |
   |         expected `i32`, found `&{integer}`
   |         help: consider dereferencing the borrow: `*b`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:63:9
   |
LL |         b
LL |         b
   |         ^
   |         |
   |         expected `i32`, found `&{integer}`
   |         help: consider dereferencing the borrow: `*b`

error[E0308]: `if` and `else` have incompatible types
  --> /checkout/src/test/ui/deref-suggestion.rs:68:12
LL |        let val = if true {
   |   _______________-
LL |  |         *a
   |  |         -- expected because of this
   |  |         -- expected because of this
LL |  |     } else if true {
   |  |____________^
LL | ||     //~^ ERROR incompatible types
LL | ||         b
LL | ||     } else {
LL | ||         &0
LL | ||     };
   | ||_____|
   | ||_____|
   | |______`if` and `else` have incompatible types
   |        expected `i32`, found `&{integer}`
error: aborting due to 13 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0308`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/drop-trait-enum.rs:68:61
   |
LL |         let v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };
   |                                                             ^^^^^^^^^^^^^^ help: replace it with: `sender`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/drop-trait-enum.rs:84:61
  --> $DIR/drop-trait-enum.rs:84:61
   |
LL |             v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };
   |                                                             ^^^^^^^^^^^^^^ help: replace it with: `sender`
warning: 2 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop-trait-enum/drop-trait-enum.stderr
To only update this specific test, also pass `--test-args drop/drop-trait-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/drop-trait-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop-trait-enum/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop-trait-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/drop-trait-enum.rs:68:61
   |
LL |         let v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };
   |                                                             ^^^^^^^^^^^^^^ help: replace it with: `sender`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/drop-trait-enum.rs:84:61
  --> /checkout/src/test/ui/drop/drop-trait-enum.rs:84:61
   |
LL |             v = Foo::FailingVariant { on_drop: SendOnDrop { sender: sender } };
   |                                                             ^^^^^^^^^^^^^^ help: replace it with: `sender`
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/drop/dynamic-drop.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/dynamic-drop.rs:38:13
   |
LL |             failing_op: failing_op,
   |             ^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `failing_op`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args drop/dynamic-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dynamic-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dynamic-drop.rs:38:13
   |
LL |             failing_op: failing_op,
   |             ^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `failing_op`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:453:13
   |
LL |         S { name: name, mark: Cell::new(0), next: Cell::new(None) }
   |             ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:471:14
  --> $DIR/dropck_legal_cycles.rs:471:14
   |
LL |         S2 { name: name, mark: Cell::new(0), next: Cell::new((None, None)) }
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:491:13
   |
   |
LL |         V { name: name,
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:513:13
   |
   |
LL |         H { name: name, mark: Cell::new(0), next: Cell::new(None) }
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:544:14
   |
   |
LL |         HM { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:578:14
   |
   |
LL |         VD { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:599:14
   |
   |
LL |         VM { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:620:14
   |
   |
LL |         LL { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:641:14
   |
   |
LL |         BH { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:682:15
   |
   |
LL |         BTM { name: name,
   |               ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:723:15
   |
   |
LL |         BTS { name: name,
   |               ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:768:13
   |
   |
LL |             name: name, mark: Cell::new(0), children: (None, None), })))
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:805:13
   |
   |
LL |             name: name, mark: Cell::new(0), children: (None, None), })))
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> $DIR/dropck_legal_cycles.rs:888:13
   |
   |
LL |             name: name, mark: Cell::new(0), children: (None, None), })))
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: 14 warnings emitted



---
To only update this specific test, also pass `--test-args drop/dropck_legal_cycles.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dropck_legal_cycles.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:453:13
   |
LL |         S { name: name, mark: Cell::new(0), next: Cell::new(None) }
   |             ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:471:14
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:471:14
   |
LL |         S2 { name: name, mark: Cell::new(0), next: Cell::new((None, None)) }
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:491:13
   |
   |
LL |         V { name: name,
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:513:13
   |
   |
LL |         H { name: name, mark: Cell::new(0), next: Cell::new(None) }
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:544:14
   |
   |
LL |         HM { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:578:14
   |
   |
LL |         VD { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:599:14
   |
   |
LL |         VM { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:620:14
   |
   |
LL |         LL { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:641:14
   |
   |
LL |         BH { name: name,
   |              ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:682:15
   |
   |
LL |         BTM { name: name,
   |               ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:723:15
   |
   |
LL |         BTS { name: name,
   |               ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:768:13
   |
   |
LL |             name: name, mark: Cell::new(0), children: (None, None), })))
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:805:13
   |
   |
LL |             name: name, mark: Cell::new(0), children: (None, None), })))
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/drop/dropck_legal_cycles.rs:888:13
   |
   |
LL |             name: name, mark: Cell::new(0), children: (None, None), })))
   |             ^^^^^^^^^^ help: replace it with: `name`
warning: 14 warnings emitted


------------------------------------------
---

+ warning: redundant field names in struct initialization
+   --> $DIR/functional-struct-update-respects-privacy.rs:16:13
+    |
+ LL |         S { a: a, b: b, secret_uid: val }
+    |             ^^^^ help: replace it with: `a`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/functional-struct-update-respects-privacy.rs:16:19
+   --> $DIR/functional-struct-update-respects-privacy.rs:16:19
+    |
+ LL |         S { a: a, b: b, secret_uid: val }
+    |                   ^^^^ help: replace it with: `b`
+ 
1 error[E0451]: field `secret_uid` of struct `S` is private
3    |


4 LL |     let s_2 = foo::S { b: format!("ess two"), ..s_1 }; // FRU ...
5    |                                                 ^^^ field `secret_uid` is private
- error: aborting due to previous error
+ error: aborting due to previous error; 2 warnings emitted
8 
9 For more information about this error, try `rustc --explain E0451`.
---
To only update this specific test, also pass `--test-args functional-struct-update/functional-struct-update-respects-privacy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/functional-struct-update/functional-struct-update-respects-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functional-struct-update/functional-struct-update-respects-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functional-struct-update/functional-struct-update-respects-privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/functional-struct-update/functional-struct-update-respects-privacy.rs:16:13
   |
LL |         S { a: a, b: b, secret_uid: val }
   |             ^^^^ help: replace it with: `a`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/functional-struct-update/functional-struct-update-respects-privacy.rs:16:19
  --> /checkout/src/test/ui/functional-struct-update/functional-struct-update-respects-privacy.rs:16:19
   |
LL |         S { a: a, b: b, secret_uid: val }
   |                   ^^^^ help: replace it with: `b`

error[E0451]: field `secret_uid` of struct `S` is private
   |
   |
LL |     let s_2 = foo::S { b: format!("ess two"), ..s_1 }; // FRU ...
   |                                                 ^^^ field `secret_uid` is private
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0451`.

---
To only update this specific test, also pass `--test-args generics/issue-2936.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/issue-2936.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-2936/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-2936/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/hrtb-trait-object-passed-to-closure.rs:21:30
   |
LL |     let annotation = NoAnn { f: f };
   |                              ^^^^ help: replace it with: `f`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/hrtb-trait-object-passed-to-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/hrtb-trait-object-passed-to-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hrtb-trait-object-passed-to-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hrtb-trait-object-passed-to-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/higher-rank-trait-bounds/hrtb-trait-object-passed-to-closure.rs:21:30
   |
LL |     let annotation = NoAnn { f: f };
   |                              ^^^^ help: replace it with: `f`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-30786.rs:83:15
+    |
+ LL |         Map { func: func, stream: self }
+    |               ^^^^^^^^^^ help: replace it with: `func`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/issue-30786.rs:91:18
+   --> $DIR/issue-30786.rs:91:18
+    |
+ LL |         Filter { func: func, stream: self }
+    |                  ^^^^^^^^^^ help: replace it with: `func`
+ 
1 error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@$DIR/issue-30786.rs:127:27: 127:36]>`, but its trait bounds were not satisfied
3    |


38            `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:140:30: 140:42]>: Stream`
39            which is required by `&mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:140:30: 140:42]>: StreamExt`
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 2 warnings emitted
42 
43 For more information about this error, try `rustc --explain E0599`.
43 For more information about this error, try `rustc --explain E0599`.
44 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.migrate/issue-30786.migrate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hrtb/issue-30786.rs`

error in revision `migrate`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-30786.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.migrate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.migrate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/hrtb/issue-30786.rs:83:15
   |
LL |         Map { func: func, stream: self }
   |               ^^^^^^^^^^ help: replace it with: `func`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/hrtb/issue-30786.rs:91:18
  --> /checkout/src/test/ui/hrtb/issue-30786.rs:91:18
   |
LL |         Filter { func: func, stream: self }
   |                  ^^^^^^^^^^ help: replace it with: `func`

error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Map<S, F> {
   | |
   | |
   | method `filterx` not found for this
   | doesn't satisfy `_: StreamExt`
...
LL |     let filter = map.filterx(|x: &_| true);
   |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `&'a mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: Stream`
           which is required by `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: StreamExt`
           `&'a mut &Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: Stream`
           which is required by `&Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: StreamExt`
           `&'a mut &mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: Stream`
           which is required by `&mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: StreamExt`

error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Filter<S, F> {
   | |
   | |
   | method `countx` not found for this
   | doesn't satisfy `_: StreamExt`
...
LL |     let count = filter.countx();
   |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `&'a mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: Stream`
           which is required by `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: StreamExt`
           `&'a mut &Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: Stream`
           which is required by `&Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: StreamExt`
           `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: Stream`
           which is required by `&mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: StreamExt`
error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0599`.

---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-30786.rs:83:15
+    |
+ LL |         Map { func: func, stream: self }
+    |               ^^^^^^^^^^ help: replace it with: `func`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/issue-30786.rs:91:18
+   --> $DIR/issue-30786.rs:91:18
+    |
+ LL |         Filter { func: func, stream: self }
+    |                  ^^^^^^^^^^ help: replace it with: `func`
+ 
1 error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@$DIR/issue-30786.rs:127:27: 127:36]>`, but its trait bounds were not satisfied
3    |


38            `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:140:30: 140:42]>: Stream`
39            which is required by `&mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:140:30: 140:42]>: StreamExt`
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 2 warnings emitted
42 
43 For more information about this error, try `rustc --explain E0599`.
43 For more information about this error, try `rustc --explain E0599`.
44 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.nll/issue-30786.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hrtb/issue-30786.rs`

error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-30786.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/hrtb/issue-30786.rs:83:15
   |
LL |         Map { func: func, stream: self }
   |               ^^^^^^^^^^ help: replace it with: `func`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/hrtb/issue-30786.rs:91:18
  --> /checkout/src/test/ui/hrtb/issue-30786.rs:91:18
   |
LL |         Filter { func: func, stream: self }
   |                  ^^^^^^^^^^ help: replace it with: `func`

error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Map<S, F> {
   | |
   | |
   | method `filterx` not found for this
   | doesn't satisfy `_: StreamExt`
...
LL |     let filter = map.filterx(|x: &_| true);
   |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `&'a mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: Stream`
           which is required by `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: StreamExt`
           `&'a mut &Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: Stream`
           which is required by `&Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: StreamExt`
           `&'a mut &mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: Stream`
           which is required by `&mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:127:27: 127:36]>: StreamExt`

error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Filter<S, F> {
   | |
   | |
   | method `countx` not found for this
   | doesn't satisfy `_: StreamExt`
...
LL |     let count = filter.countx();
   |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `&'a mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: Stream`
           which is required by `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: StreamExt`
           `&'a mut &Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: Stream`
           which is required by `&Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: StreamExt`
           `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: Stream`
           which is required by `&mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:140:30: 140:42]>: StreamExt`
error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0599`.

---
To only update this specific test, also pass `--test-args init-res-into-things.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/init-res-into-things.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-res-into-things/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-res-into-things/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-10902.rs:17:12
   |
LL |         P{ car: car, cdr: cdr }
   |            ^^^^^^^^ help: replace it with: `car`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-10902.rs:17:22
  --> $DIR/issue-10902.rs:17:22
   |
LL |         P{ car: car, cdr: cdr }
   |                      ^^^^^^^^ help: replace it with: `cdr`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-10902.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10902.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10902" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10902/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-10902.rs:17:12
   |
LL |         P{ car: car, cdr: cdr }
   |            ^^^^^^^^ help: replace it with: `car`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-10902.rs:17:22
  --> /checkout/src/test/ui/issues/issue-10902.rs:17:22
   |
LL |         P{ car: car, cdr: cdr }
   |                      ^^^^^^^^ help: replace it with: `cdr`
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-10802.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-10802.rs:30:20
   |
LL |         Whatever { w: w }
   |                    ^^^^ help: replace it with: `w`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-10802.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10802.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10802/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10802/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-10802.rs:30:20
   |
LL |         Whatever { w: w }
   |                    ^^^^ help: replace it with: `w`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/example-calendar.rs:605:13
   |
LL |             sep_width: sep_width,
   |             ^^^^^^^^^^^^^^^^^^^^ help: replace it with: `sep_width`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/example-calendar.rs:684:13
---
To only update this specific test, also pass `--test-args impl-trait/example-calendar.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/example-calendar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/impl-trait/example-calendar.rs:605:13
   |
LL |             sep_width: sep_width,
   |             ^^^^^^^^^^^^^^^^^^^^ help: replace it with: `sep_width`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/impl-trait/example-calendar.rs:684:13
---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-11374.rs:10:21
+    |
+ LL |         Container { reader: reader }
+    |                     ^^^^^^^^^^^^^^ help: replace it with: `reader`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
1 error[E0308]: mismatched types
2   --> $DIR/issue-11374.rs:26:15
---
To only update this specific test, also pass `--test-args issues/issue-11374.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-11374.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11374" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11374/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-11374.rs:26:15
   |
LL |     c.read_to(v); //~ ERROR E0308
   |               |
   |               |
   |               expected `&mut [u8]`, found struct `Vec`
   |               help: consider mutably borrowing here: `&mut v`
   = note: expected mutable reference `&mut [u8]`
                         found struct `Vec<_>`

error: aborting due to previous error; 1 warning emitted
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-13264.rs:67:23
   |
LL |     let root = Root { jsref: jsref };
   |                       ^^^^^^^^^^^^ help: replace it with: `jsref`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-13264.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-13264.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13264/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13264/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-13264.rs:67:23
   |
LL |     let root = Root { jsref: jsref };
   |                       ^^^^^^^^^^^^ help: replace it with: `jsref`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-13323.rs:51:19
   |
LL |     box EqualTo { expected: expected }
   |                   ^^^^^^^^^^^^^^^^^^ help: replace it with: `expected`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-13323.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-13323.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13323/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13323/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-13323.rs:51:19
   |
LL |     box EqualTo { expected: expected }
   |                   ^^^^^^^^^^^^^^^^^^ help: replace it with: `expected`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-14821.rs:14:83
   |
LL |     pub fn new<'b>(x: &'b dyn SomeTrait, y: &'b dyn SomeTrait) -> Foo<'b> { Foo { x: x, y: y } }
   |                                                                                   ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-14821.rs:14:89
  --> $DIR/issue-14821.rs:14:89
   |
LL |     pub fn new<'b>(x: &'b dyn SomeTrait, y: &'b dyn SomeTrait) -> Foo<'b> { Foo { x: x, y: y } }
   |                                                                                         ^^^^ help: replace it with: `y`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-14821.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14821.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14821/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14821/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-14821.rs:14:83
   |
LL |     pub fn new<'b>(x: &'b dyn SomeTrait, y: &'b dyn SomeTrait) -> Foo<'b> { Foo { x: x, y: y } }
   |                                                                                   ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-14821.rs:14:89
  --> /checkout/src/test/ui/issues/issue-14821.rs:14:89
   |
LL |     pub fn new<'b>(x: &'b dyn SomeTrait, y: &'b dyn SomeTrait) -> Foo<'b> { Foo { x: x, y: y } }
   |                                                                                         ^^^^ help: replace it with: `y`
warning: 2 warnings emitted


------------------------------------------
---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-15034.rs:7:17
+    |
+ LL |         Lexer { input: input }
+    |                 ^^^^^^^^^^^^ help: replace it with: `input`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/issue-15034.rs:17:18
+   --> $DIR/issue-15034.rs:17:18
+    |
+ LL |         Parser { lexer: lexer }
+    |                  ^^^^^^^^^^^^ help: replace it with: `lexer`
+ 
1 error[E0621]: explicit lifetime required in the type of `lexer`
3    |


6 LL |         Parser { lexer: lexer }
7    |                         ^^^^^ lifetime `'a` required
- error: aborting due to previous error
+ error: aborting due to previous error; 2 warnings emitted
10 
11 For more information about this error, try `rustc --explain E0621`.
---
To only update this specific test, also pass `--test-args issues/issue-15034.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-15034.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15034" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15034/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-15034.rs:7:17
   |
LL |         Lexer { input: input }
   |                 ^^^^^^^^^^^^ help: replace it with: `input`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-15034.rs:17:18
  --> /checkout/src/test/ui/issues/issue-15034.rs:17:18
   |
LL |         Parser { lexer: lexer }
   |                  ^^^^^^^^^^^^ help: replace it with: `lexer`

error[E0621]: explicit lifetime required in the type of `lexer`
   |
   |
LL |     pub fn new(lexer: &'a mut Lexer) -> Parser<'a> {
   |                       ------------- help: add explicit lifetime `'a` to the type of `lexer`: `&'a mut Lexer<'a>`
LL |         Parser { lexer: lexer }
   |                         ^^^^^ lifetime `'a` required
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0621`.

---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-15094.rs:20:15
+    |
+ LL |     Debuger { x: x }
+    |               ^^^^ help: replace it with: `x`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
1 error[E0053]: method `call_once` has an incompatible type for trait
2   --> $DIR/issue-15094.rs:11:5
2   --> $DIR/issue-15094.rs:11:5
3    |

7    = note: expected fn pointer `extern "rust-call" fn(Debuger<_>, ())`
8               found fn pointer `fn(Debuger<_>, ())`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
11 
12 For more information about this error, try `rustc --explain E0053`.
---
To only update this specific test, also pass `--test-args issues/issue-15094.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-15094.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15094" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15094/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-15094.rs:20:15
   |
LL |     Debuger { x: x }
   |               ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

error[E0053]: method `call_once` has an incompatible type for trait
  --> /checkout/src/test/ui/issues/issue-15094.rs:11:5
  --> /checkout/src/test/ui/issues/issue-15094.rs:11:5
   |
LL |     fn call_once(self, _args: ()) {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected "rust-call" fn, found "Rust" fn
   |
   = note: expected fn pointer `extern "rust-call" fn(Debuger<_>, ())`
              found fn pointer `fn(Debuger<_>, ())`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0053`.

---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-16048.rs:16:15
+    |
+ LL |         Foo { buf: buf }
+    |               ^^^^^^^^ help: replace it with: `buf`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0195]: lifetime parameters or bounds on method `get` do not match the trait declaration
3    |

15    |
15    |
16    = note: an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
19 
20 Some errors have detailed explanations: E0195, E0605.
---
To only update this specific test, also pass `--test-args issues/issue-16048.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16048.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16048" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16048/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-16048.rs:16:15
   |
LL |         Foo { buf: buf }
   |               ^^^^^^^^ help: replace it with: `buf`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0195]: lifetime parameters or bounds on method `get` do not match the trait declaration
   |
   |
LL |     fn get<'p, T : Test<'p>>(&self) -> T;
   |           ------------------ lifetimes in impl do not match this method in trait
...
LL |     fn get<'p, T: Test<'a> + From<Foo<'a>>>(&self) -> T {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetimes do not match method in trait

error[E0605]: non-primitive cast: `Foo<'a>` as `T`
   |
LL |         return *self as T;
LL |         return *self as T;
   |                ^^^^^^^^^^ help: consider using the `From` trait instead: `T::from(*self)`
   |
   = note: an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0195, E0605.
For more information about an error, try `rustc --explain E0195`.
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-15734.rs:11:15
   |
LL |         Mat { data: data, cols: cols }
   |               ^^^^^^^^^^ help: replace it with: `data`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-15734.rs:11:27
  --> $DIR/issue-15734.rs:11:27
   |
LL |         Mat { data: data, cols: cols }
   |                           ^^^^^^^^^^ help: replace it with: `cols`
warning: redundant field names in struct initialization
  --> $DIR/issue-15734.rs:14:26
   |
   |
LL |         Row { mat: self, row: row, }
   |                          ^^^^^^^^ help: replace it with: `row`
warning: 3 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-15734.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-15734.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15734/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15734/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-15734.rs:11:15
   |
LL |         Mat { data: data, cols: cols }
   |               ^^^^^^^^^^ help: replace it with: `data`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-15734.rs:11:27
  --> /checkout/src/test/ui/issues/issue-15734.rs:11:27
   |
LL |         Mat { data: data, cols: cols }
   |                           ^^^^^^^^^^ help: replace it with: `cols`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-15734.rs:14:26
   |
   |
LL |         Row { mat: self, row: row, }
   |                          ^^^^^^^^ help: replace it with: `row`
warning: 3 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-16492.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-16492.rs:15:13
   |
LL |             number: number,
   |             ^^^^^^^^^^^^^^ help: replace it with: `number`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-16492.rs:16:13
---
To only update this specific test, also pass `--test-args issues/issue-16492.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16492.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16492/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16492/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-16492.rs:15:13
   |
LL |             number: number,
   |             ^^^^^^^^^^^^^^ help: replace it with: `number`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-16492.rs:16:13
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-20797.rs:49:23
   |
LL |         Ok(Subpaths { stack: stack, strategy: strategy })
   |                       ^^^^^^^^^^^^ help: replace it with: `stack`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-20797.rs:49:37
  --> $DIR/issue-20797.rs:49:37
   |
LL |         Ok(Subpaths { stack: stack, strategy: strategy })
   |                                     ^^^^^^^^^^^^^^^^^^ help: replace it with: `strategy`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-20797.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20797.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20797" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20797/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-20797.rs:49:23
   |
LL |         Ok(Subpaths { stack: stack, strategy: strategy })
   |                       ^^^^^^^^^^^^ help: replace it with: `stack`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-20797.rs:49:37
  --> /checkout/src/test/ui/issues/issue-20797.rs:49:37
   |
LL |         Ok(Subpaths { stack: stack, strategy: strategy })
   |                                     ^^^^^^^^^^^^^^^^^^ help: replace it with: `strategy`
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-23338-ensure-param-drop-order.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-23338-ensure-param-drop-order.rs:143:21
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                     ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-23338-ensure-param-drop-order.rs:143:33
  --> $DIR/issue-23338-ensure-param-drop-order.rs:143:33
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                 ^^^^ help: replace it with: `i`
warning: redundant field names in struct initialization
  --> $DIR/issue-23338-ensure-param-drop-order.rs:143:39
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                       ^^^^^^^^ help: replace it with: `log`
warning: redundant field names in struct initialization
  --> $DIR/issue-23338-ensure-param-drop-order.rs:143:59
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                                           ^^^^^^^^^^^^ help: replace it with: `trail`
warning: 4 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-23338-ensure-param-drop-order.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23338-ensure-param-drop-order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23338-ensure-param-drop-order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23338-ensure-param-drop-order/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23338-ensure-param-drop-order.rs:143:21
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                     ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23338-ensure-param-drop-order.rs:143:33
  --> /checkout/src/test/ui/issues/issue-23338-ensure-param-drop-order.rs:143:33
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                 ^^^^ help: replace it with: `i`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23338-ensure-param-drop-order.rs:143:39
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                       ^^^^^^^^ help: replace it with: `log`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23338-ensure-param-drop-order.rs:143:59
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                                           ^^^^^^^^^^^^ help: replace it with: `trail`
warning: 4 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-23611-enum-swap-in-drop.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-23611-enum-swap-in-drop.rs:243:21
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                     ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-23611-enum-swap-in-drop.rs:243:33
  --> $DIR/issue-23611-enum-swap-in-drop.rs:243:33
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                 ^^^^ help: replace it with: `i`
warning: redundant field names in struct initialization
  --> $DIR/issue-23611-enum-swap-in-drop.rs:243:39
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                       ^^^^^^^^ help: replace it with: `log`
warning: redundant field names in struct initialization
  --> $DIR/issue-23611-enum-swap-in-drop.rs:243:59
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                                           ^^^^^^^^^^^^ help: replace it with: `trail`
warning: 4 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-23611-enum-swap-in-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23611-enum-swap-in-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23611-enum-swap-in-drop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23611-enum-swap-in-drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23611-enum-swap-in-drop.rs:243:21
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                     ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23611-enum-swap-in-drop.rs:243:33
  --> /checkout/src/test/ui/issues/issue-23611-enum-swap-in-drop.rs:243:33
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                 ^^^^ help: replace it with: `i`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23611-enum-swap-in-drop.rs:243:39
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                       ^^^^^^^^ help: replace it with: `log`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-23611-enum-swap-in-drop.rs:243:59
   |
   |
LL |                     name: name, i: i, log: log, uid: ctr, trail: trail
   |                                                           ^^^^^^^^^^^^ help: replace it with: `trail`
warning: 4 warnings emitted


------------------------------------------
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2445-b/issue-2445-b.stderr
To only update this specific test, also pass `--test-args issues/issue-2445-b.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2445-b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2445-b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2445-b/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args issues/issue-2445.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2445.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2445/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2445/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-2502.rs:20:9
   |
LL |         fontbuf: fontbuf
   |         ^^^^^^^^^^^^^^^^ help: replace it with: `fontbuf`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-2502.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2502.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2502" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2502/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-2502.rs:20:9
   |
LL |         fontbuf: fontbuf
   |         ^^^^^^^^^^^^^^^^ help: replace it with: `fontbuf`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-2550.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2550.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2550/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2550/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args issues/issue-2735-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2735-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2735-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2735-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2748-a/issue-2748-a.stderr
To only update this specific test, also pass `--test-args issues/issue-2748-a.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2748-a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2748-a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2748-a/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args issues/issue-2735-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2735-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2735-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2735-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:12:9
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |         ^^^^^^^^ help: replace it with: `m11`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:12:19
  --> $DIR/issue-3149.rs:12:19
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |                   ^^^^^^^^ help: replace it with: `m12`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:12:29
   |
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |                             ^^^^^^^^ help: replace it with: `m13`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:12:39
   |
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |                                       ^^^^^^^^ help: replace it with: `m14`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:13:9
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |         ^^^^^^^^ help: replace it with: `m21`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:13:19
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |                   ^^^^^^^^ help: replace it with: `m22`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:13:29
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |                             ^^^^^^^^ help: replace it with: `m23`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:13:39
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |                                       ^^^^^^^^ help: replace it with: `m24`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:14:9
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |         ^^^^^^^^ help: replace it with: `m31`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:14:19
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |                   ^^^^^^^^ help: replace it with: `m32`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:14:29
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |                             ^^^^^^^^ help: replace it with: `m33`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:14:39
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |                                       ^^^^^^^^ help: replace it with: `m34`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:15:9
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |         ^^^^^^^^ help: replace it with: `m41`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:15:19
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |                   ^^^^^^^^ help: replace it with: `m42`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:15:29
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |                             ^^^^^^^^ help: replace it with: `m43`
warning: redundant field names in struct initialization
  --> $DIR/issue-3149.rs:15:39
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |                                       ^^^^^^^^ help: replace it with: `m44`
warning: 16 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-3149.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3149.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3149" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3149/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:12:9
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |         ^^^^^^^^ help: replace it with: `m11`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:12:19
  --> /checkout/src/test/ui/issues/issue-3149.rs:12:19
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |                   ^^^^^^^^ help: replace it with: `m12`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:12:29
   |
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |                             ^^^^^^^^ help: replace it with: `m13`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:12:39
   |
   |
LL |         m11: m11, m12: m12, m13: m13, m14: m14,
   |                                       ^^^^^^^^ help: replace it with: `m14`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:13:9
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |         ^^^^^^^^ help: replace it with: `m21`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:13:19
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |                   ^^^^^^^^ help: replace it with: `m22`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:13:29
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |                             ^^^^^^^^ help: replace it with: `m23`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:13:39
   |
   |
LL |         m21: m21, m22: m22, m23: m23, m24: m24,
   |                                       ^^^^^^^^ help: replace it with: `m24`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:14:9
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |         ^^^^^^^^ help: replace it with: `m31`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:14:19
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |                   ^^^^^^^^ help: replace it with: `m32`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:14:29
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |                             ^^^^^^^^ help: replace it with: `m33`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:14:39
   |
   |
LL |         m31: m31, m32: m32, m33: m33, m34: m34,
   |                                       ^^^^^^^^ help: replace it with: `m34`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:15:9
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |         ^^^^^^^^ help: replace it with: `m41`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:15:19
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |                   ^^^^^^^^ help: replace it with: `m42`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:15:29
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |                             ^^^^^^^^ help: replace it with: `m43`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3149.rs:15:39
   |
   |
LL |         m41: m41, m42: m42, m43: m43, m44: m44
   |                                       ^^^^^^^^ help: replace it with: `m44`
warning: 16 warnings emitted


------------------------------------------
---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-3154.rs:6:13
+    |
+ LL |     Thing { x: x }
+    |             ^^^^ help: replace it with: `x`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0621]: explicit lifetime required in the type of `x`
3    |


6 LL |     Thing { x: x }
7    |     ^^^^^^^^^^^^^^ lifetime `'a` required
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
10 
11 For more information about this error, try `rustc --explain E0621`.
---
To only update this specific test, also pass `--test-args issues/issue-3154.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3154.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3154" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3154/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3154.rs:6:13
   |
LL |     Thing { x: x } //~ ERROR explicit lifetime required in the type of `x` [E0621]
   |             ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0621]: explicit lifetime required in the type of `x`
   |
   |
LL | fn thing<'a,Q>(x: &Q) -> Thing<'a,Q> {
   |                   -- help: add explicit lifetime `'a` to the type of `x`: `&'a Q`
LL |     Thing { x: x } //~ ERROR explicit lifetime required in the type of `x` [E0621]
   |     ^^^^^^^^^^^^^^ lifetime `'a` required
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0621`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-3447.rs:19:13
   |
LL |             element: element,
   |             ^^^^^^^^^^^^^^^^ help: replace it with: `element`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-3447.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3447.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3447/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3447/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3447.rs:19:13
   |
LL |             element: element,
   |             ^^^^^^^^^^^^^^^^ help: replace it with: `element`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-3563-3.rs:69:15
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |               ^^^^^^^^^^^^ help: replace it with: `width`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-3563-3.rs:69:29
  --> $DIR/issue-3563-3.rs:69:29
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |                             ^^^^^^^^^^^^^^ help: replace it with: `height`
warning: redundant field names in struct initialization
  --> $DIR/issue-3563-3.rs:69:45
   |
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |                                             ^^^^^^^^^^ help: replace it with: `fill`
warning: redundant field names in struct initialization
  --> $DIR/issue-3563-3.rs:69:57
   |
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |                                                         ^^^^^^^^^^^^ help: replace it with: `lines`
warning: 4 warnings emitted



---
To only update this specific test, also pass `--test-args issues/issue-3563-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3563-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3563-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3563-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3563-3.rs:69:15
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |               ^^^^^^^^^^^^ help: replace it with: `width`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3563-3.rs:69:29
  --> /checkout/src/test/ui/issues/issue-3563-3.rs:69:29
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |                             ^^^^^^^^^^^^^^ help: replace it with: `height`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3563-3.rs:69:45
   |
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |                                             ^^^^^^^^^^ help: replace it with: `fill`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3563-3.rs:69:57
   |
   |
LL |     AsciiArt {width: width, height: height, fill: fill, lines: lines}
   |                                                         ^^^^^^^^^^^^ help: replace it with: `lines`
warning: 4 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-3973.rs stdout ----
diff of stderr:

7 LL | |     }
8    | |_____^ not a member of trait `ToString_`
+ warning: redundant field names in struct initialization
+   --> $DIR/issue-3973.rs:13:17
+    |
+    |
+ LL |         Point { x: x, y: y }
+    |                 ^^^^ help: replace it with: `x`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/issue-3973.rs:13:23
+   --> $DIR/issue-3973.rs:13:23
+    |
+ LL |         Point { x: x, y: y }
+    |                       ^^^^ help: replace it with: `y`
10 error[E0599]: no function or associated item named `new` found for struct `Point` in the current scope
11   --> $DIR/issue-3973.rs:22:20
12    |


16 LL |     let p = Point::new(0.0, 0.0);
17    |                    ^^^ function or associated item not found in `Point`
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 2 warnings emitted
20 
21 Some errors have detailed explanations: E0407, E0599.
---
To only update this specific test, also pass `--test-args issues/issue-3973.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3973.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3973" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3973/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0407]: method `new` is not a member of trait `ToString_`
   |
   |
LL | /     fn new(x: f64, y: f64) -> Point {
LL | |     //~^ ERROR method `new` is not a member of trait `ToString_`
LL | |         Point { x: x, y: y }
LL | |     }
   | |_____^ not a member of trait `ToString_`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3973.rs:13:17
   |
   |
LL |         Point { x: x, y: y }
   |                 ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-3973.rs:13:23
  --> /checkout/src/test/ui/issues/issue-3973.rs:13:23
   |
LL |         Point { x: x, y: y }
   |                       ^^^^ help: replace it with: `y`
error[E0599]: no function or associated item named `new` found for struct `Point` in the current scope
  --> /checkout/src/test/ui/issues/issue-3973.rs:22:20
   |
LL | struct Point {
LL | struct Point {
   | ------------ function or associated item `new` not found for this
...
LL |     let p = Point::new(0.0, 0.0);
   |                    ^^^ function or associated item not found in `Point`
error: aborting due to 2 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0407, E0599.
For more information about an error, try `rustc --explain E0407`.
---
+    |         ^^^^ help: replace it with: `f`
+    |
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
1 error[E0599]: no method named `iter` found for struct `Iterate` in the current scope
3    |


7 LL |     println!("{:?}", a.iter().take(10).collect::<Vec<usize>>());
8    |                        ^^^^ method not found in `Iterate<{integer}, [closure@$DIR/issue-41880.rs:26:24: 26:31]>`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
11 
12 For more information about this error, try `rustc --explain E0599`.
---
To only update this specific test, also pass `--test-args issues/issue-41880.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41880.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41880" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41880/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |         ^^^^ help: replace it with: `f`
   |
   = note: `#[warn(redundant_field_names)]` on by default

error[E0599]: no method named `iter` found for struct `Iterate` in the current scope
   |
   |
LL | pub struct Iterate<T, F> {
   | ------------------------ method `iter` not found for this
...
LL |     println!("{:?}", a.iter().take(10).collect::<Vec<usize>>());
   |                        ^^^^ method not found in `Iterate<{integer}, [closure@/checkout/src/test/ui/issues/issue-41880.rs:26:24: 26:31]>`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0599`.

---
To only update this specific test, also pass `--test-args issues/issue-48179.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-48179.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48179" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48179/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args issues/issue-48132.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-48132.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48132/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48132/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-5192.rs:34:13
   |
LL |             event_loop: event_loop,
   |             ^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `event_loop`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-5192.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5192.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5192/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5192/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-5192.rs:34:13
   |
LL |             event_loop: event_loop,
   |             ^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `event_loop`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-5708.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5708.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5708/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5708/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-5883.rs:10:14
+    |
+ LL |     Struct { r: r }
+    |              ^^^^ help: replace it with: `r`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0277]: the size for values of type `(dyn A + 'static)` cannot be known at compilation time
3    |

27    |        ^^^^^^
28    = note: the return type of a function must have a statically known size
---
To only update this specific test, also pass `--test-args issues/issue-5883.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5883.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5883" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5883/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-5883.rs:10:14
   |
LL |     Struct { r: r }
   |              ^^^^ help: replace it with: `r`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0277]: the size for values of type `(dyn A + 'static)` cannot be known at compilation time
   |
   |
LL |     r: dyn A + 'static //~ ERROR the size for values of type
   |
   |
   = help: the trait `Sized` is not implemented for `(dyn A + 'static)`
   = help: unsized fn params are gated as an unstable feature
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL |     r: &dyn A + 'static //~ ERROR the size for values of type


error[E0277]: the size for values of type `(dyn A + 'static)` cannot be known at compilation time
   |
   |
LL | ) -> Struct {          //~ ERROR the size for values of type
   |      ^^^^^^ doesn't have a size known at compile-time
LL |     Struct { r: r }
   |     --------------- this returned value is of type `Struct`
   |
   = help: within `Struct`, the trait `Sized` is not implemented for `(dyn A + 'static)`
note: required because it appears within the type `Struct`
   |
LL | struct Struct {
   |        ^^^^^^
   = note: the return type of a function must have a statically known size
---

---- [ui] ui/issues/issue-60057.rs stdout ----
diff of stderr:

10 LL |             banana: banana
11    |                     ^^^^^^ help: you might have meant to use the available field: `self.banana`
- error: aborting due to 2 previous errors
+ warning: redundant field names in struct initialization
+   --> $DIR/issue-60057.rs:8:13
+    |
+    |
+ LL |             banana: banana
+    |             ^^^^^^^^^^^^^^ help: replace it with: `banana`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/issue-60057.rs:14:13
+   --> $DIR/issue-60057.rs:14:13
+    |
+ LL |             banana: banana
+    |             ^^^^^^^^^^^^^^ help: replace it with: `banana`
+ error: aborting due to 2 previous errors; 2 warnings emitted
14 
15 For more information about this error, try `rustc --explain E0425`.
16 
---
To only update this specific test, also pass `--test-args issues/issue-60057.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60057.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `banana` in this scope
   |
   |
LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
   |                     ^^^^^^ a field by this name exists in `Self`

error[E0425]: cannot find value `banana` in this scope
   |
   |
LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
   |                     ^^^^^^ help: you might have meant to use the available field: `self.banana`
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-60057.rs:8:13
   |
   |
LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
   |             ^^^^^^^^^^^^^^ help: replace it with: `banana`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/issues/issue-60057.rs:14:13
  --> /checkout/src/test/ui/issues/issue-60057.rs:14:13
   |
LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
   |             ^^^^^^^^^^^^^^ help: replace it with: `banana`
error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0425`.

---
To only update this specific test, also pass `--test-args issues/issue-979.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-979.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-979/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-979/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

+ warning: redundant field names in struct initialization
+   --> $DIR/lint-shorthand-field.rs:40:22
+    |
+ LL |         match (Bar { x: x }) {
+    |                      ^^^^ help: replace it with: `x`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error: the `x:` in this pattern is redundant
3    |


34 LL |             z: ref mut z,
35    |             ^^^^^^^^^^^^ help: use shorthand field pattern: `ref mut z`
- error: aborting due to 5 previous errors
+ error: aborting due to 5 previous errors; 1 warning emitted
38 
39 
---

37 
38         struct x;
39 
-         match (Bar { x: x }) {
+         match (Bar { x }) {
41             Bar { x: x } => {},
43     }


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-shorthand-field/lint-shorthand-field.fixed
To only update this specific test, also pass `--test-args lint/lint-shorthand-field.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-shorthand-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-shorthand-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-shorthand-field/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/lint/lint-shorthand-field.rs:40:22
   |
LL |         match (Bar { x: x }) {
   |                      ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default


error: the `x:` in this pattern is redundant
   |
   |
LL |             x: x, //~ ERROR the `x:` in this pattern is redundant
   |             ^^^^ help: use shorthand field pattern: `x`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-shorthand-field.rs:4:9
   |
   |
LL | #![deny(non_shorthand_field_patterns)]


error: the `y:` in this pattern is redundant
   |
   |
LL |             y: ref y, //~ ERROR the `y:` in this pattern is redundant
   |             ^^^^^^^^ help: use shorthand field pattern: `ref y`

error: the `x:` in this pattern is redundant
   |
   |
LL |             x: mut x, //~ ERROR the `x:` in this pattern is redundant
   |             ^^^^^^^^ help: use shorthand field pattern: `mut x`

error: the `y:` in this pattern is redundant
   |
   |
LL |             y: ref y, //~ ERROR the `y:` in this pattern is redundant
   |             ^^^^^^^^ help: use shorthand field pattern: `ref y`

error: the `z:` in this pattern is redundant
   |
   |
LL |             z: ref mut z, //~ ERROR the `z:` in this pattern is redundant
   |             ^^^^^^^^^^^^ help: use shorthand field pattern: `ref mut z`
error: aborting due to 5 previous errors; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/max-min-classes.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/max-min-classes.rs:26:11
   |
LL |     Foo { x: x, y: y }
   |           ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/max-min-classes.rs:26:17
  --> $DIR/max-min-classes.rs:26:17
   |
LL |     Foo { x: x, y: y }
   |                 ^^^^ help: replace it with: `y`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args max-min-classes.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/max-min-classes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/max-min-classes/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/max-min-classes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/max-min-classes.rs:26:11
   |
LL |     Foo { x: x, y: y }
   |           ^^^^ help: replace it with: `x`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/max-min-classes.rs:26:17
  --> /checkout/src/test/ui/max-min-classes.rs:26:17
   |
LL |     Foo { x: x, y: y }
   |                 ^^^^ help: replace it with: `y`
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/mir/mir_drop_order.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/mir_drop_order.rs:23:31
   |
LL |     let d = |id| DropLogger { id: id, log: &log };
   |                               ^^^^^^ help: replace it with: `id`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args mir/mir_drop_order.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir_drop_order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_order/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/mir/mir_drop_order.rs:23:31
   |
LL |     let d = |id| DropLogger { id: id, log: &log };
   |                               ^^^^^^ help: replace it with: `id`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args nested-class.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nested-class.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nested-class/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nested-class/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/newtype.rs:19:31
   |
LL |     let myval = mytype(Mytype{compute: compute, val: 30});
   |                               ^^^^^^^^^^^^^^^^ help: replace it with: `compute`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args newtype.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/newtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/newtype/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/newtype/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/newtype.rs:19:31
   |
LL |     let myval = mytype(Mytype{compute: compute, val: 30});
   |                               ^^^^^^^^^^^^^^^^ help: replace it with: `compute`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/issue-53789-2.rs:105:9
   |
LL |         element: element,
   |         ^^^^^^^^^^^^^^^^ help: replace it with: `element`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/issue-53789-2.rs:106:9
---

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-53789-2/issue-53789-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/issue-53789-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/issue-53789-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-53789-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-53789-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/nll/ty-outlives/issue-53789-2.rs:105:9
   |
LL |         element: element,
   |         ^^^^^^^^^^^^^^^^ help: replace it with: `element`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/nll/ty-outlives/issue-53789-2.rs:106:9
---
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/noncopyable-class.rs:27:9
+    |
+ LL |         i: i,
+    |         ^^^^ help: replace it with: `i`
1 error[E0599]: no method named `clone` found for struct `Foo` in the current scope
2   --> $DIR/noncopyable-class.rs:34:16
3    |


11    = note: the following trait defines an item `clone`, perhaps you need to implement it:
12            candidate #1: `Clone`
- error: aborting due to previous error
+ error: aborting due to previous error; 2 warnings emitted
15 
16 For more information about this error, try `rustc --explain E0599`.
---
To only update this specific test, also pass `--test-args noncopyable-class.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/noncopyable-class.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/noncopyable-class" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/noncopyable-class/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0599]: no method named `clone` found for struct `Foo` in the current scope
  --> /checkout/src/test/ui/noncopyable-class.rs:34:16
   |
LL | struct Foo {
   | ---------- method `clone` not found for this
...
LL |     let _y = x.clone(); //~ ERROR no method named `clone` found
   |                ^^^^^ method not found in `Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0599`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/mpsc_stress.rs:27:62
   |
LL |         (0..count).map(|_| Barrier { shared: shared.clone(), count: count }).collect()
   |                                                              ^^^^^^^^^^^^ help: replace it with: `count`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args mpsc_stress.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mpsc_stress.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mpsc_stress/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mpsc_stress/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/mpsc_stress.rs:27:62
   |
LL |         (0..count).map(|_| Barrier { shared: shared.clone(), count: count }).collect()
   |                                                              ^^^^^^^^^^^^ help: replace it with: `count`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/overloaded-autoderef-count.rs:17:13
   |
LL |             value: value
   |             ^^^^^^^^^^^^ help: replace it with: `value`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args overloaded/overloaded-autoderef-count.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/overloaded/overloaded-autoderef-count.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-autoderef-count/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-autoderef-count/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/overloaded/overloaded-autoderef-count.rs:17:13
   |
LL |             value: value
   |             ^^^^^^^^^^^^ help: replace it with: `value`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args overloaded/overloaded-autoderef-order.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/overloaded/overloaded-autoderef-order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-autoderef-order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-autoderef-order/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/overloaded-index-assoc-list.rs:18:42
   |
LL |         self.pairs.push(AssociationPair {key: key, value: value});
   |                                          ^^^^^^^^ help: replace it with: `key`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/overloaded-index-assoc-list.rs:18:52
  --> $DIR/overloaded-index-assoc-list.rs:18:52
   |
LL |         self.pairs.push(AssociationPair {key: key, value: value});
   |                                                    ^^^^^^^^^^^^ help: replace it with: `value`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args overloaded/overloaded-index-assoc-list.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/overloaded/overloaded-index-assoc-list.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-index-assoc-list/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-index-assoc-list/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/overloaded/overloaded-index-assoc-list.rs:18:42
   |
LL |         self.pairs.push(AssociationPair {key: key, value: value});
   |                                          ^^^^^^^^ help: replace it with: `key`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/overloaded/overloaded-index-assoc-list.rs:18:52
  --> /checkout/src/test/ui/overloaded/overloaded-index-assoc-list.rs:18:52
   |
LL |         self.pairs.push(AssociationPair {key: key, value: value});
   |                                                    ^^^^^^^^^^^^ help: replace it with: `value`
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/overloaded/overloaded-deref-count.rs stdout ----
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/overloaded-deref-count.rs:18:13
   |
LL |             value: value
   |             ^^^^^^^^^^^^ help: replace it with: `value`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args overloaded/overloaded-deref-count.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/overloaded/overloaded-deref-count.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-deref-count/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/overloaded/overloaded-deref-count/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/overloaded/overloaded-deref-count.rs:18:13
   |
LL |             value: value
   |             ^^^^^^^^^^^^ help: replace it with: `value`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---

+ warning: redundant field names in struct initialization
+   --> $DIR/issue-12470.rs:23:9
+    |
+ LL |     A { p: p }
+    |         ^^^^ help: replace it with: `p`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0515]: cannot return value referencing local data `*b`
3    |

6 LL |     make_a(bb)
6 LL |     make_a(bb)
7    |     ^^^^^^^^^^ returns a value referencing data owned by the current function
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
10 
11 For more information about this error, try `rustc --explain E0515`.
---
To only update this specific test, also pass `--test-args regions/issue-12470.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-12470.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-12470" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-12470/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/regions/issue-12470.rs:23:9
   |
LL |     A { p: p }
   |         ^^^^ help: replace it with: `p`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0515]: cannot return value referencing local data `*b`
   |
   |
LL |     let bb: &B = &*b;
   |                  --- `*b` is borrowed here
LL |     make_a(bb)  //~ ERROR cannot return value referencing local data `*b`
   |     ^^^^^^^^^^ returns a value referencing data owned by the current function
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0515`.

---

+ warning: redundant field names in struct initialization
+   --> $DIR/regions-glb-free-free.rs:10:16
+    |
+ LL |         Flag { name: name, desc: desc, max_count: 1, value: 0 }
+    |                ^^^^^^^^^^ help: replace it with: `name`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ warning: redundant field names in struct initialization
+   --> $DIR/regions-glb-free-free.rs:10:28
+   --> $DIR/regions-glb-free-free.rs:10:28
+    |
+ LL |         Flag { name: name, desc: desc, max_count: 1, value: 0 }
+    |                            ^^^^^^^^^^ help: replace it with: `desc`
1 error[E0621]: explicit lifetime required in the type of `s`
2   --> $DIR/regions-glb-free-free.rs:15:13
3    |


11 LL | |             }
12    | |_____________^ lifetime `'a` required
- error: aborting due to previous error
+ error: aborting due to previous error; 2 warnings emitted
15 
16 For more information about this error, try `rustc --explain E0621`.
---
To only update this specific test, also pass `--test-args regions/regions-glb-free-free.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-glb-free-free.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-glb-free-free" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-glb-free-free/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/regions/regions-glb-free-free.rs:10:16
   |
LL |         Flag { name: name, desc: desc, max_count: 1, value: 0 }
   |                ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/regions/regions-glb-free-free.rs:10:28
  --> /checkout/src/test/ui/regions/regions-glb-free-free.rs:10:28
   |
LL |         Flag { name: name, desc: desc, max_count: 1, value: 0 }
   |                            ^^^^^^^^^^ help: replace it with: `desc`
error[E0621]: explicit lifetime required in the type of `s`
  --> /checkout/src/test/ui/regions/regions-glb-free-free.rs:15:13
   |
   |
LL |           pub fn set_desc(self, s: &str) -> Flag<'a> {
   |                                    ---- help: add explicit lifetime `'a` to the type of `s`: `&'a str`
LL | /             Flag { //~ ERROR explicit lifetime required in the type of `s` [E0621]
LL | |                 name: self.name,
LL | |                 desc: s,
LL | |                 max_count: self.max_count,
LL | |                 value: self.value
LL | |             }
   | |_____________^ lifetime `'a` required
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0621`.

---

+ warning: redundant field names in struct initialization
+   --> $DIR/regions-trait-variance.rs:30:7
+    |
+ LL |     A{p:p}
+    |       ^^^ help: replace it with: `p`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0515]: cannot return value referencing local data `*b`
3    |

6 LL |     make_a(bb)
6 LL |     make_a(bb)
7    |     ^^^^^^^^^^ returns a value referencing data owned by the current function
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
10 
11 For more information about this error, try `rustc --explain E0515`.
---
To only update this specific test, also pass `--test-args regions/regions-trait-variance.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-trait-variance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-variance" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-variance/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/regions/regions-trait-variance.rs:30:7
   |
LL |     A{p:p}
   |       ^^^ help: replace it with: `p`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0515]: cannot return value referencing local data `*b`
   |
   |
LL |     let bb: &B = &*b;
   |                  --- `*b` is borrowed here
LL |     make_a(bb) //~ ERROR cannot return value referencing local data `*b`
   |     ^^^^^^^^^^ returns a value referencing data owned by the current function
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0515`.

---
To only update this specific test, also pass `--test-args resource-destruct.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resource-destruct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resource-destruct/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resource-destruct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args resource-assign-is-not-copy.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resource-assign-is-not-copy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resource-assign-is-not-copy/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resource-assign-is-not-copy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args self/explicit-self.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/explicit-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/explicit-self/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/explicit-self/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

+ warning: redundant field names in struct initialization
+   --> $DIR/dropck_direct_cycle_with_drop.rs:24:41
+    |
+ LL |     fn new(name: String) -> D<'a> { D { name: name, p: Cell::new(None) } }
+    |                                         ^^^^^^^^^^ help: replace it with: `name`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0597]: `d2` does not live long enough
3    |


24    | `d1` dropped here while still borrowed
25    | borrow might be used here, when `d1` is dropped and runs the `Drop` code for type `D`
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
28 
29 For more information about this error, try `rustc --explain E0597`.
---
To only update this specific test, also pass `--test-args span/dropck_direct_cycle_with_drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/dropck_direct_cycle_with_drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/dropck_direct_cycle_with_drop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/dropck_direct_cycle_with_drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/span/dropck_direct_cycle_with_drop.rs:24:41
   |
LL |     fn new(name: String) -> D<'a> { D { name: name, p: Cell::new(None) } }
   |                                         ^^^^^^^^^^ help: replace it with: `name`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0597]: `d2` does not live long enough
   |
   |
LL |     d1.p.set(Some(&d2));
   |                   ^^^ borrowed value does not live long enough
LL | }
   | -
   | |
   | |
   | `d2` dropped here while still borrowed
   | borrow might be used here, when `d1` is dropped and runs the `Drop` code for type `D`
   |
   = note: values in a scope are dropped in the opposite order they are defined

error[E0597]: `d1` does not live long enough
   |
   |
LL |     d2.p.set(Some(&d1));
   |                   ^^^ borrowed value does not live long enough
LL |     //~^ ERROR `d1` does not live long enough
LL | }
   | |
   | |
   | `d1` dropped here while still borrowed
   | borrow might be used here, when `d1` is dropped and runs the `Drop` code for type `D`
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0597`.

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/class-dtor.rs:21:9
   |
LL |         done: done
   |         ^^^^^^^^^^ help: replace it with: `done`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args structs-enums/class-dtor.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/class-dtor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/class-dtor/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/class-dtor/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/structs-enums/class-dtor.rs:21:9
   |
LL |         done: done
   |         ^^^^^^^^^^ help: replace it with: `done`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args structs-enums/classes-self-referential.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/classes-self-referential.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/classes-self-referential/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/classes-self-referential/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args structs-enums/resource-in-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/resource-in-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/resource-in-struct/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/resource-in-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/struct-field-shorthand.rs:14:22
   |
LL |     let a = Foo { x, y: y, z };
   |                      ^^^^ help: replace it with: `y`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args structs-enums/struct-field-shorthand.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/struct-field-shorthand.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/struct-field-shorthand/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/struct-field-shorthand/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/structs-enums/struct-field-shorthand.rs:14:22
   |
LL |     let a = Foo { x, y: y, z };
   |                      ^^^^ help: replace it with: `y`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/struct-path-self.rs:17:47
   |
LL |             Self { a, b } => Self { a: a + 1, b: b }
   |                                               ^^^^ help: replace it with: `b`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/struct-path-self.rs:26:37
  --> $DIR/struct-path-self.rs:26:37
   |
LL |             Self { a, b } => Self { a: a, b: b + 1 }
   |                                     ^^^^ help: replace it with: `a`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args structs-enums/struct-path-self.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/struct-path-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/struct-path-self/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/struct-path-self/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/structs-enums/struct-path-self.rs:17:47
   |
LL |             Self { a, b } => Self { a: a + 1, b: b }
   |                                               ^^^^ help: replace it with: `b`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/structs-enums/struct-path-self.rs:26:37
  --> /checkout/src/test/ui/structs-enums/struct-path-self.rs:26:37
   |
LL |             Self { a, b } => Self { a: a, b: b + 1 }
   |                                     ^^^^ help: replace it with: `a`
warning: 2 warnings emitted


------------------------------------------
---
+    |             ^^^^ help: replace it with: `g`
+    |
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
1 error[E0277]: the trait bound `Fancy: SomeTrait` is not satisfied
3    |


13 LL |     pub fn new(g: G) -> Self {
15 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
17 
---
To only update this specific test, also pass `--test-args suggestions/issue-84973.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-84973.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84973" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84973/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |             ^^^^ help: replace it with: `g`
   |
   = note: `#[warn(redundant_field_names)]` on by default

error[E0277]: the trait bound `Fancy: SomeTrait` is not satisfied
   |
   |
LL |     let o = Other::new(f);
   |                        |
   |                        expected an implementor of trait `SomeTrait`
   |                        help: consider borrowing here: `&f`
   |
   |
note: required by `Other::<'a, G>::new`
   |
   |
LL |     pub fn new(g: G) -> Self {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args threads-sendsync/sendable-class.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/sendable-class.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/sendable-class/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/sendable-class/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args threads-sendsync/send-resource.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/send-resource.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/send-resource/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/send-resource/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/tls-init-on-init.rs:23:15
   |
LL |         Foo { cnt: cnt }
   |               ^^^^^^^^ help: replace it with: `cnt`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args threads-sendsync/tls-init-on-init.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/tls-init-on-init.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/tls-init-on-init/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/tls-init-on-init/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/threads-sendsync/tls-init-on-init.rs:23:15
   |
LL |         Foo { cnt: cnt }
   |               ^^^^^^^^ help: replace it with: `cnt`
   = note: `#[warn(redundant_field_names)]` on by default

warning: 1 warning emitted

---

+ warning: redundant field names in struct initialization
+   --> $DIR/coercion-generic-regions.rs:18:61
+    |
+ LL |     let s: Box<dyn Trait<&'static str>> = Box::new(Struct { person: person });
+    |                                                             ^^^^^^^^^^^^^^ help: replace it with: `person`
+    = note: `#[warn(redundant_field_names)]` on by default
+ 
+ 
1 error[E0597]: `person` does not live long enough
2   --> $DIR/coercion-generic-regions.rs:17:24

10 LL | }
10 LL | }
11    | - `person` dropped here while still borrowed
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
14 
15 For more information about this error, try `rustc --explain E0597`.
---
To only update this specific test, also pass `--test-args traits/coercion-generic-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/coercion-generic-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/coercion-generic-regions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/coercion-generic-regions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/traits/coercion-generic-regions.rs:18:61
   |
LL |     let s: Box<dyn Trait<&'static str>> = Box::new(Struct { person: person });
   |                                                             ^^^^^^^^^^^^^^ help: replace it with: `person`
   = note: `#[warn(redundant_field_names)]` on by default


error[E0597]: `person` does not live long enough
   |
   |
LL |     let person: &str = &person;  //~ ERROR `person` does not live long enough
   |                        |
   |                        |
   |                        borrowed value does not live long enough
   |                        assignment requires that `person` is borrowed for `'static`
LL |     let s: Box<dyn Trait<&'static str>> = Box::new(Struct { person: person });
LL | }
   | - `person` dropped here while still borrowed
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0597`.

---
To only update this specific test, also pass `--test-args type-param-constraints.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-param-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-param-constraints/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-param-constraints/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/variance-intersection-of-ref-and-opt-ref.rs:15:23
   |
LL |     let list = List { field1: field1, field2: field2 };
   |                       ^^^^^^^^^^^^^^ help: replace it with: `field1`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/variance-intersection-of-ref-and-opt-ref.rs:15:39
  --> $DIR/variance-intersection-of-ref-and-opt-ref.rs:15:39
   |
LL |     let list = List { field1: field1, field2: field2 };
   |                                       ^^^^^^^^^^^^^^ help: replace it with: `field2`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args variance-intersection-of-ref-and-opt-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance-intersection-of-ref-and-opt-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance-intersection-of-ref-and-opt-ref/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance-intersection-of-ref-and-opt-ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/variance-intersection-of-ref-and-opt-ref.rs:15:23
   |
LL |     let list = List { field1: field1, field2: field2 };
   |                       ^^^^^^^^^^^^^^ help: replace it with: `field1`
   = note: `#[warn(redundant_field_names)]` on by default

warning: redundant field names in struct initialization
  --> /checkout/src/test/ui/variance-intersection-of-ref-and-opt-ref.rs:15:39
  --> /checkout/src/test/ui/variance-intersection-of-ref-and-opt-ref.rs:15:39
   |
LL |     let list = List { field1: field1, field2: field2 };
   |                                       ^^^^^^^^^^^^^^ help: replace it with: `field2`
warning: 2 warnings emitted


------------------------------------------
---
To only update this specific test, also pass `--test-args unwind-resource.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-resource.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-resource/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-resource/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11975 passed; 99 failed; 103 ignored; 0 measured; 0 filtered out; finished in 107.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:55
