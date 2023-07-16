plain
2019-09-13T20:06:57.5748349Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-13T20:06:57.5936242Z ##[command]git config gc.auto 0
2019-09-13T20:06:57.6017532Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-13T20:06:57.6083149Z ##[command]git config --get-all http.proxy
2019-09-13T20:06:57.6239602Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64443/merge:refs/remotes/pull/64443/merge
---
2019-09-13T21:14:21.5495193Z .................................................................................................... 1500/9012
2019-09-13T21:14:27.8546445Z .................................................................................................... 1600/9012
2019-09-13T21:14:41.4360202Z ..........................................................i...............i......................... 1700/9012
2019-09-13T21:14:49.7960920Z .................................................................................................... 1800/9012
2019-09-13T21:15:05.8881058Z .................................................iiiii.............................................. 1900/9012
2019-09-13T21:15:17.6904222Z .................................................................................................... 2100/9012
2019-09-13T21:15:20.4119172Z .................................................................................................... 2200/9012
2019-09-13T21:15:24.2814656Z .................................................................................................... 2300/9012
2019-09-13T21:15:32.9445737Z .................................................................................................... 2400/9012
---
2019-09-13T21:18:44.7541556Z ....................................i...............i............................................... 4700/9012
2019-09-13T21:18:57.0397718Z .................................................................................................... 4800/9012
2019-09-13T21:19:04.2912988Z .................................................................................................... 4900/9012
2019-09-13T21:19:15.9123898Z .................................................................................................... 5000/9012
2019-09-13T21:19:22.6795003Z ...................ii.ii............................................................................ 5100/9012
2019-09-13T21:19:33.9866330Z .................................................................................................... 5300/9012
2019-09-13T21:19:44.8581903Z ...................................................................................i................ 5400/9012
2019-09-13T21:19:53.5116958Z .................................................................................................... 5500/9012
2019-09-13T21:19:59.4879723Z .................................................................................................... 5600/9012
2019-09-13T21:19:59.4879723Z .................................................................................................... 5600/9012
2019-09-13T21:20:10.6961646Z ...............................................................................ii..i..ii...........i 5700/9012
2019-09-13T21:20:38.4597913Z .................................................................................................... 5900/9012
2019-09-13T21:20:48.6051118Z .................................................................................................... 6000/9012
2019-09-13T21:20:48.6051118Z .................................................................................................... 6000/9012
2019-09-13T21:20:53.7857299Z ................................................................................i..ii............... 6100/9012
2019-09-13T21:21:26.5176692Z .................................................................................................... 6300/9012
2019-09-13T21:21:28.9188156Z .......................................i............................................................ 6400/9012
2019-09-13T21:21:31.1814435Z .................................................................................................... 6500/9012
2019-09-13T21:21:33.9413617Z ...........i........................................................................................ 6600/9012
---
2019-09-13T21:26:27.1486814Z  finished in 5.551
2019-09-13T21:26:27.1708328Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T21:26:27.3626208Z 
2019-09-13T21:26:27.3631870Z running 150 tests
2019-09-13T21:26:30.9474073Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-13T21:26:33.0700932Z ..iiii..............i.........iii.i.......ii......
2019-09-13T21:26:33.0704608Z 
2019-09-13T21:26:33.0705638Z  finished in 5.899
2019-09-13T21:26:33.0884439Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T21:26:33.2547034Z 
---
2019-09-13T21:26:35.4766886Z  finished in 2.388
2019-09-13T21:26:35.4989494Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T21:26:35.6743942Z 
2019-09-13T21:26:35.6744310Z running 9 tests
2019-09-13T21:26:35.6747554Z iiiiiiiii
2019-09-13T21:26:35.6748021Z 
2019-09-13T21:26:35.6748584Z  finished in 0.175
2019-09-13T21:26:35.6968090Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T21:26:35.8730322Z 
---
2019-09-13T21:26:55.9456391Z  finished in 19.680
2019-09-13T21:26:55.9457078Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T21:26:55.9457316Z 
2019-09-13T21:26:55.9457952Z running 123 tests
2019-09-13T21:27:21.6240649Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-13T21:27:26.9544361Z i.i.i......iii.i.....ii
2019-09-13T21:27:26.9627386Z 
2019-09-13T21:27:26.9627473Z  finished in 31.166
2019-09-13T21:27:26.9627806Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T21:27:26.9628071Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-13T21:35:43.5651738Z failures:
2019-09-13T21:35:43.5651774Z 
2019-09-13T21:35:43.5652023Z ---- [rustdoc] rustdoc/assoc-types.rs stdout ----
2019-09-13T21:35:43.5652055Z 
2019-09-13T21:35:43.5652115Z error: htmldocck failed!
2019-09-13T21:35:43.5652381Z status: exit code: 1
2019-09-13T21:35:43.5652788Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
2019-09-13T21:35:43.5653247Z ------------------------------------------
2019-09-13T21:35:43.5653279Z 
2019-09-13T21:35:43.5653477Z ------------------------------------------
2019-09-13T21:35:43.5653537Z stderr:
2019-09-13T21:35:43.5653537Z stderr:
2019-09-13T21:35:43.5653735Z ------------------------------------------
2019-09-13T21:35:43.5653780Z 11: @has check failed
2019-09-13T21:35:43.5653841Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5654100Z      // @has - '//*[@id="tymethod.index"]//code' "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
2019-09-13T21:35:43.5654175Z Encountered 1 errors
2019-09-13T21:35:43.5654221Z 
2019-09-13T21:35:43.5661682Z ------------------------------------------
2019-09-13T21:35:43.5661751Z 
2019-09-13T21:35:43.5661751Z 
2019-09-13T21:35:43.5661798Z 
2019-09-13T21:35:43.5662045Z ---- [rustdoc] rustdoc/impl-disambiguation.rs stdout ----
2019-09-13T21:35:43.5662111Z 
2019-09-13T21:35:43.5662164Z error: htmldocck failed!
2019-09-13T21:35:43.5662206Z status: exit code: 1
2019-09-13T21:35:43.5662744Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-disambiguation" "/checkout/src/test/rustdoc/impl-disambiguation.rs"
2019-09-13T21:35:43.5662998Z ------------------------------------------
2019-09-13T21:35:43.5663026Z 
2019-09-13T21:35:43.5663236Z ------------------------------------------
2019-09-13T21:35:43.5663275Z stderr:
2019-09-13T21:35:43.5663275Z stderr:
2019-09-13T21:35:43.5663464Z ------------------------------------------
2019-09-13T21:35:43.5663527Z 13: @has check failed
2019-09-13T21:35:43.5663568Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5663806Z  // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl<'a> Foo for &'a Bar<u8>"
2019-09-13T21:35:43.5663883Z 28: @has check failed
2019-09-13T21:35:43.5663925Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5664174Z  // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl<'a> Foo for &'a foo::mod2::Baz"
2019-09-13T21:35:43.5664272Z Encountered 2 errors
2019-09-13T21:35:43.5664297Z 
2019-09-13T21:35:43.5664490Z ------------------------------------------
2019-09-13T21:35:43.5664518Z 
2019-09-13T21:35:43.5664518Z 
2019-09-13T21:35:43.5664559Z 
2019-09-13T21:35:43.5664777Z ---- [rustdoc] rustdoc/inline_cross/issue-33113.rs stdout ----
2019-09-13T21:35:43.5664808Z 
2019-09-13T21:35:43.5664845Z error: htmldocck failed!
2019-09-13T21:35:43.5664901Z status: exit code: 1
2019-09-13T21:35:43.5665240Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-33113" "/checkout/src/test/rustdoc/inline_cross/issue-33113.rs"
2019-09-13T21:35:43.5665515Z ------------------------------------------
2019-09-13T21:35:43.5665545Z 
2019-09-13T21:35:43.5665734Z ------------------------------------------
2019-09-13T21:35:43.5665774Z stderr:
2019-09-13T21:35:43.5665774Z stderr:
2019-09-13T21:35:43.5665993Z ------------------------------------------
2019-09-13T21:35:43.5666035Z 8: @has check failed
2019-09-13T21:35:43.5666074Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5666282Z  // @has - '//code' "for &'a char"
2019-09-13T21:35:43.5666349Z Encountered 1 errors
2019-09-13T21:35:43.5666374Z 
2019-09-13T21:35:43.5666585Z ------------------------------------------
2019-09-13T21:35:43.5666613Z 
2019-09-13T21:35:43.5666613Z 
2019-09-13T21:35:43.5666637Z 
2019-09-13T21:35:43.5666837Z ---- [rustdoc] rustdoc/issue-20727.rs stdout ----
2019-09-13T21:35:43.5666866Z 
2019-09-13T21:35:43.5666923Z error: htmldocck failed!
2019-09-13T21:35:43.5667346Z status: exit code: 1
2019-09-13T21:35:43.5668105Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727" "/checkout/src/test/rustdoc/issue-20727.rs"
2019-09-13T21:35:43.5668473Z ------------------------------------------
2019-09-13T21:35:43.5668625Z 
2019-09-13T21:35:43.5668871Z ------------------------------------------
2019-09-13T21:35:43.5668939Z stderr:
2019-09-13T21:35:43.5668939Z stderr:
2019-09-13T21:35:43.5669151Z ------------------------------------------
2019-09-13T21:35:43.5669199Z 12: @has check failed
2019-09-13T21:35:43.5669262Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5669521Z      // @has - '//*[@class="rust trait"]' "fn deref<'a>(&'a self) -> &'a Self::Target;"
2019-09-13T21:35:43.5669572Z 21: @has check failed
2019-09-13T21:35:43.5669635Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5669892Z      // @has - '//*[@class="rust trait"]' "fn deref(&'a self) -> &'a Self::Target;"
2019-09-13T21:35:43.5669970Z Encountered 2 errors
2019-09-13T21:35:43.5670016Z 
2019-09-13T21:35:43.5670230Z ------------------------------------------
2019-09-13T21:35:43.5670262Z 
2019-09-13T21:35:43.5670262Z 
2019-09-13T21:35:43.5670299Z 
2019-09-13T21:35:43.5670542Z ---- [rustdoc] rustdoc/issue-25001.rs stdout ----
2019-09-13T21:35:43.5670575Z 
2019-09-13T21:35:43.5670627Z error: htmldocck failed!
2019-09-13T21:35:43.5670669Z status: exit code: 1
2019-09-13T21:35:43.5671178Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001" "/checkout/src/test/rustdoc/issue-25001.rs"
2019-09-13T21:35:43.5671417Z ------------------------------------------
2019-09-13T21:35:43.5671463Z 
2019-09-13T21:35:43.5671656Z ------------------------------------------
2019-09-13T21:35:43.5671695Z stderr:
2019-09-13T21:35:43.5671695Z stderr:
2019-09-13T21:35:43.5671883Z ------------------------------------------
2019-09-13T21:35:43.5671944Z 34: @has check failed
2019-09-13T21:35:43.5671984Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5672210Z      // @has - '//*[@id="associatedtype.Item-1"]//code' "type Item = &'a T"
2019-09-13T21:35:43.5672282Z 41: @has check failed
2019-09-13T21:35:43.5672321Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5672547Z      // @has - '//*[@id="associatedtype.Item-2"]//code' "type Item = &'a mut T"
2019-09-13T21:35:43.5672642Z Encountered 2 errors
2019-09-13T21:35:43.5672666Z 
2019-09-13T21:35:43.5672857Z ------------------------------------------
2019-09-13T21:35:43.5672885Z 
2019-09-13T21:35:43.5672885Z 
2019-09-13T21:35:43.5672928Z 
2019-09-13T21:35:43.5673127Z ---- [rustdoc] rustdoc/issue-45584.rs stdout ----
2019-09-13T21:35:43.5673156Z 
2019-09-13T21:35:43.5673194Z error: htmldocck failed!
2019-09-13T21:35:43.5673252Z status: exit code: 1
2019-09-13T21:35:43.5673569Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-45584" "/checkout/src/test/rustdoc/issue-45584.rs"
2019-09-13T21:35:43.5673827Z ------------------------------------------
2019-09-13T21:35:43.5673855Z 
2019-09-13T21:35:43.5674050Z ------------------------------------------
2019-09-13T21:35:43.5674090Z stderr:
2019-09-13T21:35:43.5674090Z stderr:
2019-09-13T21:35:43.5674298Z ------------------------------------------
2019-09-13T21:35:43.5674349Z 8: @has check failed
2019-09-13T21:35:43.5674387Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5674630Z  // @has - '//*[@class="impl"]' "impl Bar<Foo1, &'static Foo1> for Foo1"
2019-09-13T21:35:43.5674675Z 14: @has check failed
2019-09-13T21:35:43.5674714Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5674949Z  // @has - '//*[@class="impl"]' "impl Bar<&'static Foo2, Foo2> for u8"
2019-09-13T21:35:43.5675607Z Encountered 2 errors
2019-09-13T21:35:43.5675667Z 
2019-09-13T21:35:43.5676006Z ------------------------------------------
2019-09-13T21:35:43.5676038Z 
2019-09-13T21:35:43.5676038Z 
2019-09-13T21:35:43.5676061Z 
2019-09-13T21:35:43.5676268Z ---- [rustdoc] rustdoc/private-type-alias.rs stdout ----
2019-09-13T21:35:43.5676297Z 
2019-09-13T21:35:43.5676352Z error: htmldocck failed!
2019-09-13T21:35:43.5676392Z status: exit code: 1
2019-09-13T21:35:43.5676880Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/private-type-alias" "/checkout/src/test/rustdoc/private-type-alias.rs"
2019-09-13T21:35:43.5677687Z ------------------------------------------
2019-09-13T21:35:43.5677777Z 
2019-09-13T21:35:43.5678011Z ------------------------------------------
2019-09-13T21:35:43.5678057Z stderr:
2019-09-13T21:35:43.5678057Z stderr:
2019-09-13T21:35:43.5678271Z ------------------------------------------
2019-09-13T21:35:43.5678317Z 28: @has check failed
2019-09-13T21:35:43.5678381Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5678633Z  // @has private_type_alias/fn.get_lifetime_priv.html '//pre' "&'static isize"
2019-09-13T21:35:43.5678727Z Encountered 1 errors
2019-09-13T21:35:43.5678755Z 
2019-09-13T21:35:43.5678967Z ------------------------------------------
2019-09-13T21:35:43.5678998Z 
2019-09-13T21:35:43.5678998Z 
2019-09-13T21:35:43.5679024Z 
2019-09-13T21:35:43.5679293Z ---- [rustdoc] rustdoc/sidebar-links-to-foreign-impl.rs stdout ----
2019-09-13T21:35:43.5679328Z 
2019-09-13T21:35:43.5679370Z error: htmldocck failed!
2019-09-13T21:35:43.5679419Z status: exit code: 1
2019-09-13T21:35:43.5679826Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-links-to-foreign-impl" "/checkout/src/test/rustdoc/sidebar-links-to-foreign-impl.rs"
2019-09-13T21:35:43.5680095Z ------------------------------------------
2019-09-13T21:35:43.5680146Z 
2019-09-13T21:35:43.5680357Z ------------------------------------------
2019-09-13T21:35:43.5680402Z stderr:
2019-09-13T21:35:43.5680402Z stderr:
2019-09-13T21:35:43.5680630Z ------------------------------------------
2019-09-13T21:35:43.5680678Z 10: @has check failed
2019-09-13T21:35:43.5680720Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5681136Z  // @has - '//*[@class="sidebar-links"]/a[@href="#impl-Foo-for-%26%27a%20str"]' "&'a str"
2019-09-13T21:35:43.5681210Z 11: @has check failed
2019-09-13T21:35:43.5681412Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5681648Z  // @has - '//h3[@id="impl-Foo-for-%26%27a%20str"]//code' "impl<'a> Foo for &'a str"
2019-09-13T21:35:43.5681744Z Encountered 2 errors
2019-09-13T21:35:43.5681770Z 
2019-09-13T21:35:43.5681963Z ------------------------------------------
2019-09-13T21:35:43.5682009Z 
2019-09-13T21:35:43.5682009Z 
2019-09-13T21:35:43.5682032Z 
2019-09-13T21:35:43.5682393Z ---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----
2019-09-13T21:35:43.5682422Z 
2019-09-13T21:35:43.5682477Z error: htmldocck failed!
2019-09-13T21:35:43.5682515Z status: exit code: 1
2019-09-13T21:35:43.5682832Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex" "/checkout/src/test/rustdoc/synthetic_auto/complex.rs"
2019-09-13T21:35:43.5683086Z ------------------------------------------
2019-09-13T21:35:43.5683121Z 
2019-09-13T21:35:43.5683306Z ------------------------------------------
2019-09-13T21:35:43.5683361Z stderr:
2019-09-13T21:35:43.5683361Z stderr:
2019-09-13T21:35:43.5683547Z ------------------------------------------
2019-09-13T21:35:43.5683597Z 23: @has check failed
2019-09-13T21:35:43.5683652Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5684003Z  // @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]//code' "impl<'a, T, K: ?Sized> Send for Outer<'a, T, K> where K: for<'b> Fn((&'b bool, &'a u8)) -> &'b i8, T: MyTrait<'a>, <T as MyTrait<'a>>::MyItem: Copy, 'a: 'static"
2019-09-13T21:35:43.5684082Z Encountered 1 errors
2019-09-13T21:35:43.5684125Z 
2019-09-13T21:35:43.5684309Z ------------------------------------------
2019-09-13T21:35:43.5684336Z 
2019-09-13T21:35:43.5684336Z 
2019-09-13T21:35:43.5684358Z 
2019-09-13T21:35:43.5684575Z ---- [rustdoc] rustdoc/synthetic_auto/lifetimes.rs stdout ----
2019-09-13T21:35:43.5684605Z 
2019-09-13T21:35:43.5684640Z error: htmldocck failed!
2019-09-13T21:35:43.5684782Z status: exit code: 1
2019-09-13T21:35:43.5685174Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/lifetimes" "/checkout/src/test/rustdoc/synthetic_auto/lifetimes.rs"
2019-09-13T21:35:43.5685523Z ------------------------------------------
2019-09-13T21:35:43.5685571Z 
2019-09-13T21:35:43.5685757Z ------------------------------------------
2019-09-13T21:35:43.5685796Z stderr:
2019-09-13T21:35:43.5685796Z stderr:
2019-09-13T21:35:43.5685977Z ------------------------------------------
2019-09-13T21:35:43.5686035Z 12: @has check failed
2019-09-13T21:35:43.5686072Z  `XPATH PATTERN` did not match
2019-09-13T21:35:43.5686365Z  // @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]//code' "impl<'c, K> Send for Foo<'c, K> where K: for<'b> Fn(&'b bool) -> &'c u8, 'c: 'static"
2019-09-13T21:35:43.5686457Z Encountered 1 errors
2019-09-13T21:35:43.5686481Z 
2019-09-13T21:35:43.5686675Z ------------------------------------------
2019-09-13T21:35:43.5686720Z 
---
2019-09-13T21:35:43.5689433Z test result: FAILED. 308 passed; 10 failed; 2 ignored; 0 measured; 0 filtered out
2019-09-13T21:35:43.5689470Z 
2019-09-13T21:35:43.5689496Z 
2019-09-13T21:35:43.5689521Z 
2019-09-13T21:35:43.5691554Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-13T21:35:43.5691806Z 
2019-09-13T21:35:43.5691851Z 
2019-09-13T21:35:43.5692090Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-13T21:35:43.5692144Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-13T21:35:43.5692144Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-13T21:35:43.5694797Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-13T21:35:43.5694993Z Build completed unsuccessfully in 1:21:00
2019-09-13T21:35:43.5719989Z == clock drift check ==
2019-09-13T21:35:43.5736698Z   local time: Fri Sep 13 21:35:43 UTC 2019
2019-09-13T21:35:43.8508332Z   network time: Fri, 13 Sep 2019 21:35:43 GMT
2019-09-13T21:35:43.8508582Z == end clock drift check ==
2019-09-13T21:35:45.7927743Z ##[error]Bash exited with code '1'.
2019-09-13T21:35:45.7972273Z ##[section]Starting: Checkout
2019-09-13T21:35:45.7974217Z ==============================================================================
2019-09-13T21:35:45.7974290Z Task         : Get sources
2019-09-13T21:35:45.7974336Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
