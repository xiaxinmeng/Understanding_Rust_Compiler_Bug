plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 528 tests
i.....F................F.....F........F.....................i........................... 88/528
.....F............................F.........F........................................... 176/528
......................................................................................F. 264/528
......................................................................................F. 352/528
...i........................................................F..............i........F..F 440/528
...................F.................................................................... 528/528
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] src/test/rustdoc/auto-trait-not-send.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-trait-not-send" "/checkout/src/test/rustdoc/auto-trait-not-send.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="impl-Send"]' 'impl !Send for Foo'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="impl-Sync"]' 'impl !Sync for Foo'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/blanket-reexport-item.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
stdout: none
--- stderr -------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.S.html '//div[@id="impl-Into%3CU%3E-for-S"]//h3[@class="code-header in-band"]' 'impl<T, U> Into<U> for T'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/const-generics/const-generics-docs.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "--deny" "warnings" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: expected item, found `>>`
   |
   |
40 | >>>>>>> 083cf2a97a8... rustdoc: Add more semantic information to impl ids
   | ^^ expected item
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/const-generics/const-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
stdout: none
--- stderr -------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Send-for-VSet%3CT%2C%20ORDER%3E"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Send for VSet<T, ORDER>'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Sync-for-VSet%3CT%2C%20ORDER%3E"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Sync for VSet<T, ORDER>'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-VSet%3CT%2C%20{%20Order%3A%3ASorted%20}%3E"]/h3[@class="code-header in-band"]' 'impl<T> VSet<T, { Order::Sorted }>'
25: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-VSet%3CT%2C%20{%20Order%3A%3AUnsorted%20}%3E"]/h3[@class="code-header in-band"]' 'impl<T> VSet<T, { Order::Unsorted }>'
34: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Escape.html '//div[@id="impl-Escape%3Cr#%22%3Cscript%3Ealert(%22Escape%22)%3B%3C/script%3E%22#%3E"]/h3[@class="code-header in-band"]' 'impl Escape<r#"<script>alert("Escape");</script>"#>'
Encountered 5 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/empty-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-impls" "/checkout/src/test/rustdoc/empty-impls.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="synthetic-implementations-list"]/div[@id="impl-Send-for-Foo"]' 'impl Send for Foo'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="trait-implementations-list"]/div[@id="impl-EmptyTrait-for-Foo"]' 'impl EmptyTrait for Foo'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="trait-implementations-list"]/details/summary/div[@id="impl-NotEmpty-for-Foo"]' 'impl NotEmpty for Foo'
Encountered 3 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/generic-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
stdout: none
--- stderr -------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//div[@id="impl-ToString-for-Foo"]//h3[@class="code-header in-band"]' 'impl<T> ToString for T'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/impl-box.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-box" "/checkout/src/test/rustdoc/impl-box.rs"
stdout: none
--- stderr -------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@id="impl-Iterator"]' 'impl Iterator for Box<MyType>'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-29503.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-29503.rs"
stdout: none
--- stderr -------------------------------
error: non-item in item list
   |
   |
9  | impl<T> MyTrait for T where T: fmt::Debug {
   |                                           - item list starts here
10 | >>>>>>> 083cf2a97a8... rustdoc: Add more semantic information to impl ids
   | ^^ non-item starts here
14 | }
14 | }
   | - item list ends here
error: Compilation failed, aborting rustdoc

error: aborting due to 2 previous errors
------------------------------------------
------------------------------------------


---- [rustdoc] src/test/rustdoc/issue-75588.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-75588" "/checkout/src/test/rustdoc/issue-75588.rs"
stdout: none
--- stderr -------------------------------
16: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/trait.Join.html '//*[@id="impl-Join"]//h3[@class="code-header in-band"]' 'impl Join for Foo'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/primitive/primitive-generic-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive/primitive-generic-impl.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/primitive.i32.html '//div[@id="impl-ToString-for-i32"]//h3[@class="code-header in-band"]' 'impl<T> ToString for T'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/sized_trait.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sized_trait/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sized_trait" "--deny" "warnings" "/checkout/src/test/rustdoc/sized_trait.rs"
stdout: none
--- stderr -------------------------------
error: expected item, found `>>`
   |
   |
15 | >>>>>>> 083cf2a97a8... rustdoc: Add more semantic information to impl ids
   | ^^ expected item
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/rfc-2632-const-trait-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/rfc-2632-const-trait-impl" "/checkout/src/test/rustdoc/rfc-2632-const-trait-impl.rs"
stdout: none
--- stderr -------------------------------
33: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//section[@id="impl-Tr%3CT%3E"]/h3[@class="code-header in-band"]/a[@class="trait"]' 'Clone'
35: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//section[@id="impl-Tr%3CT%3E"]/h3[@class="code-header in-band"]/span[@class="where fmt-newline"]' ': Clone'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/src-links-auto-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-auto-impls" "/checkout/src/test/rustdoc/src-links-auto-impls.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Sized-for-Unsized"]/h3[@class="code-header in-band"]' 'impl !Sized for Unsized'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Sync-for-Unsized"]/h3[@class="code-header in-band"]' 'impl Sync for Unsized'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Any-for-Unsized"]/h3[@class="code-header in-band"]' 'impl<T> Any for T'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Any-for-Unsized"]//a[@class="srclink"]' 'source'
Encountered 4 errors
------------------------------------------


