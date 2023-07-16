plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 521 tests
i............................F...............F...........i.............................. 88/521
............F...............................................F...................F.F.F... 176/521
.................FF...........FFFFFFF...F...F.FF........................................ 264/521
......................................................................................i. 352/521
...................................F..........F.FF...................i.........F........ 440/521
............F....................................................F...............
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] src/test/rustdoc/check-source-code-urls-to-def-std.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-source-code-urls-to-def-std" "/checkout/src/test/rustdoc/check-source-code-urls-to-def-std.rs"
stdout: none
--- stderr -------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.u32.html"]' 'u32'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.str.html"]' 'str'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.bool.html"]' 'bool'
Encountered 3 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/check-source-code-urls-to-def.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-source-code-urls-to-def" "/checkout/src/test/rustdoc/check-source-code-urls-to-def.rs"
stdout: none
--- stderr -------------------------------
31: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '/struct.String.html'
32: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '/primitive.u32.html'
33: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '/primitive.str.html'
Encountered 3 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/ensure-src-link.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ensure-src-link" "/checkout/src/test/rustdoc/ensure-src-link.rs"
stdout: none
--- stderr -------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/trait.Iterator.html '//*[@id="method.zip"]//a[@class="srclink"]' "source"
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_cross/issue-28480.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480" "/checkout/src/test/rustdoc/inline_cross/issue-28480.rs"
stdout: none
--- stderr -------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has -  '//a' 'u8'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has -  '//a' 'u8'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/anchors.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/anchors" "/checkout/src/test/rustdoc/intra-doc/anchors.rs"
stdout: none
--- stderr -------------------------------
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '{{channel}}/std/primitive.u32.html#hello'
23: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '{{channel}}/std/primitive.usize.html#x'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/associated-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/associated-items" "/checkout/src/test/rustdoc/intra-doc/associated-items.rs"
stdout: none
--- stderr -------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/fn.foo.html' '//a[@href="{{channel}}/alloc/collections/btree/map/struct.BTreeMap.html#method.into_iter"]' 'std::collections::BTreeMap::into_iter'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/fn.foo.html' '//a[@href="{{channel}}/alloc/string/struct.String.html#method.from"]' 'String::from'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/fn.foo.html' '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.into_iter"]' 'Vec::into_iter'
Encountered 3 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/builtin-macros.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/builtin-macros" "/checkout/src/test/rustdoc/intra-doc/builtin-macros.rs"
stdout: none
--- stderr -------------------------------
2: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '{{channel}}/core/macro.cfg.html'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/field.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/field" "/checkout/src/test/rustdoc/intra-doc/field.rs"
stdout: none
--- stderr -------------------------------
1: @has check failed
 `XPATH PATTERN` did not match
 // @has field/index.html '//a[@href="{{channel}}/core/ops/range/struct.Range.html#structfield.start"]' 'start'
2: @has check failed
 `XPATH PATTERN` did not match
 // @has field/index.html '//a[@href="{{channel}}/std/io/error/enum.ErrorKind.html#variant.NotFound"]' 'not_found'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/generic-params.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/generic-params" "/checkout/src/test/rustdoc/intra-doc/generic-params.rs"
