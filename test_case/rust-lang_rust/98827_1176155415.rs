plain
---- [ui] src/test/ui/extern/not-in-block.rs stdout ----
diff of stderr:

8    |
9 LL | extern fn none_fn(x: bool) -> i32 { <body> }
10    |                                   ~~~~~~~~~~
- help: if you meant to declare an externaly defined function, use an `extern` block
+ help: if you meant to declare an externally defined function, use an `extern` block
12    |
13 LL | extern { fn none_fn(x: bool) -> i32; }
14    | ~~~~~~~~                             +
23    |
23    |
24 LL | extern "C" fn c_fn(x: bool) -> i32 { <body> }
25    |                                    ~~~~~~~~~~
- help: if you meant to declare an externaly defined function, use an `extern` block
+ help: if you meant to declare an externally defined function, use an `extern` block
27    |
28 LL | extern "C" { fn c_fn(x: bool) -> i32; }
29    | ~~~~~~~~~~~~                          +

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/not-in-block/not-in-block.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extern/not-in-block.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/not-in-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/not-in-block" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/not-in-block/auxiliary"
stdout: none
--- stderr -------------------------------
error: free function without a body
   |
   |
LL | extern fn none_fn(x: bool) -> i32;
   |
help: provide a definition for the function
   |
   |
LL | extern fn none_fn(x: bool) -> i32 { <body> }
help: if you meant to declare an externally defined function, use an `extern` block
   |
   |
LL | extern { fn none_fn(x: bool) -> i32; }
   | ~~~~~~~~                             +
error: free function without a body
  --> /checkout/src/test/ui/extern/not-in-block.rs:5:1
   |
   |
LL | extern "C" fn c_fn(x: bool) -> i32;
   |
help: provide a definition for the function
   |
   |
LL | extern "C" fn c_fn(x: bool) -> i32 { <body> }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: if you meant to declare an externally defined function, use an `extern` block
   |
   |
LL | extern "C" { fn c_fn(x: bool) -> i32; }
   | ~~~~~~~~~~~~                          +
error: aborting due to 2 previous errors
------------------------------------------


