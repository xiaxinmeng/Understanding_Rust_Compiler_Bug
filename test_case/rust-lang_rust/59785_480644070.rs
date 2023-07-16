plain
travis_time:end:1873e6f3:start=1554676253351537609,finish=1554676329159014459,duration=75807476850
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:50] 
[01:15:50] running 9 tests
[01:15:50] iiiiiiiii
[01:15:50] 
[01:15:50]  finished in 0.154
[01:15:50] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:06] 
[01:16:06] running 121 tests
[01:16:32] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:16:37] i.i......iii.i.....ii
[01:16:37] 
[01:16:37]  finished in 30.790
[01:16:37] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:31] 
[01:20:31] running 302 tests
[01:21:45] FFFFFFF.F.F..FF..F.FF...FF.FiFFFF..FFF...FFFFF.FFF.FFF...FFFFF.FFF.F..FFF.F.FFFFFF.F...FF.F.FF..FFF. 100/302
[01:22:47] ...F...FF.........FFFFFFFFFFFFF.F.......Fi.F.FFF.FF.F...FF.FFF.FF..F..FFF..FFF......FFF..F..F......F 200/302
[01:23:52] F..FFF.F.FF....FFF..F..FFF.......F...F.FFF....FF..FF....FFF..FFFF.FFFFFFFFFFF....FFFFFF.FF.FF..F...F 300/302
[01:23:53] F.
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/all.rs stdout ----
[01:23:53] 
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/all" "/checkout/src/test/rustdoc/all.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 3: @has check failed
[01:23:53]  `XPATH PATTERN` did not match
[01:23:53]  // @has foo/all.html '//a[@href="struct.Struct.html"]' 'Struct'
[01:23:53] 4: @has check failed
[01:23:53]  `XPATH PATTERN` did not match
[01:23:53]  // @has foo/all.html '//a[@href="enum.Enum.html"]' 'Enum'
[01:23:53] 5: @has check failed
[01:23:53]  `XPATH PATTERN` did not match
[01:23:53]  // @has foo/all.html '//a[@href="union.Union.html"]' 'Union'
[01:23:53] 6: @has check failed
[01:23:53]  `XPATH PATTERN` did not match
[01:23:53]  // @has foo/all.html '//a[@href="constant.CONST.html"]' 'CONST'
[01:23:53] 7: @has check failed
[01:23:53]  `XPATH PATTERN` did not match
[01:23:53]  // @has foo/all.html '//a[@href="static.STATIC.html"]' 'STATIC'
[01:23:53] 8: @has check failed
[01:23:53]  `XPATH PATTERN` did not match
[01:23:53]  // @has foo/all.html '//a[@href="fn.function.html"]' 'function'
[01:23:53] 26: @has check failed
[01:23:53]  `XPATH PATTERN` did not match
[01:23:53]  // @has foo/all.html '//a[@href="struct.ReexportedStruct.html"]' 'ReexportedStruct'
[01:23:53] Encountered 7 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/assoc-consts-version.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version" "/checkout/src/test/rustdoc/assoc-consts-version.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 13: @has check failed
[01:23:53]  File does not exist 'foo/struct.SomeStruct.html'
[01:23:53]      // @has 'foo/struct.SomeStruct.html' '//*[@id="associatedconstant.SOME_CONST"]//div[@class="since"]' '1.1.2'
[01:23:53] Encountered 1 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/assoc-consts-version.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/assoc-item-cast.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast" "/checkout/src/test/rustdoc/assoc-item-cast.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 14: @has check failed
[01:23:53]  File does not exist 'foo/type.AsExprOf.html'
[01:23:53]  // @has foo/type.AsExprOf.html
[01:23:53] 15: @has check failed
[01:23:53]  File does not exist 'foo/type.AsExprOf.html'
[01:23:53]  // @has - '//*[@class="rust typedef"]' 'type AsExprOf<Item, Type> = <Item as AsExpression<Type>>::Expression;'
[01:23:53] Encountered 2 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/assoc-item-cast.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 2: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Foo.html'
[01:23:53]      // @has assoc_consts/trait.Foo.html '//*[@class="rust trait"]' 'const FOO: usize;'
[01:23:53] 4: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Foo.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.FOO"]' 'const FOO: usize'
[01:23:53] 6: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Foo.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.FOO_NO_DEFAULT"]' 'const FOO_NO_DEFAULT: bool'
[01:23:53] 8: @!has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Foo.html'
[01:23:53]      // @!has - FOO_HIDDEN
[01:23:53] 16: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has assoc_consts/struct.Bar.html '//code' 'impl Foo for Bar'
[01:23:53] 17: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.FOO"]' 'const FOO: usize'
[01:23:53] 19: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.FOO_NO_DEFAULT"]' 'const FOO_NO_DEFAULT: bool'
[01:23:53] 21: @!has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @!has - FOO_HIDDEN
[01:23:53] 27: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.BAR"]' 'const BAR: usize'
[01:23:53] 35: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.BAZ"]' "const BAZ: Baz<'static, u8, u32>"
[01:23:53] 43: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.F"]' "const F: fn(_: &(dyn ToString + 'static))"
[01:23:53] 49: @!has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @!has assoc_consts/struct.Bar.html 'BAR_PRIVATE'
[01:23:53] 51: @!has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @!has assoc_consts/struct.Bar.html 'BAR_HIDDEN'
[01:23:53] 56: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]  // @has assoc_consts/trait.Qux.html
[01:23:53] 58: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX0"]' 'const QUX0: u8'
[01:23:53] 59: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX0 in trait."
[01:23:53] 62: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX1"]' 'const QUX1: i8'
[01:23:53] 63: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX1 in trait."
[01:23:53] 66: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX_DEFAULT0"]' 'const QUX_DEFAULT0: u16'
[01:23:53] 67: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT12 in trait."
[01:23:53] 70: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX_DEFAULT1"]' 'const QUX_DEFAULT1: i16'
[01:23:53] 71: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT1 in trait."
[01:23:53] 74: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX_DEFAULT2"]' 'const QUX_DEFAULT2: u32'
[01:23:53] 75: @has check failed
[01:23:53]  File does not exist 'assoc_consts/trait.Qux.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT2 in trait."
[01:23:53] 80: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]  // @has assoc_consts/struct.Bar.html '//code' 'impl Qux for Bar'
[01:23:53] 82: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX0"]' 'const QUX0: u8'
[01:23:53] 83: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX0 in trait."
[01:23:53] 86: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX1"]' 'const QUX1: i8'
[01:23:53] 87: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX1 in impl."
[01:23:53] 90: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX_DEFAULT0"]' 'const QUX_DEFAULT0: u16'
[01:23:53] 91: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@class="docblock hidden"]' "Docs for QUX_DEFAULT12 in trait."
[01:23:53] 93: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX_DEFAULT1"]' 'const QUX_DEFAULT1: i16'
[01:23:53] 94: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT1 in impl."
[01:23:53] 97: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@id="associatedconstant.QUX_DEFAULT2"]' 'const QUX_DEFAULT2: u32'
[01:23:53] 98: @has check failed
[01:23:53]  File does not exist 'assoc_consts/struct.Bar.html'
[01:23:53]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT2 in trait."
[01:23:53] Encountered 35 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/assoc-consts.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/assoc-types.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 5: @has check failed
[01:23:53]  File does not exist 'assoc_types/trait.Index.html'
[01:23:53]  // @has assoc_types/trait.Index.html
[01:23:53] 7: @has check failed
[01:23:53]  File does not exist 'assoc_types/trait.Index.html'
[01:23:53]      // @has - '//*[@id="associatedtype.Output"]//code' 'type Output: ?Sized'
[01:23:53] 8: @has check failed
[01:23:53]  File does not exist 'assoc_types/trait.Index.html'
[01:23:53]      // @has - '//code[@id="Output.t"]' 'type Output: ?Sized'
[01:23:53] 10: @has check failed
[01:23:53]  File does not exist 'assoc_types/trait.Index.html'
[01:23:53]      // @has - '//code[@id="index.v"]' 'fn index'
[01:23:53] 11: @has check failed
[01:23:53]  File does not exist 'assoc_types/trait.Index.html'
[01:23:53]      // @has - '//*[@id="tymethod.index"]//code' "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
[01:23:53] 13: @has check failed
[01:23:53]  File does not exist 'assoc_types/trait.Index.html'
[01:23:53]      // @has - '//*[@id="tymethod.index"]//code//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' "Output"
[01:23:53] Encountered 6 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/assoc-types.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/async-fn.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 29: @has check failed
[01:23:53]  File does not exist 'async_fn/struct.Foo.html'
[01:23:53]  // @has async_fn/struct.Foo.html
[01:23:53] 30: @matches check failed
[01:23:53]  File does not exist 'async_fn/struct.Foo.html'
[01:23:53]  // @matches - '//code' 'pub async fn f\(\)$'
[01:23:53] Encountered 2 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/async-fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/attributes.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/attributes" "/checkout/src/test/rustdoc/attributes.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 11: @has check failed
[01:23:53]  File does not exist 'foo/enum.Foo.html'
[01:23:53]  // @has foo/enum.Foo.html '//*[@class="docblock attributes"]' '#[repr(i64)]'
[01:23:53] 12: @has check failed
[01:23:53]  File does not exist 'foo/enum.Foo.html'
[01:23:53]  // @has foo/enum.Foo.html '//*[@class="docblock attributes"]' '#[must_use]'
[01:23:53] Encountered 2 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/attributes.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/auto-traits.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-traits" "/checkout/src/test/rustdoc/auto-traits.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 9: @has check failed
[01:23:53]  File does not exist 'foo/trait.Foo.html'
[01:23:53]  // @has 'foo/trait.Foo.html' '//pre' 'pub unsafe auto trait Foo'
[01:23:53] 12: @has check failed
[01:23:53]  File does not exist 'foo/trait.Bar.html'
[01:23:53]  // @has 'foo/trait.Bar.html' '//pre' 'pub unsafe auto trait Bar'
[01:23:53] Encountered 2 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/auto-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 3: @has check failed
[01:23:53]  File does not exist 'foo/struct.S.html'
[01:23:53]  // @has foo/struct.S.html '//h3[@id="impl-Into"]//code' 'impl<T, U> Into for T'
[01:23:53] Encountered 1 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/blanket-reexport-item.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/cap-lints.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cap-lints" "/checkout/src/test/rustdoc/cap-lints.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 6: @has check failed
[01:23:53]  File does not exist 'cap_lints/struct.Foo.html'
[01:23:53]  // @has cap_lints/struct.Foo.html //pre '#[must_use]'
[01:23:53] Encountered 1 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/cap-lints.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/check-styled-link.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-styled-link" "/checkout/src/test/rustdoc/check-styled-link.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 5: @has check failed
[01:23:53]  File does not exist 'foo/struct.Bar.html'
[01:23:53]  // @has foo/struct.Bar.html '//a[@href="../foo/struct.Foo.html"]' 'Foo'
[01:23:53] Encountered 1 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/check-styled-link.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/const-doc.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-doc" "/checkout/src/test/rustdoc/const-doc.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 14: @has check failed
[01:23:53]  File does not exist 'const_doc/struct.ContentType.html'
[01:23:53]      // @has const_doc/struct.ContentType.html
[01:23:53] 15: @has check failed
[01:23:53]  File does not exist 'const_doc/struct.ContentType.html'
[01:23:53]      // @has  - '//*[@id="associatedconstant.Any"]' 'const Any: ContentType'
[01:23:53] Encountered 2 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/const-doc.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/const-fn.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-fn" "/checkout/src/test/rustdoc/const-fn.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 10: @has check failed
[01:23:53]  File does not exist 'foo/struct.Foo.html'
[01:23:53]  // @has foo/struct.Foo.html
[01:23:53] 11: @has check failed
[01:23:53]  File does not exist 'foo/struct.Foo.html'
[01:23:53]  // @has - '//*[@class="method"]' 'const fn new()'
[01:23:53] Encountered 2 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/const-fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/const.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const" "/checkout/src/test/rustdoc/const.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 6: @has check failed
[01:23:53]  File does not exist 'const/struct.Foo.html'
[01:23:53]      // @has const/struct.Foo.html '//code[@id="new.v"]' 'const unsafe fn new'
[01:23:53] Encountered 1 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/default-trait-method-link.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method-link" "/checkout/src/test/rustdoc/default-trait-method-link.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 3: @has check failed
[01:23:53]  File does not exist 'foo/trait.Foo.html'
[01:23:53]  // @has foo/trait.Foo.html '//a[@href="../foo/trait.Foo.html#tymethod.req"]' 'req'
[01:23:53] 4: @has check failed
[01:23:53]  File does not exist 'foo/trait.Foo.html'
[01:23:53]  // @has foo/trait.Foo.html '//a[@href="../foo/trait.Foo.html#method.prov"]' 'prov'
[01:23:53] Encountered 2 errors
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] 
[01:23:53] thread '[rustdoc] rustdoc/default-trait-method-link.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:23:53] 
[01:23:53] ---- [rustdoc] rustdoc/default_trait_method.rs stdout ----
[01:23:53] 
[01:23:53] error: htmldocck failed!
[01:23:53] status: exit code: 1
[01:23:53] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default_trait_method" "/checkout/src/test/rustdoc/default_trait_method.rs"
[01:23:53] ------------------------------------------
[01:23:53] 
[01:23:53] ------------------------------------------
[01:23:53] stderr:
[01:23:53] stderr:
[01:23:53] ------------------------------------------
[01:23:53] 8: @has check failed
[01:23:53]  File does not exist 'default_trait_method/trait.Item.html'
[01:23:53]  // @has default_trait_method/trait.Item.html
[01:23:53] 9: @has check failed
[01:23:53]  File does not exist 'default_trait_method/trait.Item.html'
[01:23:53]  // @has - '//*[@id="method.foo"]' 'default fn foo()'
[01:23:53] 10: @has check failed
---
[01:23:53] test result: FAILED. 138 passed; 162 failed; 2 ignored; 0 measured; 0 filtered out
[01:23:53] 
[01:23:53] 
[01:23:53] 
[01:23:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:53] 
[01:23:53] 
[01:23:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:53] Build completed unsuccessfully in 0:20:04
---
travis_time:end:0d8df817:start=1554681373447968286,finish=1554681373453494109,duration=5525823
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0144524a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:077c952b
travis_time:start:077c952b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:020f11c0
$ dmesg | grep -i kill
