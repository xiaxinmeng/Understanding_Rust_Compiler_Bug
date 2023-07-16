plain
........................................................................................ 2728/13432
........................................................................................ 2816/13432
........................................................................................ 2904/13432
........................................................................................ 2992/13432
.........................i................................F.F..................i........ 3080/13432
........................................................................................ 3256/13432
.......................................................iiiii............................ 3344/13432
........................................................................................ 3432/13432
........................................................................................ 3520/13432
........................................................................................ 3520/13432
........................................................................................ 3608/13432
.......................................................................................F 3696/13432
........................................................................................ 3784/13432
.........................................................................i..F.......i... 3872/13432
......................................................................i................. 4048/13432
........................................................................................ 4136/13432
...............................i........................................................ 4224/13432
........................................................................................ 4312/13432
---
To only update this specific test, also pass `--test-args crate-loading/missing-std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/missing-std.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/missing-std" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-uefi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/missing-std/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/crate-loading/missing-std.rs:6:1
   |
LL | extern crate core;
   | ^^^^^^^^^^^^^^^^^^ can't find crate
   | ^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = note: the `x86_64-unknown-uefi` target may not be installed
   = help: consider downloading the target with `rustup target add x86_64-unknown-uefi`
   = help: consider building the standard library from source with `cargo build -Zbuild-std`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: requires `sized` lang_item

error: aborting due to 2 previous errors
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/crate-loading/invalid-rlib.rs stdout ----
diff of stderr:

- error[E0786]: found invalid metadata files for crate `foo`
+ error: found invalid metadata files for crate `foo`
3    |
3    |
4 LL | use ::foo;
6    |
6    |
7    = note: failed to mmap file 'auxiliary/libfoo.rlib'
8 
- error[E0786]: found invalid metadata files for crate `foo`
+ error: found invalid metadata files for crate `foo`
11    |
11    |
12 LL | use ::foo;
16 
17 error: aborting due to 2 previous errors
18 
- For more information about this error, try `rustc --explain E0786`.
---
To only update this specific test, also pass `--test-args crate-loading/invalid-rlib.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/invalid-rlib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/invalid-rlib" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "--extern" "foo=/checkout/src/test/ui/crate-loading/auxiliary/libfoo.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/invalid-rlib/auxiliary"
stdout: none
--- stderr -------------------------------
error: found invalid metadata files for crate `foo`
   |
   |
LL | use ::foo; //~ ERROR invalid metadata files for crate `foo`
   |
   |
   = note: failed to mmap file '/checkout/src/test/ui/crate-loading/auxiliary/libfoo.rlib': memory map must have a non-zero length

error: found invalid metadata files for crate `foo`
   |
   |
LL | use ::foo; //~ ERROR invalid metadata files for crate `foo`
   |
   |
   = note: failed to mmap file '/checkout/src/test/ui/crate-loading/auxiliary/libfoo.rlib': memory map must have a non-zero length
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/error-codes/E0463.rs stdout ----
diff of stderr:

- error[E0463]: can't find crate for `cookie_monster`
+ error: can't find crate for `cookie_monster`
2   --> $DIR/E0463.rs:2:11
3    |
4 LL | #![plugin(cookie_monster)]
6 
7 error: aborting due to previous error
8 
- For more information about this error, try `rustc --explain E0463`.
---
To only update this specific test, also pass `--test-args error-codes/E0463.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0463.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0463" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0463/auxiliary"
stdout: none
--- stderr -------------------------------
error: can't find crate for `cookie_monster`
   |
   |
LL | #![plugin(cookie_monster)]

error: aborting due to previous error
------------------------------------------

---
To only update this specific test, also pass `--test-args extern/extern-crate-multiple-missing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-crate-multiple-missing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-crate-multiple-missing" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-crate-multiple-missing/auxiliary"
stdout: none
--- stderr -------------------------------
error: can't find crate for `bar`
   |
   |
LL | extern crate bar; //~ ERROR can't find crate for `bar`

error: can't find crate for `foo`
  --> /checkout/src/test/ui/extern/extern-crate-multiple-missing.rs:3:1
   |
   |
LL | extern crate foo; //~ ERROR can't find crate for `foo`

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/issues/issue-37131.rs stdout ----
diff of stderr:

