plain
2019-09-10T17:18:24.8871075Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T17:18:24.9090802Z ##[command]git config gc.auto 0
2019-09-10T17:18:24.9225228Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T17:18:24.9312454Z ##[command]git config --get-all http.proxy
2019-09-10T17:18:25.8186646Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64359/merge:refs/remotes/pull/64359/merge
---
2019-09-10T18:30:27.8764339Z .................................................................................................... 1500/9007
2019-09-10T18:30:34.6744975Z .................................................................................................... 1600/9007
2019-09-10T18:30:49.5371289Z .......................................................i...............i............................ 1700/9007
2019-09-10T18:30:58.3405540Z .................................................................................................... 1800/9007
2019-09-10T18:31:15.3991932Z ..............................................iiiii................................................. 1900/9007
2019-09-10T18:31:28.1850314Z .................................................................................................... 2100/9007
2019-09-10T18:31:31.2635744Z .................................................................................................... 2200/9007
2019-09-10T18:31:35.6240885Z .................................................................................................... 2300/9007
2019-09-10T18:31:44.9032985Z .................................................................................................... 2400/9007
---
2019-09-10T18:35:11.4892104Z .................................i...............i.................................................. 4700/9007
2019-09-10T18:35:24.9108724Z .................................................................................................... 4800/9007
2019-09-10T18:35:32.0346966Z .................................................................................................... 4900/9007
2019-09-10T18:35:44.3867804Z .................................................................................................... 5000/9007
2019-09-10T18:35:51.4064847Z ................ii.ii............................................................................... 5100/9007
2019-09-10T18:36:03.4076582Z .................................................................................................... 5300/9007
2019-09-10T18:36:14.9086507Z ................................................................................i................... 5400/9007
2019-09-10T18:36:23.8085301Z .................................................................................................... 5500/9007
2019-09-10T18:36:30.7003805Z .................................................................................................... 5600/9007
2019-09-10T18:36:30.7003805Z .................................................................................................... 5600/9007
2019-09-10T18:36:42.5728683Z ..........................................................................ii...i..ii...........i.... 5700/9007
2019-09-10T18:37:10.9393435Z .................................................................................................... 5900/9007
2019-09-10T18:37:22.3475387Z .................................................................................................... 6000/9007
2019-09-10T18:37:22.3475387Z .................................................................................................... 6000/9007
2019-09-10T18:37:33.6280178Z ............................................................................i..ii................... 6100/9007
2019-09-10T18:38:07.6855185Z .................................................................................................... 6300/9007
2019-09-10T18:38:10.2176134Z ...................................i................................................................ 6400/9007
2019-09-10T18:38:12.7738797Z .................................................................................................... 6500/9007
2019-09-10T18:38:15.8146685Z .......i............................................................................................ 6600/9007
---
2019-09-10T18:43:00.2628131Z 
2019-09-10T18:43:00.2628278Z 
2019-09-10T18:43:00.2628427Z The actual stderr differed from the expected stderr.
2019-09-10T18:43:00.2628874Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes/lint-ctypes.stderr
2019-09-10T18:43:00.2629360Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T18:43:00.2629847Z To only update this specific test, also pass `--test-args lint/lint-ctypes.rs`
2019-09-10T18:43:00.2630218Z error: 1 errors occurred comparing output.
2019-09-10T18:43:00.2630382Z status: exit code: 1
2019-09-10T18:43:00.2630382Z status: exit code: 1
2019-09-10T18:43:00.2631290Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes/auxiliary" "-A" "unused"
2019-09-10T18:43:00.2722805Z ------------------------------------------
2019-09-10T18:43:00.2722875Z 
2019-09-10T18:43:00.2723168Z ------------------------------------------
2019-09-10T18:43:00.2723222Z stderr:
2019-09-10T18:43:00.2723222Z stderr:
2019-09-10T18:43:00.2723450Z ------------------------------------------
2019-09-10T18:43:00.2723756Z error: `extern` block uses type `Foo` which is not FFI-safe: this struct has unspecified layout
2019-09-10T18:43:00.2724090Z    |
2019-09-10T18:43:00.2724090Z    |
2019-09-10T18:43:00.2724159Z LL |     pub fn ptr_type1(size: *const Foo); //~ ERROR: uses type `Foo`
2019-09-10T18:43:00.2724654Z    |
2019-09-10T18:43:00.2724731Z note: lint level defined here
2019-09-10T18:43:00.2729977Z   --> /checkout/src/test/ui/lint/lint-ctypes.rs:4:9
2019-09-10T18:43:00.2730049Z    |
---
2019-09-10T18:43:00.2730628Z    |
2019-09-10T18:43:00.2730670Z LL | pub struct Foo;
2019-09-10T18:43:00.2730733Z    | ^^^^^^^^^^^^^^^
2019-09-10T18:43:00.2730773Z 
2019-09-10T18:43:00.2731065Z error: `extern` block uses type `Foo` which is not FFI-safe: this struct has unspecified layout
2019-09-10T18:43:00.2731385Z    |
2019-09-10T18:43:00.2731385Z    |
2019-09-10T18:43:00.2731434Z LL |     pub fn ptr_type2(size: *const Foo); //~ ERROR: uses type `Foo`
2019-09-10T18:43:00.2731553Z    |
2019-09-10T18:43:00.2731605Z    = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-09-10T18:43:00.2731664Z note: type defined here
2019-09-10T18:43:00.2731929Z   --> /checkout/src/test/ui/lint/lint-ctypes.rs:24:1
2019-09-10T18:43:00.2731929Z   --> /checkout/src/test/ui/lint/lint-ctypes.rs:24:1
2019-09-10T18:43:00.2731979Z    |
2019-09-10T18:43:00.2732021Z LL | pub struct Foo;
2019-09-10T18:43:00.2732082Z    | ^^^^^^^^^^^^^^^
2019-09-10T18:43:00.2732111Z 
2019-09-10T18:43:00.2732387Z error: `extern` block uses type `[u32]` which is not FFI-safe: slices have no C equivalent
2019-09-10T18:43:00.2732945Z    |
2019-09-10T18:43:00.2732945Z    |
2019-09-10T18:43:00.2732995Z LL |     pub fn slice_type(p: &[u32]); //~ ERROR: uses type `[u32]`
2019-09-10T18:43:00.2733118Z    |
2019-09-10T18:43:00.2733164Z    = help: consider using a raw pointer instead
2019-09-10T18:43:00.2733195Z 
2019-09-10T18:43:00.2733195Z 
2019-09-10T18:43:00.2733511Z error: `extern` block uses type `str` which is not FFI-safe: string slices have no C equivalent
2019-09-10T18:43:00.2733839Z    |
2019-09-10T18:43:00.2733839Z    |
2019-09-10T18:43:00.2733887Z LL |     pub fn str_type(p: &str); //~ ERROR: uses type `str`
2019-09-10T18:43:00.2733995Z    |
2019-09-10T18:43:00.2733995Z    |
2019-09-10T18:43:00.2734042Z    = help: consider using `*const u8` and a length instead
2019-09-10T18:43:00.2734074Z 
2019-09-10T18:43:00.2734929Z error: `extern` block uses type `std::boxed::Box<u32>` which is not FFI-safe: this struct has unspecified layout
2019-09-10T18:43:00.2735248Z    |
2019-09-10T18:43:00.2735248Z    |
2019-09-10T18:43:00.2735319Z LL |     pub fn box_type(p: Box<u32>); //~ ERROR uses type `std::boxed::Box<u32>`
2019-09-10T18:43:00.2735413Z    |
2019-09-10T18:43:00.2735483Z    = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-09-10T18:43:00.2735519Z 
2019-09-10T18:43:00.2735519Z 
2019-09-10T18:43:00.2736162Z error: `extern` block uses type `char` which is not FFI-safe: the `char` type has no C equivalent
2019-09-10T18:43:00.2737013Z    |
2019-09-10T18:43:00.2737013Z    |
2019-09-10T18:43:00.2737061Z LL |     pub fn char_type(p: char); //~ ERROR uses type `char`
2019-09-10T18:43:00.2737170Z    |
2019-09-10T18:43:00.2737170Z    |
2019-09-10T18:43:00.2737228Z    = help: consider using `u32` or `libc::wchar_t` instead
2019-09-10T18:43:00.2737260Z 
2019-09-10T18:43:00.2737592Z error: `extern` block uses type `i128` which is not FFI-safe: 128-bit integers don't currently have a known stable ABI
2019-09-10T18:43:00.2737891Z    |
2019-09-10T18:43:00.2737891Z    |
2019-09-10T18:43:00.2737957Z LL |     pub fn i128_type(p: i128); //~ ERROR uses type `i128`
2019-09-10T18:43:00.2738035Z 
2019-09-10T18:43:00.2738035Z 
2019-09-10T18:43:00.2738336Z error: `extern` block uses type `u128` which is not FFI-safe: 128-bit integers don't currently have a known stable ABI
2019-09-10T18:43:00.2738665Z    |
2019-09-10T18:43:00.2738665Z    |
2019-09-10T18:43:00.2738711Z LL |     pub fn u128_type(p: u128); //~ ERROR uses type `u128`
2019-09-10T18:43:00.2738807Z 
2019-09-10T18:43:00.2738807Z 
2019-09-10T18:43:00.2739109Z error: `extern` block uses type `dyn std::clone::Clone` which is not FFI-safe: trait objects have no C equivalent
2019-09-10T18:43:00.2739422Z    |
2019-09-10T18:43:00.2739422Z    |
2019-09-10T18:43:00.2739472Z LL |     pub fn trait_type(p: &dyn Clone); //~ ERROR uses type `dyn std::clone::Clone`
2019-09-10T18:43:00.2739572Z 
2019-09-10T18:43:00.2739572Z 
2019-09-10T18:43:00.2739855Z error: `extern` block uses type `(i32, i32)` which is not FFI-safe: tuples have unspecified layout
2019-09-10T18:43:00.2740176Z    |
2019-09-10T18:43:00.2740176Z    |
2019-09-10T18:43:00.2740225Z LL |     pub fn tuple_type(p: (i32, i32)); //~ ERROR uses type `(i32, i32)`
2019-09-10T18:43:00.2740319Z    |
2019-09-10T18:43:00.2740382Z    = help: consider using a struct instead
2019-09-10T18:43:00.2740411Z 
2019-09-10T18:43:00.2740411Z 
2019-09-10T18:43:00.2741175Z error: `extern` block uses type `(i32, i32)` which is not FFI-safe: tuples have unspecified layout
2019-09-10T18:43:00.2741551Z    |
2019-09-10T18:43:00.2741551Z    |
2019-09-10T18:43:00.2741600Z LL |     pub fn tuple_type2(p: I32Pair); //~ ERROR uses type `(i32, i32)`
2019-09-10T18:43:00.2741711Z    |
2019-09-10T18:43:00.2741755Z    = help: consider using a struct instead
2019-09-10T18:43:00.2741785Z 
2019-09-10T18:43:00.2741785Z 
2019-09-10T18:43:00.2742080Z error: `extern` block uses type `ZeroSize` which is not FFI-safe: this struct has no fields
2019-09-10T18:43:00.2742386Z    |
2019-09-10T18:43:00.2742386Z    |
2019-09-10T18:43:00.2742452Z LL |     pub fn zero_size(p: ZeroSize); //~ ERROR struct has no fields
2019-09-10T18:43:00.2742544Z    |
2019-09-10T18:43:00.2742588Z    = help: consider adding a member to this struct
2019-09-10T18:43:00.2742663Z note: type defined here
2019-09-10T18:43:00.2742907Z   --> /checkout/src/test/ui/lint/lint-ctypes.rs:20:1
2019-09-10T18:43:00.2742907Z   --> /checkout/src/test/ui/lint/lint-ctypes.rs:20:1
2019-09-10T18:43:00.2742956Z    |
2019-09-10T18:43:00.2743016Z LL | pub struct ZeroSize;
2019-09-10T18:43:00.2743090Z 
2019-09-10T18:43:00.2743090Z 
2019-09-10T18:43:00.2743383Z error: `extern` block uses type `ZeroSizeWithPhantomData` which is not FFI-safe: composed only of `PhantomData`
2019-09-10T18:43:00.2743695Z    |
2019-09-10T18:43:00.2743695Z    |
2019-09-10T18:43:00.2743842Z LL |     pub fn zero_size_phantom(p: ZeroSizeWithPhantomData); //~ ERROR composed only of `PhantomData`
2019-09-10T18:43:00.2743946Z 
2019-09-10T18:43:00.2743946Z 
2019-09-10T18:43:00.2744560Z error: `extern` block uses type `std::marker::PhantomData<bool>` which is not FFI-safe: composed only of `PhantomData`
2019-09-10T18:43:00.2744923Z    |
2019-09-10T18:43:00.2744923Z    |
2019-09-10T18:43:00.2745203Z LL |         -> ::std::marker::PhantomData<bool>; //~ ERROR: composed only of `PhantomData`
2019-09-10T18:43:00.2745305Z 
2019-09-10T18:43:00.2745305Z 
2019-09-10T18:43:00.2745605Z error: `extern` block uses type `fn()` which is not FFI-safe: this function pointer has Rust-specific calling convention
2019-09-10T18:43:00.2748534Z    |
2019-09-10T18:43:00.2748534Z    |
2019-09-10T18:43:00.2748983Z LL |     pub fn fn_type(p: RustFn); //~ ERROR function pointer has Rust-specific
2019-09-10T18:43:00.2749099Z    |
2019-09-10T18:43:00.2749099Z    |
2019-09-10T18:43:00.2749369Z    = help: consider using an `extern fn(...) -> ...` function pointer instead
2019-09-10T18:43:00.2749418Z 
2019-09-10T18:43:00.2749733Z error: `extern` block uses type `fn()` which is not FFI-safe: this function pointer has Rust-specific calling convention
2019-09-10T18:43:00.2750035Z    |
2019-09-10T18:43:00.2750035Z    |
2019-09-10T18:43:00.2750308Z LL |     pub fn fn_type2(p: fn()); //~ ERROR function pointer has Rust-specific
2019-09-10T18:43:00.2750405Z    |
2019-09-10T18:43:00.2750405Z    |
2019-09-10T18:43:00.2750684Z    = help: consider using an `extern fn(...) -> ...` function pointer instead
2019-09-10T18:43:00.2750721Z 
2019-09-10T18:43:00.2751012Z error: `extern` block uses type `std::boxed::Box<u32>` which is not FFI-safe: this struct has unspecified layout
2019-09-10T18:43:00.2751338Z    |
2019-09-10T18:43:00.2751338Z    |
2019-09-10T18:43:00.2751388Z LL |     pub fn fn_contained(p: RustBadRet); //~ ERROR: uses type `std::boxed::Box<u32>`
2019-09-10T18:43:00.2751502Z    |
2019-09-10T18:43:00.2751709Z    = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-09-10T18:43:00.2751755Z 
2019-09-10T18:43:00.2751755Z 
2019-09-10T18:43:00.2752105Z error: `extern` block uses type `i128` which is not FFI-safe: 128-bit integers don't currently have a known stable ABI
2019-09-10T18:43:00.2752405Z    |
2019-09-10T18:43:00.2752405Z    |
2019-09-10T18:43:00.2752468Z LL |     pub fn transparent_i128(p: TransparentI128); //~ ERROR: uses type `i128`
2019-09-10T18:43:00.2752561Z 
2019-09-10T18:43:00.2752561Z 
2019-09-10T18:43:00.2752843Z error: `extern` block uses type `str` which is not FFI-safe: string slices have no C equivalent
2019-09-10T18:43:00.2753157Z    |
2019-09-10T18:43:00.2753157Z    |
2019-09-10T18:43:00.2753205Z LL |     pub fn transparent_str(p: TransparentStr); //~ ERROR: uses type `str`
2019-09-10T18:43:00.2753326Z    |
2019-09-10T18:43:00.2753326Z    |
2019-09-10T18:43:00.2753373Z    = help: consider using `*const u8` and a length instead
2019-09-10T18:43:00.2753423Z 
2019-09-10T18:43:00.2753724Z error: `extern` block uses type `std::boxed::Box<u32>` which is not FFI-safe: this struct has unspecified layout
2019-09-10T18:43:00.2754021Z    |
2019-09-10T18:43:00.2754021Z    |
2019-09-10T18:43:00.2754088Z LL |     pub fn transparent_fn(p: TransparentBadFn); //~ ERROR: uses type `std::boxed::Box<u32>`
2019-09-10T18:43:00.2754668Z    |
2019-09-10T18:43:00.2754834Z    = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-09-10T18:43:00.2754869Z 
2019-09-10T18:43:00.2754913Z error: aborting due to 20 previous errors
---
2019-09-10T18:43:00.2756366Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-10T18:43:00.2756427Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-10T18:43:00.2756487Z 
2019-09-10T18:43:00.2756513Z 
2019-09-10T18:43:00.2758614Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-10T18:43:00.2759052Z 
2019-09-10T18:43:00.2759095Z 
2019-09-10T18:43:00.2759147Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-10T18:43:00.2759217Z Build completed unsuccessfully in 1:16:33
2019-09-10T18:43:00.2759217Z Build completed unsuccessfully in 1:16:33
2019-09-10T18:43:00.2792167Z == clock drift check ==
2019-09-10T18:43:00.2808814Z   local time: Tue Sep 10 18:43:00 UTC 2019
2019-09-10T18:43:00.5130068Z   network time: Tue, 10 Sep 2019 18:43:00 GMT
2019-09-10T18:43:00.5137898Z == end clock drift check ==
2019-09-10T18:43:01.2685881Z ##[error]Bash exited with code '1'.
2019-09-10T18:43:01.2730827Z ##[section]Starting: Checkout
2019-09-10T18:43:01.2733415Z ==============================================================================
2019-09-10T18:43:01.2733473Z Task         : Get sources
2019-09-10T18:43:01.2733538Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
