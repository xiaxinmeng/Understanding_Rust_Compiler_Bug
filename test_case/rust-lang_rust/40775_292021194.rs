

[01:07:19] failures:
[01:07:19] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:329
[01:07:19] 
[01:07:19] ---- [ui] ui/did_you_mean/issue-35675.rs stdout ----
[01:07:19] 	ui: /checkout/src/test/ui/did_you_mean/issue-35675.rs
[01:07:19] normalized stderr:
[01:07:19] error[E0412]: cannot find type `Apple` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:16:29
[01:07:19]    |
[01:07:19] 16 | fn should_return_fruit() -> Apple {
[01:07:19]    |                             ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19] help: there is an enum variant `Fruit::Apple`, did you mean to use `Fruit`?
[01:07:19]   --> $DIR/issue-35675.rs:12:5
[01:07:19]    |
[01:07:19] 12 |     Apple(i64),
[01:07:19]    |     ^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0425]: cannot find function `Apple` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:17:5
[01:07:19]    |
[01:07:19] 17 |     Apple(5)
[01:07:19]    |     ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19]    = help: possible candidate is found in another module, you can import it into scope:
[01:07:19]              `use Fruit::Apple;`
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Fruit::Apple`
[01:07:19]   --> $DIR/issue-35675.rs:20:33
[01:07:19]    |
[01:07:19] 20 | fn should_return_fruit_too() -> Fruit::Apple {
[01:07:19]    |                                 ^^^^^^^^^^^^ not a type
[01:07:19]    |
[01:07:19] help: there is an enum variant `Fruit::Apple`, did you mean to use `Fruit`?
[01:07:19]   --> $DIR/issue-35675.rs:12:5
[01:07:19]    |
[01:07:19] 12 |     Apple(i64),
[01:07:19]    |     ^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0425]: cannot find function `Apple` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:21:5
[01:07:19]    |
[01:07:19] 21 |     Apple(5)
[01:07:19]    |     ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19]    = help: possible candidate is found in another module, you can import it into scope:
[01:07:19]              `use Fruit::Apple;`
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Ok`
[01:07:19]   --> $DIR/issue-35675.rs:24:13
[01:07:19]    |
[01:07:19] 24 | fn foo() -> Ok {
[01:07:19]    |             ^^ not a type
[01:07:19]    |
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Ok`, did you mean to use `std::prelude::v1`?
[01:07:19]    = help: there is an enum variant `std::result::Result::Ok`, did you mean to use `std::result::Result`?
[01:07:19] 
[01:07:19] error[E0412]: cannot find type `Variant3` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:28:13
[01:07:19]    |
[01:07:19] 28 | fn bar() -> Variant3 {
[01:07:19]    |             ^^^^^^^^ not found in this scope
[01:07:19]    |
[01:07:19] help: there is an enum variant `x::Enum::Variant3`, did you mean to use `x::Enum`?
[01:07:19]   --> $DIR/issue-35675.rs:41:9
[01:07:19]    |
[01:07:19] 41 |         Variant3(usize),
[01:07:19]    |         ^^^^^^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Some`
[01:07:19]   --> $DIR/issue-35675.rs:31:13
[01:07:19]    |
[01:07:19] 31 | fn qux() -> Some {
[01:07:19]    |             ^^^^ not a type
[01:07:19]    |
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Option::Some`, did you mean to use `std::prelude::v1::Option`?
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Some`, did you mean to use `std::prelude::v1`?
[01:07:19] 
[01:07:19] error: aborting due to 7 previous errors
[01:07:19] 
[01:07:19] 
[01:07:19] 
[01:07:19] expected stderr:
[01:07:19] error[E0412]: cannot find type `Apple` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:16:29
[01:07:19]    |
[01:07:19] 16 | fn should_return_fruit() -> Apple {
[01:07:19]    |                             ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19] help: there is an enum variant `Fruit::Apple`, did you mean to use `Fruit`?
[01:07:19]   --> $DIR/issue-35675.rs:12:5
[01:07:19]    |
[01:07:19] 12 |     Apple(i64),
[01:07:19]    |     ^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0425]: cannot find function `Apple` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:17:5
[01:07:19]    |
[01:07:19] 17 |     Apple(5)
[01:07:19]    |     ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19]    = help: possible candidate is found in another module, you can import it into scope:
[01:07:19]              `use Fruit::Apple;`
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Fruit::Apple`
[01:07:19]   --> $DIR/issue-35675.rs:20:33
[01:07:19]    |
[01:07:19] 20 | fn should_return_fruit_too() -> Fruit::Apple {
[01:07:19]    |                                 ^^^^^^^^^^^^ not a type
[01:07:19]    |
[01:07:19] help: there is an enum variant `Fruit::Apple`, did you mean to use `Fruit`?
[01:07:19]   --> $DIR/issue-35675.rs:12:5
[01:07:19]    |
[01:07:19] 12 |     Apple(i64),
[01:07:19]    |     ^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0425]: cannot find function `Apple` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:21:5
[01:07:19]    |
[01:07:19] 21 |     Apple(5)
[01:07:19]    |     ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19]    = help: possible candidate is found in another module, you can import it into scope:
[01:07:19]              `use Fruit::Apple;`
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Ok`
[01:07:19]   --> $DIR/issue-35675.rs:24:13
[01:07:19]    |
[01:07:19] 24 | fn foo() -> Ok {
[01:07:19]    |             ^^ not a type
[01:07:19]    |
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Ok`, did you mean to use `std::prelude::v1`?
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Result::Ok`, did you mean to use `std::prelude::v1::Result`?
[01:07:19] 
[01:07:19] error[E0412]: cannot find type `Variant3` in this scope
[01:07:19]   --> $DIR/issue-35675.rs:28:13
[01:07:19]    |
[01:07:19] 28 | fn bar() -> Variant3 {
[01:07:19]    |             ^^^^^^^^ not found in this scope
[01:07:19]    |
[01:07:19] help: there is an enum variant `x::Enum::Variant3`, did you mean to use `x::Enum`?
[01:07:19]   --> $DIR/issue-35675.rs:41:9
[01:07:19]    |
[01:07:19] 41 |         Variant3(usize),
[01:07:19]    |         ^^^^^^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Some`
[01:07:19]   --> $DIR/issue-35675.rs:31:13
[01:07:19]    |
[01:07:19] 31 | fn qux() -> Some {
[01:07:19]    |             ^^^^ not a type
[01:07:19]    |
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Option::Some`, did you mean to use `std::prelude::v1::Option`?
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Some`, did you mean to use `std::prelude::v1`?
[01:07:19] 
[01:07:19] error: aborting due to 7 previous errors
[01:07:19] 
[01:07:19] 
[01:07:19] 
[01:07:19] diff of stderr:
[01:07:19] 
[01:07:19]  49 - |   = help: there is an enum variant `std::prelude::v1::Result::Ok`, did you mean to use `std::prelude::v1::Result`?|
[01:07:19]     + |   = help: there is an enum variant `std::result::Result::Ok`, did you mean to use `std::result::Result`?|
[01:07:19] 
[01:07:19] 
[01:07:19] The actual stderr differed from the expected stderr.
[01:07:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35675.stderr
[01:07:19] To update references, run this command from build directory:
[01:07:19] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'did_you_mean/issue-35675.rs'
[01:07:19] 
[01:07:19] error: 1 errors occurred comparing output.
[01:07:19] status: exit code: 101
[01:07:19] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/ui/did_you_mean/issue-35675.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui --target=x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35675.stage2-x86_64-unknown-linux-gnu.ui.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35675.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:07:19] stdout:
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] error[E0412]: cannot find type `Apple` in this scope
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:16:29
[01:07:19]    |
[01:07:19] 16 | fn should_return_fruit() -> Apple {
[01:07:19]    |                             ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19] help: there is an enum variant `Fruit::Apple`, did you mean to use `Fruit`?
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:12:5
[01:07:19]    |
[01:07:19] 12 |     Apple(i64),
[01:07:19]    |     ^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0425]: cannot find function `Apple` in this scope
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:17:5
[01:07:19]    |
[01:07:19] 17 |     Apple(5)
[01:07:19]    |     ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19]    = help: possible candidate is found in another module, you can import it into scope:
[01:07:19]              `use Fruit::Apple;`
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Fruit::Apple`
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:20:33
[01:07:19]    |
[01:07:19] 20 | fn should_return_fruit_too() -> Fruit::Apple {
[01:07:19]    |                                 ^^^^^^^^^^^^ not a type
[01:07:19]    |
[01:07:19] help: there is an enum variant `Fruit::Apple`, did you mean to use `Fruit`?
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:12:5
[01:07:19]    |
[01:07:19] 12 |     Apple(i64),
[01:07:19]    |     ^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0425]: cannot find function `Apple` in this scope
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:21:5
[01:07:19]    |
[01:07:19] 21 |     Apple(5)
[01:07:19]    |     ^^^^^ not found in this scope
[01:07:19]    |
[01:07:19]    = help: possible candidate is found in another module, you can import it into scope:
[01:07:19]              `use Fruit::Apple;`
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Ok`
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:24:13
[01:07:19]    |
[01:07:19] 24 | fn foo() -> Ok {
[01:07:19]    |             ^^ not a type
[01:07:19]    |
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Ok`, did you mean to use `std::prelude::v1`?
[01:07:19]    = help: there is an enum variant `std::result::Result::Ok`, did you mean to use `std::result::Result`?
[01:07:19] 
[01:07:19] error[E0412]: cannot find type `Variant3` in this scope
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:28:13
[01:07:19]    |
[01:07:19] 28 | fn bar() -> Variant3 {
[01:07:19]    |             ^^^^^^^^ not found in this scope
[01:07:19]    |
[01:07:19] help: there is an enum variant `x::Enum::Variant3`, did you mean to use `x::Enum`?
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:41:9
[01:07:19]    |
[01:07:19] 41 |         Variant3(usize),
[01:07:19]    |         ^^^^^^^^^^^^^^^
[01:07:19] 
[01:07:19] error[E0573]: expected type, found variant `Some`
[01:07:19]   --> /checkout/src/test/ui/did_you_mean/issue-35675.rs:31:13
[01:07:19]    |
[01:07:19] 31 | fn qux() -> Some {
[01:07:19]    |             ^^^^ not a type
[01:07:19]    |
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Option::Some`, did you mean to use `std::prelude::v1::Option`?
[01:07:19]    = help: there is an enum variant `std::prelude::v1::Some`, did you mean to use `std::prelude::v1`?
[01:07:19] 
[01:07:19] error: aborting due to 7 previous errors
[01:07:19] 
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/did_you_mean/issue-35675.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2621
[01:07:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:19] 
[01:07:19] 
[01:07:19] failures:
[01:07:19]     [ui] ui/did_you_mean/issue-35675.rs
[01:07:19] 
[01:07:19] test result: FAILED. 240 passed; 1 failed; 1 ignored; 0 measured
[01:07:19] 
[01:07:19] 
[01:07:19] 
[01:07:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.7/bin/FileCheck" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "3.7.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp" "--android-cross-path" ""
[01:07:19] expected success, got: exit code: 101
[01:07:19] 
[01:07:19] 
[01:07:19] Build completed unsuccessfully in 0:31:54
[01:07:19] make: *** [check] Error 1
[01:07:19] Makefile:54: recipe for target 'check' failed
