plain
.................................................................................................... 9100/11288
.................................................................................................... 9200/11288
....................................................................................i......i........ 9300/11288
.................................................................................................... 9400/11288
......................iiiiii..iiiiii..i............................................................. 9500/11288
.................................................................................................... 9700/11288
.................................................................................................... 9800/11288
.................................................................................................... 9900/11288
.................................................................................................... 10000/11288
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.073 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..i.i...i.i....ii..........i.iii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.25s

 finished in 2.320 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; generated diffs will be harder to read

running 413 tests
...........FF............F..........i.....F.......................F.....F....F......FF........F..FF. 100/413
.F.F.FFFFF.F..F....................F................................................FF..F.........F. 200/413
F...........FF............................................F......FF...........FF............F...F... 300/413
.........................F........i.................................FF.F.FFF.....F.F................ 400/413
failures:

---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----


error: htmldocck failed!
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/primitive.i16.html' '//h2[@id="synthetic-implementations"]' 'Auto Trait Implementation'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/auto-impl-primitive.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive.nightly" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"`', src/tools/compiletest/src/runtest.rs:1871:33

---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.S.html '//h3[@id="impl-Into%3CU%3E"]//code' 'impl<T, U> Into<U> for T'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/blanket-reexport-item.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item.nightly" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//h3[@id="impl-Send"]/code' 'impl<T, const ORDER: Order> Send for VSet<T, ORDER>'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//h3[@id="impl-Sync"]/code' 'impl<T, const ORDER: Order> Sync for VSet<T, ORDER>'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/const-generics/const-impl.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl.nightly" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/deref-recursive-pathbuf.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf" "/checkout/src/test/rustdoc/deref-recursive-pathbuf.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@id="deref-methods-PathBuf"]' 'Methods from Deref<Target = PathBuf>'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.as_path"]' 'pub fn as_path(&self)'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@id="deref-methods-Path"]' 'Methods from Deref<Target = Path>'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.exists"]' 'pub fn exists(&self)'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"][@href="#deref-methods-PathBuf"]' 'Methods from Deref<Target=PathBuf>'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.as_path"]' 'as_path'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"][@href="#deref-methods-Path"]' 'Methods from Deref<Target=Path>'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.exists"]' 'exists'
Encountered 8 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/deref-recursive-pathbuf.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf.nightly" "/checkout/src/test/rustdoc/deref-recursive-pathbuf.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/escape-deref-methods.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods" "/checkout/src/test/rustdoc/escape-deref-methods.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]' 'Methods from Deref<Target=Vec<Title>>'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/escape-deref-methods.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods.nightly" "/checkout/src/test/rustdoc/escape-deref-methods.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/extern-default-method.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method" "/checkout/src/test/rustdoc/extern-default-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @count check failed
 Expected 1 occurrences but found 0
 // @count extern_default_method/struct.Struct.html '//*[@id="method.provided"]' 1
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/extern-default-method.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/auxiliary/rustdoc-extern-default-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/extern-impl-trait.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl-trait" "/checkout/src/test/rustdoc/extern-impl-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/struct.X.html' '//code' "impl Foo<Associated = ()> + 'a"
10: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/struct.Y.html' '//code' "impl ?Sized + Foo<Associated = ()> + 'a"
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/extern-impl-trait.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/auxiliary/extern-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl-trait/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl-trait/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/generic-impl.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//h3[@id="impl-ToString"]//code' 'impl<T> ToString for T'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//div[@class="sidebar-links"]/a[@href="#impl-ToString"]' 'ToString'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/generic-impl.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl.nightly" "/checkout/src/test/rustdoc/generic-impl.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/hidden-impls.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls" "/checkout/src/test/rustdoc/hidden-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @has check failed
 File does not exist 'implementors/foo/trait.Clone.js'
 // @has implementors/foo/trait.Clone.js
16: @!has check failed
 File does not exist 'implementors/foo/trait.Clone.js'
 // @!has - 'Foo'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/hidden-impls.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls.nightly" "/checkout/src/test/rustdoc/hidden-impls.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/hide-unstable-trait.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hide-unstable-trait" "/checkout/src/test/rustdoc/hide-unstable-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @has check failed
 `PATTERN` did not match
 // @has foo/struct.Foo.html 'bar2'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/hide-unstable-trait.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/auxiliary/unstable-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hide-unstable-trait/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hide-unstable-trait/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/inline_cross/default-trait-method.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/default-trait-method" "/checkout/src/test/rustdoc/inline_cross/default-trait-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.foo"]' 'default fn foo()'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.bar"]' 'fn bar()'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.baz"]' 'fn baz()'
Encountered 3 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_cross/default-trait-method.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/inline_cross/auxiliary/default-trait-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/default-trait-method/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/default-trait-method/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/inline_cross/impl_trait.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl_trait" "/checkout/src/test/rustdoc/inline_cross/impl_trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
35: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.method"]//code' "pub fn method<'a>(_x: impl Clone + Into<Vec<u8, Global>> + 'a)"
40: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.async_foo"]' "pub async fn async_foo("
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_cross/impl_trait.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/inline_cross/auxiliary/impl_trait_aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl_trait/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl_trait/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="associatedconstant.PublicConst"]' 'pub const PublicConst: u8'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock"]' 'docs for PublicConst'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.public_method"]' 'pub fn public_method()'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock"]' 'docs for public_method'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="associatedconstant.ConstNoDefault"]' 'const ConstNoDefault: i16'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock"]' 'dox for ConstNoDefault'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="associatedconstant.ConstWithDefault"]' 'const ConstWithDefault: u16'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock hidden"]' 'docs for ConstWithDefault'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="associatedtype.TypeNoDefault"]' 'type TypeNoDefault = i32'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock"]' 'dox for TypeNoDefault'
22: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="associatedtype.TypeWithDefault"]' 'type TypeWithDefault = u32'
23: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock hidden"]' 'docs for TypeWithDefault'
24: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.method_no_default"]' 'fn method_no_default()'
25: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock"]' 'dox for method_no_default'
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
27: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock hidden"]' 'docs for method_with_default'
Encountered 16 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_cross/assoc-items.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/inline_cross/auxiliary/assoc-items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait" "/checkout/src/test/rustdoc/inline_cross/impl-inline-without-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.my_trait_method"]' 'fn my_trait_method()'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock hidden"]' 'docs for my_trait_method'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/inline_cross/auxiliary/impl-inline-without-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/inline_cross/issue-28480.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480" "/checkout/src/test/rustdoc/inline_cross/issue-28480.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has -  '//a' 'u8'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_cross/issue-28480.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-hidden-sig.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/inline_cross/issue-31948-1.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1" "/checkout/src/test/rustdoc/inline_cross/issue-31948-1.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl"]//code' 'Bark for'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl"]//code' 'Woof for'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'for Foo'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'for Wobble'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'for Foo'
22: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'for Wobble'
Encountered 6 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_cross/issue-31948-1.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/inline_cross/auxiliary/rustdoc-nonreachable-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/inline_cross/issue-31948-2.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-2" "/checkout/src/test/rustdoc/inline_cross/issue-31948-2.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl"]//code' 'Qux for'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl"]//code' 'Bark for'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl"]//code' 'Woof for'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'for Foo'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'for Wobble'
Encountered 5 errors

------------------------------------------

---
test result: FAILED. 366 passed; 45 failed; 2 ignored; 0 measured; 0 filtered out; finished in 11.20s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:59
