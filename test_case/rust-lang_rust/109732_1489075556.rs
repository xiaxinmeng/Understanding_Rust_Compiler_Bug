plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
..................................F.....................................................  1496/14733
.......................i................................................................  1584/14733
........................................................................................  1672/14733
..i.....................................................................................  1760/14733
...............................................................F...................F...F  1848/14733
...........................................i.....................i...........ii.........  2024/14733
........................................................................................  2112/14733
........................................................................................  2200/14733
......................................................i.................................  2288/14733
---
........................................................................................  3080/14733
....F...................................................................................  3168/14733
..F.....................................................................................  3256/14733
........................................................................................  3344/14733
...............................i......................................F...............i.  3432/14733
........................................................................................  3608/14733
.............................................................................iii.ii.....  3696/14733
..............................................................F.........................  3784/14733
........................................................................................  3872/14733
---
.......................................................................iii..............  4488/14733
......................F.................................................................  4576/14733
.........................................................i..............................  4664/14733
........................................................................................  4752/14733
.....F...F....................................F......F.........F........................  4840/14733
..............F.........................................................................  4928/14733
................................................................i.......................  5104/14733
........................................................................................  5192/14733
........................................................................................  5280/14733
........................................................................................  5280/14733
...........F..................F.........................................................  5368/14733
........................................................................................  5544/14733
........................................................................................  5632/14733
........................................................................................  5720/14733
........................................................................................  5808/14733
---
......i......i..ii......................................................................  8888/14733
........................................................F...............................  8976/14733
........................................................................................  9064/14733
........................................................................................  9152/14733
...............................................F................F.......................  9240/14733
............i............................ii.............................................  9416/14733
........................................................................................  9504/14733
...............F........................................................................  9592/14733
.......i........................................i.......................................  9680/14733
.......i........................................i.......................................  9680/14733
................................i.......................................................  9768/14733
........................................................................................  9856/14733
.....................................................................i..................  9944/14733
........................................................................................ 10032/14733
...................................................................................i.... 10120/14733
........................................................................................ 10208/14733
........................................................................................ 10296/14733
.......................................................F.F.............................. 10384/14733
....................F.....F............................................................. 10472/14733
.............................................................................F....F..... 10560/14733
........................................................................................ 10736/14733
.....................................................ii...............i.....iii......... 10824/14733
........................................................................................ 10912/14733
........................................................................................ 11000/14733
........................................................................................ 11000/14733
........................................................................................ 11088/14733
........................................................................................ 11176/14733
...................................F....F............................................... 11264/14733
........................................................................................ 11440/14733
..................F..................................................................... 11528/14733
....................................................iiiii...i....i.i.................... 11616/14733
..........F............................................................................. 11704/14733
---
failures:

---- [ui] tests/ui/async-await/multiple-lifetimes/partial-relation.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop((a, c));
   |
   |
note: argument has type `(&u32, &u32)`
   |
   |
LL |     drop((a, c));
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/partial-relation.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/multiple-lifetimes/partial-relation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/partial-relation/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/partial-relation/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/async-await/multiple-lifetimes/partial-relation.rs:7:5
   |
LL |     drop((a, c));
   |
   |
note: argument has type `(&u32, &u32)`
  --> fake-test-src-base/async-await/multiple-lifetimes/partial-relation.rs:7:10
   |
LL |     drop((a, c));
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/borrowck/borrowck-closures-slice-patterns-ok.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(r);
   |     ^^^^^^^
   |
   |
note: argument has type `&[String; 3]`
   |
LL |     drop(r);
   |          ^
   |          ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(r);
   |     ^^^^^^^
   |
   |
note: argument has type `&&[String; 3]`
   |
LL |     drop(r);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `&[String; 3]`
   |
LL |     drop(x);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `&mut [String; 3]`
   |
LL |     drop(x);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(r);
   |     ^^^^^^^
   |
   |
note: argument has type `&&[String]`
   |
LL |     drop(r);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `&[String]`
  --> $DIR/borrowck-closures-slice-patterns-ok.rs:88:10
   |
LL |     drop(x);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(x);
   |     ^^^^^^^
   |
---
To only update this specific test, also pass `--test-args borrowck/borrowck-closures-slice-patterns-ok.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/borrowck/borrowck-closures-slice-patterns-ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-slice-patterns-ok/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-slice-patterns-ok/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:15:5
LL |     drop(r);
   |     ^^^^^^^
   |
   |
note: argument has type `&[String; 3]`
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:15:10
LL |     drop(r);
   |          ^
   |          ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:48:5
LL |     drop(r);
   |     ^^^^^^^
   |
   |
note: argument has type `&&[String; 3]`
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:48:10
LL |     drop(r);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:49:5
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `&[String; 3]`
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:49:10
LL |     drop(x);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:65:5
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `&mut [String; 3]`
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:65:10
LL |     drop(x);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:87:5
LL |     drop(r);
   |     ^^^^^^^
   |
   |
note: argument has type `&&[String]`
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:87:10
LL |     drop(r);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:88:5
LL |     drop(x);
   |     ^^^^^^^
   |
note: argument has type `&[String]`
note: argument has type `&[String]`
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:88:10
   |
LL |     drop(x);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/borrowck/borrowck-closures-slice-patterns-ok.rs:105:5
LL |     drop(x);
   |     ^^^^^^^
   |
note: argument has type `&mut [String]`
---


---- [ui] tests/ui/borrowck/borrowck-field-sensitivity-rpass.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:11:10
   |
LL |     drop(x.a);
   |          ^^^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:23:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:36:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:42:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(*p);
   |     ^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:61:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(*p);
   |     ^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:68:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(*p);
   |     ^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:75:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(*p);
   |     ^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:82:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(*p);
   |     ^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:89:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(**q);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:90:10
   |
   |
LL |     drop(**q);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:122:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:129:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:136:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:143:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:151:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:159:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:167:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:175:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-field-sensitivity-rpass.rs:209:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(*p);
   |     ^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args borrowck/borrowck-field-sensitivity-rpass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/borrowck/borrowck-field-sensitivity-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity-rpass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity-rpass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:11:5
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:11:10
   |
LL |     drop(x.a);
   |          ^^^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:23:5
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:23:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:36:5
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:36:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:42:5
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:42:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:61:5
LL |     drop(*p);
   |     ^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:61:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:68:5
LL |     drop(*p);
   |     ^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:68:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:75:5
LL |     drop(*p);
   |     ^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:75:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:82:5
LL |     drop(*p);
   |     ^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:82:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:89:5
LL |     drop(*p);
   |     ^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:89:10
   |
LL |     drop(*p);
   |          ^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:90:5
   |
LL |     drop(**q);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:90:10
   |
   |
LL |     drop(**q);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:122:5
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:122:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:129:5
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:129:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:136:5
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:136:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:143:5
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:143:10
   |
   |
LL |     drop(*x.b);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:151:5
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:151:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:159:5
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:159:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:167:5
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:167:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:175:5
   |
LL |     drop(**p);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:175:10
   |
   |
LL |     drop(**p);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:209:5
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:209:10
   |
LL |     drop(x.a);
   |          ^^^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-field-sensitivity-rpass.rs:216:5
LL |     drop(*p);
   |     ^^^^^^^^
   |
note: argument has type `isize`
---


---- [ui] tests/ui/borrowck/borrowck-use-mut-borrow-rpass.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
   |
note: argument has type `isize`
  --> $DIR/borrowck-use-mut-borrow-rpass.rs:9:10
   |
LL |     drop(x.a);
   |          ^^^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> $DIR/borrowck-use-mut-borrow-rpass.rs:24:10
   |
   |
LL |     drop(*x.b);

warning: 2 warnings emitted


---
To only update this specific test, also pass `--test-args borrowck/borrowck-use-mut-borrow-rpass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/borrowck/borrowck-use-mut-borrow-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-use-mut-borrow-rpass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-use-mut-borrow-rpass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-use-mut-borrow-rpass.rs:9:5
LL |     drop(x.a);
   |     ^^^^^^^^^
   |
