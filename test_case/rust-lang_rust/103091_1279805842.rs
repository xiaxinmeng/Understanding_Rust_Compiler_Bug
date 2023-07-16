plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 565 tests
i...............F................................................i..F..FF............... 88/565
.......................F................................................................ 176/565
........................................................................................ 352/565
.................................i..................................F................... 440/565
......................i.....................FF.......................................... 528/565
.............F.......................
.............F.......................
failures:
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] src/test/rustdoc/associated-consts.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/associated-consts" "/checkout/src/test/rustdoc/associated-consts.rs"
stdout: none
--- stderr -------------------------------
36: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="sidebar-title"]' 'Associated Constants'
47: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="sidebar-title"]' 'Associated Constants'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/deref-recursive-pathbuf.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf" "/checkout/src/test/rustdoc/deref-recursive-pathbuf.rs"
stdout: none
--- stderr -------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-PathBuf"]' 'Methods from Deref<Target=PathBuf>'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-Path"]' 'Methods from Deref<Target=Path>'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/deref-typedef.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef" "/checkout/src/test/rustdoc/deref-typedef.rs"
stdout: none
--- stderr -------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-FooJ"]' 'Methods from Deref<Target=FooJ>'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/deref-recursive.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive" "/checkout/src/test/rustdoc/deref-recursive.rs"
stdout: none
--- stderr -------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-Bar"]' 'Methods from Deref<Target=Bar>'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-Baz"]' 'Methods from Deref<Target=Baz>'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/escape-deref-methods.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods" "/checkout/src/test/rustdoc/escape-deref-methods.rs"
stdout: none
--- stderr -------------------------------
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]' 'Methods from Deref<Target=Vec<Title>>'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/negative-impl-sidebar.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/negative-impl-sidebar" "/checkout/src/test/rustdoc/negative-impl-sidebar.rs"
stdout: none
--- stderr -------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#trait-implementations"]' 'Trait Implementations'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/sidebar-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-items" "/checkout/src/test/rustdoc/sidebar-items.rs"
stdout: none
--- stderr -------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#required-methods"]' 'Required Methods'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#provided-methods"]' 'Provided Methods'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#required-associated-consts"]' 'Required Associated Constants'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#provided-associated-consts"]' 'Provided Associated Constants'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#required-associated-types"]' 'Required Associated Types'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#provided-associated-types"]' 'Provided Associated Types'
28: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#fields"]' 'Fields'
39: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#variants"]' 'Variants'
48: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#fields"]' 'Fields'
Encountered 9 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/sidebar-links-to-foreign-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-links-to-foreign-impl" "/checkout/src/test/rustdoc/sidebar-links-to-foreign-impl.rs"
stdout: none
--- stderr -------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]/a[@href="#foreign-impls"]' 'Implementations on Foreign Types'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/tuple-struct-fields-doc.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/tuple-struct-fields-doc" "/checkout/src/test/rustdoc/tuple-struct-fields-doc.rs"
stdout: none
--- stderr -------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="sidebar-title"]/a[@href="#fields"]' 'Tuple Fields'
Encountered 1 errors
------------------------------------------


