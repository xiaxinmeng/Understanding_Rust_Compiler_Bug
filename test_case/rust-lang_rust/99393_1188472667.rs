plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 543 tests
i...............................F...F.........F................i........................ 88/543
........................................................................................ 264/543
........................................................................................ 352/543
..............i......................................................................... 440/543
.i...................................................................................... 528/543
.i...................................................................................... 528/543
...............
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] src/test/rustdoc/const-generics/add-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/add-impl" "/checkout/src/test/rustdoc/const-generics/add-impl.rs"
stdout: none
--- stderr -------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Simd.html '//div[@id="trait-implementations-list"]//h3[@class="code-header in-band"]' 'impl Add<Simd<u8, 16_usize>> for Simd<u8, 16>'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/const-generics/const-generic-defaults.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generic-defaults" "/checkout/src/test/rustdoc/const-generics/const-generic-defaults.rs"
stdout: none
--- stderr -------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//pre[@class="rust struct"]' 'pub struct Foo<const M: usize = 10_usize, const N: usize = M, T = i32>(_);'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/const-generics/const-generics-docs.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs"
stdout: none
--- stderr -------------------------------
22: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="impl-Trait%3C1_usize%3E-for-u8"]//h3[@class="code-header in-band"]' 'impl Trait<1_usize> for u8'
23: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="impl-Trait%3C2_usize%3E-for-u8"]//h3[@class="code-header in-band"]' 'impl Trait<2_usize> for u8'
Encountered 2 errors
------------------------------------------