stdout: none
--- stderr -------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html"]' 'Vec<T>'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/boxed/struct.Box.html"]' 'Box<Vec<Option<T>>>'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item"]' 'Iterator<Box<T>>::Item'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/option/enum.Option.html"]' 'just Option'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/option/enum.Option.html"]' 'with the generic, Option<T>'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/result/enum.Result.html"]' 'Result<T, E>'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/result/enum.Result.html"]' 'Result<T, !>'
22: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/result/enum.Result.html"]' 'Result<!, E>'
28: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.new"]' 'Vec::<T>::new'
29: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.new"]' 'with parentheses as Vec::<T>::new()'
30: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.new"]' 'Vec::<Box<T>>::new()'
35: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/any/struct.TypeId.html#method.of"]' 'TypeId::of::<String>()'
36: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.len"]' 'Vec::<std::error::Error>::len'
41: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/boxed/struct.Box.html#method.new"]' 'Box::<T>new()'
Encountered 14 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/non-path-primitives.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/non-path-primitives" "/checkout/src/test/rustdoc/intra-doc/non-path-primitives.rs"
stdout: none
--- stderr -------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/std/primitive.slice.html#method.rotate_left"]' 'slice::rotate_left'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.array.html#method.map"]' 'array::map'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.str.html"]' 'owned str'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.str.html"]' 'str ref'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.str.html#method.is_empty"]' 'str::is_empty'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.str.html#method.len"]' '&str::len'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.pointer.html#method.is_null"]' 'pointer::is_null'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.pointer.html#method.is_null"]' '*const::is_null'
22: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.pointer.html#method.is_null"]' '*mut::is_null'
27: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.unit.html"]' 'unit'
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.tuple.html"]' 'tuple'
33: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.reference.html"]' 'reference'
34: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.reference.html"]' '&'
35: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.reference.html"]' '&mut'
40: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.fn.html"]' 'fn'
43: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.never.html"]' 'never'
44: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/primitive.never.html"]' '!'
Encountered 17 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/prim-assoc.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-assoc" "/checkout/src/test/rustdoc/intra-doc/prim-assoc.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has prim_assoc/index.html '//a[@href="{{channel}}/std/primitive.i32.html#associatedconstant.MAX"]' "i32::MAX"
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/prim-associated-traits.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-associated-traits" "/checkout/src/test/rustdoc/intra-doc/prim-associated-traits.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.f64.html#method.from_str"]' 'f64::from_str()'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.f32.html#method.from_str"]' 'f32::from_str()'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.isize.html#method.from_str"]' 'isize::from_str()'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.i8.html#method.from_str"]' 'i8::from_str()'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.i16.html#method.from_str"]' 'i16::from_str()'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.i32.html#method.from_str"]' 'i32::from_str()'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.i64.html#method.from_str"]' 'i64::from_str()'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.i128.html#method.from_str"]' 'i128::from_str()'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.usize.html#method.from_str"]' 'usize::from_str()'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.u8.html#method.from_str"]' 'u8::from_str()'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.u16.html#method.from_str"]' 'u16::from_str()'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.u32.html#method.from_str"]' 'u32::from_str()'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.u64.html#method.from_str"]' 'u64::from_str()'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.u128.html#method.from_str"]' 'u128::from_str()'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.char.html#method.from_str"]' 'char::from_str()'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.bool.html#method.from_str"]' 'bool::from_str()'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.str.html#method.eq"]' 'str::eq()'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has 'prim_associated_traits/struct.Number.html' '//a[@href="{{channel}}/std/primitive.never.html#method.eq"]' 'never::eq()'
Encountered 18 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/prim-methods.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods" "/checkout/src/test/rustdoc/intra-doc/prim-methods.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main-content"]//a[@href="{{channel}}/std/primitive.char.html"]' 'char'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main-content"]//a[@href="{{channel}}/std/primitive.char.html#method.len_utf8"]' 'char::len_utf8'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/prim-precedence.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-precedence" "/checkout/src/test/rustdoc/intra-doc/prim-precedence.rs"
stdout: none
--- stderr -------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
     // @has prim_precedence/char/struct.Inner.html '//a/@href' '{{channel}}/std/primitive.char.html'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has prim_precedence/struct.MyString.html '//a/@href' '{{channel}}/std/primitive.char.html'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/primitive-disambiguators.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/primitive-disambiguators" "/checkout/src/test/rustdoc/intra-doc/primitive-disambiguators.rs"
stdout: none
--- stderr -------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '{{channel}}/std/primitive.str.html#method.trim'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/primitive-non-default-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/primitive-non-default-impl" "/checkout/src/test/rustdoc/intra-doc/primitive-non-default-impl.rs"
stdout: none
--- stderr -------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.str.html#method.trim"]' 'str::trim'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.str.html#method.to_lowercase"]' 'str::to_lowercase'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.str.html#method.into_boxed_bytes"]' 'str::into_boxed_bytes'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.str.html#method.replace"]' 'str::replace'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.f32.html#method.powi"]' 'f32::powi'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.f32.html#method.sqrt"]' 'f32::sqrt'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.f32.html#method.mul_add"]' 'f32::mul_add'
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.f64.html#method.powi"]' 'f64::powi'
28: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.f64.html#method.sqrt"]' 'f64::sqrt'
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/std/primitive.f64.html#method.mul_add"]' 'f64::mul_add'
Encountered 10 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/pub-use.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/pub-use" "/checkout/src/test/rustdoc/intra-doc/pub-use.rs"
stdout: none
--- stderr -------------------------------
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/env/index.html"]' "std::env"
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/trait-item.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/trait-item" "/checkout/src/test/rustdoc/intra-doc/trait-item.rs"
stdout: none
--- stderr -------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/core/default/trait.Default.html#tymethod.default"]' 'Default::default()'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/true-false.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/true-false" "/checkout/src/test/rustdoc/intra-doc/true-false.rs"
stdout: none
--- stderr -------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main-content"]//a[@href="{{channel}}/std/primitive.bool.html"]' 'true'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main-content"]//a[@href="{{channel}}/std/primitive.bool.html"]' 'false'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/type-alias.rs stdout ----
