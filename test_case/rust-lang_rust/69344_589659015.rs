plain
2020-02-21T10:51:30.5158460Z ========================== Starting Command Output ===========================
2020-02-21T10:51:30.5164145Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/483a775b-1033-469e-a2e9-0c7ee115c692.sh
2020-02-21T10:51:30.5164681Z 
2020-02-21T10:51:30.5170097Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T10:51:30.5193362Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69344/merge to s
2020-02-21T10:51:30.5197147Z Task         : Get sources
2020-02-21T10:51:30.5197436Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T10:51:30.5197715Z Version      : 1.0.0
2020-02-21T10:51:30.5197905Z Author       : Microsoft
---
2020-02-21T10:51:31.5065268Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T10:51:31.5070633Z ##[command]git config gc.auto 0
2020-02-21T10:51:31.5074208Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T10:51:31.5077282Z ##[command]git config --get-all http.proxy
2020-02-21T10:51:31.5083791Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69344/merge:refs/remotes/pull/69344/merge
---
2020-02-21T12:57:42.2259965Z  finished in 70.847
2020-02-21T12:57:42.2266789Z Testing rustbook src/doc/nomicon
2020-02-21T12:57:50.1910478Z Error: Rustdoc returned an error: 
2020-02-21T12:57:50.1910791Z running 6 tests
2020-02-21T12:57:50.1911791Z test /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 128) ... FAILED
2020-02-21T12:57:50.1912416Z test /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 28) ... FAILED
2020-02-21T12:57:50.1913024Z test /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 110) ... ok
2020-02-21T12:57:50.1913889Z test /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 6) ... ignored
2020-02-21T12:57:50.1914481Z test /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 55) ... FAILED
2020-02-21T12:57:50.1915087Z test /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 96) ... ok
2020-02-21T12:57:50.1915458Z failures:
2020-02-21T12:57:50.1915578Z 
2020-02-21T12:57:50.1915578Z 
2020-02-21T12:57:50.1916184Z ---- /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 128) stdout ----
2020-02-21T12:57:50.1916572Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-02-21T12:57:50.1917112Z  --> /tmp/mdbook-yZ5OVm/destructors.md:131:18
2020-02-21T12:57:50.1917595Z 4 | use std::alloc::{Alloc, GlobalAlloc, Global, Layout};
2020-02-21T12:57:50.1917878Z   |                  ^^^^^
2020-02-21T12:57:50.1918102Z   |                  |
2020-02-21T12:57:50.1918338Z   |                  no `Alloc` in `alloc`
2020-02-21T12:57:50.1918338Z   |                  no `Alloc` in `alloc`
2020-02-21T12:57:50.1918917Z   |                  help: a similar name exists in the module: `alloc`
2020-02-21T12:57:50.1919182Z 
2020-02-21T12:57:50.1919533Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-02-21T12:57:50.1920184Z   --> /tmp/mdbook-yZ5OVm/destructors.md:142:20
2020-02-21T12:57:50.1920697Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-02-21T12:57:50.1921127Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-02-21T12:57:50.1921417Z    |
2020-02-21T12:57:50.1921698Z    = help: items from traits can only be used if the trait is in scope
2020-02-21T12:57:50.1921698Z    = help: items from traits can only be used if the trait is in scope
2020-02-21T12:57:50.1922112Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-02-21T12:57:50.1922397Z    |
2020-02-21T12:57:50.1922607Z 4  | use std::alloc::AllocRef;
2020-02-21T12:57:50.1922798Z    |
2020-02-21T12:57:50.1922906Z 
2020-02-21T12:57:50.1923256Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-02-21T12:57:50.1923883Z   --> /tmp/mdbook-yZ5OVm/destructors.md:157:20
2020-02-21T12:57:50.1924371Z 30 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-02-21T12:57:50.1924813Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-02-21T12:57:50.1925101Z    |
2020-02-21T12:57:50.1925368Z    = help: items from traits can only be used if the trait is in scope
---
2020-02-21T12:57:50.1927561Z 
2020-02-21T12:57:50.1927832Z Some errors have detailed explanations: E0432, E0599.
2020-02-21T12:57:50.1929869Z For more information about an error, try `rustc --explain E0432`.
2020-02-21T12:57:50.1930353Z Couldn't compile the test.
2020-02-21T12:57:50.1930890Z ---- /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 28) stdout ----
2020-02-21T12:57:50.1931265Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-02-21T12:57:50.1931771Z  --> /tmp/mdbook-yZ5OVm/destructors.md:31:18
2020-02-21T12:57:50.1932248Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-02-21T12:57:50.1932535Z   |                  ^^^^^
2020-02-21T12:57:50.1932759Z   |                  |
2020-02-21T12:57:50.1932996Z   |                  no `Alloc` in `alloc`
2020-02-21T12:57:50.1932996Z   |                  no `Alloc` in `alloc`
2020-02-21T12:57:50.1933353Z   |                  help: a similar name exists in the module: `alloc`
2020-02-21T12:57:50.1933616Z 
2020-02-21T12:57:50.1933968Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-02-21T12:57:50.1934581Z   --> /tmp/mdbook-yZ5OVm/destructors.md:42:20
2020-02-21T12:57:50.1935416Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>())
2020-02-21T12:57:50.1936448Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-02-21T12:57:50.1937397Z    |
2020-02-21T12:57:50.1937718Z    = help: items from traits can only be used if the trait is in scope
---
2020-02-21T12:57:50.1939514Z 
2020-02-21T12:57:50.1939750Z Some errors have detailed explanations: E0432, E0599.
2020-02-21T12:57:50.1940532Z For more information about an error, try `rustc --explain E0432`.
2020-02-21T12:57:50.1941015Z Couldn't compile the test.
2020-02-21T12:57:50.1947118Z ---- /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 55) stdout ----
2020-02-21T12:57:50.1947554Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-02-21T12:57:50.1948266Z  --> /tmp/mdbook-yZ5OVm/destructors.md:58:18
2020-02-21T12:57:50.1948731Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-02-21T12:57:50.1949033Z   |                  ^^^^^
2020-02-21T12:57:50.1949240Z   |                  |
2020-02-21T12:57:50.1949474Z   |                  no `Alloc` in `alloc`
2020-02-21T12:57:50.1949474Z   |                  no `Alloc` in `alloc`
2020-02-21T12:57:50.1950802Z   |                  help: a similar name exists in the module: `alloc`
2020-02-21T12:57:50.1951330Z 
2020-02-21T12:57:50.1951669Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-02-21T12:57:50.1953853Z   --> /tmp/mdbook-yZ5OVm/destructors.md:69:20
2020-02-21T12:57:50.1972117Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-02-21T12:57:50.1972570Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-02-21T12:57:50.1972904Z    |
2020-02-21T12:57:50.1973166Z    = help: items from traits can only be used if the trait is in scope
2020-02-21T12:57:50.1973166Z    = help: items from traits can only be used if the trait is in scope
2020-02-21T12:57:50.1973582Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-02-21T12:57:50.1973884Z    |
2020-02-21T12:57:50.1975058Z 4  | use std::alloc::AllocRef;
2020-02-21T12:57:50.1977965Z    |
2020-02-21T12:57:50.1978093Z 
2020-02-21T12:57:50.1978470Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-02-21T12:57:50.1979929Z   --> /tmp/mdbook-yZ5OVm/destructors.md:82:20
2020-02-21T12:57:50.1980472Z 28 |             Global.dealloc(c.cast::<u8>(), Layout::new::<T>());
2020-02-21T12:57:50.1980913Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-02-21T12:57:50.1981200Z    |
2020-02-21T12:57:50.1981480Z    = help: items from traits can only be used if the trait is in scope
---
2020-02-21T12:57:50.1985698Z For more information about an error, try `rustc --explain E0432`.
2020-02-21T12:57:50.1986190Z Couldn't compile the test.
2020-02-21T12:57:50.1986344Z 
2020-02-21T12:57:50.1986479Z failures:
2020-02-21T12:57:50.1990791Z     /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 128)
2020-02-21T12:57:50.1991570Z     /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 28)
2020-02-21T12:57:50.1992119Z     /tmp/mdbook-yZ5OVm/destructors.md - Destructors (line 55)
2020-02-21T12:57:50.1993384Z test result: FAILED. 2 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
2020-02-21T12:57:50.1993803Z 
2020-02-21T12:57:50.1993900Z 
2020-02-21T12:57:50.1994072Z 
---
2020-02-21T12:57:50.2002283Z  finished in 7.965
2020-02-21T12:57:50.2002822Z Testing rustbook src/doc/reference
2020-02-21T12:58:09.7544163Z Error: Rustdoc returned an error: 
2020-02-21T12:58:09.7545112Z running 2 tests
2020-02-21T12:58:09.7546458Z test /tmp/mdbook-1tvfk3/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) ... FAILED
2020-02-21T12:58:09.7547631Z test /tmp/mdbook-1tvfk3/expressions/array-expr.md - Array_and_array_index_expressions::Array_expressions (line 27) ... ok
2020-02-21T12:58:09.7549098Z failures:
2020-02-21T12:58:09.7549328Z 
2020-02-21T12:58:09.7549328Z 
2020-02-21T12:58:09.7550195Z ---- /tmp/mdbook-1tvfk3/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) stdout ----
2020-02-21T12:58:09.7550843Z error: this operation will panic at runtime
2020-02-21T12:58:09.7551707Z   --> /tmp/mdbook-1tvfk3/expressions/array-expr.md:72:9
2020-02-21T12:58:09.7553618Z 11 | let x = (["a", "b"])[10]; // warning: index out of bounds
2020-02-21T12:58:09.7554217Z    |         ^^^^^^^^^^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-02-21T12:58:09.7554634Z    |
2020-02-21T12:58:09.7555008Z    = note: `#[deny(unconditional_panic)]` on by default
2020-02-21T12:58:09.7555008Z    = note: `#[deny(unconditional_panic)]` on by default
2020-02-21T12:58:09.7555329Z 
2020-02-21T12:58:09.7555648Z error: this operation will panic at runtime
2020-02-21T12:58:09.7556427Z   --> /tmp/mdbook-1tvfk3/expressions/array-expr.md:78:1
2020-02-21T12:58:09.7557212Z 17 | arr[10];                  // warning: index out of bounds
2020-02-21T12:58:09.7557753Z    | ^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-02-21T12:58:09.7558104Z 
2020-02-21T12:58:09.7558414Z error: aborting due to 2 previous errors
2020-02-21T12:58:09.7558414Z error: aborting due to 2 previous errors
2020-02-21T12:58:09.7558720Z 
2020-02-21T12:58:09.7559219Z Couldn't compile the test.
2020-02-21T12:58:09.7562852Z 
2020-02-21T12:58:09.7563171Z failures:
2020-02-21T12:58:09.7564152Z     /tmp/mdbook-1tvfk3/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63)
2020-02-21T12:58:09.7565104Z test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-21T12:58:09.7565470Z 
2020-02-21T12:58:09.7565684Z 
2020-02-21T12:58:09.7569105Z 
---
2020-02-21T13:01:00.6399065Z  106 │ The full interface is defined [here][code_parse_int].
2020-02-21T13:01:00.6399841Z      │                               ^ Server responded with 404 Not Found
2020-02-21T13:01:00.6401257Z      │
2020-02-21T13:01:00.6401550Z 
2020-02-21T13:01:00.6402881Z error: The server responded with 404 Not Found for "***/tree/master/src/librustc/infer/higher_ranked/README.md"
2020-02-21T13:01:00.6408461Z     ┌── traits/hrtb.md:35:62 ───
2020-02-21T13:01:00.6409103Z     │
2020-02-21T13:01:00.6414161Z  35 │ to the subtyping for higher-ranked types (which is described [here][hrsubtype]
2020-02-21T13:01:00.6415438Z     │                                                              ^ Server responded with 404 Not Found
---
2020-02-21T13:45:34.4514667Z Caught panic message (&str): assertion failed: false
2020-02-21T13:45:34.4535879Z error: Miri evaluation error: dangling pointer was dereferenced
2020-02-21T13:45:34.4536660Z     --> /checkout/src/libcore/intrinsics.rs:1528:5
2020-02-21T13:45:34.4537289Z      |
2020-02-21T13:45:34.4537697Z 1528 |     copy_nonoverlapping(src, dst, count)
2020-02-21T13:45:34.4538052Z      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dangling pointer was dereferenced
2020-02-21T13:45:34.4538732Z      = note: inside call to `std::intrinsics::copy_nonoverlapping::<i32>` at /checkout/src/libcore/ptr/mod.rs:623:5
2020-02-21T13:45:34.4539315Z      = note: inside call to `std::ptr::read::<i32>` at /checkout/src/libcore/ptr/const_ptr.rs:585:9
2020-02-21T13:45:34.4542904Z note: inside call to `std::ptr::const_ptr::<impl *const i32>::read` at $DIR/catch_panic.rs:67:32
2020-02-21T13:45:34.4543665Z     --> $DIR/catch_panic.rs:67:32
2020-02-21T13:45:34.4543665Z     --> $DIR/catch_panic.rs:67:32
2020-02-21T13:45:34.4543872Z      |
2020-02-21T13:45:34.4544489Z 67   |     test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd
2020-02-21T13:45:34.4545280Z note: inside call to closure at $DIR/catch_panic.rs:39:5
2020-02-21T13:45:34.4545787Z     --> $DIR/catch_panic.rs:39:5
2020-02-21T13:45:34.4545987Z      |
2020-02-21T13:45:34.4546170Z 39   |     do_panic(old_val);
2020-02-21T13:45:34.4546170Z 39   |     do_panic(old_val);
2020-02-21T13:45:34.4546393Z      |     ^^^^^^^^^^^^^^^^^
2020-02-21T13:45:34.4546828Z note: inside call to `do_panic_counter::<[closure@$DIR/catch_panic.rs:67:10: 67:69]>` at $DIR/catch_panic.rs:83:9
2020-02-21T13:45:34.4547628Z      |
2020-02-21T13:45:34.4547854Z 83   |         do_panic_counter(do_panic)
2020-02-21T13:45:34.4548122Z      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-21T13:45:34.4589109Z      = note: inside call to closure at /checkout/src/libstd/panic.rs:318:9
2020-02-21T13:45:34.4589109Z      = note: inside call to closure at /checkout/src/libstd/panic.rs:318:9
2020-02-21T13:45:34.4590329Z      = note: inside call to `<std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]> as std::ops::FnOnce<()>>::call_once` at /checkout/src/libstd/panicking.rs:303:40
2020-02-21T13:45:34.4591637Z      = note: inside call to `std::panicking::r#try::do_call::<std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]>, ()>` at /checkout/src/libstd/panicking.rs:281:13
2020-02-21T13:45:34.4592767Z      = note: inside call to `std::panicking::r#try::<(), std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]>>` at /checkout/src/libstd/panic.rs:394:14
2020-02-21T13:45:34.4593804Z note: inside call to `std::panic::catch_unwind::<std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]>, ()>` at $DIR/catch_panic.rs:81:15
2020-02-21T13:45:34.4595067Z      |
2020-02-21T13:45:34.4595307Z 81   |       let res = catch_unwind(AssertUnwindSafe(|| {
2020-02-21T13:45:34.4595595Z      |  _______________^
2020-02-21T13:45:34.4595595Z      |  _______________^
2020-02-21T13:45:34.4595882Z 82   | |         let _string = "LEAKED FROM CLOSURE".to_string();
2020-02-21T13:45:34.4596416Z 83   | |         do_panic_counter(do_panic)
2020-02-21T13:45:34.4596739Z 84   | |     })).expect_err("do_panic() did not panic!");
2020-02-21T13:45:34.4596994Z      | |_______^
2020-02-21T13:45:34.4597361Z note: inside call to `test::<[closure@$DIR/catch_panic.rs:67:10: 67:69]>` at $DIR/catch_panic.rs:67:5
2020-02-21T13:45:34.4598217Z      |
2020-02-21T13:45:34.4598217Z      |
2020-02-21T13:45:34.4598907Z 67   |     test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd
2020-02-21T13:45:34.4599802Z      = note: inside call to `main` at /checkout/src/libstd/rt.rs:67:34
2020-02-21T13:45:34.4600358Z      = note: inside call to closure at /checkout/src/libstd/rt.rs:52:73
2020-02-21T13:45:34.4600974Z      = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:130:5
2020-02-21T13:45:34.4602442Z      = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:6028 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:52:13
---
2020-02-21T13:45:34.4618785Z thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', $DIR/catch_panic.rs:61:34
2020-02-21T13:45:34.4619420Z Caught panic message (String): index out of bounds: the len is 3 but the index is 4
2020-02-21T13:45:34.4620115Z thread 'main' panicked at 'attempt to divide by zero', $DIR/catch_panic.rs:62:34
2020-02-21T13:45:34.4620869Z Caught panic message (String): attempt to divide by zero
2020-02-21T13:45:34.4621935Z thread 'main' panicked at 'assertion failed: false', $DIR/catch_panic.rs:65:23
2020-02-21T13:45:34.4622855Z Caught panic message (&str): assertion failed: false
2020-02-21T13:45:34.4623482Z thread 'main' panicked at 'assertion failed: false', $DIR/catch_panic.rs:66:23
2020-02-21T13:45:34.4624079Z Caught panic message (&str): assertion failed: false
2020-02-21T13:45:34.4624676Z thread 'main' panicked at 'attempt to copy from unaligned or null pointer', $LOC
2020-02-21T13:45:34.4625098Z Caught panic message (String): attempt to copy from unaligned or null pointer
2020-02-21T13:45:34.4625528Z 
2020-02-21T13:45:34.4625628Z 
2020-02-21T13:45:34.4625773Z diff of stderr:
2020-02-21T13:45:34.4625920Z 
---
2020-02-21T13:45:34.4635793Z  thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', $DIR/catch_panic.rs:61:34
2020-02-21T13:45:34.4636345Z  Caught panic message (String): index out of bounds: the len is 3 but the index is 4
2020-02-21T13:45:34.4637032Z  thread 'main' panicked at 'attempt to divide by zero', $DIR/catch_panic.rs:62:34
2020-02-21T13:45:34.4637452Z  Caught panic message (String): attempt to divide by zero
2020-02-21T13:45:34.4638078Z  thread 'main' panicked at 'assertion failed: false', $DIR/catch_panic.rs:65:23
2020-02-21T13:45:34.4638496Z  Caught panic message (&str): assertion failed: false
2020-02-21T13:45:34.4639148Z  thread 'main' panicked at 'assertion failed: false', $DIR/catch_panic.rs:66:23
2020-02-21T13:45:34.4639563Z  Caught panic message (&str): assertion failed: false
2020-02-21T13:45:34.4640159Z -thread 'main' panicked at 'attempt to copy from unaligned or null pointer', $LOC
2020-02-21T13:45:34.4640810Z -Caught panic message (String): attempt to copy from unaligned or null pointer
2020-02-21T13:45:34.4641521Z +error: Miri evaluation error: dangling pointer was dereferenced
2020-02-21T13:45:34.4642082Z +    --> /checkout/src/libcore/intrinsics.rs:1528:5
2020-02-21T13:45:34.4642320Z +     |
2020-02-21T13:45:34.4642320Z +     |
2020-02-21T13:45:34.4642542Z +1528 |     copy_nonoverlapping(src, dst, count)
2020-02-21T13:45:34.4642915Z +     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dangling pointer was dereferenced
2020-02-21T13:45:34.4643576Z +     = note: inside call to `std::intrinsics::copy_nonoverlapping::<i32>` at /checkout/src/libcore/ptr/mod.rs:623:5
2020-02-21T13:45:34.4644337Z +     = note: inside call to `std::ptr::read::<i32>` at /checkout/src/libcore/ptr/const_ptr.rs:585:9
2020-02-21T13:45:34.4644890Z +note: inside call to `std::ptr::const_ptr::<impl *const i32>::read` at $DIR/catch_panic.rs:67:32
2020-02-21T13:45:34.4645516Z +    --> $DIR/catch_panic.rs:67:32
2020-02-21T13:45:34.4645516Z +    --> $DIR/catch_panic.rs:67:32
2020-02-21T13:45:34.4645739Z +     |
2020-02-21T13:45:34.4646430Z +67   |     test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd
2020-02-21T13:45:34.4647259Z +note: inside call to closure at $DIR/catch_panic.rs:39:5
2020-02-21T13:45:34.4647760Z +    --> $DIR/catch_panic.rs:39:5
2020-02-21T13:45:34.4647966Z +     |
2020-02-21T13:45:34.4648169Z +39   |     do_panic(old_val);
2020-02-21T13:45:34.4648169Z +39   |     do_panic(old_val);
2020-02-21T13:45:34.4648393Z +     |     ^^^^^^^^^^^^^^^^^
2020-02-21T13:45:34.4648811Z +note: inside call to `do_panic_counter::<[closure@$DIR/catch_panic.rs:67:10: 67:69]>` at $DIR/catch_panic.rs:83:9
2020-02-21T13:45:34.4649644Z +     |
2020-02-21T13:45:34.4650011Z +83   |         do_panic_counter(do_panic)
2020-02-21T13:45:34.4650289Z +     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-21T13:45:34.4650624Z +     = note: inside call to closure at /checkout/src/libstd/panic.rs:318:9
2020-02-21T13:45:34.4650624Z +     = note: inside call to closure at /checkout/src/libstd/panic.rs:318:9
2020-02-21T13:45:34.4651583Z +     = note: inside call to `<std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]> as std::ops::FnOnce<()>>::call_once` at /checkout/src/libstd/panicking.rs:303:40
2020-02-21T13:45:34.4654475Z +     = note: inside call to `std::panicking::r#try::do_call::<std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]>, ()>` at /checkout/src/libstd/panicking.rs:281:13
2020-02-21T13:45:34.4655701Z +     = note: inside call to `std::panicking::r#try::<(), std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]>>` at /checkout/src/libstd/panic.rs:394:14
2020-02-21T13:45:34.4656729Z +note: inside call to `std::panic::catch_unwind::<std::panic::AssertUnwindSafe<[closure@$DIR/catch_panic.rs:81:45: 84:6 do_panic:[closure@$DIR/catch_panic.rs:67:10: 67:69]]>, ()>` at $DIR/catch_panic.rs:81:15
2020-02-21T13:45:34.4657972Z +     |
2020-02-21T13:45:34.4658214Z +81   |       let res = catch_unwind(AssertUnwindSafe(|| {
2020-02-21T13:45:34.4658644Z +     |  _______________^
2020-02-21T13:45:34.4658644Z +     |  _______________^
2020-02-21T13:45:34.4658923Z +82   | |         let _string = "LEAKED FROM CLOSURE".to_string();
2020-02-21T13:45:34.4659250Z +83   | |         do_panic_counter(do_panic)
2020-02-21T13:45:34.4659545Z +84   | |     })).expect_err("do_panic() did not panic!");
2020-02-21T13:45:34.4659791Z +     | |_______^
2020-02-21T13:45:34.4660349Z +note: inside call to `test::<[closure@$DIR/catch_panic.rs:67:10: 67:69]>` at $DIR/catch_panic.rs:67:5
2020-02-21T13:45:34.4661150Z +     |
2020-02-21T13:45:34.4661150Z +     |
2020-02-21T13:45:34.4662115Z +67   |     test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd
2020-02-21T13:45:34.4663353Z +     = note: inside call to `main` at /checkout/src/libstd/rt.rs:67:34
2020-02-21T13:45:34.4663943Z +     = note: inside call to closure at /checkout/src/libstd/rt.rs:52:73
2020-02-21T13:45:34.4664386Z +     = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:130:5
2020-02-21T13:45:34.4665812Z +     = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:6028 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:52:13
---
2020-02-21T13:45:34.4715888Z +
2020-02-21T13:45:34.4716006Z  
2020-02-21T13:45:34.4716112Z 
2020-02-21T13:45:34.4716353Z The actual stderr differed from the expected stderr.
2020-02-21T13:45:34.4716846Z Actual stderr saved to /tmp/compiletestnYvk8q/panic/catch_panic.stderr
2020-02-21T13:45:34.4717187Z To update references, run this command from build directory:
2020-02-21T13:45:34.4718171Z tests/run-pass/update-references.sh '/tmp/compiletestnYvk8q' 'panic/catch_panic.rs'
2020-02-21T13:45:34.4718611Z error: 1 errors occurred comparing output.
2020-02-21T13:45:34.4718862Z status: exit code: 1
2020-02-21T13:45:34.4718862Z status: exit code: 1
2020-02-21T13:45:34.4720349Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestnYvk8q" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestnYvk8q/panic/catch_panic.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestnYvk8q/panic/catch_panic.stage-id.aux" "-A" "unused"
2020-02-21T13:45:34.4721670Z ------------------------------------------
2020-02-21T13:45:34.4721846Z 
2020-02-21T13:45:34.4722382Z ------------------------------------------
2020-02-21T13:45:34.4722777Z stderr:
---
2020-02-21T13:45:34.4731704Z thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', tests/run-pass/panic/catch_panic.rs:61:34
2020-02-21T13:45:34.4732280Z Caught panic message (String): index out of bounds: the len is 3 but the index is 4
2020-02-21T13:45:34.4733158Z thread 'main' panicked at 'attempt to divide by zero', tests/run-pass/panic/catch_panic.rs:62:34
2020-02-21T13:45:34.4733871Z Caught panic message (String): attempt to divide by zero
2020-02-21T13:45:34.4734581Z thread 'main' panicked at 'assertion failed: false', tests/run-pass/panic/catch_panic.rs:65:23
2020-02-21T13:45:34.4735024Z Caught panic message (&str): assertion failed: false
2020-02-21T13:45:34.4735686Z thread 'main' panicked at 'assertion failed: false', tests/run-pass/panic/catch_panic.rs:66:23
2020-02-21T13:45:34.4736146Z Caught panic message (&str): assertion failed: false
2020-02-21T13:45:34.4780453Z {"message":"Miri evaluation error: dangling pointer was dereferenced","code":null,"level":"error","spans":[{"file_name":"/checkout/src/libcore/intrinsics.rs","byte_start":75706,"byte_end":75742,"line_start":1528,"line_end":1528,"column_start":5,"column_end":41,"is_primary":true,"text":[{"text":"    copy_nonoverlapping(src, dst, count)","highlight_start":5,"highlight_end":41}],"label":"dangling pointer was dereferenced","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"inside call to `std::intrinsics::copy_nonoverlapping::<i32>` at /checkout/src/libcore/ptr/mod.rs:623:5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::ptr::read::<i32>` at /checkout/src/libcore/ptr/const_ptr.rs:585:9","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::ptr::const_ptr::<impl *const i32>::read` at tests/run-pass/panic/catch_panic.rs:67:32","code":null,"level":"note","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":2201,"byte_end":2225,"line_start":67,"line_end":67,"column_start":32,"column_end":56,"is_primary":true,"text":[{"text":"    test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd","highlight_start":***@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]>` at tests/run-pass/panic/catch_panic.rs:83:9","code":null,"level":"note","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":2712,"byte_end":2738,"line_start":83,"line_end":83,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"        do_panic_counter(do_panic)","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"inside call to closure at /checkout/src/libstd/panic.rs:318:9","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `<std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]> as std::ops::FnOnce<()>>::call_once` at /checkout/src/libstd/panicking.rs:303:40","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::panicking::r#try::do_call::<std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]>, ()>` at /checkout/src/libstd/panicking.rs:281:13","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::panicking::r#try::<(), std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]>>` at /checkout/src/libstd/panic.rs:394:14","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::panic::catch_unwind::<std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]>, ()>` at tests/run-pass/panic/catch_panic.rs:81:15","code":null,"level":"note","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":2612,"byte_end":2746,"line_start":81,"line_end":84,"column_start":15,"column_end":8,"is_primary":true,"text":[{"text":"    let res = catch_unwind(AssertUnwindSafe(|| {","highlight_start":15,"highlight_end":49},{"text":"        let _string = \"LEAKED FROM CLOSURE\".to_string();","highlight_start":1,"highlight_end":57},{"text":"        do_panic_counter(do_panic)","highlight_start":1,"highlight_end":35},{"text":"    })).expect_err(\"do_panic() did not panic!\");","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"inside call to `test::<[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]>` at tests/run-pass/panic/catch_panic.rs:67:5","code":null,"level":"note","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":2174,"byte_end":2239,"line_start":67,"line_end":67,"column_start":5,"column_end":70,"is_primary":true,"text":[{"text":"    test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd","highlight_start":***@DefId(1:6028 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:52:13","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to closure at /checkout/src/libstd/panicking.rs:303:40","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::panicking::r#try::do_call::<[closure@DefId(1:6027 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:281:13","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::panicking::r#try::<i32, [closure@DefId(1:6027 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:14","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::panic::catch_unwind::<[closure@DefId(1:6027 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:51:25","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::rt::lang_start_internal` at /checkout/src/libstd/rt.rs:67:5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside call to `std::rt::lang_start::<()>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: Miri evaluation error: dangling pointer was dereferenced\n    --> /checkout/src/libcore/intrinsics.rs:1528:5\n     |\n1528 |     copy_nonoverlapping(src, dst, count)\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dangling pointer was dereferenced\n     |\n     = note: inside call to `std::intrinsics::copy_nonoverlapping::<i32>` at /checkout/src/libcore/ptr/mod.rs:623:5\n     = note: inside call to `std::ptr::read::<i32>` at /checkout/src/libcore/ptr/const_ptr.rs:585:9\nnote: inside call to `std::ptr::const_ptr::<impl *const i32>::read` at tests/run-pass/panic/catch_panic.rs:67:32\n    --> tests/run-pass/panic/catch_panic.rs:67:32\n     |\n67   |     test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd\n     |                                ^^^^^^^^^^^^^^^^^^^^^^^^\nnote:***@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]>` at tests/run-pass/panic/catch_panic.rs:83:9\n    --> tests/run-pass/panic/catch_panic.rs:83:9\n     |\n83   |         do_panic_counter(do_panic)\n     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n     = note: inside call to closure at /checkout/src/libstd/panic.rs:318:9\n     = note: inside call to `<std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]> as std::ops::FnOnce<()>>::call_once` at /checkout/src/libstd/panicking.rs:303:40\n     = note: inside call to `std::panicking::r#try::do_call::<std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]>, ()>` at /checkout/src/libstd/panicking.rs:281:13\n     = note: inside call to `std::panicking::r#try::<(), std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]>>` at /checkout/src/libstd/panic.rs:394:14\nnote: inside call to `std::panic::catch_unwind::<std::panic::AssertUnwindSafe<[closure@tests/run-pass/panic/catch_panic.rs:81:45: 84:6 do_panic:[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]]>, ()>` at tests/run-pass/panic/catch_panic.rs:81:15\n    --> tests/run-pass/panic/catch_panic.rs:81:15\n     |\n81   |       let res = catch_unwind(AssertUnwindSafe(|| {\n     |  _______________^\n82   | |         let _string = \"LEAKED FROM CLOSURE\".to_string();\n83   | |         do_panic_counter(do_panic)\n84   | |     })).expect_err(\"do_panic() did not panic!\");\n     | |_______^\nnote: inside call to `test::<[closure@tests/run-pass/panic/catch_panic.rs:67:10: 67:69]>` at tests/run-pass/panic/catch_panic.rs:67:5\n    --> tests/run-pass/panic/catch_panic.rs:67:5\n     |\n67   |     test(|_old_val| { unsafe { (1 as *const i32).read() }; loop {} }); // trigger debug-assertion in libstd\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n     = note:***@DefId(1:6028 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:52:13\n     = note: inside call to closure at /checkout/src/libstd/panicking.rs:303:40\n     = note: inside call to `std::panicking::r#try::do_call::<[closure@DefId(1:6027 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:281:13\n     = note: inside call to `std::panicking::r#try::<i32, [closure@DefId(1:6027 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:14\n     = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:6027 ~ std[bc00]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:51:25\n     = note: inside call to `std::rt::lang_start_internal` at /checkout/src/libstd/rt.rs:67:5\n     = note: inside call to `std::rt::lang_start::<()>`\n\n"}
2020-02-21T13:45:34.4807222Z 
2020-02-21T13:45:34.4808350Z ------------------------------------------
2020-02-21T13:45:34.4808545Z 
2020-02-21T13:45:34.4809132Z test [ui] run-pass/panic/catch_panic.rs ... FAILED
---
2020-02-21T13:45:43.8337218Z 
2020-02-21T13:45:43.8337685Z If you do intend to update 'miri', please check the error messages above and
2020-02-21T13:45:43.8337985Z commit another update.
2020-02-21T13:45:43.8338337Z 
2020-02-21T13:45:43.8338844Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2020-02-21T13:45:43.8339447Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2020-02-21T13:45:43.8339746Z proper steps.
2020-02-21T13:45:43.8340683Z Build completed unsuccessfully in 0:00:01
2020-02-21T13:45:43.8391586Z == clock drift check ==
2020-02-21T13:45:43.8402704Z   local time: Fri Feb 21 13:45:43 UTC 2020
2020-02-21T13:45:44.1336893Z   network time: Fri, 21 Feb 2020 13:45:44 GMT
2020-02-21T13:45:44.1336893Z   network time: Fri, 21 Feb 2020 13:45:44 GMT
2020-02-21T13:45:44.1337219Z == end clock drift check ==
2020-02-21T13:45:44.6907026Z 
2020-02-21T13:45:44.6997473Z ##[error]Bash exited with code '1'.
2020-02-21T13:45:44.7012436Z ##[section]Finishing: Run build
2020-02-21T13:45:44.7075138Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69344/merge to s
2020-02-21T13:45:44.7080806Z Task         : Get sources
2020-02-21T13:45:44.7081337Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T13:45:44.7081662Z Version      : 1.0.0
2020-02-21T13:45:44.7082064Z Author       : Microsoft
2020-02-21T13:45:44.7082064Z Author       : Microsoft
2020-02-21T13:45:44.7082448Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T13:45:44.7082880Z ==============================================================================
2020-02-21T13:45:45.0853912Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T13:45:45.0903082Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69344/merge to s
2020-02-21T13:45:45.0996684Z Cleaning up task key
2020-02-21T13:45:45.0998473Z Start cleaning up orphan processes.
2020-02-21T13:45:45.1183301Z Terminate orphan process: pid (3723) (python)
2020-02-21T13:45:45.1442231Z ##[section]Finishing: Finalize Job
