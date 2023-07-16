plain
---- [mir-opt] src/test/mir-opt/remove-never-const.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/remove-never-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/remove-never-const" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/remove-never-const" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--emit" "mir,link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/remove-never-const/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/mir-opt/remove-never-const.rs:11:8
   |
   |
11 | struct PrintName<T>(T);
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `no_codegen` is never used
warning: function `no_codegen` is never used
  --> /checkout/src/test/mir-opt/remove-never-const.rs:18:4
   |
18 | fn no_codegen<T>() {

warning: associated constant `VOID` is never used
  --> /checkout/src/test/mir-opt/remove-never-const.rs:14:11
   |
