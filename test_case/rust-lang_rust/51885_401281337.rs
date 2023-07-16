plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:39] 
[01:03:39] running 240 tests
[01:04:53] ....................i..F...F................F...F.FF.........FF.F...................................
[01:05:36] .............i.......F.............F..................F..................F..........................
[01:05:47] .......................................F
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/doc-cfg.rs stdout ----
[01:05:47] 
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 39: @count check failed
[01:05:47]  Expected 2 occurrences but found 3
[01:05:47]      // @count - '//*[@class="stab portability"]' 2
[01:05:47] Encountered 1 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/doc-cfg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/duplicate_impls/issue-33054.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/issue-33054" "/checkout/src/test/rustdoc/duplicate_impls/issue-33054.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 17: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has - '//code' 'impl Bar for Foo'
[01:05:47] 18: @count check failed
[01:05:47]  Expected 1 occurrences but found 0
[01:05:47]  // @count - '//*[@class="struct"]' 1
[01:05:47] Encountered 2 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/duplicate_impls/issue-33054.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/foreigntype.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/foreigntype" "/checkout/src/test/rustdoc/foreigntype.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 25: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foreigntype/trait.Trait.html '//a[@class="foreigntype"]' 'ExtType'
[01:05:47] Encountered 1 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/foreigntype.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/hidden-trait-struct-impls.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-struct-impls" "/checkout/src/test/rustdoc/hidden-trait-struct-impls.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 29: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Bam.html '//*[@id="implementors-list"]' 'impl Bam for Bar'
[01:05:47] Encountered 1 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/hidden-trait-struct-impls.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/impl-disambiguation.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-disambiguation" "/checkout/src/test/rustdoc/impl-disambiguation.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 17: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl Foo for Bar<u8>"
[01:05:47] 20: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl Foo for Bar<u16>"
[01:05:47] 23: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl<'a> Foo for &'a Bar<u8>"
[01:05:47] 35: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl Foo for foo::mod1::Baz"
[01:05:47] 38: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl<'a> Foo for &'a foo::mod2::Baz"
[01:05:47] Encountered 5 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/impl-disambiguation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/impl-parts.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] commandt match
[01:05:47] commandt match
[01:05:47]  // @has - '//code' 'for Wobble'
[01:05:47] Encountered 2 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/inline_cross/issue-31948-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/inline_cross/issue-31948-2.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-2" "/checkout/src/test/rustdoc/inline_cross/issue-31948-2.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 26: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has - '//code' 'for Wobble'
[01:05:47] Encountered 1 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/inline_cross/issue-31948-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/inline_cross/issue-31948.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948" "/checkout/src/test/rustdoc/inline_cross/issue-31948.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 25: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has - '//code' 'for Foo'
[01:05:47] 31: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has - '//code' 'for Foo'
[01:05:47] Encountered 2 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/inline_cross/issue-31948.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/issue-29503.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 18: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has - "//ul[@id='implementors-list']/li" "impl<T> MyTrait for T where T: Debug"
[01:05:47] Encountered 1 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/issue-29503.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/issue-33592.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33592" "/checkout/src/test/rustdoc/issue-33592.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 19: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Foo.html '//code' 'impl Foo<i32> for Bar'
[01:05:47] 22: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.Foo.html '//code' 'impl<T> Foo<T> for Baz'
[01:05:47] Encountered 2 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/issue-33592.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/issue-43893.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43893" "/checkout/src/test/rustdoc/issue-43893.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 21: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.SomeTrait.html '//a/@href' '../src/foo/issue-43893.rs.html#22-24'
[01:05:47] 28: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.AnotherTrait.html '//a/@href' '../src/foo/issue-43893.rs.html#29'
[01:05:47] Encountered 2 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/issue-43893.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/issue-46727.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46727" "/checkout/src/test/rustdoc/issue-46727.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 16: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has - '//code' 'impl<T> Foo for Bar<[T; 3]>'
[01:05:47] Encountered 1 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] 
[01:05:47] thread '[rustdoc] rustdoc/issue-46727.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:05:47] 
[01:05:47] ---- [rustdoc] rustdoc/where.rs stdout ----
[01:05:47] 
[01:05:47] error: htmldocck failed!
[01:05:47] status: exit code: 1
[01:05:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where" "/checkout/src/test/rustdoc/where.rs"
[01:05:47] ------------------------------------------
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] stderr:
[01:05:47] stderr:
[01:05:47] ------------------------------------------
[01:05:47] 34: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.MyTrait.html '//*[@id="implementors-list"]//code' "impl<E> MyTrait for Echo<E> where E: MyTrait"
[01:05:47] 42: @has check failed
[01:05:47]  `XPATH PATTERN` did not match
[01:05:47]  // @has foo/trait.MyTrait.html '//*[@id="implementors-list"]//code' "impl<F> MyTrait for Foxtrot<F> where F: MyTrait"
[01:05:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:05:47] Encountered 2 errors
[01:05:47] 
[01:05:47] ------------------------------------------
[01:05:47] 
---
[01:05:47] test result: FAILED. 224 passed; 14 failed; 2 ignored; 0 measured; 0 filtered out
[01:05:47] 
[01:05:47] 
[01:05:47] 
[01:05:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:47] 
[01:05:47] 
[01:05:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:47] Build completed unsuccessfully in 0:23:35
[01:05:47] Build completed unsuccessfully in 0:23:35
[01:05:47] Makefile:58: recipe for target 'check' failed
[01:05:47] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:087fe70c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