note: argument has type `isize`
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-use-mut-borrow-rpass.rs:9:10
   |
LL |     drop(x.a);
   |          ^^^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/borrowck/borrowck-use-mut-borrow-rpass.rs:24:5
   |
LL |     drop(*x.b);
   |
note: argument has type `isize`
  --> fake-test-src-base/borrowck/borrowck-use-mut-borrow-rpass.rs:24:10
   |
   |
LL |     drop(*x.b);

warning: 2 warnings emitted
------------------------------------------

---
8    = help: consider replacing the `if let` with a `let`
9    = note: `#[warn(irrefutable_let_patterns)]` on by default
10 
- warning: 1 warning emitted
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+    |
+    |
+ LL |         drop(|_: ()| drop(a));
+    |
+ note: argument has type `[closure@$DIR/issue-78720.rs:8:14: 8:21]`
+   --> $DIR/issue-78720.rs:8:14
+    |
+    |
+ LL |         drop(|_: ()| drop(a));
+    = note: `#[warn(drop_copy)]` on by default
+ 
+ 
+ warning: calls to `std::mem::drop` with a reference instead of an owned value
+    |
+    |
+ LL |         drop(|_: ()| drop(a));
+    |
+ note: argument has type `&str`
+   --> $DIR/issue-78720.rs:8:27
+    |
+    |
+ LL |         drop(|_: ()| drop(a));
+    |                           ^
+    = note: `#[warn(drop_ref)]` on by default
+ warning: 3 warnings emitted
12 
13 

---
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/issue-78720.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closures/2229_closure_analysis/migrations/issue-78720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/issue-78720/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/issue-78720/auxiliary"
stdout: none
--- stderr -------------------------------
warning: irrefutable `if let` pattern
  --> fake-test-src-base/closures/2229_closure_analysis/migrations/issue-78720.rs:6:8
