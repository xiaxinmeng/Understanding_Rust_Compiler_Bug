plain
[00:48:12] ....................................................................................................
[00:48:15] ....................................................................................................
[00:48:18] ....................................................................................................
[00:48:21] ....................................................................................................
[00:48:23] ...........iiiiiiiii................................................................................
[00:48:29] ....................................................................................................
[00:48:33] ................i...................................................................................
[00:48:35] .........................i..........................................................................
[00:48:39] ....................................................................................................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:12] 
[01:01:12] running 254 tests
[01:01:35] .....................i...FF........F.F............F......F.F..F..FFFFF..............................
[01:01:56] FF.......F..........Fi.....................F..........................F.............................
[01:02:07] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:02:07] .F..........................................F...F.....
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/doc-cfg.rs stdout ----
[01:02:07] 
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 17: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="stab portability"]' 'This is supported on Unix and ARM only.'
[01:02:07] Encountered 1 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/doc-cfg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/doc-spotlight.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-spotlight" "/checkout/src/test/rustdoc/doc-spotlight.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 22: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]      // @has - '//code[@class="content"]' 'impl<T: SomeTrait> SomeTrait for Wrapper<T>'
[01:02:07] 36: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]      // @has - '//code[@class="content"]' 'impl<T: SomeTrait> SomeTrait for Wrapper<T>'
[01:02:07] Encountered 2 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/doc-spotlight.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/escape-deref-methods.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods" "/checkout/src/test/rustdoc/escape-deref-methods.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 40: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="sidebar-title"]' 'Methods from Deref<Target=Vec<Title>>'
[01:02:07] Encountered 1 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/escape-deref-methods.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/extern-default-method.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method" "/checkout/src/test/rustdoc/extern-default-method.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 16: @count check failed
[01:02:07]  Expected 1 occurrences but found 0
[01:02:07]  // @count extern_default_method/struct.Struct.html '//*[@id="method.provided"]' 1
[01:02:07] Encountered 1 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/extern-default-method.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/hidden-impls.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls" "/checkout/src/test/rustdoc/hidden-impls.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 25: @has check failed
[01:02:07]  File does not exist 'implementors/foo/trait.Clone.js'
[01:02:07]  // @has implementors/foo/trait.Clone.js
[01:02:07] 26: @!has check failed
[01:02:07]  File does not exist 'implementors/foo/trait.Clone.js'
[01:02:07]  // @!has - 'Foo'
[01:02:07] Encountered 2 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/hidden-impls.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/impl-parts.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-parts" "/checkout/src/test/rustdoc/impl-parts.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 17: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has impl_parts/struct.Foo.html '//*[@class="impl"]//code' "impl<T: Clone> !AnOibit for Foo<T> where T: Sync,"
[01:02:07] 19: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has impl_parts/trait.AnOibit.html '//*[@class="item-list"]//code' "impl<T: Clone> !AnOibit for Foo<T> where T: Sync,"
[01:02:07] Encountered 2 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/impl-parts.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 27: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@id="associatedconstant.ConstNoDefault"]' 'const ConstNoDefault: i16'
[01:02:07] 28: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07no_default'
[01:02:07] 39: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
[01:02:07] 40: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="docblock"]' 'docs for method_with_default'
[01:02:07] Encountered 14 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/inline_cross/assoc-items.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait" "/checkout/src/test/rustdoc/inline_cross/impl-inline-without-trait.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 20: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@id="method.my_trait_method"]' 'fn my_trait_method()'
[01:02:07] 21: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="docblock"]' 'docs for my_trait_method'
[01:02:07] Encountered 2 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/inline_cross/issue-31948-1.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1" "/checkout/src/test/rustdoc/inline_cross/issue-31948-1.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 18: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="impl"]//code' 'Bark for'
[01:02:07] 19: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="impl"]//code' 'Woof for'
[01:02:07] 26: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//code' 'for Wobble'
[01:02:07] 32: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//code' 'for Wobble'
[01:02:07] Encountered 4 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/inline_cross/issue-31948-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/inline_cross/issue-3194_cross/issue-31948" "/checkout/src/test/rustdoc/inline_cross/issue-31948.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 18: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="impl"]//code' 'Bark for'
[01:02:07] 19: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="impl"]//code' 'Woof for'
[01:02:07] 25: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//code' 'for Foo'
[01:02:07] 31: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//code' 'for Foo'
[01:02:07] Encountered 4 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/inline_cross/issue-31948.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/inline_cross/issue-32881.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-32881" "/checkout/src/test/rustdoc/inline_cross/issue-32881.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ----------------- did not match
[01:02:07]  // @has - '//*[@id="method.baz"]' 'fn baz(&self)'
[01:02:07] 30: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@id="method.baz"]' 'fn baz(&self)'
[01:02:07] Encountered 3 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/issue-19190-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/issue-21092.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21092" "/checkout/src/test/rustdoc/issue-21092.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 18: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@id="associatedtype.Bar"]' 'type Bar = i32'
[01:02:07] Encountered 1 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/issue-21092.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/issue-25001.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001" "/checkout/src/test/rustdoc/issue-25001.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 40: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]      // @has - '//*[@id="method.quux"]//code' 'fn quux(self)'
[01:02:07] 47: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]      // @has - '//*[@id="method.quux-1"]//code' 'fn quux(self)'
[01:02:07] 54: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]      // @has - '//*[@id="method.quux-2"]//code' 'fn quux(self)'
[01:02:07] Encountered 3 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/issue-25001.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/issue-33302.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302" "/checkout/src/test/rustdoc/issue-33302.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 40: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]          // @has - '//*[@class="docblock"]' 'C: [i32; 16] = [0; 4 * 4]'
[01:02:07] Encountered 1 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/issue-33302.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/issue-46727.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-46727" "/checkout/src/test/rustdoc/issue-46727.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 16: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//code' 'impl<T> Foo for Bar<[T; 3]>'
[01:02:07] Encountered 1 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/issue-46727.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/primitive-generic-impl.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive-generic-impl.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 18: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has foo/primitive.i32.html '//h3[@id="impl-ToString"]//code' 'impl<T> ToString for T'
[01:02:07] Encountered 1 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/primitive-generic-impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/typedef.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/typedef" "/checkout/src/test/rustdoc/typedef.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 23: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="impl"]//code' 'impl MyTrait for MyAlias'
[01:02:07] 27: @has check failed
[01:02:07]  `XPATH PATTERN` did not match
[01:02:07]  // @has - '//*[@class="sidebar"]//a[@href="#implementations"]' 'Trait Implementations'
[01:02:07] Encountered 2 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] 
[01:02:07] thread '[rustdoc] rustdoc/typedef.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:02:07] 
[01:02:07] ---- [rustdoc] rustdoc/universal-impl-trait.rs stdout ----
[01:02:07] 
[01:02:07] error: htmldocck failed!
[01:02:07] status: exit code: 1
[01:02:07] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/universal-impl-trait" "/checkout/src/test/rustdoc/universal-impl-trait.rs"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] 51: @has check failed
[01:02:07]  `PATTERN` did not match
[01:02:07]  // @has - 'method</a>('
[01:02:07] 52: @matches check failed
[01:02:07]  `PATTERN` did not match
[01:02:07]  // @matches - '_x: impl <a class="trait" href="[^"]+/trait\.Debug\.html"'
[01:02:07] Encountered 2 errors
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] 
---
[01:02:07] test result: FAILED. 230 passed; 22 failed; 2 ignored; 0 measured; 0 filtered out
[01:02:07] 
[01:02:07] 
[01:02:07] 
[01:02:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:07] 
[01:02:07] 
[01:02:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:07] Build completed unsuccessfully in 0:16:36
[01:02:07] Build completed unsuccessfully in 0:16:36
[01:02:07] Makefile:58: recipe for target 'check' failed
[01:02:07] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05344c70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Aug  7 17:36:13 UTC 2018
ernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] kvm-clock: using sched offset of 1522258766 cycles
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Zone ranges:
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   Device   empty
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Movable zone start for each node
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Early memory node ranges
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [29-8a37-c0b88e02bc5e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Policy zone: Normal
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.000000] Calgary: Un 0.354831] Initializing cgroup subsys devices
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.355452] Initializing cgroup subsys freezer
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.356215] Initializing cgroup subsys net_cls
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.357211] Initializing cgroup subsys perf_event
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.358162] Initializing cgroup subsys net_prio
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.358893] Initializing cgroup subsys hugetlb
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.359537] Initializing cgroup subsys pids
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.360378] CPU: Physical Processor ID: 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.361063] CPU: Processor Core ID: 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.362971] mce: CPU supports 32 MCE banks
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.363792] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.364661] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.367775] Freeing SMP alternatives memory: 32K
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.376958] ftrace: allocating 32185 entries in 126 pages
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.427725] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.429282] smpboot: Max logical packages: 2
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.430782] x2apic enabled
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.433172] Switched APIC routing to physical x2apic.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.437802] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.545480] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.547828] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.551232] x86: Booting SMP configuration:
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.552104] .... node  #0, CPUs:      #1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.553348] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.558642]  #2
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.559352] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.563955]  #3
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.564700] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.569124] x86: Booted up 1 node, 4 CPUs
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.570127] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.572810] devtmpfs: initialized
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.577504] evm: security.selinux
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.578246] evm: security.SMACK64
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.578914] evm: security.SMACK64EXEC
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.579543] evm: security.SMACK64TRANSMUTE
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.580328] evm: security.SMACK64MMAP
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.581194] evm: security.ima
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.581834] evm: security.capability
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.582923] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.585075]s-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.702930] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.704208] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.705528] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.706829] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.707756] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.708213] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.728151] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.744884] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.746558] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.753560] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.758895] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.776737] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.785137] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.790317] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.806559] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.809734] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.813390] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.816636] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.819003] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.841468] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.842723] vgaarb: loaded
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.843374] SCSI subsystem initialized
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.844352] libata version 3.00 loaded.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.844381] ACPI: bus type USB registered
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.845127] usbcore: registered new interface driver usbfs
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.845930] usbcore: registered new interface driver hub
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.847022] usbcore: registered new device driver usb
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.848265] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.849447] dmi: Firmware registration failed.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.850370] PCI: Using ACPI for IRQ routing
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.851098] PCI: pci_cache_line_size set to 64 bytes
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.851205] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.851206] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.851344] NetLabel: Initializing
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.851845] NetLabel:  domain hash size = 128
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.852590] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.853327] NetLabel:  unlabeled traffic allowed by default
Aug0xffffff, max_idle_ns: 2085701024 ns
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.873909] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.873912] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.873913] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.873914] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.873953] NET: Registered protocol family 2
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.874756] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.876939] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.878256] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.879434] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.880398] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    0.882857] NET: Registered protocol family 1
Aug  7 16:31:57 travis-job-683d2e82-c2553d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.043780] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.045426] io scheduler noop registered
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.046025] io scheduler deadline registered (default)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.046824] io scheduler cfq registered
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.047498] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.048940] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.050310] intel_idle: does not run on family 6 model 63
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.050471] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.051839] ACPI: Power Button [PWRF]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.052441] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.053704] ACPI: Sleep Button [SLPF]
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.054874] GHES: HEST is not enabled!
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.057678] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.058721] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.064252] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.065417] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.071003] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.093845] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.117541] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.141335] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.165236] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.168788] Linux agpgart interface v0.103
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.172288] loop: module loaded
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.173376] libphy: Fixed MDIO Bus: probed
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-3.228813] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.229842] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.231124] mousedev: PS/2 mouse device common for all mice
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.232470] rtc_cmos 00:00: RTC can wake from S4
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.233849] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.235958] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.237541] i2c /dev entries driver
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.238608] device-mapper: uevent: version 1.0.3
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.239853] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.241783] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.244280] NET: Registered protocol family 10
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.245649] NET: Registered protocol family 17
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.246592] Key type dns_resolver registered
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.247784] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.248820] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.249820] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.251145] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.252226] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.254856] registered taskstats version 1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.256177] Loading compiled-in X.509 certificates
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.258453] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.261155] zswap: loaded using pool lzo/zbud
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.264963] Key type trusted registered
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.270444] Key type encrypted registered
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.271710] ima: No TPM chip found, activating TPM-bypass!
Aug  7 16:31:57 t6:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.374721] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.383541] AVX2 version of gcm_enc/dec engaged.
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.385069] AES CTR mode by8 optimization enabled
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.416946] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.418731] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.421341] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.423709] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.425161] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.425294] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.430732]  sda: sda1
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.432013] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    3.435853] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    4.023647] tsc: Refined TSC clocksource calibration: 2299.793 MHz
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    4.025904] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212671cc3ca, max_idle_ns: 440795297541 ns
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    4.268726] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    6.351757] floppy0: no floppy controllers found
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.523579] raid6: sse2x1   gen()  8684 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.591569] raid6: sse2x1   xor()  6464 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.659568] raid6: sse2x2   gen() 10898 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.727520] raid6: sse2x2   xor()  7511 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.795527] raid6: sse2x4   gen() 12103 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.863532] raid6: sse2x4   xor()  8191 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.931520] raid6: avx2x1   gen() 16959 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    7.999518] raid6: avx2x2   gen() 19456 MB/s
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a3d urandom read (12 bytes read, 24 bits of entropy available)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    8.703775] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    8.769165] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    9.023592] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    9.706462] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [    9.875708] systemd-udevd[702]: starting version 204
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   10.002592] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   10.067281] intel_rapl: no valid rapl domains found in package 0
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   10.122711] ppdev: user-space parallel port driver
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   10.260938] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  7 16:31:57 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   10.325068] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entro83d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-accounts: INFO Starting Google Accounts daemon.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-accounts: INFO Creating a new user account for me.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-accounts: INFO Created user account me.
Aug  7 16:31:58 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-accounts: INFO Removing user packer.
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   12.879458] random: nonblocking pool is initialized
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.048679] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.053592] Bridge firewalling registered
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.069815] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.079579] floppy0: no floppy controllers found
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.132822] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.244580] Initializing XFRM netlink socket
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.252708] Netfilter messages via NETLINK v0.30.
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.256690] ctnetlink v0.93: registering with nfnetlink.
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 16:31:59 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e cron[1596]: (CRON) INFO (pidfile fd = 3)
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e cron[1635]: (CRON) STARTUP (fork ok)
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e cron[1635]: (CRON) INFO (Running @reboot jobs)
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e acpid: starting up with netlink and the input layer
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e acpid: 1 rule loaded
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e acpid: waiting for events: event logging is off
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e haveged: haveged starting up
Aug  7 16:32:00 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [   13.803270] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ntpd[nds will be executed using /bin/sh
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e startup-script: INFO startup-script: job 1 at Tue Aug  7 19:42:00 2018
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e startup-script: INFO startup-script: Return code 0.
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e startup-script: INFO Finished running startup scripts.
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e startup-script: INFO Finished running startup scripts.
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: 
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: #############################################################
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: 1024 81:f8:1c:f4:d1:12:2d:61:04:d1:18:d2:1d:ab:6f:a4  root@travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e (DSA)
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: 256 75:c2:12:e8:35:73:18:2e:2f:39:4f:a9:60:08:35:72  root@travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e (ECDSA)
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: 256 78:40:9a:ad:d8:10:87:16:77:c2:11:59:16:e9:e3:d1  root@travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e (ED25519)
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: 2048 78:0c:ca:81:be:11:f8:4b:aa:5d:43:d2:9d:41:43:80  root@travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e (RSA)
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 16:32:05 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ec2: #############################################################
Aug  7 16:32:14 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ntpdate[2136]: the NTP socket is in use, exiting
Aug  7 16:34:04 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  137.916917] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.199365] device vethd98c3b1 entered promiscuous mode
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.323634] cgroup: docker-runc (4761) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.323637] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.399564] eth0: renamed from veth7fab31a
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.438411] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.439713] docker0: port 1(vethd98c3b1) entered forwarding state
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.439731] docker0: port 1(vethd98c3b1) entered forwarding state
Aug  7 16:35:09 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [  203.439753] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 16:35:13 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e ntpd[1738]: Listen normally on 5 docker0 fe80::42:63ff:fef706] a[22357]: segfault at 0 ip 0000559469f48658 sp 00007fff1f19d480 error 6 in a[559469f45000+5000]
Aug  7 17:27:25 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [ 3338.389901] a[23163]: segfault at 1 ip 000055e49da67c6c sp 00007ffe5ff82620 error 6 in a[55e49da65000+4000]
Aug  7 17:27:28 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [ 3342.159075] traps: a[23548] trap invalid opcode ip:55bc9b7645bc sp:7ffe0d168690 error:0 in a[55bc9b761000+7000]
Aug  7 17:36:12 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [ 3866.103201] docker0: port 1(vethd98c3b1) entered disabled state
Aug  7 17:36:12 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [ 3866.103285] veth7fab31a: renamed from eth0
Aug  7 17:36:12 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [ 3866.176288] docker0: port 1(vethd98c3b1) entered disabled state
Aug  7 17:36:12 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [ 3866.178208] device vethd98c3b1 left promiscuous mode
Aug  7 17:36:12 travis-job-683d2e82-c255-4729-8a37-c0b88e02bc5e kernel: [ 3866.178211] docker0: port 1(vethd98c3b1) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:02fc3b6c
