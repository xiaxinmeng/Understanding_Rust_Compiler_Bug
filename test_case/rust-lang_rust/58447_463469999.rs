plain
travis_time:end:006f4492:start=1550109066358109354,finish=1550109142645167532,duration=76287058178
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:07] 
[01:11:07] running 119 tests
[01:11:32] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:36] i......iii.i.....ii
[01:11:36] 
[01:11:36]  finished in 29.043
[01:11:36] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:19] 
[01:15:19] running 296 tests
[01:16:20] ....FF..F.......F..FFF.....i....F...F...........F.F...FF.F...........FF..F.......F..........F.FF.... 100/296
[01:17:12] .....F...........FFFFF.FFFF...F..F.....iFF..F................F......FFF..F....F..................... 200/296
[01:18:04] ......................F............FF.F.........F..............F...................F..F...FFFFFF
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/assoc-types.rs stdout ----
[01:18:04] 
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 10: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//code[@id="index.v"]' 'fn index'
[01:18:04] 11: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//*[@id="tymethod.index"]//code' "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
[01:18:04] Encountered 2 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/assoc-types.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/async-fn.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 5: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has async_fn/fn.foo.html '//pre[@class="rust fn"]' 'pub async fn foo() -> Option<Foo>'
[01:18:04] 10: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has async_fn/fn.bar.html '//pre[@class="rust fn"]' 'pub async fn bar(a: i32, b: i32) -> i32'
[01:18:04] 15: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has async_fn/fn.baz.html '//pre[@class="rust fn"]' 'pub async fn baz<T>(a: T) -> T'
[01:18:04] 24: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has async_fn/fn.quux.html '//pre[@class="rust fn"]' 'pub async fn quux() -> impl Bar'
[01:18:04] 30: @matches check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @matches - '//code' 'pub async fn f\(\)$'
[01:18:04] Encountered 5 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/async-fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/auto-traits.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-traits" "/checkout/src/test/rustdoc/auto-traits.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 9: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/trait.Foo.html' '//pre' 'pub unsafe auto trait Foo'
[01:18:04] 12: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/trait.Bar.html' '//pre' 'pub unsafe auto trait Bar'
[01:18:04] Encountered 2 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/auto-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/const-display.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display" "/checkout/src/test/rustdoc/const-display.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 10: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/fn.foo.html' '//pre' 'pub unsafe fn foo() -> u32'
[01:18:04] 15: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/fn.foo2.html' '//pre' 'pub fn foo2() -> u32'
[01:18:04] 19: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/fn.bar2.html' '//pre' 'pub const fn bar2() -> u32'
[01:18:04] 23: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/fn.foo2_gated.html' '//pre' 'pub unsafe fn foo2_gated() -> u32'
[01:18:04] 27: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/fn.bar2_gated.html' '//pre' 'pub const unsafe fn bar2_gated() -> u32'
[01:18:04] 31: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has 'foo/fn.bar_not_gated.html' '//pre' 'pub unsafe fn bar_not_gated() -> u32'
[01:18:04] Encountered 6 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/const-display.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/const-fn.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-fn" "/checkout/src/test/rustdoc/const-fn.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 4: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@class="rust fn"]' 'pub const fn bar() -> '
[01:18:04] 11: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@class="method"]' 'const fn new()'
[01:18:04] Encountered 2 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/const-fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/const.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const" "/checkout/src/test/rustdoc/const.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 6: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has const/struct.Foo.html '//code[@id="new.v"]' 'const unsafe fn new'
[01:18:04] Encountered 1 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/constructor-imports.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/constructor-imports" "/checkout/src/test/rustdoc/constructor-imports.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 10: @count check failed
[01:18:04]  Expected 1 occurrences but found 0
[01:18:04]  // @count 'foo/index.html' '//*[code="pub use a::Foo;"]' 1
[01:18:04] 13: @count check failed
[01:18:04]  Expected 1 occurrences but found 0
[01:18:04]  // @count 'foo/index.html' '//*[code="pub use a::Bar::Baz;"]' 1
[01:18:04] Encountered 2 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/constructor-imports.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/doc-cfg.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 6: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="method.unix_and_arm_only_function"]' 'fn unix_and_arm_only_function()'
[01:18:04] Encountered 1 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/doc-cfg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/double-quote-escape.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/double-quote-escape" "/checkout/src/test/rustdoc/double-quote-escape.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 12: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@class="sidebar-links"]/a[@href="#impl-Foo%3Cunsafe%20extern%20%22C%22%20fn()%3E"]' 'Foo<unsafe extern "C" fn()>'
[01:18:04] Encountered 1 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/double-quote-escape.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/extern-impl.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl" "/checkout/src/test/rustdoc/extern-impl.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 7: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//code' 'fn rust0()'
[01:18:04] 9: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//code' 'fn rust1()'
[01:18:04] 11: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//code' 'extern "C" fn c0()'
[01:18:04] 13: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//code' 'extern "C" fn c1()'
[01:18:04] 15: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//code' 'extern "system" fn system0()'
[01:18:04] Encountered 5 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/extern-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/extern-method.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-method" "/checkout/src/test/rustdoc/extern-method.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 8: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has extern_method/trait.Foo.html //pre "pub trait Foo"
[01:18:04] 9: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="tymethod.foo"]//code' 'extern "rust-call" fn foo'
[01:18:04] 10: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="method.foo_"]//code' 'extern "rust-call" fn foo_'
[01:18:04] 13: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has extern_method/trait.Bar.html //pre "pub trait Bar"
[01:18:04] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:18:04] 15: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//*[@id="tymethod.bar"]//code' 'extern "rust-call" fn bar'
[01:18:04] 17: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has - '//*[@id="method.bar_"]//code' 'extern "rust-call" fn bar_'
[01:18:04] Encountered 6 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/extern-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/ffi.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi" "/checkout/src/test/rustdoc/ffi.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 6: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has ffi/fn.foreigner.html //pre 'pub unsafe extern "C" fn foreigner(cold_as_ice: u32)'
[01:18:04] 10: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]      // @has ffi/fn.another.html //pre 'pub unsafe extern "C" fn another(cold_as_ice: u32)'
[01:18:04] Encountered 2 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/ffi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/fn-pointer-arg-name.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/fn-pointer-arg-name" "/checkout/src/test/rustdoc/fn-pointer-arg-name.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 4: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@class="rust fn"]' 'pub fn f(callback: fn(len: usize, foo: u32))'
[01:18:04] Encountered 1 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/fn-pointer-arg-name.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/foreigntype-reexport.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/foreigntype-reexport" "/checkout/src/test/rustdoc/foreigntype-reexport.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 43: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has foreigntype_reexport/index.html '//code' 'pub use self::sub2::C as C3;'
[01:18:04] 44: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has foreigntype_reexport/index.html '//code' 'pub use self::sub2::f as f3;'
[01:18:04] 45: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has foreigntype_reexport/index.html '//code' 'pub use self::sub2::K3;'
[01:18:04] Encountered 3 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/foreigntype-reexport.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/inline-default-methods.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline-default-methods" "/checkout/src/test/rustdoc/inline-default-methods.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 7: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@class="rust trait"]' 'fn bar(&self);'
[01:18:04] 8: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@class="rust trait"]' 'fn foo(&mut self) { ... }'
[01:18:04] Encountered 2 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/inline-default-methods.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 11: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="associatedconstant.PublicConst"]' 'pub const PublicConst: u8'
[01:18:04] 14: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="method.public_method"]' 'pub fn public_method()'
[01:18:04] 24: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="method.method_no_default"]' 'fn method_no_default()'
[01:18:04] 26: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
[01:18:04] 38: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="tymethod.method_no_default"]' 'fn method_no_default()'
[01:18:04] 40: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
[01:18:04] Encountered 6 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/inline_cross/assoc-items.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait" "/checkout/src/test/rustdoc/inline_cross/impl-inline-without-trait.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 10: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//*[@id="method.my_trait_method"]' 'fn my_trait_method()'
[01:18:04] Encountered 1 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/inline_cross/macro-vis.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macro-vis" "/checkout/src/test/rustdoc/inline_cross/macro-vis.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 23: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has macro_vis/index.html '//code' 'pub use qwop::super_macro;'
[01:18:04] Encountered 1 errors
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] 
[01:18:04] thread '[rustdoc] rustdoc/inline_cross/macro-vis.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:04] 
[01:18:04] ---- [rustdoc] rustdoc/inline_local/issue-32343.rs stdout ----
[01:18:04] 
[01:18:04] error: htmldocck failed!
[01:18:04] status: exit code: 1
[01:18:04] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-32343" "/checkout/src/test/rustdoc/inline_local/issue-32343.rs"
[01:18:04] ------------------------------------------
[01:18:04] 
[01:18:04] ------------------------------------------
[01:18:04] stderr:
[01:18:04] stderr:
[01:18:04] ------------------------------------------
[01:18:04] 3: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//code' 'pub use foo::Foo'
[01:18:04] 10: @has check failed
[01:18:04]  `XPATH PATTERN` did not match
[01:18:04]  // @has - '//code' 'pub use foo::Bar'
[01:18:04] Encountered 2 errors
---
[01:18:04] test result: FAILED. 238 passed; 56 failed; 2 ignored; 0 measured; 0 filtered out
[01:18:04] 
[01:18:04] 
[01:18:04] 
[01:18:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:04] 
[01:18:04] 
[01:18:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:04] Build completed unsuccessfully in 0:18:12
[01:18:04] Build completed unsuccessfully in 0:18:12
[01:18:04] make: *** [check] Error 1
[01:18:04] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0701be45
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 03:10:36 UTC 2019
---
travis_time:end:09e06310:start=1550113838466550278,finish=1550113838471369018,duration=4818740
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:028fe9ed
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0045f7a8
travis_time:start:0045f7a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:297204e8
$ dmesg | grep -i kill
