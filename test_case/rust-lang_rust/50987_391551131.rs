plain
    100% |████████████████████████████████| 4.2MB 308kB/s 
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a0/70/2c27740f08e477499ce19eefe05dbcae6f19fdc49e9e82ce4768be0643b9/pyasn1-0.4.3-py2.py3-none-any.whl (72kB)
    14% |████▌                           | 10kB 47.6MB/s eta 0:00:01
    28% |█████████                       | 20kB 48.4MB/s eta 0:00:01
    42% |█████████████▌                  | 30kB 53.2MB/s eta 0:00:01
    56% |██████████████████              | 40kB 50.3MB/s eta 0:00:01
---
[00:44:17] 
[00:44:17] running 1456 tests
[00:44:21] ........................................................................................i...........
[00:44:27] ...........................................i........................................................
[00:44:31] ..................F.................................................................................
[00:44:34] ....................................................................................................
[00:44:38] ..............................................F.....................................................
[00:44:46] ....................................................................................................
[00:44:50] ....................................................................................................
[00:44:55] ....................................................................................................
[00:44:55] ....................................................................................................
[00:45:00] ..............F......................................F.....................i........................
[00:45:05] ....................................................i...............................................
[00:45:09] ...................................................F....................ii..........................
[00:45:20] ..............................................................................i.................iiii
[00:45:20] ..............................................................................i.................iiii
[00:45:23] - error[E0618]: expected function, found `empty_struct::XEmpty2`
[00:45:23] -   --> $DIR/empty-struct-unit-expr.rs:28:15
[00:45:23] -    |
[00:45:23] - LL |     let xe2 = XEmpty2(); //~ ERROR expected function, found `empty_struct::XEmpty2`
[00:45:23] -    |               ^^^^^^^^^ not a function
[00:45:23] - 
[00:45:23] - error[E0618]: expected function, found enum variant `XE::XEmpty4`
[00:45:23] -   --> $DIR/empty-struct-unit-expr.rs:29:15
[00:45:23] -    |
[00:45:23] - LL |     let xe4 = XE::XEmpty4();
[00:45:23] -    |               ^^^^^^^^^^^^^ not a function
[00:45:23] - help: `XE::XEmpty4` is a unit variant, you need to write it without the parenthesis
[00:45:23] -    |
[00:45:23] - LL |     let xe4 = XE::XEmpty4;
[00:45:23] - 
[00:45:23] - error: aborting due to 4 previous errors
[00:45:23] + error: aborting due to previous error
[00:45:23] 40 
[00:45:23] 40 
[00:45:23] 41 For more information about this error, try `rustc --explain E0618`.
[00:45:23] 42 
[00:45:23] 
[00:45:23] 
[00:45:23] The actual stderr differed from the expected stderr.
[00:45:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty-struct-unit-expr/empty-struct-unit-expr.stderr
[00:45:23] To update references, rerun the tests and pass the `--bless` flag
[00:45:23] To only update this specific test, also pass `--test-args empty-struct-unit-expr.rs`
[00:45:23] error: 1 errors occurred comparing output.
[00:45:23] status: exit code: 101
[00:45:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test
[00:45:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test
[00:45:23] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:45:23] 
[00:45:23] ------------------------------------------
[00:45:23] 
[00:45:23] thread '[ui] ui/empty-struct-unit-expr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
[00:45:23] thread '[ui] ui/empty-struct-unit-expr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
[00:45:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:23] 
[00:45:23] ---- [ui] ui/error-codes/E0618.rs stdout ----
[00:45:23] diff of stderr:
[00:45:23] 
[00:45:23] - error[E0618]: expected function, found enum variant `X::Entry`
[00:45:23] -   --> $DIR/E0618.rs:16:5
[00:45:23] -    |
[00:45:23] - LL |     Entry,
[00:45:23] -    |     ----- `X::Entry` defined here
[00:45:23] - ...
[00:45:23] - LL |     X::Entry();
[00:45:23] -    |     ^^^^^^^^^^ not a function
[00:45:23] - help: `X::Entry` is a unit variant, you need to write it without the parenthesis
[00:45:23] -    |
[00:45:23] - LL |     X::Entry;
[00:45:23] - 
[00:45:23] - 
[00:45:23] - error[E0618]: expected function, found `i32`
[00:45:23] -   --> $DIR/E0618.rs:19:5
[00:45:23] -    |
[00:45:23] - LL |     let x = 0i32;
[00:45:23] -    |         - `i32` defined here
[00:45:23] - LL |     x();
[00:45:23] -    |     ^^^ not a function
[00:45:23] - error: aborting due to 2 previous errors
[00:45:23] - 
[00:45:23] - For more information about this error, try `rustc --explain E0618`.
[00:45:23] - 
[00:45:23] - 
[00:45:23] 
[00:45:23] 
[00:45:23] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0618/E0618.stderr`: No such file or directory (os error 2)
[00:45: single 2-tuple as argument
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
[00:45:23] -    |               ^^^^^^^ ----------------- takes a single 2-tuple as argument
[00:45:23] -    |               |
[00:45:23] -    |               expected closure that takes 2 distinct arguments
[00:45:23] - help: change the closure to take multiple arguments instead of a single tuple
[00:45:23] -    |
[00:45:23] - LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());
[00:45:23] - 
[00:45:23] - 
[00:45:23] - error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     [1, 2, 3].sort_by(|(tuple, tuple2): (usize, _)| panic!());
[00:45:23] -    |               ^^^^^^^ ----------------------------- takes a single 2-tuple as argument
[00:45:23] -    |               |
[00:45:23] -    |               expected closure that takes 2 distinct arguments
[00:45:23] - help: change the closure to take multiple arguments instead of a single tuple
[00:45:23] -    |
[00:45:23] - LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());
[00:45:23] - 
[00:45:23] - 
[00:45:23] - error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     f(|| panic!());
[00:45:23] -    |     ^ -- takes 0 arguments
[00:45:23] -   --> $DIR/closure-arg-count.rs:35:53
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     let bar = |i, x, y| i;
[00:45:23] -    |               --------- takes 3 distinct arguments
[00:45:23] - LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(bar);
[00:45:23] -    |                                                     ^^^ expected closure that takes a single 2-tuple as argument
[00:45:23] - 
[00:45:23] - error[E0593]: function is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(qux);
[00:45:23] -    |                                                     ^^^ expected function that takes a single 2-tuple as argument
[00:45:23] - ...
[00:45:23] - LL | fn qux(x: usize, y: usize) {}
[00:45:23] -    | -------------------------- takes 2 distinct arguments
[00:45:23] - 
[00:45:23] - error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     let _it = vec![1, 2, 3].into_iter().map(usize::checked_add);
[00:45:23] -    |                                         ^^^ expected function that takes 1 argument
[00:45:23] - 
[00:45:23] - error[E0593]: function is expected to take 0 arguments, but it takes 1 argument
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     call(Foo);
[00:45:23] -    |     ^^^^ expected function that takes 0 arguments
[00:45:23] - ...
[00:45:23] - LL | struct Foo(u8);
[00:45:23] -    | --------------- takes 1 argument
[00:45:23] -    |
[00:45:23] - note: required by `call`
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL | fn call<F, R>(_: F) where F: FnOnce() -> R {}
[00:45:23] - 
[00:45:23] - error: aborting due to 13 previous errors
[00:45:23] + error: aborting due to 2 previous errors
[00:45:23] 135 
[00:45:23] 135 
[00:45:23] 136 For more information about this error, try `rustc --explain E0593`.
[00:45:23] 137 
[00:45:23] 
[00:45:23] 
[00:45:23] The actual stderr differed from the expected stderr.
[00:45:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/closure-arg-count.stderr
[00:45:23] To update references, rerun the tests and pass the `--bless` flag
[00:45:23] To only update this specific test, also pass `--test-args mismatched_types/closure-arg-count.rs`
[00:45:23] error: 1 errors occurred comparing output.
[00:45:23] status: exit code: 101
[00:45:23] status: exit code: 101
[00:45:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-arg-count.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/auxiliary" "-45:23] 165               found type `fn(u8) -> m::n::Z {m::n::Z::Fn}`
[00:45:23] 166 
[00:45:23] - error[E0618]: expected function, found enum variant `Z::Unit`
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |             Unit,
[00:45:23] -    |             ---- `Z::Unit` defined here
[00:45:23] - ...
[00:45:23] - LL |         let _ = Z::Unit();
[00:45:23] -    |                 ^^^^^^^^^ not a function
[00:45:23] - help: `Z::Unit` is a unit variant, you need to write it without the parenthesis
[00:45:23] -    |
[00:45:23] - LL |         let _ = Z::Unit;
[00:45:23] - 
[00:45:23] - error[E0308]: mismatched types
[00:45:23] -   --> $DIR/privacy-enum-ctor.rs:53:16
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |     let _: E = m::E::Fn;
[00:45:23] -    |                ^^^^^^^^ expected enum `m::E`, found fn item
[00:45:23] -    |
[00:45:23] -    = note: expected type `m::E`
[00:45:23] -               found type `fn(u8) -> m::E {m::E::Fn}`
[00:45:23] - 
[00:45:23] - error[E0618]: expected function, found enum variant `m::E::Unit`
[00:45:23] -    |
[00:45:23] -    |
[00:45:23] - LL |         Unit,
[00:45:23] -    |         ---- `m::E::Unit` defined here
[00:45:23] - ...
[00:45:23] - LL |     let _: E = m::E::Unit();
[00:45:23] -    |                ^^^^^^^^^^^^ not a function
[00:45:23] - help: `m::E::Unit` is a unit variant, you need to write it without the parenthesis
[00:45:23] -    |
[00:45:23] - LL |     let _: E = m::E::Unit;
[00:45:23] - 
[00:45:23] - 
[00:45,"spans":[],"children":[],"rendered":null}],"rendered":"error[E0423]: expected value, found enum `Z`\n  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:35:9\n   |\nLL |         Z;\n   |         ^ did you mean `f`?\n   |\n   = note: did you mean to use one of the following variants?\n           - `m::Z::Fn`\n           - `m::Z::Struct`\n           - `m::Z::Unit`\n\n"}
[00:45:23] {"message":"expected value, found struct variant `Z::Struct`","code":{"code":"E0423","explanation":"\nA `struct` variant name was used like a function name.\n\nErroneous code example:\n\n