LL |     if let a = "" {
   |        ^^^^^^^^^^
   |
   |
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`
   = note: `#[warn(irrefutable_let_patterns)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/closures/2229_closure_analysis/migrations/issue-78720.rs:8:9
   |
LL |         drop(|_: ()| drop(a));
   |
   |
note: argument has type `[closure@fake-test-src-base/closures/2229_closure_analysis/migrations/issue-78720.rs:8:14: 8:21]`
  --> fake-test-src-base/closures/2229_closure_analysis/migrations/issue-78720.rs:8:14
   |
LL |         drop(|_: ()| drop(a));
   = note: `#[warn(drop_copy)]` on by default


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/closures/2229_closure_analysis/migrations/issue-78720.rs:8:22
   |
LL |         drop(|_: ()| drop(a));
   |
note: argument has type `&str`
  --> fake-test-src-base/closures/2229_closure_analysis/migrations/issue-78720.rs:8:27
   |
   |
LL |         drop(|_: ()| drop(a));
   |                           ^
   = note: `#[warn(drop_ref)]` on by default
warning: 3 warnings emitted
------------------------------------------



---- [ui] tests/ui/closures/2229_closure_analysis/optimization/edge_case_run_pass.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |     let c = || drop(&m.a.0);
   |
note: argument has type `&i32`
  --> $DIR/edge_case_run_pass.rs:19:21
   |
   |
LL |     let c = || drop(&m.a.0);
   |                     ^^^^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/optimization/edge_case_run_pass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closures/2229_closure_analysis/optimization/edge_case_run_pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/optimization/edge_case_run_pass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/optimization/edge_case_run_pass/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/closures/2229_closure_analysis/optimization/edge_case_run_pass.rs:19:16
   |
LL |     let c = || drop(&m.a.0);
   |
note: argument has type `&i32`
  --> fake-test-src-base/closures/2229_closure_analysis/optimization/edge_case_run_pass.rs:19:21
   |
   |
LL |     let c = || drop(&m.a.0);
   |                     ^^^^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/closures/2229_closure_analysis/run_pass/drop_then_use_fake_reads.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |         drop(&mut x);
   |         ^^^^^^^^^^^^
   |
   |
note: argument has type `&mut i32`
  --> $DIR/drop_then_use_fake_reads.rs:8:14
   |
LL |         drop(&mut x);
   |              ^^^^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/run_pass/drop_then_use_fake_reads.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closures/2229_closure_analysis/run_pass/drop_then_use_fake_reads.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/run_pass/drop_then_use_fake_reads" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/run_pass/drop_then_use_fake_reads/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/closures/2229_closure_analysis/run_pass/drop_then_use_fake_reads.rs:8:9
LL |         drop(&mut x);
   |         ^^^^^^^^^^^^
   |
note: argument has type `&mut i32`
note: argument has type `&mut i32`
  --> fake-test-src-base/closures/2229_closure_analysis/run_pass/drop_then_use_fake_reads.rs:8:14
   |
LL |         drop(&mut x);
   |              ^^^^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/consts/const_forget.rs stdout ----
normalized stderr:
warning: calls to `std::mem::forget` with a value that implements `Copy`.
   |
LL | const _: () = forget(0i32);
   |               ^^^^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args consts/const_forget.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const_forget.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_forget" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_forget/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::forget` with a value that implements `Copy`.
  --> fake-test-src-base/consts/const_forget.rs:5:15
LL | const _: () = forget(0i32);
   |               ^^^^^^^^^^^^
   |
note: argument has type `i32`
---


---- [ui] tests/ui/consts/issue-104155.rs stdout ----
normalized stderr:
warning: calls to `std::mem::forget` with a value that implements `Copy`.
   |
   |
LL | const _: () = core::mem::forget(Box::<u32>::default);
   |
   |
note: argument has type `fn() -> Box<u32> {<Box<u32> as Default>::default}`
   |
   |
LL | const _: () = core::mem::forget(Box::<u32>::default);
   = note: `#[warn(forget_copy)]` on by default


warning: calls to `std::mem::forget` with a value that implements `Copy`.
   |
   |
LL | const _: () = core::mem::forget(|| Box::<u32>::default());
   |
note: argument has type `[closure@$DIR/issue-104155.rs:3:33: 3:35]`
  --> $DIR/issue-104155.rs:3:33
   |
   |
LL | const _: () = core::mem::forget(|| Box::<u32>::default());

warning: 2 warnings emitted


---
To only update this specific test, also pass `--test-args consts/issue-104155.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/issue-104155.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-104155" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-104155/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::forget` with a value that implements `Copy`.
  --> fake-test-src-base/consts/issue-104155.rs:2:15
   |
LL | const _: () = core::mem::forget(Box::<u32>::default);
   |
   |
note: argument has type `fn() -> Box<u32> {<Box<u32> as Default>::default}`
  --> fake-test-src-base/consts/issue-104155.rs:2:33
   |
LL | const _: () = core::mem::forget(Box::<u32>::default);
   = note: `#[warn(forget_copy)]` on by default


warning: calls to `std::mem::forget` with a value that implements `Copy`.
  --> fake-test-src-base/consts/issue-104155.rs:3:15
   |
LL | const _: () = core::mem::forget(|| Box::<u32>::default());
   |
   |
note: argument has type `[closure@fake-test-src-base/consts/issue-104155.rs:3:33: 3:35]`
  --> fake-test-src-base/consts/issue-104155.rs:3:33
   |
LL | const _: () = core::mem::forget(|| Box::<u32>::default());

warning: 2 warnings emitted
------------------------------------------



---- [ui] tests/ui/crate-leading-sep.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     mem::drop(2_usize);
   |     ^^^^^^^^^^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args crate-leading-sep.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/crate-leading-sep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-leading-sep/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-leading-sep/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/crate-leading-sep.rs:6:5
LL |     mem::drop(2_usize);
   |     ^^^^^^^^^^^^^^^^^^
   |
note: argument has type `usize`
---


---- [ui] tests/ui/drop/repeat-drop.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     std::mem::drop(v);
   |     ^^^^^^^^^^^^^^^^^
   |
   |
note: argument has type `&[DropChecker; 0]`
   |
LL |     std::mem::drop(v);
   |                    ^
   |                    ^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args drop/repeat-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/drop/repeat-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/repeat-drop/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/repeat-drop/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/drop/repeat-drop.rs:92:5
LL |     std::mem::drop(v);
   |     ^^^^^^^^^^^^^^^^^
   |
   |
note: argument has type `&[DropChecker; 0]`
  --> fake-test-src-base/drop/repeat-drop.rs:92:20
LL |     std::mem::drop(v);
   |                    ^
   |                    ^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/explicit/explicit-call-to-supertrait-dtor.rs stdout ----

error: fixed code is still producing diagnostics
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/explicit/explicit-call-to-supertrait-dtor.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explicit/explicit-call-to-supertrait-dtor/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explicit/explicit-call-to-supertrait-dtor/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/explicit/explicit-call-to-supertrait-dtor.fixed:19:9
   |
LL |         drop(self);    //~ ERROR explicit use of destructor method
   |
note: argument has type `&Foo`
note: argument has type `&Foo`
  --> fake-test-src-base/explicit/explicit-call-to-supertrait-dtor.fixed:19:14
   |
LL |         drop(self);    //~ ERROR explicit use of destructor method
   |              ^^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/feature-gates/feature-gate-unsafe_pin_internals.rs stdout ----
diff of stderr:

10 LL | #![forbid(incomplete_features, unsafe_code)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ warning: calls to `std::mem::forget` with a value that implements `Copy`.
+    |
+    |
+ LL |     core::mem::forget(self_referential); // move and disable drop glue!
+    |
+    |
+ note: argument has type `PhantomPinned`
+    |
+    |
+ LL |     core::mem::forget(self_referential); // move and disable drop glue!
+    = note: `#[warn(forget_copy)]` on by default
+ 
+ error: aborting due to previous error; 1 warning emitted
14 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-unsafe_pin_internals.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/feature-gate-unsafe_pin_internals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_pin_internals" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unsafe_pin_internals/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: the feature `unsafe_pin_internals` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/feature-gates/feature-gate-unsafe_pin_internals.rs:3:12
LL | #![feature(unsafe_pin_internals)]
   |            ^^^^^^^^^^^^^^^^^^^^
   |
note: the lint level is defined here
note: the lint level is defined here
  --> fake-test-src-base/feature-gates/feature-gate-unsafe_pin_internals.rs:2:11
   |
LL | #![forbid(incomplete_features, unsafe_code)]


warning: calls to `std::mem::forget` with a value that implements `Copy`.
  --> fake-test-src-base/feature-gates/feature-gate-unsafe_pin_internals.rs:16:5
   |
LL |     core::mem::forget(self_referential); // move and disable drop glue!
   |
   |
note: argument has type `PhantomPinned`
  --> fake-test-src-base/feature-gates/feature-gate-unsafe_pin_internals.rs:16:23
   |
LL |     core::mem::forget(self_referential); // move and disable drop glue!
   = note: `#[warn(forget_copy)]` on by default

error: aborting due to previous error; 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/drop-env.rs#nomiropt stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   |
note: argument has type `GeneratorState<(), ()>`
  --> $DIR/drop-env.rs:36:10
   |
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-env.nomiropt/drop-env.nomiropt.stderr
To only update this specific test, also pass `--test-args generator/drop-env.rs`


error in revision `nomiropt`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/drop-env.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-env.nomiropt/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-env.nomiropt/auxiliary" "-Z" "mir-opt-level=0"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/generator/drop-env.rs:36:5
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   |
note: argument has type `GeneratorState<(), ()>`
  --> fake-test-src-base/generator/drop-env.rs:36:10
   |
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/drop-env.rs#default stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   |
note: argument has type `GeneratorState<(), ()>`
  --> $DIR/drop-env.rs:36:10
   |
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted

---
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-env.default/drop-env.default.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/drop-env.rs`

error in revision `default`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/drop-env.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-env.default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-env.default/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/generator/drop-env.rs:36:5
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   |
note: argument has type `GeneratorState<(), ()>`
  --> fake-test-src-base/generator/drop-env.rs:36:10
   |
   |
LL |     drop(Pin::new(&mut foo).resume(()));
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/issue-57017.rs#drop_tracking stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&copy::unsync::Client`
  --> $DIR/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   | |_____- in this macro invocation
   = note: `#[warn(drop_ref)]` on by default
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `copy::unsend::Client`
  --> $DIR/issue-57017.rs:40:40
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&derived_drop::unsync::Client`
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&significant_drop::unsync::Client`
  --> $DIR/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 4 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57017.drop_tracking/issue-57017.drop_tracking.stderr
To only update this specific test, also pass `--test-args generator/issue-57017.rs`


error in revision `drop_tracking`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/issue-57017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57017.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57017.drop_tracking/auxiliary" "-Zdrop-tracking"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/generator/issue-57017.rs:28:35
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&copy::unsync::Client`
  --> fake-test-src-base/generator/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   | |_____- in this macro invocation
   = note: `#[warn(drop_ref)]` on by default
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/generator/issue-57017.rs:40:35
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `copy::unsend::Client`
  --> fake-test-src-base/generator/issue-57017.rs:40:40
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/generator/issue-57017.rs:28:35
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&derived_drop::unsync::Client`
  --> fake-test-src-base/generator/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/generator/issue-57017.rs:28:35
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&significant_drop::unsync::Client`
  --> fake-test-src-base/generator/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 4 warnings emitted
------------------------------------------


---- [ui] tests/ui/generator/issue-57017.rs#drop_tracking_mir stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&copy::unsync::Client`
  --> $DIR/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   | |_____- in this macro invocation
   = note: `#[warn(drop_ref)]` on by default
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `copy::unsend::Client`
  --> $DIR/issue-57017.rs:40:40
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&derived_drop::unsync::Client`
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&significant_drop::unsync::Client`
  --> $DIR/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 4 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57017.drop_tracking_mir/issue-57017.drop_tracking_mir.stderr
To only update this specific test, also pass `--test-args generator/issue-57017.rs`


error in revision `drop_tracking_mir`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/issue-57017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking_mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57017.drop_tracking_mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57017.drop_tracking_mir/auxiliary" "-Zdrop-tracking-mir"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/generator/issue-57017.rs:28:35
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `&copy::unsync::Client`
  --> fake-test-src-base/generator/issue-57017.rs:28:40
   |
LL |               let g = move || match drop(&$name::unsync::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   | |_____- in this macro invocation
   = note: `#[warn(drop_ref)]` on by default
   = note: this warning originates in the macro `type_combinations` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/generator/issue-57017.rs:40:35
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
LL | |     );
   | |_____- in this macro invocation
   |
   |
note: argument has type `copy::unsend::Client`
  --> fake-test-src-base/generator/issue-57017.rs:40:40
   |
LL |               let g = move || match drop($name::unsend::Client::default()) {
...
LL | /     type_combinations!(
LL | /     type_combinations!(
LL | |         copy => { #[derive(Copy, Clone, Default)] pub struct Client; };
LL | |         derived_drop => { #[derive(Default)] pub struct Client { pub nickname: String } };
LL | |         significant_drop => {
LL | |         }
---
To only update this specific test, also pass `--test-args generator/non-static-is-unpin.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/non-static-is-unpin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/non-static-is-unpin/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/non-static-is-unpin/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/generator/non-static-is-unpin.rs:16:9
   |
LL |         drop(pinned);
   |
   |
note: argument has type `PhantomPinned`
  --> fake-test-src-base/generator/non-static-is-unpin.rs:16:14
   |
LL |         drop(pinned);
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generator/resume-arg-size.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(x);
   |             ^^^^^^^
   |
---
To only update this specific test, also pass `--test-args generator/resume-arg-size.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/resume-arg-size.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-arg-size/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-arg-size/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/generator/resume-arg-size.rs:11:13
LL |             drop(x);
   |             ^^^^^^^
   |
note: argument has type `usize`
---


---- [ui] tests/ui/hygiene/stdlib-prelude-from-opaque-late.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |             std::mem::drop(0); // OK (extern prelude)
...
LL | mac!();
   | ------ in this macro invocation
   |
   |
note: argument has type `i32`
  --> $DIR/stdlib-prelude-from-opaque-late.rs:8:28
   |
LL |             std::mem::drop(0); // OK (extern prelude)
...
LL | mac!();
   | ------ in this macro invocation
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default
   = note: this warning originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |             drop(0); // OK (stdlib prelude)
...
LL | mac!();
   | ------ in this macro invocation
   |
   |
note: argument has type `i32`
  --> $DIR/stdlib-prelude-from-opaque-late.rs:9:18
   |
LL |             drop(0); // OK (stdlib prelude)
...
LL | mac!();
   | ------ in this macro invocation
   = note: this warning originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
---
To only update this specific test, also pass `--test-args hygiene/stdlib-prelude-from-opaque-late.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/hygiene/stdlib-prelude-from-opaque-late.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/stdlib-prelude-from-opaque-late" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/stdlib-prelude-from-opaque-late/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/hygiene/stdlib-prelude-from-opaque-late.rs:8:13
   |
LL |             std::mem::drop(0); // OK (extern prelude)
...
LL | mac!();
   | ------ in this macro invocation
   |
   |
note: argument has type `i32`
  --> fake-test-src-base/hygiene/stdlib-prelude-from-opaque-late.rs:8:28
   |
LL |             std::mem::drop(0); // OK (extern prelude)
...
LL | mac!();
   | ------ in this macro invocation
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default
   = note: this warning originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/hygiene/stdlib-prelude-from-opaque-late.rs:9:13
   |
LL |             drop(0); // OK (stdlib prelude)
...
LL | mac!();
   | ------ in this macro invocation
   |
   |
note: argument has type `i32`
  --> fake-test-src-base/hygiene/stdlib-prelude-from-opaque-late.rs:9:18
   |
LL |             drop(0); // OK (stdlib prelude)
...
LL | mac!();
   | ------ in this macro invocation
   = note: this warning originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
---
---- [ui] tests/ui/illegal-ufcs-drop.rs stdout ----

error: fixed code is still producing diagnostics
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/illegal-ufcs-drop.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/illegal-ufcs-drop/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/illegal-ufcs-drop/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/illegal-ufcs-drop.fixed:9:5
   |
LL |     drop(&mut Foo) //~ ERROR explicit use of destructor method
   |
note: argument has type `&mut Foo`
note: argument has type `&mut Foo`
  --> fake-test-src-base/illegal-ufcs-drop.fixed:9:10
   |
LL |     drop(&mut Foo) //~ ERROR explicit use of destructor method
   |          ^^^^^^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/liveness/liveness-unused.rs stdout ----
diff of stderr:

112    |
113    = help: maybe it is overwritten before being read?
- error: aborting due to 13 previous errors; 1 warning emitted
- error: aborting due to 13 previous errors; 1 warning emitted
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+    |
+    |
+ LL |         drop(*x as i32);
+    |
+ note: argument has type `i32`
+   --> $DIR/liveness-unused.rs:92:14
+    |
+    |
+ LL |         drop(*x as i32);
+    = note: `#[warn(drop_copy)]` on by default
+ 
+ 
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+    |
+ LL |     drop(a);
+    |     ^^^^^^^
+    |
---
To only update this specific test, also pass `--test-args liveness/liveness-unused.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/liveness/liveness-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/liveness/liveness-unused.rs:92:9
   |
LL |         continue;
   |         -------- any code following this expression is unreachable
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

error: value assigned to `x` is never read
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

error: value assigned to `x` is never read
  --> fake-test-src-base/liveness/liveness-unused.rs:116:9
   |
LL |         x = 0;  //~ ERROR value assigned to `x` is never read
   |
   |
   = help: maybe it is overwritten before being read?

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/liveness/liveness-unused.rs:92:9
   |
LL |         drop(*x as i32); //~ WARNING unreachable statement
   |
note: argument has type `i32`
  --> fake-test-src-base/liveness/liveness-unused.rs:92:14
   |
   |
LL |         drop(*x as i32); //~ WARNING unreachable statement
   = note: `#[warn(drop_copy)]` on by default


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/liveness/liveness-unused.rs:137:5
LL |     drop(a);
   |     ^^^^^^^
   |
note: argument has type `i32`
---


---- [ui] tests/ui/macros/parse-complex-macro-invoc-op.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |     id![drop](1);
   |
note: argument has type `i32`
  --> $DIR/parse-complex-macro-invoc-op.rs:36:15
   |
   |
LL |     id![drop](1);
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args macros/parse-complex-macro-invoc-op.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/parse-complex-macro-invoc-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/parse-complex-macro-invoc-op/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/parse-complex-macro-invoc-op/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/macros/parse-complex-macro-invoc-op.rs:36:5
   |
LL |     id![drop](1);
   |
note: argument has type `i32`
  --> fake-test-src-base/macros/parse-complex-macro-invoc-op.rs:36:15
   |
   |
LL |     id![drop](1);
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted
------------------------------------------
---
29    |
30    = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`
31 
- warning: 3 warnings emitted
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+   --> $DIR/never-assign-dead-code.rs:10:5
+ LL |     drop(x);
+    |     ^^^^^^^
+    |
+ note: argument has type `!`
---
To only update this specific test, also pass `--test-args never_type/never-assign-dead-code.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/never_type/never-assign-dead-code.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-assign-dead-code" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-assign-dead-code/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/never_type/never-assign-dead-code.rs:10:5
   |
   |
LL |     let x: ! = panic!("aah"); //~ WARN unused
   |                ------------- any code following this expression is unreachable
LL |     drop(x); //~ WARN unreachable
   |     ^^^^^^^^ unreachable statement
note: the lint level is defined here
  --> fake-test-src-base/never_type/never-assign-dead-code.rs:6:9
   |
LL | #![warn(unused)]
LL | #![warn(unused)]
   |         ^^^^^^
   = note: `#[warn(unreachable_code)]` implied by `#[warn(unused)]`

warning: unreachable call
  --> fake-test-src-base/never_type/never-assign-dead-code.rs:10:5
   |
LL |     drop(x); //~ WARN unreachable
   |     ^^^^ - any code following this expression is unreachable
   |     unreachable call

warning: unused variable: `x`
  --> fake-test-src-base/never_type/never-assign-dead-code.rs:9:9
  --> fake-test-src-base/never_type/never-assign-dead-code.rs:9:9
   |
LL |     let x: ! = panic!("aah"); //~ WARN unused
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/never_type/never-assign-dead-code.rs:10:5
   |
LL |     drop(x); //~ WARN unreachable
   |
note: argument has type `!`
  --> fake-test-src-base/never_type/never-assign-dead-code.rs:10:10
   |
   |
LL |     drop(x); //~ WARN unreachable
   = note: `#[warn(drop_copy)]` on by default

warning: 4 warnings emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/nll/relate_tys/hr-fn-aba-as-aaa.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/hr-fn-aba-as-aaa.rs:14:5
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `for<'a> fn(&'a u32, &'a u32) -> &'a u32`
  --> $DIR/hr-fn-aba-as-aaa.rs:14:10
LL |     drop(a);
   |          ^
   = note: `#[warn(drop_copy)]` on by default


warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/hr-fn-aba-as-aaa/hr-fn-aba-as-aaa.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/relate_tys/hr-fn-aba-as-aaa.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/relate_tys/hr-fn-aba-as-aaa.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/hr-fn-aba-as-aaa" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/hr-fn-aba-as-aaa/auxiliary" "-Zno-leak-check"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/nll/relate_tys/hr-fn-aba-as-aaa.rs:14:5
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `for<'a> fn(&'a u32, &'a u32) -> &'a u32`
  --> fake-test-src-base/nll/relate_tys/hr-fn-aba-as-aaa.rs:14:10
LL |     drop(a);
   |          ^
   = note: `#[warn(drop_copy)]` on by default


warning: 1 warning emitted
------------------------------------------


---- [ui] tests/ui/nll/ty-outlives/projection-body.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `&()`
  --> $DIR/projection-body.rs:17:10
   |
LL |     drop(x);
   |          ^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-body.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/ty-outlives/projection-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-body" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-body/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/nll/ty-outlives/projection-body.rs:17:5
LL |     drop(x);
   |     ^^^^^^^
   |
note: argument has type `&()`
note: argument has type `&()`
  --> fake-test-src-base/nll/ty-outlives/projection-body.rs:17:10
LL |     drop(x);
   |          ^
   |          ^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/or-patterns/or-patterns-default-binding-modes.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:25:36
   |
LL |         Ok(mut x) | &Err(mut x) => drop::<u8>(x),
   |
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:25:47
   |
   |
LL |         Ok(mut x) | &Err(mut x) => drop::<u8>(x),
   = note: `#[warn(drop_copy)]` on by default


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:28:30
   |
LL |         &(Ok(x) | Err(x)) => drop::<u8>(x),
   |
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:28:41
   |
   |
LL |         &(Ok(x) | Err(x)) => drop::<u8>(x),


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:31:27
   |
LL |         Ok(x) | Err(x) => drop::<&u8>(x),
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:31:39
   |
   |
LL |         Ok(x) | Err(x) => drop::<&u8>(x),
   |                                       ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:34:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:34:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:37:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:37:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:40:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:40:16
   |
LL |     drop::<u8>(x);
   |                ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:42:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:42:16
   |
LL |     drop::<u8>(x);
   |                ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:44:5
   |
LL |     drop::<&u8>(x);
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:44:17
   |
   |
LL |     drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:46:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:46:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:49:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:49:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:52:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:52:21
   |
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:55:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:55:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:58:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:58:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:61:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:61:21
   |
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:70:42
   |
LL |         Wrap(Ok(mut x) | &Err(mut x)) => drop::<u8>(x),
   |
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:70:53
   |
   |
LL |         Wrap(Ok(mut x) | &Err(mut x)) => drop::<u8>(x),


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:73:36
   |
LL |         Wrap(&(Ok(x) | Err(x))) => drop::<u8>(x),
   |
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:73:47
   |
   |
LL |         Wrap(&(Ok(x) | Err(x))) => drop::<u8>(x),


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:76:33
   |
LL |         Wrap(Ok(x) | Err(x)) => drop::<&u8>(x),
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:76:45
   |
   |
LL |         Wrap(Ok(x) | Err(x)) => drop::<&u8>(x),


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:79:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:79:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:82:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:82:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:85:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:85:21
   |
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:88:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:88:16
   |
LL |     drop::<u8>(x);
   |                ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:90:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:90:16
   |
LL |     drop::<u8>(x);
   |                ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:92:5
   |
LL |     drop::<&u8>(x);
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:92:17
   |
   |
LL |     drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:94:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:94:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:97:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:97:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:100:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:100:21
   |
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:103:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:103:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:106:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:106:20
   |
LL |         drop::<u8>(x);
   |                    ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> $DIR/or-patterns-default-binding-modes.rs:109:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
  --> $DIR/or-patterns-default-binding-modes.rs:109:21
   |
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:124:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:124:16
   |
LL |     drop::<u8>(x);
   |                ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> $DIR/or-patterns-default-binding-modes.rs:129:46
   |
LL |         | &Tri::C(Ok(mut x) | Err(mut x)) => drop::<u8>(x),
   |
note: argument has type `u8`
  --> $DIR/or-patterns-default-binding-modes.rs:129:57
   |
   |
LL |         | &Tri::C(Ok(mut x) | Err(mut x)) => drop::<u8>(x),

warning: 31 warnings emitted


---
To only update this specific test, also pass `--test-args or-patterns/or-patterns-default-binding-modes.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/or-patterns/or-patterns-default-binding-modes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-default-binding-modes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-default-binding-modes/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:25:36
   |
LL |         Ok(mut x) | &Err(mut x) => drop::<u8>(x),
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:25:47
   |
LL |         Ok(mut x) | &Err(mut x) => drop::<u8>(x),
   = note: `#[warn(drop_copy)]` on by default


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:28:30
   |
LL |         &(Ok(x) | Err(x)) => drop::<u8>(x),
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:28:41
   |
LL |         &(Ok(x) | Err(x)) => drop::<u8>(x),


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:31:27
   |
LL |         Ok(x) | Err(x) => drop::<&u8>(x),
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:31:39
   |
LL |         Ok(x) | Err(x) => drop::<&u8>(x),
   |                                       ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:34:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:34:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:37:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:37:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:40:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:40:16
LL |     drop::<u8>(x);
   |                ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:42:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:42:16
LL |     drop::<u8>(x);
   |                ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:44:5
   |
LL |     drop::<&u8>(x);
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:44:17
   |
LL |     drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:46:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:46:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:49:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:49:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:52:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:52:21
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:55:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:55:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:58:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:58:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:61:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:61:21
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:70:42
   |
LL |         Wrap(Ok(mut x) | &Err(mut x)) => drop::<u8>(x),
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:70:53
   |
LL |         Wrap(Ok(mut x) | &Err(mut x)) => drop::<u8>(x),


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:73:36
   |
LL |         Wrap(&(Ok(x) | Err(x))) => drop::<u8>(x),
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:73:47
   |
LL |         Wrap(&(Ok(x) | Err(x))) => drop::<u8>(x),


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:76:33
   |
LL |         Wrap(Ok(x) | Err(x)) => drop::<&u8>(x),
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:76:45
   |
LL |         Wrap(Ok(x) | Err(x)) => drop::<&u8>(x),


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:79:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:79:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:82:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:82:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:85:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:85:21
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:88:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:88:16
LL |     drop::<u8>(x);
   |                ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:90:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:90:16
LL |     drop::<u8>(x);
   |                ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:92:5
   |
LL |     drop::<&u8>(x);
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:92:17
   |
LL |     drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:94:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:94:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:97:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:97:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:100:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:100:21
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:103:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:103:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:106:9
LL |         drop::<u8>(x);
   |         ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:106:20
LL |         drop::<u8>(x);
   |                    ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:109:9
   |
LL |         drop::<&u8>(x);
   |
note: argument has type `&u8`
note: argument has type `&u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:109:21
   |
LL |         drop::<&u8>(x);


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:124:5
LL |     drop::<u8>(x);
   |     ^^^^^^^^^^^^^
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:124:16
LL |     drop::<u8>(x);
   |                ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:129:46
   |
LL |         | &Tri::C(Ok(mut x) | Err(mut x)) => drop::<u8>(x),
   |
note: argument has type `u8`
note: argument has type `u8`
  --> fake-test-src-base/or-patterns/or-patterns-default-binding-modes.rs:129:57
   |
LL |         | &Tri::C(Ok(mut x) | Err(mut x)) => drop::<u8>(x),

warning: 31 warnings emitted
------------------------------------------



---- [ui] tests/ui/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(b);
   |     ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-at-and-box-pass.rs:18:10
   |
LL |     drop(b);
   |          ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(b);
   |     ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-at-and-box-pass.rs:19:10
   |
LL |     drop(b);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `&Box<C>`
   |
LL |     drop(a);
   |          ^
   |          ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(b);
   |     ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-at-and-box-pass.rs:23:10
   |
LL |     drop(b);
   |          ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(b);
   |     ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-at-and-box-pass.rs:24:10
   |
LL |     drop(b);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `&Box<C>`
   |
LL |     drop(a);
   |          ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |         drop(b);
   |         ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-at-and-box-pass.rs:28:14
   |
LL |         drop(b);
   |              ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |         drop(b);
   |         ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-at-and-box-pass.rs:29:14
   |
LL |         drop(b);
   |              ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |         drop(a);
   |         ^^^^^^^
   |
   |
note: argument has type `&Box<C>`
   |
LL |         drop(a);
   |              ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(b);
---
To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-at-and-box-pass" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-at-and-box-pass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:18:5
LL |     drop(b);
   |     ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:18:10
   |
LL |     drop(b);
   |          ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:19:5
LL |     drop(b);
   |     ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:19:10
   |
LL |     drop(b);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:20:5
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `&Box<C>`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:20:10
LL |     drop(a);
   |          ^
   |          ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:23:5
LL |     drop(b);
   |     ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:23:10
   |
LL |     drop(b);
   |          ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:24:5
LL |     drop(b);
   |     ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:24:10
   |
LL |     drop(b);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:25:5
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `&Box<C>`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:25:10
LL |     drop(a);
   |          ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:28:9
LL |         drop(b);
   |         ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:28:14
   |
LL |         drop(b);
   |              ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:29:9
LL |         drop(b);
   |         ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:29:14
   |
LL |         drop(b);
   |              ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:30:9
LL |         drop(a);
   |         ^^^^^^^
   |
   |
note: argument has type `&Box<C>`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:30:14
LL |         drop(a);
   |              ^


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:34:13
LL |             drop(b);
   |             ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:34:18
   |
LL |             drop(b);
   |                  ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:35:13
LL |             drop(b);
   |             ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:35:18
   |
LL |             drop(b);
   |                  ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:36:13
LL |             drop(a);
   |             ^^^^^^^
   |
   |
note: argument has type `&Box<C>`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:36:18
LL |             drop(a);
   |                  ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:41:5
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `&Box<NC>`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:41:10
LL |     drop(a);
   |          ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:42:5
LL |     drop(b);
   |     ^^^^^^^
   |
note: argument has type `&NC`
note: argument has type `&NC`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:42:10
   |
LL |     drop(b);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:45:9
LL |         drop(a);
   |         ^^^^^^^
   |
   |
note: argument has type `&Box<NC>`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:45:14
LL |         drop(a);
   |              ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:46:9
LL |         drop(b)
   |         ^^^^^^^
   |
note: argument has type `&NC`
note: argument has type `&NC`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:46:14
   |
LL |         drop(b)
   |              ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:51:13
LL |             drop(a);
   |             ^^^^^^^
   |
   |
note: argument has type `&Box<NC>`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:51:18
LL |             drop(a);
   |                  ^


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-at-and-box-pass.rs:52:13
LL |             drop(b);
   |             ^^^^^^^
   |
note: argument has type `&NC`
---


---- [ui] tests/ui/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(a);
   |             ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-by-copy-bindings-in-at.rs:31:18
   |
LL |             drop(a);
   |                  ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(a);
   |             ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-by-copy-bindings-in-at.rs:32:18
   |
LL |             drop(a);
   |                  ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(a);
   |             ^^^^^^^
   |
   |
note: argument has type `C`
  --> $DIR/borrowck-pat-by-copy-bindings-in-at.rs:38:18
   |
LL |             drop(a);
   |                  ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(a);
   |             ^^^^^^^
   |
---
To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs:31:13
LL |             drop(a);
   |             ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs:31:18
   |
LL |             drop(a);
   |                  ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs:32:13
LL |             drop(a);
   |             ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs:32:18
   |
LL |             drop(a);
   |                  ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs:38:13
LL |             drop(a);
   |             ^^^^^^^
   |
note: argument has type `C`
note: argument has type `C`
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs:38:18
   |
LL |             drop(a);
   |                  ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/pattern/bindings-after-at/borrowck-pat-by-copy-bindings-in-at.rs:39:13
LL |             drop(a);
   |             ^^^^^^^
   |
note: argument has type `C`
---


---- [ui] tests/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern-pass.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(_x2);
   |     ^^^^^^^^^
   |
   |
note: argument has type `&U`
  --> $DIR/borrowck-move-ref-pattern-pass.rs:26:10
   |
LL |     drop(_x2);
   |          ^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args pattern/move-ref-patterns/borrowck-move-ref-pattern-pass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern-pass" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern-pass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/move-ref-patterns/borrowck-move-ref-pattern-pass.rs:26:5
LL |     drop(_x2);
   |     ^^^^^^^^^
   |
note: argument has type `&U`
note: argument has type `&U`
  --> fake-test-src-base/pattern/move-ref-patterns/borrowck-move-ref-pattern-pass.rs:26:10
   |
LL |     drop(_x2);
   |          ^^^
   = note: `#[warn(drop_ref)]` on by default
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |         drop::<&U>(_x0);
   |
note: argument has type `&U`
  --> $DIR/move-ref-patterns-closure-captures-pass.rs:12:20
   |
   |
LL |         drop::<&U>(_x0);
   |                    ^^^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |         drop::<&mut U>(_x2);
   |
note: argument has type `&mut U`
  --> $DIR/move-ref-patterns-closure-captures-pass.rs:14:24
   |
   |
LL |         drop::<&mut U>(_x2);


warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |         drop::<&U>(_x0);
   |
note: argument has type `&U`
  --> $DIR/move-ref-patterns-closure-captures-pass.rs:19:20
   |
   |
LL |         drop::<&U>(_x0);


warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |         drop::<&mut U>(_x2);
   |
note: argument has type `&mut U`
  --> $DIR/move-ref-patterns-closure-captures-pass.rs:20:24
   |
   |
LL |         drop::<&mut U>(_x2);


warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
   |
LL |         drop::<&U>(_x0);
   |
note: argument has type `&U`
  --> $DIR/move-ref-patterns-closure-captures-pass.rs:25:20
   |
   |
LL |         drop::<&U>(_x0);

warning: 5 warnings emitted


---
To only update this specific test, also pass `--test-args pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:12:9
   |
LL |         drop::<&U>(_x0);
   |
note: argument has type `&U`
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:12:20
   |
   |
LL |         drop::<&U>(_x0);
   |                    ^^^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:14:9
   |
LL |         drop::<&mut U>(_x2);
   |
note: argument has type `&mut U`
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:14:24
   |
   |
LL |         drop::<&mut U>(_x2);


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:19:9
   |
LL |         drop::<&U>(_x0);
   |
note: argument has type `&U`
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:19:20
   |
   |
LL |         drop::<&U>(_x0);


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:20:9
   |
LL |         drop::<&mut U>(_x2);
   |
note: argument has type `&mut U`
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:20:24
   |
   |
LL |         drop::<&mut U>(_x2);


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:25:9
   |
LL |         drop::<&U>(_x0);
   |
note: argument has type `&U`
  --> fake-test-src-base/pattern/move-ref-patterns/move-ref-patterns-closure-captures-pass.rs:25:20
   |
   |
LL |         drop::<&U>(_x0);

warning: 5 warnings emitted
------------------------------------------



---- [ui] tests/ui/print_type_sizes/generator_discr_placement.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(w);
   |             ^^^^^^^
   |
   |
note: argument has type `i32`
  --> $DIR/generator_discr_placement.rs:15:18
   |
LL |             drop(w);
   |                  ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |             drop(z);
   |             ^^^^^^^
   |
---
To only update this specific test, also pass `--test-args print_type_sizes/generator_discr_placement.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/print_type_sizes/generator_discr_placement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/generator_discr_placement" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/generator_discr_placement/auxiliary" "-Z" "print-type-sizes" "--crate-type" "lib"
--- stdout -------------------------------
print-type-size type: `[generator@fake-test-src-base/print_type_sizes/generator_discr_placement.rs:11:13: 11:15]`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Suspend0`: 7 bytes
print-type-size         padding: 3 bytes
print-type-size         local `.w`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Suspend1`: 7 bytes
print-type-size         padding: 3 bytes
print-type-size         local `.z`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
--- stderr -------------------------------
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/print_type_sizes/generator_discr_placement.rs:15:13
LL |             drop(w);
   |             ^^^^^^^
   |
note: argument has type `i32`
note: argument has type `i32`
  --> fake-test-src-base/print_type_sizes/generator_discr_placement.rs:15:18
   |
LL |             drop(w);
   |                  ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/print_type_sizes/generator_discr_placement.rs:20:13
LL |             drop(z);
   |             ^^^^^^^
   |
note: argument has type `i32`
---


---- [ui] tests/ui/print_type_sizes/async.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(arg);
   |     ^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args print_type_sizes/async.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/print_type_sizes/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/async/auxiliary" "-Z" "print-type-sizes" "--crate-type" "lib" "--edition=2021"
--- stdout -------------------------------
print-type-size type: `[async fn body@fake-test-src-base/print_type_sizes/async.rs:8:36: 11:2]`: 16386 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 8192 bytes
print-type-size         upvar `.arg`: 8192 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size     variant `Suspend0`: 16385 bytes
print-type-size         upvar `.arg`: 8192 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size         local `.arg`: 8192 bytes
print-type-size         local `.__awaitee`: 1 bytes
print-type-size     variant `Returned`: 8192 bytes
print-type-size         upvar `.arg`: 8192 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size     variant `Panicked`: 8192 bytes
print-type-size         upvar `.arg`: 8192 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size type: `std::mem::ManuallyDrop<[u8; 8192]>`: 8192 bytes, alignment: 1 bytes
print-type-size     field `.value`: 8192 bytes
print-type-size type: `std::mem::MaybeUninit<[u8; 8192]>`: 8192 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 8192 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 8192 bytes
print-type-size type: `[async fn body@fake-test-src-base/print_type_sizes/async.rs:6:17: 6:19]`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
print-type-size type: `std::mem::ManuallyDrop<[async fn body@fake-test-src-base/print_type_sizes/async.rs:6:17: 6:19]>`: 1 bytes, alignment: 1 bytes
print-type-size     field `.value`: 1 bytes
print-type-size type: `std::mem::MaybeUninit<[async fn body@fake-test-src-base/print_type_sizes/async.rs:6:17: 6:19]>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 1 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 1 bytes
print-type-size type: `std::task::Poll<()>`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Ready`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Pending`: 0 bytes
--- stderr -------------------------------
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/print_type_sizes/async.rs:10:5
LL |     drop(arg);
   |     ^^^^^^^^^
   |
note: argument has type `[u8; 8192]`
---


---- [ui] tests/ui/regions/type-param-outlives-reempty-issue-74429.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |         drop(r);
   |         ^^^^^^^
   |
   |
note: argument has type `Invariant<&T>`
   |
LL |         drop(r);
   |              ^
   = note: `#[warn(drop_copy)]` on by default
---
To only update this specific test, also pass `--test-args regions/type-param-outlives-reempty-issue-74429.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/regions/type-param-outlives-reempty-issue-74429.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/type-param-outlives-reempty-issue-74429" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/type-param-outlives-reempty-issue-74429/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/regions/type-param-outlives-reempty-issue-74429.rs:31:9
LL |         drop(r);
   |         ^^^^^^^
   |
   |
note: argument has type `Invariant<&T>`
  --> fake-test-src-base/regions/type-param-outlives-reempty-issue-74429.rs:31:14
LL |         drop(r);
   |              ^
   = note: `#[warn(drop_copy)]` on by default


warning: 1 warning emitted
------------------------------------------


---- [ui] tests/ui/regions/type-param-outlives-reempty-issue-74429-2.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |         drop(y);
   |         ^^^^^^^
   |
   |
note: argument has type `ArrayBase<ViewRepr<&T>>`
   |
LL |         drop(y);
   |              ^
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/type-param-outlives-reempty-issue-74429-2/type-param-outlives-reempty-issue-74429-2.stderr
To only update this specific test, also pass `--test-args regions/type-param-outlives-reempty-issue-74429-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/regions/type-param-outlives-reempty-issue-74429-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/type-param-outlives-reempty-issue-74429-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/type-param-outlives-reempty-issue-74429-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/regions/type-param-outlives-reempty-issue-74429-2.rs:62:9
LL |         drop(y);
   |         ^^^^^^^
   |
   |
note: argument has type `ArrayBase<ViewRepr<&T>>`
  --> fake-test-src-base/regions/type-param-outlives-reempty-issue-74429-2.rs:62:14
LL |         drop(y);
   |              ^
   = note: `#[warn(drop_copy)]` on by default


warning: 1 warning emitted
------------------------------------------


---- [ui] tests/ui/rfc-2008-non-exhaustive/borrowck-exhaustive.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(y);
   |     ^^^^^^^
   |
   |
note: argument has type `&mut ExhaustiveMonovariant`
   |
LL |     drop(y);
   |          ^
   |          ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(y);
   |     ^^^^^^^
   |
   |
note: argument has type `&mut Local`
  --> $DIR/borrowck-exhaustive.rs:34:10
   |
LL |     drop(y);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
   |
LL |     drop(y);
   |     ^^^^^^^
   |
---
To only update this specific test, also pass `--test-args rfc-2008-non-exhaustive/borrowck-exhaustive.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfc-2008-non-exhaustive/borrowck-exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/borrowck-exhaustive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/borrowck-exhaustive/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/rfc-2008-non-exhaustive/borrowck-exhaustive.rs:27:5
LL |     drop(y);
   |     ^^^^^^^
   |
   |
note: argument has type `&mut ExhaustiveMonovariant`
  --> fake-test-src-base/rfc-2008-non-exhaustive/borrowck-exhaustive.rs:27:10
LL |     drop(y);
   |          ^
   |          ^
   = note: `#[warn(drop_ref)]` on by default

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/rfc-2008-non-exhaustive/borrowck-exhaustive.rs:34:5
LL |     drop(y);
   |     ^^^^^^^
   |
note: argument has type `&mut Local`
note: argument has type `&mut Local`
  --> fake-test-src-base/rfc-2008-non-exhaustive/borrowck-exhaustive.rs:34:10
   |
LL |     drop(y);
   |          ^

warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/rfc-2008-non-exhaustive/borrowck-exhaustive.rs:41:5
LL |     drop(y);
   |     ^^^^^^^
   |
note: argument has type `&mut LocalNonExhaustive`
---


---- [ui] tests/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(a);
   |     ^^^^^^^
   |
   |
note: argument has type `Unit`
  --> $DIR/dbg-macro-expected-behavior.rs:23:10
   |
LL |     drop(a);
   |          ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(b);
   |     ^^^^^^^
   |
---
To only update this specific test, also pass `--test-args rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:23:5
LL |     drop(a);
   |     ^^^^^^^
   |
note: argument has type `Unit`
note: argument has type `Unit`
  --> fake-test-src-base/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:23:10
   |
LL |     drop(a);
   |          ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:33:5
LL |     drop(b);
   |     ^^^^^^^
   |
note: argument has type `Point<u8>`
---
33 LL |     pub use core;
34    |         ~~~
35 
- warning: 3 warnings emitted
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+    |
+ LL |     another_name::mem::drop(3);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: argument has type `i32`
+   --> $DIR/remove-extern-crate.rs:24:29
+    |
+ LL |     another_name::mem::drop(3);
+    |                             ^
+    = note: `#[warn(drop_copy)]` on by default
+ 
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+    |
+ LL |         core::mem::drop(4);
+    |         ^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: argument has type `i32`
+   --> $DIR/remove-extern-crate.rs:37:25
+    |
+ LL |         core::mem::drop(4);
+    |                         ^
+ 
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+    |
+ LL |         core::mem::drop(4);
+    |         ^^^^^^^^^^^^^^^^^^
+    |
---
To only update this specific test, also pass `--test-args rust-2018/remove-extern-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rust-2018/remove-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/auxiliary" "--edition=2018" "--extern" "remove_extern_crate"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:9:1
   |
   |
LL | extern crate core; //~ WARNING unused extern crate
   |
note: the lint level is defined here
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:7:9
   |
   |
LL | #![warn(rust_2018_idioms)]
   |         ^^^^^^^^^^^^^^^^
   = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`

warning: `extern crate` is not idiomatic in the new edition
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:33:5
   |
LL |     extern crate core; //~ WARNING `extern crate` is not idiomatic
   |
help: convert it to a `use`
   |
   |
LL |     use core; //~ WARNING `extern crate` is not idiomatic

warning: `extern crate` is not idiomatic in the new edition
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:43:5
   |
   |
LL |     pub extern crate core; //~ WARNING `extern crate` is not idiomatic
   |
help: convert it to a `use`
   |
   |
LL |     pub use core; //~ WARNING `extern crate` is not idiomatic


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:24:5
LL |     another_name::mem::drop(3);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: argument has type `i32`
note: argument has type `i32`
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:24:29
   |
LL |     another_name::mem::drop(3);
   |                             ^
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:37:9
LL |         core::mem::drop(4);
   |         ^^^^^^^^^^^^^^^^^^
   |
note: argument has type `i32`
note: argument has type `i32`
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:37:25
   |
LL |         core::mem::drop(4);
   |                         ^

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/rust-2018/remove-extern-crate.rs:46:9
LL |         core::mem::drop(4);
   |         ^^^^^^^^^^^^^^^^^^
   |
note: argument has type `i32`
---


---- [ui] tests/ui/statics/issue-91050-1.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
   |
LL |         drop(unsafe { (GLOBAL1, GLOBAL2) });
   |
note: argument has type `(u8, u8)`
  --> $DIR/issue-91050-1.rs:27:14
   |
   |
LL |         drop(unsafe { (GLOBAL1, GLOBAL2) });
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args statics/issue-91050-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/statics/issue-91050-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-91050-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-91050-1/auxiliary" "--crate-type=rlib" "--emit=llvm-ir" "-Cno-prepopulate-passes"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/statics/issue-91050-1.rs:27:9
   |
LL |         drop(unsafe { (GLOBAL1, GLOBAL2) });
   |
note: argument has type `(u8, u8)`
  --> fake-test-src-base/statics/issue-91050-1.rs:27:14
   |
   |
LL |         drop(unsafe { (GLOBAL1, GLOBAL2) });
   = note: `#[warn(drop_copy)]` on by default

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/traits/copy-guessing.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |         drop(e);
   |         ^^^^^^^
   |
   |
note: argument has type `S<'_, Option<U>>`
   |
LL |         drop(e);
   |              ^
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |         drop(e);
   |         ^^^^^^^
   |
   |
note: argument has type `S<'_, Option<U>>`
   |
LL |         drop(e);
   |              ^

---
To only update this specific test, also pass `--test-args traits/copy-guessing.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/copy-guessing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/copy-guessing/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/copy-guessing/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/traits/copy-guessing.rs:23:9
LL |         drop(e);
   |         ^^^^^^^
   |
   |
note: argument has type `S<'_, Option<U>>`
  --> fake-test-src-base/traits/copy-guessing.rs:23:14
LL |         drop(e);
   |              ^
   = note: `#[warn(drop_copy)]` on by default


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/traits/copy-guessing.rs:35:9
LL |         drop(e);
   |         ^^^^^^^
   |
   |
note: argument has type `S<'_, Option<U>>`
  --> fake-test-src-base/traits/copy-guessing.rs:35:14
LL |         drop(e);
   |              ^

warning: 2 warnings emitted
warning: 2 warnings emitted
------------------------------------------


---- [ui] tests/ui/traits/impl-evaluation-order.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `G<(), ()>`
   |
LL |     drop(x);
   |          ^
   = note: `#[warn(drop_copy)]` on by default
   = note: `#[warn(drop_copy)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `G<(), ()>`
   |
LL |     drop(x);
   |          ^

---
To only update this specific test, also pass `--test-args traits/impl-evaluation-order.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/impl-evaluation-order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-evaluation-order" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-evaluation-order/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/traits/impl-evaluation-order.rs:37:5
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `G<(), ()>`
  --> fake-test-src-base/traits/impl-evaluation-order.rs:37:10
LL |     drop(x);
   |          ^
   = note: `#[warn(drop_copy)]` on by default


warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/traits/impl-evaluation-order.rs:38:5
LL |     drop(x);
   |     ^^^^^^^
   |
   |
note: argument has type `G<(), ()>`
  --> fake-test-src-base/traits/impl-evaluation-order.rs:38:10
LL |     drop(x);
   |          ^

warning: 2 warnings emitted
warning: 2 warnings emitted
------------------------------------------


---- [ui] tests/ui/traits/new-solver/temporary-ambiguity.rs stdout ----
normalized stderr:
warning: calls to `std::mem::drop` with a value that implements `Copy`.
   |
LL |     drop(x);
   |     ^^^^^^^
   |
---
To only update this specific test, also pass `--test-args traits/new-solver/temporary-ambiguity.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/temporary-ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/temporary-ambiguity" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/temporary-ambiguity/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/traits/new-solver/temporary-ambiguity.rs:21:5
LL |     drop(x);
   |     ^^^^^^^
   |
note: argument has type `i32`
---
---- [ui] tests/ui/trivial-bounds/trivial-bounds-inconsistent-copy.rs stdout ----
diff of stderr:

6    |
7    = note: `#[warn(trivial_bounds)]` on by default
8 
+ warning: calls to `std::mem::drop` with a value that implements `Copy`.
+    |
+ LL |     drop(t);
+    |     ^^^^^^^
+    |
---
9 warning: trait bound String: Copy does not depend on any type or lifetime parameters
10   --> $DIR/trivial-bounds-inconsistent-copy.rs:12:56
11    |

24 LL | fn copy_mut<'a>(t: &&'a mut i32) -> &'a mut i32 where for<'b> &'b mut i32: Copy {
26 
- warning: 4 warnings emitted
- warning: 4 warnings emitted
+ warning: calls to `std::mem::drop` with a reference instead of an owned value
+    |
+ LL |     drop(x);
+    |     ^^^^^^^
+    |
+    |
+ note: argument has type `&mut i32`
+   --> $DIR/trivial-bounds-inconsistent-copy.rs:26:10
+    |
+ LL |     drop(x);
+    |          ^
+    = note: `#[warn(drop_ref)]` on by default
+ warning: 6 warnings emitted
28 
29 

---
To only update this specific test, also pass `--test-args trivial-bounds/trivial-bounds-inconsistent-copy.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/trivial-bounds/trivial-bounds-inconsistent-copy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-copy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-copy/auxiliary"
stdout: none
--- stderr -------------------------------
warning: trait bound String: Copy does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/trivial-bounds-inconsistent-copy.rs:5:51
   |
LL | fn copy_string(t: String) -> String where String: Copy { //~ WARNING trivial_bounds
   |
   |
   = note: `#[warn(trivial_bounds)]` on by default

warning: calls to `std::mem::drop` with a value that implements `Copy`.
  --> fake-test-src-base/trivial-bounds/trivial-bounds-inconsistent-copy.rs:8:5
LL |     drop(t);
   |     ^^^^^^^
   |
note: argument has type `String`
---

warning: trait bound String: Copy does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/trivial-bounds-inconsistent-copy.rs:12:56
   |
LL | fn copy_out_string(t: &String) -> String where String: Copy { //~ WARNING trivial_bounds

warning: trait bound String: Copy does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/trivial-bounds-inconsistent-copy.rs:16:55
   |
   |
LL | fn copy_string_with_param<T>(x: String) where String: Copy { //~ WARNING trivial_bounds


warning: trait bound for<'b> &'b mut i32: Copy does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/trivial-bounds-inconsistent-copy.rs:22:76
   |
LL | fn copy_mut<'a>(t: &&'a mut i32) -> &'a mut i32 where for<'b> &'b mut i32: Copy {


warning: calls to `std::mem::drop` with a reference instead of an owned value
  --> fake-test-src-base/trivial-bounds/trivial-bounds-inconsistent-copy.rs:26:5
LL |     drop(x);
   |     ^^^^^^^
   |
note: argument has type `&mut i32`
note: argument has type `&mut i32`
  --> fake-test-src-base/trivial-bounds/trivial-bounds-inconsistent-copy.rs:26:10
   |
LL |     drop(x);
   |          ^
   = note: `#[warn(drop_ref)]` on by default
warning: 6 warnings emitted
------------------------------------------