- error[E0463]: can't find crate for `std`
+ error: can't find crate for `std`
2    |
3    = note: the `thumbv6m-none-eabi` target may not be installed
4    = help: consider downloading the target with `rustup target add thumbv6m-none-eabi`
8 
9 error: aborting due to 2 previous errors
10 
- For more information about this error, try `rustc --explain E0463`.
---
To only update this specific test, also pass `--test-args issues/issue-37131.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37131.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=thumbv6m-none-eabi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131/auxiliary"
stdout: none
--- stderr -------------------------------
   |
   |
   = note: the `thumbv6m-none-eabi` target may not be installed
   = help: consider downloading the target with `rustup target add thumbv6m-none-eabi`
   = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors
------------------------------------------
---

- error[E0463]: can't find crate for `core`
+ error: can't find crate for `core`
2    |
3    = note: the `thumbv7em-none-eabihf` target may not be installed
4    = help: consider downloading the target with `rustup target add thumbv7em-none-eabihf`

5    = help: consider building the standard library from source with `cargo build -Zbuild-std`
- error[E0463]: can't find crate for `compiler_builtins`
+ error: can't find crate for `compiler_builtins`
8 
- error[E0463]: can't find crate for `cortex_m`
---
To only update this specific test, also pass `--test-args issues/issue-49851/compiler-builtins-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-49851/compiler-builtins-error.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49851/compiler-builtins-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv7em-none-eabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49851/compiler-builtins-error/auxiliary"
stdout: none
--- stderr -------------------------------
   |
   |
   = note: the `thumbv7em-none-eabihf` target may not be installed
   = help: consider downloading the target with `rustup target add thumbv7em-none-eabihf`
   = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: can't find crate for `compiler_builtins`

error: can't find crate for `cortex_m`
  --> /checkout/src/test/ui/issues/issue-49851/compiler-builtins-error.rs:10:1
---
To only update this specific test, also pass `--test-args no-link-unknown-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-link-unknown-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-link-unknown-crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-link-unknown-crate/auxiliary"
stdout: none
--- stderr -------------------------------
error: can't find crate for `doesnt_exist`
   |
   |
LL | extern crate doesnt_exist; //~ ERROR can't find crate

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/parser/bad-crate-name.rs stdout ----
diff of stderr:

9 LL | extern crate krate_name_here;
11 
11 
- error[E0463]: can't find crate for `krate_name_here`
+ error: can't find crate for `krate_name_here`
14    |
14    |
15 LL | extern crate krate-name-here;
17 
18 error: aborting due to 2 previous errors
19 
- For more information about this error, try `rustc --explain E0463`.
---
To only update this specific test, also pass `--test-args parser/bad-crate-name.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/bad-crate-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bad-crate-name" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bad-crate-name/auxiliary"
stdout: none
--- stderr -------------------------------
error: crate name using dashes are not valid in `extern crate` statements
   |
   |
LL | extern crate krate-name-here;
   |              ^^^^^^^^^^^^^^^ dash-separated idents are not valid
   |
help: if the original crate name uses dashes you need to use underscores in the code
   |
LL | extern crate krate_name_here;


error: can't find crate for `krate_name_here`
   |
   |
LL | extern crate krate-name-here;

error: aborting due to 2 previous errors
------------------------------------------

---
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/deadlock.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/deadlock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/deadlock" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "foo" "--extern" "bar" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/deadlock/auxiliary"
stdout: none
--- stderr -------------------------------
error: can't find crate for `bar`
   |
   |
LL | use bar::foo; //~ ERROR can't find crate for `bar`

error: can't find crate for `foo`
  --> /checkout/src/test/ui/rust-2018/uniform-paths/deadlock.rs:5:5
   |
   |
LL | use foo::bar; //~ ERROR can't find crate for `foo`


error[E0432]: unresolved imports `bar::foo`, `foo::bar`
   |
   |
LL | use bar::foo; //~ ERROR can't find crate for `bar`
   |     ^^^^^^^^
LL | use foo::bar; //~ ERROR can't find crate for `foo`

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.
---
To only update this specific test, also pass `--test-args use/use-meta-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-meta-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-meta-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-meta-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error: can't find crate for `fake_crate`
   |
LL | extern crate fake_crate as extra;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

