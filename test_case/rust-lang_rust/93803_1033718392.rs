plain
Successfully built 126fbf228290
Successfully tagged rust-ci:latest
Built container sha256:126fbf228290a03c302d776d5b3d3b9249eb3ecaacae6ad1bd52548080f9b448
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 500 tests
i......F.......F..................F..................i..........F............F...................... 100/500
........................F........................................................................... 200/500
.....................................F.............................................................. 300/500
..................................i.............................................................F... 400/500
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.i.........................................F...........................................F........F... 500/500
failures:

---- [rustdoc] rustdoc/assoc-types.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
31: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust fn"]' 'where T::Input: PartialEq<U::Input>'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/async-fn.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
53: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//pre[@class="rust fn"]' 'pub async fn const_generics<const N: usize>(_: impl Trait<N>)'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/const-generics/const-generics-docs.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
71: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/fn.b_sink.html '//pre[@class="rust fn"]' 'pub async fn b_sink<const N: usize>(_: impl Trait<N>)'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/doc-assoc-item.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-assoc-item" "/checkout/src/test/rustdoc/doc-assoc-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has doc_assoc_item/struct.Foo.html '//*[@class="impl has-srclink"]' 'impl<T: Bar<Fuu = u32>> Foo<T>'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/doc-notable_trait.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-notable_trait" "/checkout/src/test/rustdoc/doc-notable_trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//code[@class="content"]' 'impl<T: SomeTrait> SomeTrait for Wrapper<T>'
26: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//code[@class="content"]' 'impl<T: SomeTrait> SomeTrait for Wrapper<T>'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/impl-parts.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-parts" "/checkout/src/test/rustdoc/impl-parts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has impl_parts/struct.Foo.html '//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<T: Clone> !AnAutoTrait for Foo<T> where T: Sync,"
10: @has check failed
 `XPATH PATTERN` did not match
 // @has impl_parts/trait.AnAutoTrait.html '//*[@class="item-list"]//h3[@class="code-header in-band"]' "impl<T: Clone> !AnAutoTrait for Foo<T> where T: Sync,"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-20727-4.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4" "/checkout/src/test/rustdoc/issue-20727-4.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@class="rust trait"]' 'trait Index<Idx: ?Sized> {'
19: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@class="rust trait"]' 'trait IndexMut<Idx: ?Sized>: Index<Idx> {'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/normalize-assoc-item.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/normalize-assoc-item" "/checkout/src/test/rustdoc/normalize-assoc-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
39: @has check failed
 `XPATH PATTERN` did not match
 // @has 'normalize_assoc_item/struct.Unknown.html' '//pre[@class="rust struct"]' 'pub struct Unknown<Inner: Trait>(pub <Inner as Trait>::X);'
42: @has check failed
 `XPATH PATTERN` did not match
 // @has 'normalize_assoc_item/struct.Unknown2.html' '//pre[@class="rust struct"]' 'pub struct Unknown2<Inner: Trait>(pub Inner::X);'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex" "/checkout/src/test/rustdoc/synthetic_auto/complex.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
23: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'a, T, K: ?Sized> Send for Outer<'a, T, K> where K: for<'b> Fn((&'b bool, &'a u8)) -> &'b i8, T: MyTrait<'a>, <T as MyTrait<'a>>::MyItem: Copy, 'a: 'static"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/universal-impl-trait.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/universal-impl-trait" "/checkout/src/test/rustdoc/universal-impl-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @matches check failed
 `PATTERN` did not match
 // @matches - '_x: impl <a class="trait" href="[^"]+/trait\.Clone\.html"'
9: @matches check failed
 `PATTERN` did not match
 // @matches - '_z: .+impl.+trait\.Copy\.html.+, impl.+trait\.Clone\.html'
16: @matches check failed
 `PATTERN` did not match
     // @matches - '_x: impl <a class="trait" href="[^"]+/trait\.Debug\.html"'
26: @matches check failed
 `PATTERN` did not match
     // @matches - '_bar: impl <a class="trait" href="[^"]+/trait\.Copy\.html"'
31: @matches check failed
 `PATTERN` did not match
     // @matches - '_baz:.+struct\.S\.html.+impl .+trait\.Clone\.html'
36: @matches check failed
 `PATTERN` did not match
     // @matches - 'trait\.Read\.html'
42: @matches check failed
 `PATTERN` did not match
 // @matches - '_x: impl <a class="trait" href="[^"]+/trait\.Debug\.html"'
46: @matches check failed
 `PATTERN` did not match
 // @matches - 'T:.+Borrow.+impl .+trait\.Trait\.html'
47: @matches check failed
 `PATTERN` did not match
 // @matches - 'U:.+IntoIterator.+= impl.+Iterator\.html.+= impl.+Clone\.html'
48: @matches check failed
 `PATTERN` did not match
 // @matches - '_: impl .+trait\.Read\.html.+ \+ .+trait\.Clone\.html'
Encountered 10 errors

------------------------------------------



---- [rustdoc] rustdoc/where-sized.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where-sized" "/checkout/src/test/rustdoc/where-sized.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust fn"]' 'pub fn foo<X, Y: ?Sized>(_: &X)'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust fn"]' 'where X: ?Sized,'
Encountered 2 errors

------------------------------------------

