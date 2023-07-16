plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [mir-opt] tests/mir-opt/slice_filter.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/mir-opt/slice_filter.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/slice_filter" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/slice_filter" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/slice_filter/auxiliary"
stdout: none
--- stderr -------------------------------
error: binary operation on reference to `Copy` type `usize`
  |
  |
8 |     input.iter().filter(|(a, b, c, d)| a <= c && d <= b || c <= a && b <= d).count()
  |
  |
  = note: `usize` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
  = note: `#[deny(ref_binop_on_copy_type)]` on by default
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
  |
8 |     input.iter().filter(|(a, b, c, d)| *a <= *c && d <= b || c <= a && b <= d).count()

error: binary operation on reference to `Copy` type `usize`
 --> /checkout/tests/mir-opt/slice_filter.rs:8:52
  |
  |
8 |     input.iter().filter(|(a, b, c, d)| a <= c && d <= b || c <= a && b <= d).count()
  |
  |
  = note: `usize` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
  |
8 |     input.iter().filter(|(a, b, c, d)| a <= c && *d <= *b || c <= a && b <= d).count()

error: binary operation on reference to `Copy` type `usize`
 --> /checkout/tests/mir-opt/slice_filter.rs:8:62
  |
  |
8 |     input.iter().filter(|(a, b, c, d)| a <= c && d <= b || c <= a && b <= d).count()
  |
  |
  = note: `usize` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
  |
8 |     input.iter().filter(|(a, b, c, d)| a <= c && d <= b || *c <= *a && b <= d).count()

error: binary operation on reference to `Copy` type `usize`
 --> /checkout/tests/mir-opt/slice_filter.rs:8:72
  |
  |
8 |     input.iter().filter(|(a, b, c, d)| a <= c && d <= b || c <= a && b <= d).count()
  |
  |
  = note: `usize` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
  |
8 |     input.iter().filter(|(a, b, c, d)| a <= c && d <= b || c <= a && *b <= *d).count()

error: aborting due to 4 previous errors
------------------------------------------

