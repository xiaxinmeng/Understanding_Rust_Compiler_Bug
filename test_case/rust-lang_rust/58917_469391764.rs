plain
travis_time:end:060fe6c0:start=1551723163924214757,finish=1551723236659655083,duration=72735440326
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
[01:17:07] 
[01:17:07] running 119 tests
[01:17:32] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:17:36] i......iii.i.....ii
[01:17:36] 
[01:17:36]  finished in 29.064
[01:17:36] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:18] 
[01:21:18] running 300 tests
[01:22:19] ........FFF................i................FF.F..FF..F....FF........FFF.F.FFFFFFFFF...F......F..... 100/300
[01:23:11] .................F.FF..FFFFFF..F.....F..i..........F........F......FF......................F........ 200/300
[01:24:03] ...............F................F.....F..F.....................F.........................F.......... 300/300
[01:24:03] failures:
[01:24:03] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----
[01:24:03] ---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 4: @has check failed
[01:24:03]  `XPATH PATTERN` did not match
[01:24:03]  // @has 'foo/primitive.i16.html' '//h2[@id="synthetic-implementations"]' 'Auto Trait Implementation'
[01:24:03] Encountered 1 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/auto-impl-primitive.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/auto-traits.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-traits" "/checkout/src/test/rustdoc/auto-traits.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 12: @has check failed
[01:24:03]  File does not exist 'foo/trait.Bar.html'
[01:24:03]  // @has 'foo/trait.Bar.html' '//pre' 'pub unsafe auto trait Bar'
[01:24:03] Encountered 1 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/auto-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 3: @has check failed
[01:24:03]  `XPATH PATTERN` did not match
[01:24:03]  // @has foo/struct.S.html '//h3[@id="impl-Into"]//code' 'impl<T, U> Into for T'
[01:24:03] Encountered 1 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/blanket-reexport-item.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/escape-deref-methods.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods" "/checkout/src/test/rustdoc/escape-deref-methods.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 30: @has check failed
[01:24:03]  `XPATH PATTERN` did not match
[01:24:03]  // @has - '//*[@class="sidebar-title"]' 'Methods from Deref<Target=Vec<Title>>'
[01:24:03] Encountered 1 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/escape-deref-methods.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/extern-default-method.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method" "/checkout/src/test/rustdoc/extern-default-method.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 6: @count check failed
[01:24:03]  File does not exist 'extern_default_method/struct.Struct.html'
[01:24:03]  // @count extern_default_method/struct.Struct.html '//*[@id="method.provided"]' 1
[01:24:03] Encountered 1 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/extern-default-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/extern-impl-trait.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl-trait" "/checkout/src/test/rustdoc/extern-impl-trait.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 7: @has check failed
[01:24:03]  File does not exist 'foo/struct.X.html'
[01:24:03]  // @has 'foo/struct.X.html' '//code' "impl Foo<Associated = ()> + 'a"
[01:24:03] 10: @has check failed
[01:24:03]  File does not exist 'foo/struct.Y.html'
[01:24:03]  // @has 'foo/struct.Y.html' '//code' "impl ?Sized + Foo<Associated = ()> + 'a"
[01:24:03] Encountered 2 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/extern-impl-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/extern-method.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-method" "/checkout/src/test/rustdoc/extern-method.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 8: @has check failed
[01:24:03]  File does not exist 'extern_method/trait.Foo.html'
[01:24:03]  // @has extern_method/trait.Foo.html //pre "pub trait Foo"
[01:24:03] 9: @has check failed
[01:24:03]  File does not exist 'extern_method/trait.Foo.html'
[01:24:03]  // @has - '//*[@id="tymethod.foo"]//code' 'extern "rust-call" fn foo'
[01:24:03] 10: @has check failed
[01:24:03]  File does not exist 'extern_method/trait.Foo.html'
[01:24:03]  // @has - '//*[@id="method.foo_"]//code' 'extern "rust-call" fn foo_'
[01:24:03] Encountered 3 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/extern-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/external-cross.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/external-cross" "/checkout/src/test/rustdoc/external-cross.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 8: @has check failed
[01:24:03]  File does not exist 'host/struct.NeedMoreDocs.html'
[01:24:03]  // @has host/struct.NeedMoreDocs.html
[01:24:03] 9: @has check failed
[01:24:03]  File does not exist 'host/struct.NeedMoreDocs.html'
[01:24:03]  // @has - '//h1' 'Cross-crate imported docs'
[01:24:03] Encountered 2 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/external-cross.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/ffi.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi" "/checkout/src/test/rustdoc/ffi.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 6: @has check failed
[01:24:03]  File does not exist 'ffi/fn.foreigner.html'
[01:24:03]  // @has ffi/fn.foreigner.html //pre 'pub unsafe extern "C" fn foreigner(cold_as_ice: u32)'
[01:24:03] Encountered 1 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/ffi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/hidden-impls.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls" "/checkout/src/test/rustdoc/hidden-impls.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 13: @has check failed
[01:24:03]  File does not exist 'foo/trait.Clone.html'
[01:24:03]  // @has foo/trait.Clone.html
[01:24:03] 14: @!has check failed
[01:24:03]  File does not exist 'foo/trait.Clone.html'
[01:24:03]  // @!has - 'Foo'
[01:24:03] 15: @has check failed
[01:24:03]  File does not exist 'implementors/foo/trait.Clone.js'
[01:24:03]  // @has implementors/foo/trait.Clone.js
[01:24:03] 16: @!has check failed
[01:24:03]  File does not exist 'implementors/foo/trait.Clone.js'
[01:24:03]  // @!has - 'Foo'
[01:24:03] Encountered 4 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/hidden-impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/generic-impl.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 8: @has check failed
[01:24:03]  `XPATH PATTERN` did not match
[01:24:03]  // @has foo/struct.Foo.html '//h3[@id="impl-ToString"]//code' 'impl<T> ToString for T'
[01:24:03] 10: @has check failed
[01:24:03]  `XPATH PATTERN` did not match
[01:24:03]  // @has foo/struct.Foo.html '//div[@class="sidebar-links"]/a[@href="#impl-ToString"]' 'ToString'
[01:24:03] Encountered 2 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/generic-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/inline-default-methods.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline-default-methods" "/checkout/src/test/rustdoc/inline-default-methods.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 6: @has check failed
[01:24:03]  File does not exist 'inline_default_methods/trait.Foo.html'
[01:24:03]  // @has inline_default_methods/trait.Foo.html
[01:24:03] 7: @has check failed
[01:24:03]  File does not exist 'inline_default_methods/trait.Foo.html'
[01:24:03]  // @has - '//*[@class="rust trait"]' 'fn bar(&self);'
[01:24:03] 8: @has check failed
[01:24:03]  File does not exist 'inline_default_methods/trait.Foo.html'
[01:24:03]  // @has - '//*[@class="rust trait"]' 'fn foo(&mut self) { ... }'
[01:24:03] Encountered 3 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/inline-default-methods.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 9: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has foo/struct.MyStruct.html
[01:24:03] 10: @!has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @!has - 'PrivateConst'
[01:24:03] 11: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="associatedconstant.PublicConst"]' 'pub const PublicConst: u8'
[01:24:03] 12: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for PublicConst'
[01:24:03] 13: @!has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @!has - 'private_method'
[01:24:03] 14: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="method.public_method"]' 'pub fn public_method()'
[01:24:03] 15: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for public_method'
[01:24:03] 16: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="associatedconstant.ConstNoDefault"]' 'const ConstNoDefault: i16'
[01:24:03] 17: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'dox for ConstNoDefault'
[01:24:03] 18: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="associatedconstant.ConstWithDefault"]' 'const ConstWithDefault: u16'
[01:24:03] 19: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for ConstWithDefault'
[01:24:03] 20: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="associatedtype.TypeNoDefault"]' 'type TypeNoDefault = i32'
[01:24:03] 21: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'dox for TypeNoDefault'
[01:24:03] 22: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="associatedtype.TypeWithDefault"]' 'type TypeWithDefault = u32'
[01:24:03] 23: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for TypeWithDefault'
[01:24:03] 24: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="method.method_no_default"]' 'fn method_no_default()'
[01:24:03] 25: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'dox for method_no_default'
[01:24:03] 26: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
[01:24:03] 27: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for method_with_default'
[01:24:03] 30: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has foo/trait.MyTrait.html
[01:24:03] 31: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@id="associatedconstant.ConstNoDefault"]' 'const ConstNoDefault: i16'
[01:24:03] 32: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for ConstNoDefault'
[01:24:03] 33: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@id="associatedconstant.ConstWithDefault"]' 'const ConstWithDefault: u16'
[01:24:03] 34: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for ConstWithDefault'
[01:24:03] 35: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@id="associatedtype.TypeNoDefault"]' 'type TypeNoDefault'
[01:24:03] 36: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for TypeNoDefault'
[01:24:03] 37: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for TypeWithDefault'
[01:24:03] 38: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@id="tymethod.method_no_default"]' 'fn method_no_default()'
[01:24:03] 39: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for method_no_default'
[01:24:03] 40: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
[01:24:03] 41: @has check failed
[01:24:03]  File does not exist 'foo/trait.MyTrait.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for method_with_default'
[01:24:03] Encountered 31 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/inline_cross/assoc-items.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/inline_cross/cross-glob.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/cross-glob" "/checkout/src/test/rustdoc/inline_cross/cross-glob.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 7: @has check failed
[01:24:03]  File does not exist 'cross_glob/struct.SomeStruct.html'
[01:24:03]  // @has cross_glob/struct.SomeStruct.html
[01:24:03] 8: @has check failed
[01:24:03]  File does not exist 'cross_glob/fn.some_fn.html'
[01:24:03]  // @has cross_glob/fn.some_fn.html
[01:24:03] Encountered 2 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/inline_cross/cross-glob.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait" "/checkout/src/test/rustdoc/inline_cross/impl-inline-without-trait.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 9: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has 'foo/struct.MyStruct.html'
[01:24:03] 10: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@id="method.my_trait_method"]' 'fn my_trait_method()'
[01:24:03] 11: @has check failed
[01:24:03]  File does not exist 'foo/struct.MyStruct.html'
[01:24:03]  // @has - '//*[@class="docblock"]' 'docs for my_trait_method'
[01:24:03] Encountered 3 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/inline_cross/issue-28480.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480" "/checkout/src/test/rustdoc/inline_cross/issue-28480.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 10: @has check failed
[01:24:03]  File does not exist 'issue_28480/struct.Bar.html'
[01:24:03]  // @has issue_28480/struct.Bar.html
[01:24:03] 11: @!has check failed
[01:24:03]  File does not exist 'issue_28480/struct.Bar.html'
[01:24:03]  // @!has -  '//a/@title' 'Hidden'
[01:24:03] 12: @has check failed
[01:24:03]  File does not exist 'issue_28480/struct.Bar.html'
[01:24:03]  // @has -  '//a' 'u8'
[01:24:03] Encountered 3 errors
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] 
[01:24:03] thread '[rustdoc] rustdoc/inline_cross/issue-28480.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:24:03] 
[01:24:03] ---- [rustdoc] rustdoc/inline_cross/issue-31948-1.rs stdout ----
[01:24:03] 
[01:24:03] error: htmldocck failed!
[01:24:03] status: exit code: 1
[01:24:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1" "/checkout/src/test/rustdoc/inline_cross/issue-31948-1.rs"
[01:24:03] ------------------------------------------
[01:24:03] 
[01:24:03] ------------------------------------------
[01:24:03] stderr:
[01:24:03] stderr:
[01:24:03] ------------------------------------------
[01:24:03] 7: @has check failed
[01:24:03]  File does not exist 'issue_31948_1/struct.Wobble.html'
[01:24:03]  // @has issue_31948_1/struct.Wobble.html
[01:24:03] 8: @has check failed
[01:24:03]  File does not exist 'issue_31948_1/struct.Wobble.html'
[01:24:03]  // @has - '//*[@class="impl"]//code' 'Bark for'
---
[01:24:03] test result: FAILED. 250 passed; 48 failed; 2 ignored; 0 measured; 0 filtered out
[01:24:03] 
[01:24:03] 
[01:24:03] 
[01:24:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:24:03] 
[01:24:03] 
[01:24:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:03] Build completed unsuccessfully in 0:18:31
[01:24:03] Build completed unsuccessfully in 0:18:31
[01:24:03] Makefile:48: recipe for target 'check' failed
[01:24:03] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:038b40ab
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar  4 19:38:10 UTC 2019
---
travis_time:end:0099408e:start=1551728291596489992,finish=1551728291647053814,duration=50563822
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17a21600
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04ac5724
$ dmesg | grep -i kill
