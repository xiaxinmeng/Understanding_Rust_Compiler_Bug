plain
travis_time:end:1c78860c:start=1560585662043220414,finish=1560585664303217672,duration=2259997258
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:56:28] .................................................................................................... 400/5680
[00:56:31] .................................................................................................... 500/5680
[00:56:35] ...................................i................................................................ 600/5680
[00:56:39] .................................................................................................... 700/5680
[00:56:43] ........................................................................................F........... 800/5680
[00:56:53] ......................................i...........i................................................. 1000/5680
[00:56:55] ...................................................................iiiii............................ 1100/5680
[00:56:59] .................................................................................................... 1200/5680
[00:57:02] .................................................................................................... 1300/5680
[00:57:02] .................................................................................................... 1300/5680
[00:57:04] ................................................................................................F.F. 1400/5680
[00:57:07] .....................................F.............................................................. 1500/5680
[00:57:10] ...........................F........................................................................ 1600/5680
[00:57:16] ..i................................................................................................. 1800/5680
[00:57:20] .................................................................................................... 1900/5680
[00:57:23] .................................................................................................... 2000/5680
[00:57:27] .................................................................................................... 2100/5680
[00:57:27] .................................................................................................... 2100/5680
[00:57:30] ...........................................i........................F............................... 2200/5680
[00:57:38] .................................................................................................... 2400/5680
[00:57:42] .................................................................................................... 2500/5680
[00:57:46] .................................................................................................... 2600/5680
[00:57:50] .................................................................................................... 2700/5680
---
[00:58:10] .................................................................................................... 3200/5680
[00:58:14] .................................................................................................... 3300/5680
[00:58:17] ........................................................................................i........... 3400/5680
[00:58:21] .................................................................................................... 3500/5680
[00:58:24] ..............................F...............................ii...i...ii........................... 3600/5680
[00:58:32] .................................................................................................... 3800/5680
[00:58:35] ........................................................................ii.......................... 3900/5680
[00:58:38] .............................................................................................i...... 4000/5680
[00:58:40] .................................................................................................... 4100/5680
---
[00:59:49] failures:
[00:59:49] 
[00:59:49] ---- [ui] ui/consts/const-eval/ice-generic-assoc-const.rs stdout ----
[00:59:49] 
[00:59:49] error: test compilation failed although it shouldn't!
[00:59:49] status: exit code: 1
[00:59:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ice-generic-assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ice-generic-assoc-const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ice-generic-assoc-const/auxiliary" "-A" "unused"
[00:59:49] ------------------------------------------
[00:59:49] 
[00:59:49] ------------------------------------------
[00:59:49] stderr:
[00:59:49] stderr:
[00:59:49] ------------------------------------------
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/consts/const-eval/ice-generic-assoc-const.rs:10:39
[00:59:49]    |
[00:59:49] LL |     const NULL: Self = core::ptr::null<T>();
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] error[E0423]: expected value, found type parameter `T`
[00:59:49]   --> /checkout/src/test/ui/consts/const-eval/ice-generic-assoc-const.rs:10:40
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     const NULL: Self = core::ptr::null<T>();
[00:59:49]    |                                        ^ not a value
[00:59:49] 
[00:59:49] error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     const NULL: Self = core::ptr::null<T>();
[00:59:49]    |                        ---------------^- _
[00:59:49]    |                        |
[00:59:49]    |                        fn() -> *const _ {std::ptr::null::<_>}
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] error[E0308]: mismatched types
[00:59:49]   --> /checkout/src/test/ui/consts/const-eval/ice-generic-assoc-const.rs:10:24
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     const NULL: Self = core::ptr::null<T>();
[00:59:49]    |                        ^^^^^^^^^^^^^^^^^^^^ expected *-ptr, found bool
[00:59:49]    = note: expected type `*const T`
[00:59:49]               found type `bool`
[00:59:49] 
[00:59:49] error: aborting due to 4 previous errors
---
[00:59:49] 
[00:59:49] + error: chained comparison operators require parentheses
[00:59:49] +   --> $DIR/E0605.rs:5:27
[00:59:49] +    |
[00:59:49] + LL |     let v = std::ptr::null<u8>;
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] +    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49] +    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] + error: expected expression, found `;`
[00:59:49] +   --> $DIR/E0605.rs:5:31
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let v = std::ptr::null<u8>;
[00:59:49] + 
[00:59:49] 1 error[E0605]: non-primitive cast: `u8` as `std::vec::Vec<u8>`
[00:59:49] 2   --> $DIR/E0605.rs:3:5
[00:59:49] 3    |
[00:59:49] 3    |
[00:59:49] 
[00:59:49] 6    |
[00:59:49] 7    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 8 
[00:59:49] - error[E0605]: non-primitive cast: `*const u8` as `&u8`
[00:59:49] -   --> $DIR/E0605.rs:6:5
[00:59:49] -    |
[00:59:49] - LL |     v as &u8;
[00:59:49] -    |
[00:59:49] -    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] - 
[00:59:49] - error: aborting due to 2 previous errors
[00:59:49] - error: aborting due to 2 previous errors
[00:59:49] + error: aborting due to 3 previous errors
[00:59:49] 18 
[00:59:49] 19 For more information about this error, try `rustc --explain E0605`.
[00:59:49] 20 
[00:59:49] 
[00:59:49] 
[00:59:49] The actual stderr differed from the expected stderr.
[00:59:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0605/E0605.stderr
[00:59:49] To update references, rerun the tests and pass the `--bless` flag
[00:59:49] To only update this specific test, also pass `--test-args error-codes/E0605.rs`
[00:59:49] error: 1 errors occurred comparing output.
[00:59:49] status: exit code: 1
[00:59:49] status: exit code: 1
[00:59:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0605.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0605" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0605/auxiliary" "-A" "unused"
[00:59:49] ------------------------------------------
[00:59:49] 
[00:59:49] ------------------------------------------
[00:59:49] stderr:
[00:59:49] stderr:
[00:59:49] ------------------------------------------
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/error-codes/E0605.rs:5:27
[00:59:49]    |
[00:59:49] LL |     let v = std::ptr::null<u8>;
[00:59:49]    |                           ^^^^^
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] error: expected expression, found `;`
[00:59:49]   --> /checkout/src/test/ui/error-codes/E0605.rs:5:31
[00:59:49]    |
[00:59:49] LL |     let v = std::ptr::null<u8>;
[00:59:49] LL |     let v = std::ptr::null<u8>;
[00:59:49]    |                               ^ expected expression
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `u8` as `std::vec::Vec<u8>`
[00:59:49]   --> /checkout/src/test/ui/error-codes/E0605.rs:3:5
[00:59:49]    |
[00:59:49] LL |     x as Vec<u8>; //~ ERROR E0605
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] error: aborting due to 3 previous errors
---
[00:59:49] 
[00:59:49] ---- [ui] ui/error-codes/E0607.rs stdout ----
[00:59:49] diff of stderr:
[00:59:49] 
[00:59:49] - error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
[00:59:49] + error: chained comparison operators require parentheses
[00:59:49] +   --> $DIR/E0607.rs:2:28
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] +    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49] +    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] + error[E0423]: expected value, found builtin type `u8`
[00:59:49] +   --> $DIR/E0607.rs:2:29
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |                             ^^ not a value
[00:59:49] + 
[00:59:49] + error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] +   --> $DIR/E0607.rs:2:28
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |             ---------------^-- _
[00:59:49] +    |             |
[00:59:49] +    |             fn() -> *const _ {std::ptr::null::<_>}
[00:59:49] +    |
[00:59:49] +    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] + 
[00:59:49] + error[E0606]: casting `bool` as `*const [u8]` is invalid
[00:59:49] 2   --> $DIR/E0607.rs:3:5
[00:59:49] 3    |
[00:59:49] 4 LL |     v as *const [u8];
[00:59:49] 5    |     ^^^^^^^^^^^^^^^^
[00:59:49] 6 
[00:59:49] - error: aborting due to previous error
[00:59:49] + error: aborting due to 4 previous errors
---
[00:59:49] 10 
[00:59:49] 
[00:59:49] 
[00:59:49] The actual stderr differed from the expected stderr.
[00:59:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0607/E0607.stderr
[00:59:49] To update references, rerun the tests and pass the `--bless` flag
[00:59:49] To only update this specific test, also pass `--test-args error-codes/E0607.rs`
[00:59:49] error: 1 errors occurred comparing output.
[00:59:49] status: exit code: 1
[00:59:49] status: exit code: 1
[00:59:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0607.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0607" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0607/auxiliary" "-A" "unused"
[00:59:49] ------------------------------------------
[00:59:49] 
[00:59:49] ------------------------------------------
[00:59:49] stderr:
[00:59:49] stderr:
[00:59:49] ------------------------------------------
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/error-codes/E0607.rs:2:28
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] error[E0423]: expected value, found builtin type `u8`
[00:59:49]   --> /checkout/src/test/ui/error-codes/E0607.rs:2:29
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |                             ^^ not a value
[00:59:49] 
[00:59:49] error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |             ---------------^-- _
[00:59:49]    |             |
[00:59:49]    |             fn() -> *const _ {std::ptr::null::<_>}
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] 
[00:59:49] error[E0606]: casting `bool` as `*const [u8]` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     v as *const [u8]; //~ ERROR E0607
[00:59:49] 
[00:59:49] error: aborting due to 4 previous errors
[00:59:49] 
[00:59:49] Some errors have detailed explanations: E0369, E0423, E0606.
---
[00:59:49] 
[00:59:49] + error: chained comparison operators require parentheses
[00:59:49] +   --> $DIR/error-festival.rs:40:28
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] +    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49] +    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] 1 error[E0425]: cannot find value `y` in this scope
[00:59:49] 2   --> $DIR/error-festival.rs:14:5
[00:59:49] 3    |
[00:59:49] 
[00:59:49] 
[00:59:49] 4 LL |     y = 2;
[00:59:49] 5    |     ^ help: a local variable with a similar name exists: `x`
[00:59:49] 6 
[00:59:49] + error[E0423]: expected value, found builtin type `u8`
[00:59:49] +   --> $DIR/error-festival.rs:40:29
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |                             ^^ not a value
[00:59:49] 7 error[E0603]: constant `FOO` is private
[00:59:49] 8   --> $DIR/error-festival.rs:22:10
[00:59:49] 9    |
[00:59:49] 
[00:59:49] 
[00:59:49] 34    |
[00:59:49] 35    = note: an implementation of `std::ops::Not` might be missing for `Question`
[00:59:49] 36 
[00:59:49] + error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |             ---------------^-- _
[00:59:49] +    |             |
[00:59:49] +    |             fn() -> *const _ {std::ptr::null::<_>}
[00:59:49] +    |
[00:59:49] +    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] + 
[00:59:49] 37 error[E0604]: only `u8` can be cast as `char`, not `u32`
[00:59:49] 39    |
[00:59:49] 
[00:59:49] 
[00:59:49] 63    |                  cannot cast `&u8` as `u32`
[00:59:49] 64    |                  help: dereference the expression: `*x`
[00:59:49] 65 
[00:59:49] - error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
[00:59:49] + error[E0606]: casting `bool` as `*const [u8]` is invalid
[00:59:49] 68    |
[00:59:49] 68    |
[00:59:49] 69 LL |     v as *const [u8];
[00:59:49] 70    |     ^^^^^^^^^^^^^^^^
[00:59:49] 71 
[00:59:49] - error: aborting due to 10 previous errors
[00:59:49] + error: aborting due to 13 previous errors
[00:59:49] + error: aborting due to 13 previous errors
[00:59:49] 73 
[00:59:49] - Some errors have detailed explanations: E0054, E0368, E0425, E0599, E0600, E0603, E0604, E0605, E0606...
[00:59:49] + Some errors have detailed explanations: E0054, E0368, E0369, E0423, E0425, E0599, E0600, E0603, E0604...
[00:59:49] 76 
[00:59:49] 
[00:59:49] 
[00:59:49] The actual stderr differed from the expected stderr.
[00:59:49] The actual stderr differed from the expected stderr.
[00:59:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/error-festival.stderr
[00:59:49] To update references, rerun the tests and pass the `--bless` flag
[00:59:49] To only update this specific test, also pass `--test-args error-festival.rs`
[00:59:49] error: 1 errors occurred comparing output.
[00:59:49] status: exit code: 1
[00:59:49] status: exit code: 1
[00:59:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-festival.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/auxiliary" "-A" "unused"
[00:59:49] ------------------------------------------
[00:59:49] 
[00:59:49] ------------------------------------------
[00:59:49] stderr:
[00:59:49] stderr:
[00:59:49] ------------------------------------------
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/error-festival.rs:40:28
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] error[E0425]: cannot find value `y` in this scope
[00:59:49]   --> /checkout/src/test/ui/error-festival.rs:14:5
[00:59:49]    |
[00:59:49] LL |     y = 2;
[00:59:49] LL |     y = 2;
[00:59:49]    |     ^ help: a local variable with a similar name exists: `x`
[00:59:49] 
[00:59:49] error[E0423]: expected value, found builtin type `u8`
[00:59:49]   --> /checkout/src/test/ui/error-festival.rs:40:29
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |                             ^^ not a value
[00:59:49] error[E0603]: constant `FOO` is private
[00:59:49]   --> /checkout/src/test/ui/error-festival.rs:22:10
[00:59:49]    |
[00:59:49] LL |     foo::FOO;
---
[00:59:49]    |
[00:59:49] LL |     x += 2;
[00:59:49]    |     -^^^^^
[00:59:49]    |     |
[00:59:49]    |     cannot use `+=` on type `&str`
[00:59:49]    = note: an implementation of `std::ops::AddAssign` might be missing for `&str`
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0599]: no method named `z` found for type `&str` in the current scope
[00:59:49]    |
[00:59:49] LL |     x.z();
[00:59:49]    |       ^
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0600]: cannot apply unary operator `!` to type `Question`
[00:59:49]   --> /checkout/src/test/ui/error-festival.rs:19:5
[00:59:49]    |
[00:59:49] LL |     !Question::Yes;
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::ops::Not` might be missing for `Question`
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |             ---------------^-- _
[00:59:49]    |             |
[00:59:49]    |             fn() -> *const _ {std::ptr::null::<_>}
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] 
[00:59:49] error[E0604]: only `u8` can be cast as `char`, not `u32`
[00:59:49]    |
[00:59:49] LL |     0u32 as char;
[00:59:49]    |     ^^^^^^^^^^^^
[00:59:49] 
---
[00:59:49] 
[00:59:49] error[E0054]: cannot cast as `bool`
[00:59:49]   --> /checkout/src/test/ui/error-festival.rs:33:24
[00:59:49]    |
[00:59:49] LL |     let x_is_nonzero = x as bool;
[00:59:49]    |                        ^^^^^^^^^ help: compare with zero instead: `x != 0`
[00:59:49] error[E0606]: casting `&u8` as `u32` is invalid
[00:59:49]   --> /checkout/src/test/ui/error-festival.rs:37:18
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let y: u32 = x as u32;
[00:59:49]    |                  -^^^^^^^
[00:59:49]    |                  |
[00:59:49]    |                  cannot cast `&u8` as `u32`
[00:59:49]    |                  help: dereference the expression: `*x`
[00:59:49] 
[00:59:49] error[E0606]: casting `bool` as `*const [u8]` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     v as *const [u8];
[00:59:49] 
[00:59:49] error: aborting due to 13 previous errors
[00:59:49] 
[00:59:49] Some errors have detailed explanations: E0054, E0368, E0369, E0423, E0425, E0599, E0600, E0603, E0604...
---
[00:59:49] 
[00:59:49] + error[E0308]: mismatched types
[00:59:49] +   --> $DIR/fat-ptr-cast.rs:22:41
[00:59:49] +    |
[00:59:49] + LL |     let t: *mut (dyn Trait + 'static) = core::ptr::null();
[00:59:49] +    |                                         ^^^^^^^^^^^^^^^^^ types differ in mutability
[00:59:49] +    |
[00:59:49] +    = note: expected type `*mut (dyn Trait + 'static)`
[00:59:49] +               found type `*const _`
[00:59:49] + error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:59:49] +   --> $DIR/fat-ptr-cast.rs:23:32
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let mut fail: *const str = core::ptr::null();
[00:59:49] +    |
[00:59:49] +    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:59:49] +    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:59:49] +    = note: required by `std::ptr::null`
[00:59:49] +    = note: required by `std::ptr::null`
[00:59:49] + 
[00:59:49] 1 error[E0606]: casting `&[i32]` as `usize` is invalid
[00:59:49] 2   --> $DIR/fat-ptr-cast.rs:10:5
[00:59:49] 
[00:59:49] 
[00:59:49] 52 LL |     q as *const [i32];
[00:59:49] 54 
[00:59:49] 54 
[00:59:49] - error[E0606]: casting `usize` as `*mut (dyn Trait + 'static)` is invalid
[00:59:49] -   --> $DIR/fat-ptr-cast.rs:22:41
[00:59:49] -    |
[00:59:49] - LL |     let t: *mut (dyn Trait + 'static) = core::ptr::null();
[00:59:49] - 
[00:59:49] - error[E0606]: casting `usize` as `*const str` is invalid
[00:59:49] -   --> $DIR/fat-ptr-cast.rs:23:32
[00:59:49] -    |
[00:59:49] -    |
[00:59:49] - LL |     let mut fail: *const str = core::ptr::null();
[00:59:49] - 
[00:59:49] 67 error: aborting due to 9 previous errors
[00:59:49] 68 
[00:59:49] - Some errors have detailed explanations: E0605, E0606, E0607.
[00:59:49] - Some errors have detailed explanations: E0605, E0606, E0607.
[00:59:49] - For more information about an error, try `rustc --explain E0605`.
[00:59:49] + Some errors have detailed explanations: E0277, E0308, E0605, E0606, E0607.
[00:59:49] + For more information about an error, try `rustc --explain E0277`.
[00:59:49] 71 
[00:59:49] 
[00:59:49] 
[00:59:49] The actual stderr differed from the expected stderr.
[00:59:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast/fat-ptr-cast.stderr
[00:59:49] To update references, rerun the tests and pass the `--bless` flag
[00:59:49] To only update this specific test, also pass `--test-args fat-ptr-cast.rs`
[00:59:49] error: 1 errors occurred comparing output.
[00:59:49] status: exit code: 1
[00:59:49] status: exit code: 1
[00:59:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fat-ptr-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast/auxiliary" "-A" "unused"
[00:59:49] ------------------------------------------
[00:59:49] 
[00:59:49] ------------------------------------------
[00:59:49] stderr:
[00:59:49] stderr:
[00:59:49] ------------------------------------------
[00:59:49] error[E0308]: mismatched types
[00:59:49]   --> /checkout/src/test/ui/fat-ptr-cast.rs:22:41
[00:59:49]    |
[00:59:49] LL |     let t: *mut (dyn Trait + 'static) = core::ptr::null(); //~ ERROR casting
[00:59:49]    |                                         ^^^^^^^^^^^^^^^^^ types differ in mutability
[00:59:49]    |
[00:59:49]    = note: expected type `*mut (dyn Trait + 'static)`
[00:59:49]               found type `*const _`
[00:59:49] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:59:49]   --> /checkout/src/test/ui/fat-ptr-cast.rs:23:32
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let mut fail: *const str = core::ptr::null(); //~ ERROR casting
[00:59:49]    |
[00:59:49]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:59:49]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:59:49]    = note: required by `std::ptr::null`
[00:59:49]    = note: required by `std::ptr::null`
[00:59:49] 
[00:59:49] error[E0606]: casting `&[i32]` as `usize` is invalid
[00:59:49]    |
[00:59:49] LL |     a as usize; //~ ERROR casting
[00:59:49]    |     ^^^^^^^^^^
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: cast through a raw pointer first
[00:59:49] 
[00:59:49] error[E0606]: casting `&[i32]` as `isize` is invalid
[00:59:49]    |
[00:59:49] LL |     a as isize; //~ ERROR casting
[00:59:49]    |     ^^^^^^^^^^
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: cast through a raw pointer first
[00:59:49] 
[00:59:49] error[E0606]: casting `&[i32]` as `i16` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     a as i16; //~ ERROR casting `&[i32]` as `i16` is invalid
[00:59:49]    |
[00:59:49]    = help: cast through a raw pointer first
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `&[i32]` as `u32` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     a as u32; //~ ERROR casting `&[i32]` as `u32` is invalid
[00:59:49]    |
[00:59:49]    = help: cast through a raw pointer first
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `std::boxed::Box<[i32]>` as `usize`
[00:59:49] error[E0605]: non-primitive cast: `std::boxed::Box<[i32]>` as `usize`
[00:59:49]   --> /checkout/src/test/ui/fat-ptr-cast.rs:14:5
[00:59:49]    |
[00:59:49] LL |     b as usize; //~ ERROR non-primitive cast
[00:59:49]    |     ^^^^^^^^^^
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] error[E0606]: casting `*const [i32]` as `usize` is invalid
[00:59:49]    |
[00:59:49] LL |     p as usize;
[00:59:49]    |     ^^^^^^^^^^
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: cast through a thin pointer first
[00:59:49] 
[00:59:49] error[E0607]: cannot cast thin pointer `*const i32` to fat pointer `*const [i32]`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     q as *const [i32]; //~ ERROR cannot cast
[00:59:49] 
[00:59:49] error: aborting due to 9 previous errors
[00:59:49] 
[00:59:49] Some errors have detailed explanations: E0277, E0308, E0605, E0606, E0607.
---
[00:59:49] - error[E0658]: casting pointers to integers in statics is unstable
[00:59:49] + error: chained comparison operators require parentheses
[00:59:49] +   --> $DIR/issue-17458.rs:1:43
[00:59:49] +    |
[00:59:49] + LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] +    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49] +    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] + error[E0423]: expected value, found builtin type `usize`
[00:59:49] +   --> $DIR/issue-17458.rs:1:44
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49] + 
[00:59:49] + 
[00:59:49] + error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49] +    |                            ---------------^----- _
[00:59:49] +    |                            |
[00:59:49] +    |                            fn() -> *const _ {std::ptr::null::<_>}
[00:59:49] +    |
[00:59:49] +    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] + error[E0308]: mismatched types
[00:59:49] 2   --> $DIR/issue-17458.rs:1:28
[00:59:49] 3    |
[00:59:49] 3    |
[00:59:49] 4 LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49] -    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:49] +    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected usize, found bool
[00:59:49] + 
[00:59:49] + 
[00:59:49] + error[E0605]: non-primitive cast: `()` as `usize`
[00:59:49] 6    |
[00:59:49] -    = note: for more information, see https://github.com/rust-lang/rust/issues/51910
[00:59:49] -    = help: add #![feature(const_raw_ptr_to_usize_cast)] to the crate attributes to enable
[00:59:49] -    = help: add #![feature(const_raw_ptr_to_usize_cast)] to the crate attributes to enable
[00:59:49] + LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49] +    |
[00:59:49] +    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 9 
[00:59:49] - error: aborting due to previous error
---
[00:59:49] 
[00:59:49] 
[00:59:49] The actual stderr differed from the expected stderr.
[00:59:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17458/issue-17458.stderr
[00:59:49] To update references, rerun the tests and pass the `--bless` flag
[00:59:49] To only update this specific test, also pass `--test-args issues/issue-17458.rs`
[00:59:49] error: 1 errors occurred comparing output.
[00:59:49] status: exit code: 1
[00:59:49] status: exit code: 1
[00:59:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17458.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17458" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17458/auxiliary" "-A" "unused"
[00:59:49] ------------------------------------------
[00:59:49] 
[00:59:49] ------------------------------------------
[00:59:49] stderr:
[00:59:49] stderr:
[00:59:49] ------------------------------------------
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/issues/issue-17458.rs:1:43
[00:59:49]    |
[00:59:49] LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] 
[00:59:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:49] error[E0423]: expected value, found builtin type `usize`
[00:59:49]   --> /checkout/src/test/ui/issues/issue-17458.rs:1:44
[00:59:49]   --> /checkout/src/test/ui/issues/issue-17458.rs:1:44
[00:59:49]    |
[00:59:49] LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49]    |                            ---------------^----- _
[00:59:49]    |                            |
[00:59:49]    |                            fn() -> *const _ {std::ptr::null::<_>}
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] error[E0308]: mismatched types
[00:59:49]   --> /checkout/src/test/ui/issues/issue-17458.rs:1:28
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `()` as `usize`
[00:59:49]   --> /checkout/src/test/ui/issues/issue-17458.rs:1:50
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL | static X: usize = unsafe { core::ptr::null<usize>() as usize };
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] error: aborting due to 5 previous errors
---
[00:59:49] 
[00:59:49] + error: chained comparison operators require parentheses
[00:59:49] +   --> $DIR/cast-rfc0401.rs:24:28
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] +    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49] +    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] + error: chained comparison operators require parentheses
[00:59:49] +   --> $DIR/cast-rfc0401.rs:25:58
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] +    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49] +    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] + error: chained comparison operators require parentheses
[00:59:49] +   --> $DIR/cast-rfc0401.rs:26:59
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] +    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49] +    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] + error[E0423]: expected value, found builtin type `u8`
[00:59:49] +   --> $DIR/cast-rfc0401.rs:24:29
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |                             ^^ not a value
[00:59:49] + error[E0423]: expected value, found builtin type `u8`
[00:59:49] +   --> $DIR/cast-rfc0401.rs:25:60
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49] +    |                                                            ^^ not a value
[00:59:49] + error[E0423]: expected value, found builtin type `i8`
[00:59:49] +   --> $DIR/cast-rfc0401.rs:26:61
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49] +    |                                                             ^^ not a value
[00:59:49] + 
[00:59:49] 1 error[E0606]: casting `*const U` as `*const V` is invalid
[00:59:49] 3    |
[00:59:49] 
[00:59:49] 14    |
[00:59:49] 14    |
[00:59:49] 15    = note: vtable kinds may not match
[00:59:49] 16 
[00:59:49] + error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let v = core::ptr::null<u8>();
[00:59:49] +    |             ---------------^-- _
[00:59:49] +    |             |
[00:59:49] +    |             fn() -> *const _ {std::ptr::null::<_>}
[00:59:49] +    |
[00:59:49] +    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] + 
[00:59:49] + error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49] +    |                                           ---------------^------- _
[00:59:49] +    |                                           |
[00:59:49] +    |                                           fn() -> *const _ {std::ptr::null::<_>}
[00:59:49] +    |
[00:59:49] +    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] + error[E0614]: type `bool` cannot be dereferenced
[00:59:49] +   --> $DIR/cast-rfc0401.rs:25:41
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49] + 
[00:59:49] + 
[00:59:49] + error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49] +    |                                            ---------------^------- _
[00:59:49] +    |                                            |
[00:59:49] +    |                                            fn() -> *const _ {std::ptr::null::<_>}
[00:59:49] +    |
[00:59:49] +    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] + error[E0614]: type `bool` cannot be dereferenced
[00:59:49] +   --> $DIR/cast-rfc0401.rs:26:42
[00:59:49] +    |
[00:59:49] +    |
[00:59:49] + LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49] + 
[00:59:49] + 
[00:59:49] 17 error[E0609]: no field `f` on type `fn() {main}`
[00:59:49] 19    |
[00:59:49] 
[00:59:49] 
[00:59:49] 20 LL |     let _ = main.f as *const u32;
[00:59:49] 22 
[00:59:49] 22 
[00:59:49] - error[E0605]: non-primitive cast: `*const u8` as `&u8`
[00:59:49] + error[E0605]: non-primitive cast: `bool` as `&u8`
[00:59:49] 25    |
[00:59:49] 25    |
[00:59:49] 26 LL |     let _ = v as &u8;
[00:59:49] 28    |
[00:59:49] 29    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 30 
[00:59:49] 30 
[00:59:49] - error[E0605]: non-primitive cast: `*const u8` as `E`
[00:59:49] + error[E0605]: non-primitive cast: `bool` as `E`
[00:59:49] 33    |
[00:59:49] 33    |
[00:59:49] 34 LL |     let _ = v as E;
[00:59:49] 36    |
[00:59:49] 37    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 38 
[00:59:49] 38 
[00:59:49] - error[E0605]: non-primitive cast: `*const u8` as `fn()`
[00:59:49] + error[E0605]: non-primitive cast: `bool` as `fn()`
[00:59:49] 41    |
[00:59:49] 41    |
[00:59:49] 42 LL |     let _ = v as fn();
[00:59:49] 44    |
[00:59:49] 45    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 46 
[00:59:49] 46 
[00:59:49] - error[E0605]: non-primitive cast: `*const u8` as `(u32,)`
[00:59:49] + error[E0605]: non-primitive cast: `bool` as `(u32,)`
[00:59:49] 49    |
[00:59:49] 49    |
[00:59:49] 50 LL |     let _ = v as (u32,);
[00:59:49] 52    |
[00:59:49] 53    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 54 
[00:59:49] 54 
[00:59:49] - error[E0605]: non-primitive cast: `std::option::Option<&*const u8>` as `*const u8`
[00:59:49] + error[E0605]: non-primitive cast: `std::option::Option<&bool>` as `*const u8`
[00:59:49] 57    |
[00:59:49] 58 LL |     let _ = Some(&v) as *const u8;
[00:59:49] 
[00:59:49] 60    |
[00:59:49] 60    |
[00:59:49] 61    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 62 
[00:59:49] - error[E0606]: casting `*const u8` as `f32` is invalid
[00:59:49] + error[E0606]: casting `bool` as `f32` is invalid
[00:59:49] 65    |
[00:59:49] 65    |
[00:59:49] 66 LL |     let _ = v as f32;
[00:59:49] 67    |             ^^^^^^^^
[00:59:49] +    |
[00:59:49] +    = help: cast through an integer first
[00:59:49] 68 
[00:59:49] 68 
[00:59:49] 69 error[E0606]: casting `fn() {main}` as `f64` is invalid
[00:59:49] 
[00:59:49] 72 LL |     let _ = main as f64;
[00:59:49] 73    |             ^^^^^^^^^^^
[00:59:49] 74 
[00:59:49] 74 
[00:59:49] - error[E0606]: casting `&*const u8` as `usize` is invalid
[00:59:49] + error[E0606]: casting `&bool` as `usize` is invalid
[00:59:49] 77    |
[00:59:49] 77    |
[00:59:49] 78 LL |     let _ = &v as usize;
[00:59:49] 152 LL |     let _ = 42usize as *const [u8];
[00:59:49] 153    |             ^^^^^^^^^^^^^^^^^^^^^^
[00:59:49] 154 
[00:59:49] 154 
[00:59:49] - error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
[00:59:49] + error[E0606]: casting `bool` as `*const [u8]` is invalid
[00:59:49] 157    |
[00:59:49] 157    |
[00:59:49] 158 LL |     let _ = v as *const [u8];
[00:59:49] 
[00:59:49] 241    |                              cannot cast `&{float}` as `f32`
[00:59:49] 242    |                              help: dereference the expression: `*s`
[00:59:49] - error: aborting due to 34 previous errors
[00:59:49] + error: aborting due to 45 previous errors
[00:59:49] 245 
[00:59:49] - Some errors have detailed explanations: E0054, E0277, E0604, E0605, E0606, E0607, E0609.
[00:59:49] - Some errors have detailed explanations: E0054, E0277, E0604, E0605, E0606, E0607, E0609.
[00:59:49] + Some errors have detailed explanations: E0054, E0277, E0369, E0423, E0604, E0605, E0606, E0609, E0614.
[00:59:49] 247 For more information about an error, try `rustc --explain E0054`.
[00:59:49] 248 
[00:59:49] 
[00:59:49] 
[00:59:49] The actual stderr differed from the expected stderr.
[00:59:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/cast-rfc0401.stderr
[00:59:49] To update references, rerun the tests and pass the `--bless` flag
[00:59:49] To only update this specific test, also pass `--test-args mismatched_types/cast-rfc0401.rs`
[00:59:49] error: 1 errors occurred comparing output.
[00:59:49] status: exit code: 1
[00:59:49] status: exit code: 1
[00:59:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary" "-A" "unused"
[00:59:49] ------------------------------------------
[00:59:49] 
[00:59:49] ------------------------------------------
[00:59:49] stderr:
[00:59:49] stderr:
[00:59:49] ------------------------------------------
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:24:28
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:25:58
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] error: chained comparison operators require parentheses
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:26:59
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
[00:59:49]    = help: or use `(...)` if you meant to specify fn arguments
[00:59:49] error[E0423]: expected value, found builtin type `u8`
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:24:29
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |                             ^^ not a value
[00:59:49] error[E0423]: expected value, found builtin type `u8`
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:25:60
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49]    |                                                            ^^ not a value
[00:59:49] error[E0423]: expected value, found builtin type `i8`
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:26:61
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49]    |                                                             ^^ not a value
[00:59:49] 
[00:59:49] error[E0606]: casting `*const U` as `*const V` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     u as *const V //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = note: vtable kinds may not match
[00:59:49] error[E0606]: casting `*const U` as `*const str` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:8:5
[00:59:49]    |
[00:59:49] LL |     u as *const str //~ ERROR is invalid
[00:59:49] LL |     u as *const str //~ ERROR is invalid
[00:59:49]    |     ^^^^^^^^^^^^^^^
[00:59:49]    |
[00:59:49]    = note: vtable kinds may not match
[00:59:49] 
[00:59:49] error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let v = core::ptr::null<u8>();
[00:59:49]    |             ---------------^-- _
[00:59:49]    |             |
[00:59:49]    |             fn() -> *const _ {std::ptr::null::<_>}
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] 
[00:59:49] error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49]    |                                           ---------------^------- _
[00:59:49]    |                                           |
[00:59:49]    |                                           fn() -> *const _ {std::ptr::null::<_>}
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] error[E0614]: type `bool` cannot be dereferenced
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:25:41
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_v : *const [u8] = unsafe { &*(core::ptr::null<[u8; 1]>())};
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0369]: binary operation `<` cannot be applied to type `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49]    |                                            ---------------^------- _
[00:59:49]    |                                            |
[00:59:49]    |                                            fn() -> *const _ {std::ptr::null::<_>}
[00:59:49]    |
[00:59:49]    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> *const _ {std::ptr::null::<_>}`
[00:59:49] error[E0614]: type `bool` cannot be dereferenced
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:26:42
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let fat_sv : *const [i8] = unsafe { &*(core::ptr::null<[i8; 1]>())};
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0609]: no field `f` on type `fn() {main}`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = main.f as *const u32; //~ ERROR no field
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `bool` as `&u8`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = v as &u8; //~ ERROR non-primitive cast
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `bool` as `E`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = v as E; //~ ERROR non-primitive cast
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `bool` as `fn()`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = v as fn(); //~ ERROR non-primitive cast
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `bool` as `(u32,)`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = v as (u32,); //~ ERROR non-primitive cast
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0605]: non-primitive cast: `std::option::Option<&bool>` as `*const u8`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = Some(&v) as *const u8; //~ ERROR non-primitive cast
[00:59:49]    |
[00:59:49]    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:59:49] 
[00:59:49] error[E0606]: casting `bool` as `f32` is invalid
[00:59:49] error[E0606]: casting `bool` as `f32` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:35:13
[00:59:49]    |
[00:59:49] LL |     let _ = v as f32; //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    = help: cast through an integer first
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `fn() {main}` as `f64` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = main as f64; //~ ERROR is invalid
[00:59:49] 
[00:59:49] error[E0606]: casting `&bool` as `usize` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:37:13
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = &v as usize; //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    = help: cast through a raw pointer first
[00:59:49] 
[00:59:49] error[E0606]: casting `f32` as `*const u8` is invalid
[00:59:49] error[E0606]: casting `f32` as `*const u8` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:38:13
[00:59:49]    |
[00:59:49] LL |     let _ = f as *const u8; //~ ERROR is invalid
[00:59:49] 
[00:59:49] error[E0054]: cannot cast as `bool`
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:39:13
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = 3_i32 as bool; //~ ERROR cannot cast
[00:59:49]    |             ^^^^^^^^^^^^^ help: compare with zero instead: `3_i32 != 0`
[00:59:49] error[E0054]: cannot cast as `bool`
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:40:13
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = E::A as bool; //~ ERROR cannot cast
[00:59:49]    |             ^^^^^^^^^^^^ unsupported cast
[00:59:49] 
[00:59:49] error[E0604]: only `u8` can be cast as `char`, not `u32`
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = 0x61u32 as char; //~ ERROR can be cast as
[00:59:49] 
[00:59:49] error[E0606]: casting `bool` as `f32` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:43:13
[00:59:49]    |
---
[00:59:49] 
[00:59:49] error[E0606]: casting `E` as `f32` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:44:13
[00:59:49]    |
[00:59:49] LL |     let _ = E::A as f32; //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    = help: cast through an integer first
[00:59:49] 
[00:59:49] error[E0606]: casting `char` as `f32` is invalid
[00:59:49] error[E0606]: casting `char` as `f32` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:45:13
[00:59:49]    |
[00:59:49] LL |     let _ = 'a' as f32; //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    = help: cast through an integer first
[00:59:49] 
[00:59:49] error[E0606]: casting `bool` as `*const u8` is invalid
---
[00:59:49] 
[00:59:49] error[E0606]: casting `E` as `*const u8` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:48:13
[00:59:49]    |
[00:59:49] LL |     let _ = E::A as *const u8; //~ ERROR is invalid
[00:59:49] 
[00:59:49] error[E0606]: casting `char` as `*const u8` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:49:13
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = 'a' as *const u8; //~ ERROR is invalid
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `usize` as `*const [u8]` is invalid
[00:59:49]    |
[00:59:49] LL |     let _ = 42usize as *const [u8]; //~ ERROR is invalid
[00:59:49]    |             ^^^^^^^^^^^^^^^^^^^^^^
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `bool` as `*const [u8]` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = v as *const [u8]; //~ ERROR cannot cast
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `&dyn Foo` as `*const str` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = foo as *const str; //~ ERROR is invalid
[00:59:49] 
[00:59:49] error[E0606]: casting `&dyn Foo` as `*mut str` is invalid
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:55:13
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = foo as *mut str; //~ ERROR is invalid
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `fn() {main}` as `*mut str` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = main as *mut str; //~ ERROR is invalid
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `&f32` as `*mut f32` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = &f as *mut f32; //~ ERROR is invalid
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `&f32` as `*const f64` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = &f as *const f64; //~ ERROR is invalid
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `*const [i8]` as `usize` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = fat_sv as usize; //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    = help: cast through a thin pointer first
[00:59:49] 
[00:59:49] 
[00:59:49] error[E0606]: casting `*const dyn Foo` as `*const [u16]` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = cf as *const [u16]; //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = note: vtable kinds may not match
[00:59:49] 
[00:59:49] error[E0606]: casting `*const dyn Foo` as `*const dyn Bar` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = cf as *const dyn Bar; //~ ERROR is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49]    = note: vtable kinds may not match
[00:59:49] error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
[00:59:49]   --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:53:13
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     let _ = fat_v as *const dyn Foo; //~ ERROR the size for values of type
[00:59:49]    |
[00:59:49]    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
[00:59:49]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:59:49]    = note: required for the cast to the object type `dyn Foo`
---
[00:59:49]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:59:49]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:59:49]    = note: required for the cast to the object type `dyn Foo`
[00:59:49] 
[00:59:49] error[E0606]: casting `&{float}` as `f32` is invalid
[00:59:49]    |
[00:59:49]    |
[00:59:49] LL |     vec![0.0].iter().map(|s| s as f32).collect::<Vec<f32>>(); //~ ERROR is invalid
[00:59:49]    |                              -^^^^^^^
[00:59:49]    |                              |
[00:59:49]    |                              cannot cast `&{float}` as `f32`
[00:59:49]    |                              help: dereference the expression: `*s`
[00:59:49] error: aborting due to 45 previous errors
[00:59:49] 
[00:59:49] Some errors have detailed explanations: E0054, E0277, E0369, E0423, E0604, E0605, E0606, E0609, E0614.
[00:59:49] For more information about an error, try `rustc --explain E0054`.
---
[00:59:49] test result: FAILED. 5652 passed; 7 failed; 21 ignored; 0 measured; 0 filtered out
[00:59:49] 
[00:59:49] 
[00:59:49] 
[00:59:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:49] 
[00:59:49] 
[00:59:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:49] Build completed unsuccessfully in 0:55:04
---
travis_time:end:154a03de:start=1560589265689105713,finish=1560589265694203670,duration=5097957
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0389f5e9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:260bd30e
travis_time:start:260bd30e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d92c9b2
$ dmesg | grep -i kill
