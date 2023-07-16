plain
2019-09-23T03:44:47.0871814Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-23T03:44:47.1113251Z ##[command]git config gc.auto 0
2019-09-23T03:44:47.1192621Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-23T03:44:47.1268212Z ##[command]git config --get-all http.proxy
2019-09-23T03:44:47.1423324Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64699/merge:refs/remotes/pull/64699/merge
---
2019-09-23T04:48:08.7933706Z .................................................................................................... 1500/9035
2019-09-23T04:48:15.0482479Z .................................................................................................... 1600/9035
2019-09-23T04:48:27.4628069Z ........................................................................i...............i........... 1700/9035
2019-09-23T04:48:34.4329152Z .................................................................................................... 1800/9035
2019-09-23T04:48:43.4660083Z ...............................................................iiiii................................ 1900/9035
2019-09-23T04:49:03.1617571Z .................................................................................................... 2100/9035
2019-09-23T04:49:05.8267107Z .................................................................................................... 2200/9035
2019-09-23T04:49:09.1266140Z .................................................................................................... 2300/9035
2019-09-23T04:49:17.5694320Z .................................................................................................... 2400/9035
---
2019-09-23T04:52:20.5114348Z ...................................................i...............i................................ 4700/9035
2019-09-23T04:52:30.2231949Z .................................................................................................... 4800/9035
2019-09-23T04:52:38.4617592Z .................................................................................................... 4900/9035
2019-09-23T04:52:46.3783911Z .................................................................................................... 5000/9035
2019-09-23T04:52:56.2772073Z ......................................ii.ii......................................................... 5100/9035
2019-09-23T04:53:06.5837423Z .................................................................................................... 5300/9035
2019-09-23T04:53:17.1651877Z .................................................................................................... 5400/9035
2019-09-23T04:53:25.1219768Z ...i................................................................................................ 5500/9035
2019-09-23T04:53:30.5990440Z .................................................................................................... 5600/9035
2019-09-23T04:53:30.5990440Z .................................................................................................... 5600/9035
2019-09-23T04:53:42.3025499Z ..................................................................................................ii 5700/9035
2019-09-23T04:53:55.9064650Z ...i..ii...........i................................................................................ 5800/9035
2019-09-23T04:54:18.1263577Z .................................................................................................... 6000/9035
2019-09-23T04:54:24.2505254Z .................................................................................................... 6100/9035
2019-09-23T04:54:24.2505254Z .................................................................................................... 6100/9035
2019-09-23T04:54:38.4114936Z i..ii............................................................................................... 6200/9035
2019-09-23T04:54:57.3405444Z ...........................................................i........................................ 6400/9035
2019-09-23T04:54:59.6881816Z .................................................................................................... 6500/9035
2019-09-23T04:55:02.4457536Z ...............................i.................................................................... 6600/9035
2019-09-23T04:55:06.9319756Z .................................................................................................... 6700/9035
---
2019-09-23T04:59:41.8036863Z  finished in 5.527
2019-09-23T04:59:41.8227606Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T04:59:41.9850523Z 
2019-09-23T04:59:41.9853269Z running 150 tests
2019-09-23T04:59:45.4162939Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-23T04:59:47.5360167Z ..iiii..............i.........iii.i.......ii......
2019-09-23T04:59:47.5364278Z 
2019-09-23T04:59:47.5368684Z  finished in 5.714
2019-09-23T04:59:47.5580631Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T04:59:47.7244683Z 
---
2019-09-23T04:59:49.9539498Z  finished in 2.396
2019-09-23T04:59:49.9746801Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T04:59:50.1445476Z 
2019-09-23T04:59:50.1445860Z running 9 tests
2019-09-23T04:59:50.1446988Z iiiiiiiii
2019-09-23T04:59:50.1448350Z 
2019-09-23T04:59:50.1448742Z  finished in 0.169
2019-09-23T04:59:50.1660213Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T04:59:50.3375967Z 
---
2019-09-23T05:00:08.9895802Z  finished in 18.823
2019-09-23T05:00:09.0127373Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T05:00:09.1796604Z 
2019-09-23T05:00:09.1796896Z running 123 tests
2019-09-23T05:00:34.2479153Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-23T05:00:39.1583406Z i.i.i......iii.i.....ii
2019-09-23T05:00:39.1584014Z 
2019-09-23T05:00:39.1584074Z  finished in 30.145
2019-09-23T05:00:39.1591516Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T05:00:39.1591826Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-23T05:04:26.0006709Z     Finished release [optimized] target(s) in 2m 44s
2019-09-23T05:04:26.0291583Z Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T05:04:26.1967769Z 
2019-09-23T05:04:26.1968830Z running 320 tests
2019-09-23T05:05:43.9174592Z FFFFFF.F.FFFFFFFF.FF.FFFFFFF.F.FFFFFFiFF.F.FFF...FFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 100/320
2019-09-23T05:06:53.2001440Z FFFFFFFFFF....FF.F.FFFFFFF.FF.F.FFFFFFFFFFFFF..FF.FFF.FFiFFFF.FFF.FF.FF.FFFFFFF..FFFFFF...F.F.F.FFF. 200/320
2019-09-23T05:08:00.7506865Z FFFFFF.FF...FF.FF.F.F.F.FF.FFFFFF.FFFFF.FF.FFFFFFF.FF.FFFF...FFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF..FF 300/320
2019-09-23T05:08:14.1723481Z .FFFFFFFFFFFFFFFFFFF
2019-09-23T05:08:14.1771014Z 
2019-09-23T05:08:14.1781204Z ---- [rustdoc] rustdoc/all.rs stdout ----
2019-09-23T05:08:14.1781691Z 
2019-09-23T05:08:14.1781691Z 
2019-09-23T05:08:14.1781860Z error: htmldocck failed!
2019-09-23T05:08:14.1782004Z status: exit code: 1
2019-09-23T05:08:14.1782581Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/all" "/checkout/src/test/rustdoc/all.rs"
2019-09-23T05:08:14.1783144Z ------------------------------------------
2019-09-23T05:08:14.1783302Z 
2019-09-23T05:08:14.1783643Z ------------------------------------------
2019-09-23T05:08:14.1784146Z stderr:
2019-09-23T05:08:14.1784146Z stderr:
2019-09-23T05:08:14.1784725Z ------------------------------------------
2019-09-23T05:08:14.1784889Z 3: @has check failed
2019-09-23T05:08:14.1785368Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1785722Z  // @has foo/all.html '//a[@href="struct.Struct.html"]' 'Struct'
2019-09-23T05:08:14.1786038Z 4: @has check failed
2019-09-23T05:08:14.1786432Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1786784Z  // @has foo/all.html '//a[@href="enum.Enum.html"]' 'Enum'
2019-09-23T05:08:14.1787102Z 5: @has check failed
2019-09-23T05:08:14.1787403Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1787736Z  // @has foo/all.html '//a[@href="union.Union.html"]' 'Union'
2019-09-23T05:08:14.1787890Z 6: @has check failed
2019-09-23T05:08:14.1788188Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1788544Z  // @has foo/all.html '//a[@href="constant.CONST.html"]' 'CONST'
2019-09-23T05:08:14.1788702Z 7: @has check failed
2019-09-23T05:08:14.1789288Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1789821Z  // @has foo/all.html '//a[@href="static.STATIC.html"]' 'STATIC'
2019-09-23T05:08:14.1790151Z 8: @has check failed
2019-09-23T05:08:14.1790419Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1790743Z  // @has foo/all.html '//a[@href="fn.function.html"]' 'function'
2019-09-23T05:08:14.1790914Z 26: @has check failed
2019-09-23T05:08:14.1791186Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1792034Z  // @has foo/all.html '//a[@href="struct.ReexportedStruct.html"]' 'ReexportedStruct'
2019-09-23T05:08:14.1792262Z 27: @!has check failed
2019-09-23T05:08:14.1792596Z  File does not exist 'foo/all.html'
2019-09-23T05:08:14.1792958Z  // @!has foo/all.html 'private_module'
2019-09-23T05:08:14.1793274Z Encountered 8 errors
2019-09-23T05:08:14.1793392Z 
2019-09-23T05:08:14.1793744Z ------------------------------------------
2019-09-23T05:08:14.1793899Z 
2019-09-23T05:08:14.1793899Z 
2019-09-23T05:08:14.1794017Z 
2019-09-23T05:08:14.1794388Z ---- [rustdoc] rustdoc/assoc-consts-version.rs stdout ----
2019-09-23T05:08:14.1794562Z 
2019-09-23T05:08:14.1794702Z error: htmldocck failed!
2019-09-23T05:08:14.1795006Z status: exit code: 1
2019-09-23T05:08:14.1795432Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version" "/checkout/src/test/rustdoc/assoc-consts-version.rs"
2019-09-23T05:08:14.1795873Z ------------------------------------------
2019-09-23T05:08:14.1796020Z 
2019-09-23T05:08:14.1796291Z ------------------------------------------
2019-09-23T05:08:14.1796435Z stderr:
2019-09-23T05:08:14.1796435Z stderr:
2019-09-23T05:08:14.1796722Z ------------------------------------------
2019-09-23T05:08:14.1797023Z 11: @has check failed
2019-09-23T05:08:14.1797292Z  File does not exist 'foo/struct.SomeStruct.html'
2019-09-23T05:08:14.1797841Z      // @has 'foo/struct.SomeStruct.html' '//*[@id="associatedconstant.SOME_CONST"]//span[@class="since"]' '1.1.2'
2019-09-23T05:08:14.1798099Z Encountered 1 errors
2019-09-23T05:08:14.1798215Z 
2019-09-23T05:08:14.1798487Z ------------------------------------------
2019-09-23T05:08:14.1798613Z 
2019-09-23T05:08:14.1798613Z 
2019-09-23T05:08:14.1798710Z 
2019-09-23T05:08:14.1799009Z ---- [rustdoc] rustdoc/assoc-item-cast.rs stdout ----
2019-09-23T05:08:14.1799147Z 
2019-09-23T05:08:14.1799261Z error: htmldocck failed!
2019-09-23T05:08:14.1799393Z status: exit code: 1
2019-09-23T05:08:14.1799784Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast" "/checkout/src/test/rustdoc/assoc-item-cast.rs"
2019-09-23T05:08:14.1800221Z ------------------------------------------
2019-09-23T05:08:14.1800349Z 
2019-09-23T05:08:14.1800616Z ------------------------------------------
2019-09-23T05:08:14.1800773Z stderr:
2019-09-23T05:08:14.1800773Z stderr:
2019-09-23T05:08:14.1801044Z ------------------------------------------
2019-09-23T05:08:14.1801550Z 14: @has check failed
2019-09-23T05:08:14.1802461Z  File does not exist 'foo/type.AsExprOf.html'
2019-09-23T05:08:14.1802661Z  // @has foo/type.AsExprOf.html
2019-09-23T05:08:14.1802820Z 15: @has check failed
2019-09-23T05:08:14.1803166Z  File does not exist 'foo/type.AsExprOf.html'
2019-09-23T05:08:14.1803739Z  // @has - '//*[@class="rust typedef"]' 'type AsExprOf<Item, Type> = <Item as AsExpression<Type>>::Expression;'
2019-09-23T05:08:14.1804107Z Encountered 2 errors
2019-09-23T05:08:14.1804229Z 
2019-09-23T05:08:14.1804605Z ------------------------------------------
2019-09-23T05:08:14.1804789Z 
2019-09-23T05:08:14.1804789Z 
2019-09-23T05:08:14.1804911Z 
2019-09-23T05:08:14.1805604Z ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
2019-09-23T05:08:14.1805748Z 
2019-09-23T05:08:14.1805864Z error: htmldocck failed!
2019-09-23T05:08:14.1805975Z status: exit code: 1
2019-09-23T05:08:14.1806380Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
2019-09-23T05:08:14.1806822Z ------------------------------------------
2019-09-23T05:08:14.1839674Z 
2019-09-23T05:08:14.1840114Z ------------------------------------------
2019-09-23T05:08:14.1840160Z stderr:
2019-09-23T05:08:14.1840160Z stderr:
2019-09-23T05:08:14.1840349Z ------------------------------------------
2019-09-23T05:08:14.1840407Z 2: @has check failed
2019-09-23T05:08:14.1840590Z  File does not exist 'assoc_consts/trait.Foo.html'
2019-09-23T05:08:14.1840798Z      // @has assoc_consts/trait.Foo.html '//*[@class="rust trait"]' 'const FOO: usize;'
2019-09-23T05:08:14.1840855Z 4: @has check failed
2019-09-23T05:08:14.1841029Z  File does not exist 'assoc_consts/trait.Foo.html'
2019-09-23T05:08:14.1841221Z      // @has - '//*[@id="associatedconstant.FOO"]' 'const FOO: usize'
2019-09-23T05:08:14.1841719Z 6: @has check failed
2019-09-23T05:08:14.1841985Z  File does not exist 'assoc_consts/trait.Foo.html'
2019-09-23T05:08:14.1842263Z      // @has - '//*[@id="associatedconstant.FOO_NO_DEFAULT"]' 'const FOO_NO_DEFAULT: bool'
2019-09-23T05:08:14.1842332Z 8: @!has check failed
2019-09-23T05:08:14.1842549Z  File does not exist 'assoc_consts/trait.Foo.html'
2019-09-23T05:08:14.1842747Z      // @!has - FOO_HIDDEN
2019-09-23T05:08:14.1842815Z 16: @has check failed
2019-09-23T05:08:14.1843083Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1843351Z      // @has assoc_consts/struct.Bar.html '//code' 'impl Foo for Bar'
2019-09-23T05:08:14.1843403Z 17: @has check failed
2019-09-23T05:08:14.1843658Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1844157Z      // @has - '//*[@id="associatedconstant.FOO"]' 'const FOO: usize'
2019-09-23T05:08:14.1844216Z 19: @has check failed
2019-09-23T05:08:14.1844481Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1844764Z      // @has - '//*[@id="associatedconstant.FOO_NO_DEFAULT"]' 'const FOO_NO_DEFAULT: bool'
2019-09-23T05:08:14.1844995Z 21: @!has check failed
2019-09-23T05:08:14.1845200Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1845365Z      // @!has - FOO_HIDDEN
2019-09-23T05:08:14.1845405Z 27: @has check failed
2019-09-23T05:08:14.1845771Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1846156Z      // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.BAR"]' 'const BAR: usize'
2019-09-23T05:08:14.1846199Z 35: @has check failed
2019-09-23T05:08:14.1846386Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1846616Z      // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.BAZ"]' "const BAZ: Baz<'static, u8, u32>"
2019-09-23T05:08:14.1846658Z 43: @has check failed
2019-09-23T05:08:14.1846844Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1847073Z      // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.F"]' "const F: fn(_: &(dyn ToString + 'static))"
2019-09-23T05:08:14.1847116Z 49: @!has check failed
2019-09-23T05:08:14.1847504Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1847705Z      // @!has assoc_consts/struct.Bar.html 'BAR_PRIVATE'
2019-09-23T05:08:14.1847744Z 51: @!has check failed
2019-09-23T05:08:14.1847914Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1848186Z      // @!has assoc_consts/struct.Bar.html 'BAR_HIDDEN'
2019-09-23T05:08:14.1848232Z 56: @has check failed
2019-09-23T05:08:14.1848433Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1848486Z  // @has assoc_consts/trait.Qux.html
2019-09-23T05:08:14.1848521Z 58: @has check failed
2019-09-23T05:08:14.1848695Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1848898Z      // @has - '//*[@id="associatedconstant.QUX0"]' 'const QUX0: u8'
2019-09-23T05:08:14.1848938Z 59: @has check failed
2019-09-23T05:08:14.1849109Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1849304Z      // @has - '//*[@class="docblock"]' "Docs for QUX0 in trait."
2019-09-23T05:08:14.1849353Z 62: @has check failed
2019-09-23T05:08:14.1849527Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1849715Z      // @has - '//*[@id="associatedconstant.QUX1"]' 'const QUX1: i8'
2019-09-23T05:08:14.1849765Z 63: @has check failed
2019-09-23T05:08:14.1849944Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1850133Z      // @has - '//*[@class="docblock"]' "Docs for QUX1 in trait."
2019-09-23T05:08:14.1850183Z 66: @has check failed
2019-09-23T05:08:14.1850354Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1850557Z      // @has - '//*[@id="associatedconstant.QUX_DEFAULT0"]' 'const QUX_DEFAULT0: u16'
2019-09-23T05:08:14.1850603Z 67: @has check failed
2019-09-23T05:08:14.1850773Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1850967Z      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT12 in trait."
2019-09-23T05:08:14.1851016Z 70: @has check failed
2019-09-23T05:08:14.1851621Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1851881Z      // @has - '//*[@id="associatedconstant.QUX_DEFAULT1"]' 'const QUX_DEFAULT1: i16'
2019-09-23T05:08:14.1851940Z 71: @has check failed
2019-09-23T05:08:14.1852156Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1852408Z      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT1 in trait."
2019-09-23T05:08:14.1852465Z 74: @has check failed
2019-09-23T05:08:14.1852686Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1852941Z      // @has - '//*[@id="associatedconstant.QUX_DEFAULT2"]' 'const QUX_DEFAULT2: u32'
2019-09-23T05:08:14.1852989Z 75: @has check failed
2019-09-23T05:08:14.1853216Z  File does not exist 'assoc_consts/trait.Qux.html'
2019-09-23T05:08:14.1853458Z      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT2 in trait."
2019-09-23T05:08:14.1853506Z 80: @has check failed
2019-09-23T05:08:14.1853735Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1853981Z  // @has assoc_consts/struct.Bar.html '//code' 'impl Qux for Bar'
2019-09-23T05:08:14.1854030Z 82: @has check failed
2019-09-23T05:08:14.1854257Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1854497Z      // @has - '//*[@id="associatedconstant.QUX0"]' 'const QUX0: u8'
2019-09-23T05:08:14.1854553Z 83: @has check failed
2019-09-23T05:08:14.1854923Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1855110Z      // @has - '//*[@class="docblock"]' "Docs for QUX0 in trait."
2019-09-23T05:08:14.1855148Z 86: @has check failed
2019-09-23T05:08:14.1855325Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1855516Z      // @has - '//*[@id="associatedconstant.QUX1"]' 'const QUX1: i8'
2019-09-23T05:08:14.1855556Z 87: @has check failed
2019-09-23T05:08:14.1855725Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1855918Z      // @has - '//*[@class="docblock"]' "Docs for QUX1 in impl."
2019-09-23T05:08:14.1856068Z 90: @has check failed
2019-09-23T05:08:14.1856271Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1856487Z      // @has - '//*[@id="associatedconstant.QUX_DEFAULT0"]' 'const QUX_DEFAULT0: u16'
2019-09-23T05:08:14.1856526Z 91: @has check failed
2019-09-23T05:08:14.1856770Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1857022Z      // @has - '//*[@class="docblock hidden"]' "Docs for QUX_DEFAULT12 in trait."
2019-09-23T05:08:14.1857064Z 93: @has check failed
2019-09-23T05:08:14.1857235Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1857452Z      // @has - '//*[@id="associatedconstant.QUX_DEFAULT1"]' 'const QUX_DEFAULT1: i16'
2019-09-23T05:08:14.1857492Z 94: @has check failed
2019-09-23T05:08:14.1857663Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1857865Z      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT1 in impl."
2019-09-23T05:08:14.1857904Z 97: @has check failed
2019-09-23T05:08:14.1859279Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1859538Z      // @has - '//*[@id="associatedconstant.QUX_DEFAULT2"]' 'const QUX_DEFAULT2: u32'
2019-09-23T05:08:14.1859584Z 98: @has check failed
2019-09-23T05:08:14.1859770Z  File does not exist 'assoc_consts/struct.Bar.html'
2019-09-23T05:08:14.1859993Z      // @has - '//*[@class="docblock hidden"]' "Docs for QUX_DEFAULT2 in trait."
2019-09-23T05:08:14.1860080Z Encountered 35 errors
2019-09-23T05:08:14.1860103Z 
2019-09-23T05:08:14.1860282Z ------------------------------------------
2019-09-23T05:08:14.1860321Z 
2019-09-23T05:08:14.1860321Z 
2019-09-23T05:08:14.1860342Z 
2019-09-23T05:08:14.1860521Z ---- [rustdoc] rustdoc/async-fn.rs stdout ----
2019-09-23T05:08:14.1860548Z 
2019-09-23T05:08:14.1860589Z error: htmldocck failed!
2019-09-23T05:08:14.1860625Z status: exit code: 1
2019-09-23T05:08:14.1860912Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
2019-09-23T05:08:14.1861156Z ------------------------------------------
2019-09-23T05:08:14.1861183Z 
2019-09-23T05:08:14.1862498Z ------------------------------------------
2019-09-23T05:08:14.1862572Z stderr:
2019-09-23T05:08:14.1862572Z stderr:
2019-09-23T05:08:14.1862798Z ------------------------------------------
2019-09-23T05:08:14.1862845Z 3: @has check failed
2019-09-23T05:08:14.1863064Z  File does not exist 'async_fn/fn.foo.html'
2019-09-23T05:08:14.1863328Z  // @has async_fn/fn.foo.html '//pre[@class="rust fn"]' 'pub async fn foo() -> Option<Foo>'
2019-09-23T05:08:14.1863381Z 8: @has check failed
2019-09-23T05:08:14.1863594Z  File does not exist 'async_fn/fn.bar.html'
2019-09-23T05:08:14.1863867Z  // @has async_fn/fn.bar.html '//pre[@class="rust fn"]' 'pub async fn bar(a: i32, b: i32) -> i32'
2019-09-23T05:08:14.1863920Z 13: @has check failed
2019-09-23T05:08:14.1864128Z  File does not exist 'async_fn/fn.baz.html'
2019-09-23T05:08:14.1864395Z  // @has async_fn/fn.baz.html '//pre[@class="rust fn"]' 'pub async fn baz<T>(a: T) -> T'
2019-09-23T05:08:14.1864456Z 22: @has check failed
2019-09-23T05:08:14.1864670Z  File does not exist 'async_fn/fn.quux.html'
2019-09-23T05:08:14.1865236Z  // @has async_fn/fn.quux.html '//pre[@class="rust fn"]' 'pub async fn quux() -> impl Bar'
2019-09-23T05:08:14.1865286Z 27: @has check failed
2019-09-23T05:08:14.1865461Z  File does not exist 'async_fn/struct.Foo.html'
2019-09-23T05:08:14.1865509Z  // @has async_fn/struct.Foo.html
2019-09-23T05:08:14.1865546Z 28: @matches check failed
2019-09-23T05:08:14.1865720Z  File does not exist 'async_fn/struct.Foo.html'
2019-09-23T05:08:14.1865901Z  // @matches - '//code' 'pub async fn f\(\)$'
2019-09-23T05:08:14.1865961Z Encountered 6 errors
2019-09-23T05:08:14.1865983Z 
2019-09-23T05:08:14.1866158Z ------------------------------------------
2019-09-23T05:08:14.1866183Z 
2019-09-23T05:08:14.1866183Z 
2019-09-23T05:08:14.1866204Z 
2019-09-23T05:08:14.1866377Z ---- [rustdoc] rustdoc/assoc-types.rs stdout ----
2019-09-23T05:08:14.1866745Z 
2019-09-23T05:08:14.1866793Z error: htmldocck failed!
2019-09-23T05:08:14.1866828Z status: exit code: 1
2019-09-23T05:08:14.1867408Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
2019-09-23T05:08:14.1867688Z ------------------------------------------
2019-09-23T05:08:14.1867717Z 
2019-09-23T05:08:14.1867895Z ------------------------------------------
2019-09-23T05:08:14.1867949Z stderr:
2019-09-23T05:08:14.1867949Z stderr:
2019-09-23T05:08:14.1868290Z ------------------------------------------
2019-09-23T05:08:14.1868330Z 5: @has check failed
2019-09-23T05:08:14.1868711Z  File does not exist 'assoc_types/trait.Index.html'
2019-09-23T05:08:14.1868790Z  // @has assoc_types/trait.Index.html
2019-09-23T05:08:14.1868828Z 7: @has check failed
2019-09-23T05:08:14.1869588Z  File does not exist 'assoc_types/trait.Index.html'
2019-09-23T05:08:14.1869838Z      // @has - '//*[@id="associatedtype.Output"]//code' 'type Output: ?Sized'
2019-09-23T05:08:14.1869884Z 8: @has check failed
2019-09-23T05:08:14.1872255Z  File does not exist 'assoc_types/trait.Index.html'
2019-09-23T05:08:14.1872535Z      // @has - '//code[@id="Output.t"]' 'type Output: ?Sized'
2019-09-23T05:08:14.1872604Z 10: @has check failed
2019-09-23T05:08:14.1872834Z  File does not exist 'assoc_types/trait.Index.html'
2019-09-23T05:08:14.1873052Z      // @has - '//code[@id="index.v"]' 'fn index'
2019-09-23T05:08:14.1873100Z 11: @has check failed
2019-09-23T05:08:14.1873314Z  File does not exist 'assoc_types/trait.Index.html'
2019-09-23T05:08:14.1873603Z      // @has - '//*[@id="tymethod.index"]//code' "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
2019-09-23T05:08:14.1873655Z 13: @has check failed
2019-09-23T05:08:14.1873870Z  File does not exist 'assoc_types/trait.Index.html'
2019-09-23T05:08:14.1874174Z      // @has - '//*[@id="tymethod.index"]//code//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' "Output"
2019-09-23T05:08:14.1874238Z 18: @has check failed
2019-09-23T05:08:14.1874457Z  File does not exist 'assoc_types/fn.use_output.html'
2019-09-23T05:08:14.1874516Z  // @has assoc_types/fn.use_output.html
2019-09-23T05:08:14.1874559Z 19: @has check failed
2019-09-23T05:08:14.1875119Z  File does not exist 'assoc_types/fn.use_output.html'
2019-09-23T05:08:14.1875568Z  // @has - '//*[@class="rust fn"]' '-> &T::Output'
2019-09-23T05:08:14.1875622Z 20: @has check failed
2019-09-23T05:08:14.1875802Z  File does not exist 'assoc_types/fn.use_output.html'
2019-09-23T05:08:14.1876035Z  // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' 'Output'
2019-09-23T05:08:14.1876078Z 29: @has check failed
2019-09-23T05:08:14.1876250Z  File does not exist 'assoc_types/fn.use_input.html'
2019-09-23T05:08:14.1901161Z  // @has assoc_types/fn.use_input.html
2019-09-23T05:08:14.1901263Z 30: @has check failed
2019-09-23T05:08:14.1908318Z  File does not exist 'assoc_types/fn.use_input.html'
2019-09-23T05:08:14.1908865Z  // @has - '//*[@class="rust fn"]' 'T::Input'
2019-09-23T05:08:14.1939936Z 31: @has check failed
2019-09-23T05:08:14.1940318Z  File does not exist 'assoc_types/fn.use_input.html'
2019-09-23T05:08:14.1940568Z  // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Feed.html#associatedtype.Input"]' 'Input'
2019-09-23T05:08:14.1940638Z 34: @has check failed
2019-09-23T05:08:14.1940822Z  File does not exist 'assoc_types/fn.cmp_input.html'
2019-09-23T05:08:14.1940864Z  // @has assoc_types/fn.cmp_input.html
2019-09-23T05:08:14.1940915Z 35: @has check failed
2019-09-23T05:08:14.1941090Z  File does not exist 'assoc_types/fn.cmp_input.html'
2019-09-23T05:08:14.1941281Z  // @has - '//*[@class="rust fn"]' 'where T::Input: PartialEq<U::Input>'
2019-09-23T05:08:14.1941331Z 36: @has check failed
2019-09-23T05:08:14.1942001Z  File does not exist 'assoc_types/fn.cmp_input.html'
2019-09-23T05:08:14.1942282Z  // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Feed.html#associatedtype.Input"]' 'Input'
2019-09-23T05:08:14.1942636Z Encountered 15 errors
2019-09-23T05:08:14.1942663Z 
2019-09-23T05:08:14.1942932Z ------------------------------------------
2019-09-23T05:08:14.1942965Z 
2019-09-23T05:08:14.1942965Z 
2019-09-23T05:08:14.1943004Z 
2019-09-23T05:08:14.1943324Z ---- [rustdoc] rustdoc/attributes.rs stdout ----
2019-09-23T05:08:14.1943365Z 
2019-09-23T05:08:14.1943408Z error: htmldocck failed!
2019-09-23T05:08:14.1943463Z status: exit code: 1
2019-09-23T05:08:14.1943847Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/attributes" "/checkout/src/test/rustdoc/attributes.rs"
2019-09-23T05:08:14.1944137Z ------------------------------------------
2019-09-23T05:08:14.1944168Z 
2019-09-23T05:08:14.1944372Z ------------------------------------------
2019-09-23T05:08:14.1944417Z stderr:
2019-09-23T05:08:14.1944417Z stderr:
2019-09-23T05:08:14.1944641Z ------------------------------------------
2019-09-23T05:08:14.1944699Z 3: @has check failed
2019-09-23T05:08:14.1944907Z  File does not exist 'foo/fn.f.html'
2019-09-23T05:08:14.1945457Z  // @has foo/fn.f.html '//*[@class="docblock attributes"]' '#[no_mangle]'
2019-09-23T05:08:14.1945496Z 7: @has check failed
2019-09-23T05:08:14.1945663Z  File does not exist 'foo/fn.g.html'
2019-09-23T05:08:14.1945873Z  // @has foo/fn.g.html '//*[@class="docblock attributes"]' '#[export_name = "bar"]'
2019-09-23T05:08:14.1945914Z 11: @has check failed
2019-09-23T05:08:14.1946079Z  File does not exist 'foo/enum.Foo.html'
2019-09-23T05:08:14.1946290Z  // @has foo/enum.Foo.html '//*[@class="docblock attributes top-attr"]' '#[repr(i64)]'
2019-09-23T05:08:14.1946330Z 12: @has check failed
2019-09-23T05:08:14.1946496Z  File does not exist 'foo/enum.Foo.html'
2019-09-23T05:08:14.1946704Z  // @has foo/enum.Foo.html '//*[@class="docblock attributes top-attr"]' '#[must_use]'
2019-09-23T05:08:14.1946765Z Encountered 4 errors
2019-09-23T05:08:14.1946794Z 
2019-09-23T05:08:14.1946963Z ------------------------------------------
2019-09-23T05:08:14.1946999Z 
2019-09-23T05:08:14.1946999Z 
2019-09-23T05:08:14.1947019Z 
2019-09-23T05:08:14.1947195Z ---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----
2019-09-23T05:08:14.1947222Z 
2019-09-23T05:08:14.1947264Z error: htmldocck failed!
2019-09-23T05:08:14.1947305Z status: exit code: 1
2019-09-23T05:08:14.1947594Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"
2019-09-23T05:08:14.1947853Z ------------------------------------------
2019-09-23T05:08:14.1947880Z 
2019-09-23T05:08:14.1948062Z ------------------------------------------
2019-09-23T05:08:14.1948113Z stderr:
2019-09-23T05:08:14.1948113Z stderr:
2019-09-23T05:08:14.1948294Z ------------------------------------------
2019-09-23T05:08:14.1948333Z 4: @has check failed
2019-09-23T05:08:14.1948527Z  File does not exist 'foo/primitive.i16.html'
2019-09-23T05:08:14.1948771Z  // @has 'foo/primitive.i16.html' '//h2[@id="synthetic-implementations"]' 'Auto Trait Implementation'
2019-09-23T05:08:14.1948837Z Encountered 1 errors
2019-09-23T05:08:14.1948875Z 
2019-09-23T05:08:14.1949597Z ------------------------------------------
2019-09-23T05:08:14.1949641Z 
2019-09-23T05:08:14.1949641Z 
2019-09-23T05:08:14.1949663Z 
2019-09-23T05:08:14.1949867Z ---- [rustdoc] rustdoc/auto-traits.rs stdout ----
2019-09-23T05:08:14.1949895Z 
2019-09-23T05:08:14.1949930Z error: htmldocck failed!
2019-09-23T05:08:14.1949965Z status: exit code: 1
2019-09-23T05:08:14.1950270Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-traits" "/checkout/src/test/rustdoc/auto-traits.rs"
2019-09-23T05:08:14.1950495Z ------------------------------------------
2019-09-23T05:08:14.1950531Z 
2019-09-23T05:08:14.1950710Z ------------------------------------------
2019-09-23T05:08:14.1950880Z stderr:
2019-09-23T05:08:14.1950880Z stderr:
2019-09-23T05:08:14.1951467Z ------------------------------------------
2019-09-23T05:08:14.1951539Z 9: @has check failed
2019-09-23T05:08:14.1951788Z  File does not exist 'foo/trait.Foo.html'
2019-09-23T05:08:14.1952023Z  // @has 'foo/trait.Foo.html' '//pre' 'pub unsafe auto trait Foo'
2019-09-23T05:08:14.1952206Z 12: @has check failed
2019-09-23T05:08:14.1952464Z  File does not exist 'foo/trait.Bar.html'
2019-09-23T05:08:14.1952698Z  // @has 'foo/trait.Bar.html' '//pre' 'pub unsafe auto trait Bar'
2019-09-23T05:08:14.1952789Z Encountered 2 errors
2019-09-23T05:08:14.1952816Z 
2019-09-23T05:08:14.1953026Z ------------------------------------------
2019-09-23T05:08:14.1953057Z 
2019-09-23T05:08:14.1953057Z 
2019-09-23T05:08:14.1953096Z 
2019-09-23T05:08:14.1953319Z ---- [rustdoc] rustdoc/bad-codeblock-syntax.rs stdout ----
2019-09-23T05:08:14.1953352Z 
2019-09-23T05:08:14.1953393Z error: htmldocck failed!
2019-09-23T05:08:14.1953447Z status: exit code: 1
2019-09-23T05:08:14.1953827Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/bad-codeblock-syntax" "/checkout/src/test/rustdoc/bad-codeblock-syntax.rs"
2019-09-23T05:08:14.1954107Z ------------------------------------------
2019-09-23T05:08:14.1954146Z 
2019-09-23T05:08:14.1954353Z ------------------------------------------
2019-09-23T05:08:14.1954397Z stderr:
2019-09-23T05:08:14.1954397Z stderr:
2019-09-23T05:08:14.1954848Z ------------------------------------------
2019-09-23T05:08:14.1955051Z 1: @has check failed
2019-09-23T05:08:14.1955226Z  File does not exist 'bad_codeblock_syntax/fn.foo.html'
2019-09-23T05:08:14.1955279Z  // @has bad_codeblock_syntax/fn.foo.html
2019-09-23T05:08:14.1955312Z 2: @has check failed
2019-09-23T05:08:14.1955487Z  File does not exist 'bad_codeblock_syntax/fn.foo.html'
2019-09-23T05:08:14.1955673Z  // @has - '//*[@class="docblock"]/pre/code' '\_'
2019-09-23T05:08:14.1955712Z 8: @has check failed
2019-09-23T05:08:14.1955917Z  File does not exist 'bad_codeblock_syntax/fn.bar.html'
2019-09-23T05:08:14.1955973Z  // @has bad_codeblock_syntax/fn.bar.html
2019-09-23T05:08:14.1956009Z 9: @has check failed
2019-09-23T05:08:14.1956200Z  File does not exist 'bad_codeblock_syntax/fn.bar.html'
2019-09-23T05:08:14.1956404Z  // @has - '//*[@class="docblock"]/pre/code' '`baz::foobar`'
2019-09-23T05:08:14.1956460Z 15: @has check failed
2019-09-23T05:08:14.1956653Z  File does not exist 'bad_codeblock_syntax/fn.quux.html'
2019-09-23T05:08:14.1956695Z  // @has bad_codeblock_syntax/fn.quux.html
2019-09-23T05:08:14.1956745Z 16: @has check failed
2019-09-23T05:08:14.1956935Z  File does not exist 'bad_codeblock_syntax/fn.quux.html'
2019-09-23T05:08:14.1957123Z  // @has - '//*[@class="docblock"]/pre/code' '\_'
2019-09-23T05:08:14.1957178Z 22: @has check failed
2019-09-23T05:08:14.1957367Z  File does not exist 'bad_codeblock_syntax/fn.ok.html'
2019-09-23T05:08:14.1957409Z  // @has bad_codeblock_syntax/fn.ok.html
2019-09-23T05:08:14.1957463Z 23: @has check failed
2019-09-23T05:08:14.1957653Z  File does not exist 'bad_codeblock_syntax/fn.ok.html'
2019-09-23T05:08:14.1957862Z  // @has - '//*[@class="docblock"]/pre/code[@class="language-text"]' '\_'
2019-09-23T05:08:14.1957934Z Encountered 8 errors
2019-09-23T05:08:14.1957957Z 
2019-09-23T05:08:14.1958144Z ------------------------------------------
2019-09-23T05:08:14.1958170Z 
2019-09-23T05:08:14.1958170Z 
2019-09-23T05:08:14.1958203Z 
2019-09-23T05:08:14.1958396Z ---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----
2019-09-23T05:08:14.1958423Z 
2019-09-23T05:08:14.1958457Z error: htmldocck failed!
2019-09-23T05:08:14.1958502Z status: exit code: 1
2019-09-23T05:08:14.1958805Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
2019-09-23T05:08:14.1959040Z ------------------------------------------
2019-09-23T05:08:14.1959155Z 
2019-09-23T05:08:14.1959365Z ------------------------------------------
2019-09-23T05:08:14.1959405Z stderr:
2019-09-23T05:08:14.1959405Z stderr:
2019-09-23T05:08:14.1959596Z ------------------------------------------
2019-09-23T05:08:14.1959634Z 3: @has check failed
2019-09-23T05:08:14.1959813Z  File does not exist 'foo/struct.S.html'
2019-09-23T05:08:14.1960119Z  // @has foo/struct.S.html '//h3[@id="impl-Into%3CU%3E"]//code' 'impl<T, U> Into<U> for T'
2019-09-23T05:08:14.1960191Z Encountered 1 errors
2019-09-23T05:08:14.1960214Z 
2019-09-23T05:08:14.1960436Z ------------------------------------------
2019-09-23T05:08:14.1960463Z 
2019-09-23T05:08:14.1960463Z 
2019-09-23T05:08:14.1960485Z 
2019-09-23T05:08:14.1960668Z ---- [rustdoc] rustdoc/cap-lints.rs stdout ----
2019-09-23T05:08:14.1960695Z 
2019-09-23T05:08:14.1960748Z error: htmldocck failed!
2019-09-23T05:08:14.1960783Z status: exit code: 1
2019-09-23T05:08:14.1961434Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cap-lints" "/checkout/src/test/rustdoc/cap-lints.rs"
2019-09-23T05:08:14.1961777Z ------------------------------------------
2019-09-23T05:08:14.1961808Z 
2019-09-23T05:08:14.1962028Z ------------------------------------------
2019-09-23T05:08:14.1962073Z stderr:
2019-09-23T05:08:14.1962073Z stderr:
2019-09-23T05:08:14.1962291Z ------------------------------------------
2019-09-23T05:08:14.1962339Z 6: @has check failed
2019-09-23T05:08:14.1962574Z  File does not exist 'cap_lints/struct.Foo.html'
2019-09-23T05:08:14.1962798Z  // @has cap_lints/struct.Foo.html //pre '#[must_use]'
2019-09-23T05:08:14.1962882Z Encountered 1 errors
2019-09-23T05:08:14.1962910Z 
2019-09-23T05:08:14.1963116Z ------------------------------------------
2019-09-23T05:08:14.1963147Z 
2019-09-23T05:08:14.1963147Z 
2019-09-23T05:08:14.1963172Z 
2019-09-23T05:08:14.1963400Z ---- [rustdoc] rustdoc/cfg-doctest.rs stdout ----
2019-09-23T05:08:14.1963431Z 
2019-09-23T05:08:14.1963473Z error: htmldocck failed!
2019-09-23T05:08:14.1963523Z status: exit code: 1
2019-09-23T05:08:14.1963883Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cfg-doctest" "/checkout/src/test/rustdoc/cfg-doctest.rs"
2019-09-23T05:08:14.1964155Z ------------------------------------------
2019-09-23T05:08:14.1964196Z 
2019-09-23T05:08:14.1964406Z ------------------------------------------
2019-09-23T05:08:14.1964450Z stderr:
2019-09-23T05:08:14.1964450Z stderr:
2019-09-23T05:08:14.1964835Z ------------------------------------------
2019-09-23T05:08:14.1964877Z 4: @!has check failed
2019-09-23T05:08:14.1965214Z  File does not exist 'cfg_doctest/index.html'
2019-09-23T05:08:14.1965411Z  // @!has cfg_doctest/index.html '//a/@href' 'struct.SomeStruct.html'
2019-09-23T05:08:14.1965483Z Encountered 1 errors
2019-09-23T05:08:14.1965505Z 
2019-09-23T05:08:14.1965675Z ------------------------------------------
2019-09-23T05:08:14.1965709Z 
2019-09-23T05:08:14.1965709Z 
2019-09-23T05:08:14.1965737Z 
2019-09-23T05:08:14.1965919Z ---- [rustdoc] rustdoc/check-styled-link.rs stdout ----
2019-09-23T05:08:14.1965945Z 
2019-09-23T05:08:14.1965993Z error: htmldocck failed!
2019-09-23T05:08:14.1966027Z status: exit code: 1
2019-09-23T05:08:14.1966326Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-styled-link" "/checkout/src/test/rustdoc/check-styled-link.rs"
2019-09-23T05:08:14.1966717Z ------------------------------------------
2019-09-23T05:08:14.1966741Z 
2019-09-23T05:08:14.1966901Z ------------------------------------------
2019-09-23T05:08:14.1966950Z stderr:
2019-09-23T05:08:14.1966950Z stderr:
2019-09-23T05:08:14.1967113Z ------------------------------------------
2019-09-23T05:08:14.1967150Z 5: @has check failed
2019-09-23T05:08:14.1967328Z  File does not exist 'foo/struct.Bar.html'
2019-09-23T05:08:14.1967520Z  // @has foo/struct.Bar.html '//a[@href="../foo/struct.Foo.html"]' 'Foo'
---
2019-09-23T05:08:14.2456571Z stderr:
2019-09-23T05:08:14.2456731Z ------------------------------------------
2019-09-23T05:08:14.2456920Z error: couldn't generate documentation: No such file or directory (os error 2)
2019-09-23T05:08:14.2456958Z   |
2019-09-23T05:08:14.2457236Z   = note: failed to create or modify "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-libstd-re-export.0/intra_link_libstd_re_export/panic/index.html"
2019-09-23T05:08:14.2457291Z 
2019-09-23T05:08:14.2457467Z ------------------------------------------
2019-09-23T05:08:14.2457491Z 
2019-09-23T05:08:14.2457511Z 
2019-09-23T05:08:14.2457511Z 
2019-09-23T05:08:14.2457675Z ---- [rustdoc] rustdoc/intra-links.rs stdout ----
2019-09-23T05:08:14.2457707Z 
2019-09-23T05:08:14.2457739Z error: htmldocck failed!
2019-09-23T05:08:14.2457771Z status: exit code: 1
2019-09-23T05:08:14.2458036Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-links" "/checkout/src/test/rustdoc/intra-links.rs"
2019-09-23T05:08:14.2458395Z ------------------------------------------
2019-09-23T05:08:14.2458421Z 
2019-09-23T05:08:14.2458586Z ------------------------------------------
2019-09-23T05:08:14.2458621Z stderr:
2019-09-23T05:08:14.2458621Z stderr:
2019-09-23T05:08:14.2458855Z ------------------------------------------
2019-09-23T05:08:14.2458898Z 1: @has check failed
2019-09-23T05:08:14.2459100Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2459139Z  // @has intra_links/index.html
2019-09-23T05:08:14.2459171Z 2: @has check failed
2019-09-23T05:08:14.2459342Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2459517Z  // @has - '//a/@href' '../intra_links/struct.ThisType.html'
2019-09-23T05:08:14.2459554Z 3: @has check failed
2019-09-23T05:08:14.2459722Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2459914Z  // @has - '//a/@href' '../intra_links/struct.ThisType.html#method.this_method'
2019-09-23T05:08:14.2459960Z 4: @has check failed
2019-09-23T05:08:14.2460127Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2460302Z  // @has - '//a/@href' '../intra_links/enum.ThisEnum.html'
2019-09-23T05:08:14.2460339Z 5: @has check failed
2019-09-23T05:08:14.2460513Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2460707Z  // @has - '//a/@href' '../intra_links/enum.ThisEnum.html#ThisVariant.v'
2019-09-23T05:08:14.2460745Z 6: @has check failed
2019-09-23T05:08:14.2461295Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2461546Z  // @has - '//a/@href' '../intra_links/trait.ThisTrait.html'
2019-09-23T05:08:14.2461594Z 7: @has check failed
2019-09-23T05:08:14.2461805Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2462074Z  // @has - '//a/@href' '../intra_links/trait.ThisTrait.html#tymethod.this_associated_method'
2019-09-23T05:08:14.2462126Z 8: @has check failed
2019-09-23T05:08:14.2462335Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2462617Z  // @has - '//a/@href' '../intra_links/trait.ThisTrait.html#associatedtype.ThisAssociatedType'
2019-09-23T05:08:14.2462669Z 9: @has check failed
2019-09-23T05:08:14.2462877Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2463161Z  // @has - '//a/@href' '../intra_links/trait.ThisTrait.html#associatedconstant.THIS_ASSOCIATED_CONST'
2019-09-23T05:08:14.2463214Z 10: @has check failed
2019-09-23T05:08:14.2463426Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2463660Z  // @has - '//a/@href' '../intra_links/trait.ThisTrait.html'
2019-09-23T05:08:14.2463708Z 11: @has check failed
2019-09-23T05:08:14.2463918Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2464153Z  // @has - '//a/@href' '../intra_links/type.ThisAlias.html'
2019-09-23T05:08:14.2464202Z 12: @has check failed
2019-09-23T05:08:14.2464584Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2464765Z  // @has - '//a/@href' '../intra_links/union.ThisUnion.html'
2019-09-23T05:08:14.2464816Z 13: @has check failed
2019-09-23T05:08:14.2464986Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2465165Z  // @has - '//a/@href' '../intra_links/fn.this_function.html'
2019-09-23T05:08:14.2465209Z 14: @has check failed
2019-09-23T05:08:14.2465386Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2465572Z  // @has - '//a/@href' '../intra_links/constant.THIS_CONST.html'
2019-09-23T05:08:14.2465617Z 15: @has check failed
2019-09-23T05:08:14.2465785Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2465967Z  // @has - '//a/@href' '../intra_links/static.THIS_STATIC.html'
2019-09-23T05:08:14.2466010Z 16: @has check failed
2019-09-23T05:08:14.2466179Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2466359Z  // @has - '//a/@href' '../intra_links/macro.this_macro.html'
2019-09-23T05:08:14.2466396Z 17: @has check failed
2019-09-23T05:08:14.2466570Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2466892Z  // @has - '//a/@href' '../intra_links/trait.SoAmbiguous.html'
2019-09-23T05:08:14.2466932Z 18: @has check failed
2019-09-23T05:08:14.2467111Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2467291Z  // @has - '//a/@href' '../intra_links/fn.SoAmbiguous.html'
2019-09-23T05:08:14.2467413Z 73: @has check failed
2019-09-23T05:08:14.2467626Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2467808Z  // @has - '//a/@href' '../intra_links/struct.ThisType.html'
2019-09-23T05:08:14.2467846Z 74: @has check failed
2019-09-23T05:08:14.2468019Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2468216Z  // @has - '//a/@href' '../intra_links/struct.ThisType.html#method.this_method'
2019-09-23T05:08:14.2468255Z 75: @has check failed
2019-09-23T05:08:14.2468426Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2468610Z  // @has - '//a/@href' '../intra_links/enum.ThisEnum.html'
2019-09-23T05:08:14.2468647Z 76: @has check failed
2019-09-23T05:08:14.2468824Z  File does not exist 'intra_links/index.html'
2019-09-23T05:08:14.2469188Z  // @has - '//a/@href' '../intra_links/enum.ThisEnum.html#ThisVariant.v'
2019-09-23T05:08:14.2469251Z Encountered 22 errors
2019-09-23T05:08:14.2469272Z 
2019-09-23T05:08:14.2469460Z ------------------------------------------
2019-09-23T05:08:14.2469486Z 
2019-09-23T05:08:14.2469486Z 
2019-09-23T05:08:14.2469505Z 
2019-09-23T05:08:14.2469679Z ---- [rustdoc] rustdoc/issue-12834.rs stdout ----
2019-09-23T05:08:14.2469711Z 
2019-09-23T05:08:14.2469743Z error: htmldocck failed!
2019-09-23T05:08:14.2469776Z status: exit code: 1
2019-09-23T05:08:14.2470058Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-12834" "/checkout/src/test/rustdoc/issue-12834.rs"
2019-09-23T05:08:14.2470271Z ------------------------------------------
2019-09-23T05:08:14.2470295Z 
2019-09-23T05:08:14.2470465Z ------------------------------------------
2019-09-23T05:08:14.2470508Z stderr:
2019-09-23T05:08:14.2470508Z stderr:
2019-09-23T05:08:14.2470832Z ------------------------------------------
2019-09-23T05:08:14.2470874Z 5: @has check failed
2019-09-23T05:08:14.2471041Z  File does not exist 'issue_12834/fn.foo.html'
2019-09-23T05:08:14.2471078Z  // @has issue_12834/fn.foo.html
2019-09-23T05:08:14.2471119Z 6: @has check failed
2019-09-23T05:08:14.2471699Z  File does not exist 'issue_12834/fn.foo.html'
2019-09-23T05:08:14.2471939Z  // @has - //pre 'a + b '
2019-09-23T05:08:14.2472020Z Encountered 2 errors
2019-09-23T05:08:14.2472047Z 
2019-09-23T05:08:14.2472257Z ------------------------------------------
2019-09-23T05:08:14.2472288Z 
2019-09-23T05:08:14.2472288Z 
2019-09-23T05:08:14.2472312Z 
2019-09-23T05:08:14.2472536Z ---- [rustdoc] rustdoc/issue-13698.rs stdout ----
2019-09-23T05:08:14.2472568Z 
2019-09-23T05:08:14.2472609Z error: htmldocck failed!
2019-09-23T05:08:14.2472652Z status: exit code: 1
2019-09-23T05:08:14.2473006Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-13698" "/checkout/src/test/rustdoc/issue-13698.rs"
2019-09-23T05:08:14.2473285Z ------------------------------------------
2019-09-23T05:08:14.2473324Z 
2019-09-23T05:08:14.2473542Z ------------------------------------------
2019-09-23T05:08:14.2473587Z stderr:
2019-09-23T05:08:14.2473587Z stderr:
2019-09-23T05:08:14.2473800Z ------------------------------------------
2019-09-23T05:08:14.2473847Z 7: @!has check failed
2019-09-23T05:08:14.2474068Z  File does not exist 'issue_13698/struct.Foo.html'
2019-09-23T05:08:14.2474310Z  // @!has issue_13698/struct.Foo.html '//*[@id="method.foo"]' 'fn foo'
2019-09-23T05:08:14.2474370Z 15: @!has check failed
2019-09-23T05:08:14.2474594Z  File does not exist 'issue_13698/struct.Foo.html'
2019-09-23T05:08:14.2474996Z  // @!has issue_13698/struct.Foo.html '//*[@id="method.bar"]' 'fn bar'
2019-09-23T05:08:14.2475061Z Encountered 2 errors
2019-09-23T05:08:14.2475204Z 
2019-09-23T05:08:14.2475398Z ------------------------------------------
2019-09-23T05:08:14.2475431Z 
2019-09-23T05:08:14.2475431Z 
2019-09-23T05:08:14.2475450Z 
2019-09-23T05:08:14.2475619Z ---- [rustdoc] rustdoc/issue-15169.rs stdout ----
2019-09-23T05:08:14.2475643Z 
2019-09-23T05:08:14.2475681Z error: htmldocck failed!
2019-09-23T05:08:14.2475793Z status: exit code: 1
2019-09-23T05:08:14.2476097Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15169" "/checkout/src/test/rustdoc/issue-15169.rs"
2019-09-23T05:08:14.2476315Z ------------------------------------------
2019-09-23T05:08:14.2476339Z 
2019-09-23T05:08:14.2476497Z ------------------------------------------
2019-09-23T05:08:14.2476538Z stderr:
2019-09-23T05:08:14.2476538Z stderr:
2019-09-23T05:08:14.2476698Z ------------------------------------------
2019-09-23T05:08:14.2476733Z 1: @has check failed
2019-09-23T05:08:14.2476901Z  File does not exist 'issue_15169/struct.Foo.html'
2019-09-23T05:08:14.2477106Z  // @has issue_15169/struct.Foo.html '//*[@id="method.eq"]' 'fn eq'
2019-09-23T05:08:14.2477164Z Encountered 1 errors
2019-09-23T05:08:14.2477185Z 
2019-09-23T05:08:14.2477352Z ------------------------------------------
2019-09-23T05:08:14.2477376Z 
2019-09-23T05:08:14.2477376Z 
2019-09-23T05:08:14.2477395Z 
2019-09-23T05:08:14.2477570Z ---- [rustdoc] rustdoc/issue-15318-2.rs stdout ----
2019-09-23T05:08:14.2477602Z 
2019-09-23T05:08:14.2477634Z error: htmldocck failed!
2019-09-23T05:08:14.2477666Z status: exit code: 1
2019-09-23T05:08:14.2477946Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318-2" "/checkout/src/test/rustdoc/issue-15318-2.rs"
2019-09-23T05:08:14.2478151Z ------------------------------------------
2019-09-23T05:08:14.2478175Z 
2019-09-23T05:08:14.2478341Z ------------------------------------------
2019-09-23T05:08:14.2478376Z stderr:
2019-09-23T05:08:14.2478376Z stderr:
2019-09-23T05:08:14.2478541Z ------------------------------------------
2019-09-23T05:08:14.2478583Z 8: @has check failed
2019-09-23T05:08:14.2478751Z  File does not exist 'issue_15318_2/fn.bar.html'
2019-09-23T05:08:14.2478948Z  // @has issue_15318_2/fn.bar.html '//*[@href="primitive.pointer.html"]' '*mut T'
2019-09-23T05:08:14.2479020Z Encountered 1 errors
2019-09-23T05:08:14.2479041Z 
2019-09-23T05:08:14.2479203Z ------------------------------------------
2019-09-23T05:08:14.2479226Z 
2019-09-23T05:08:14.2479226Z 
2019-09-23T05:08:14.2479251Z 
2019-09-23T05:08:14.2479419Z ---- [rustdoc] rustdoc/issue-15318-3.rs stdout ----
2019-09-23T05:08:14.2479444Z 
2019-09-23T05:08:14.2479475Z error: htmldocck failed!
2019-09-23T05:08:14.2479514Z status: exit code: 1
2019-09-23T05:08:14.2479783Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318-3" "/checkout/src/test/rustdoc/issue-15318-3.rs"
2019-09-23T05:08:14.2479997Z ------------------------------------------
2019-09-23T05:08:14.2480021Z 
2019-09-23T05:08:14.2480180Z ------------------------------------------
2019-09-23T05:08:14.2480213Z stderr:
2019-09-23T05:08:14.2480213Z stderr:
2019-09-23T05:08:14.2480379Z ------------------------------------------
2019-09-23T05:08:14.2480413Z 1: @has check failed
2019-09-23T05:08:14.2480595Z  File does not exist 'issue_15318_3/primitive.pointer.html'
2019-09-23T05:08:14.2480643Z  // @has issue_15318_3/primitive.pointer.html
2019-09-23T05:08:14.2480695Z Encountered 1 errors
2019-09-23T05:08:14.2480716Z 
2019-09-23T05:08:14.2480884Z ------------------------------------------
2019-09-23T05:08:14.2480908Z 
2019-09-23T05:08:14.2480908Z 
2019-09-23T05:08:14.2480927Z 
2019-09-23T05:08:14.2481728Z ---- [rustdoc] rustdoc/issue-15347.rs stdout ----
2019-09-23T05:08:14.2481768Z 
2019-09-23T05:08:14.2481821Z error: htmldocck failed!
2019-09-23T05:08:14.2481863Z status: exit code: 1
2019-09-23T05:08:14.2482252Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15347" "/checkout/src/test/rustdoc/issue-15347.rs"
2019-09-23T05:08:14.2482692Z ------------------------------------------
2019-09-23T05:08:14.2482724Z 
2019-09-23T05:08:14.2482930Z ------------------------------------------
2019-09-23T05:08:14.2483071Z stderr:
2019-09-23T05:08:14.2483071Z stderr:
2019-09-23T05:08:14.2483320Z ------------------------------------------
2019-09-23T05:08:14.2483369Z 3: @has check failed
2019-09-23T05:08:14.2483596Z  File does not exist 'issue_15347/fn.foo.html'
2019-09-23T05:08:14.2483645Z  // @has issue_15347/fn.foo.html
2019-09-23T05:08:14.2483713Z Encountered 1 errors
2019-09-23T05:08:14.2483749Z 
2019-09-23T05:08:14.2483959Z ------------------------------------------
2019-09-23T05:08:14.2483990Z 
2019-09-23T05:08:14.2483990Z 
2019-09-23T05:08:14.2484015Z 
2019-09-23T05:08:14.2484235Z ---- [rustdoc] rustdoc/issue-15318.rs stdout ----
2019-09-23T05:08:14.2484266Z 
2019-09-23T05:08:14.2484307Z error: htmldocck failed!
2019-09-23T05:08:14.2484359Z status: exit code: 1
2019-09-23T05:08:14.2485013Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318" "/checkout/src/test/rustdoc/issue-15318.rs"
2019-09-23T05:08:14.2485222Z ------------------------------------------
2019-09-23T05:08:14.2485254Z 
2019-09-23T05:08:14.2485414Z ------------------------------------------
2019-09-23T05:08:14.2485449Z stderr:
2019-09-23T05:08:14.2485449Z stderr:
2019-09-23T05:08:14.2485605Z ------------------------------------------
2019-09-23T05:08:14.2485648Z 8: @has check failed
2019-09-23T05:08:14.2485814Z  File does not exist 'issue_15318/fn.bar.html'
2019-09-23T05:08:14.2486031Z  // @has issue_15318/fn.bar.html '//*[@href="http://example.com/issue_15318/primitive.pointer.html"]' '*mut T'
2019-09-23T05:08:14.2486099Z Encountered 1 errors
2019-09-23T05:08:14.2486119Z 
2019-09-23T05:08:14.2486282Z ------------------------------------------
2019-09-23T05:08:14.2486321Z 
2019-09-23T05:08:14.2486321Z 
2019-09-23T05:08:14.2486340Z 
2019-09-23T05:08:14.2486511Z ---- [rustdoc] rustdoc/issue-16265-1.rs stdout ----
2019-09-23T05:08:14.2486537Z 
2019-09-23T05:08:14.2486574Z error: htmldocck failed!
2019-09-23T05:08:14.2486607Z status: exit code: 1
2019-09-23T05:08:14.2486883Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-1" "/checkout/src/test/rustdoc/issue-16265-1.rs"
2019-09-23T05:08:14.2487097Z ------------------------------------------
2019-09-23T05:08:14.2487121Z 
2019-09-23T05:08:14.2487278Z ------------------------------------------
2019-09-23T05:08:14.2487320Z stderr:
2019-09-23T05:08:14.2487320Z stderr:
2019-09-23T05:08:14.2487481Z ------------------------------------------
2019-09-23T05:08:14.2487517Z 3: @has check failed
2019-09-23T05:08:14.2487693Z  File does not exist 'issue_16265_1/traits/index.html'
2019-09-23T05:08:14.2488035Z  // @has issue_16265_1/traits/index.html '[src]'
2019-09-23T05:08:14.2488096Z Encountered 1 errors
2019-09-23T05:08:14.2488115Z 
2019-09-23T05:08:14.2488277Z ------------------------------------------
2019-09-23T05:08:14.2488301Z 
2019-09-23T05:08:14.2488301Z 
2019-09-23T05:08:14.2488318Z 
2019-09-23T05:08:14.2488475Z ---- [rustdoc] rustdoc/issue-16265-2.rs stdout ----
2019-09-23T05:08:14.2488512Z 
2019-09-23T05:08:14.2488542Z error: htmldocck failed!
2019-09-23T05:08:14.2488571Z status: exit code: 1
2019-09-23T05:08:14.2488835Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-2" "/checkout/src/test/rustdoc/issue-16265-2.rs"
2019-09-23T05:08:14.2489030Z ------------------------------------------
2019-09-23T05:08:14.2489052Z 
2019-09-23T05:08:14.2489209Z ------------------------------------------
2019-09-23T05:08:14.2489241Z stderr:
2019-09-23T05:08:14.2489241Z stderr:
2019-09-23T05:08:14.2489390Z ------------------------------------------
2019-09-23T05:08:14.2489428Z 1: @has check failed
2019-09-23T05:08:14.2489890Z  File does not exist 'issue_16265_2/index.html'
2019-09-23T05:08:14.2490057Z  // @has issue_16265_2/index.html '[src]'
2019-09-23T05:08:14.2490120Z Encountered 1 errors
2019-09-23T05:08:14.2490140Z 
2019-09-23T05:08:14.2490299Z ------------------------------------------
2019-09-23T05:08:14.2490393Z 
2019-09-23T05:08:14.2490393Z 
2019-09-23T05:08:14.2490426Z 
2019-09-23T05:08:14.2490624Z ---- [rustdoc] rustdoc/issue-17476.rs stdout ----
2019-09-23T05:08:14.2490650Z 
2019-09-23T05:08:14.2490681Z error: htmldocck failed!
2019-09-23T05:08:14.2490721Z status: exit code: 1
2019-09-23T05:08:14.2490988Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-17476" "/checkout/src/test/rustdoc/issue-17476.rs"
2019-09-23T05:08:14.2491578Z ------------------------------------------
2019-09-23T05:08:14.2491615Z 
2019-09-23T05:08:14.2491858Z ------------------------------------------
2019-09-23T05:08:14.2491916Z stderr:
2019-09-23T05:08:14.2491916Z stderr:
2019-09-23T05:08:14.2492133Z ------------------------------------------
2019-09-23T05:08:14.2492179Z 8: @has check failed
2019-09-23T05:08:14.2492397Z  File does not exist 'issue_17476/struct.Foo.html'
2019-09-23T05:08:14.2492697Z  // @has issue_17476/struct.Foo.html '//*[@href="http://example.com/issue_17476/trait.Foo.html#method.foo"]' 'foo'
2019-09-23T05:08:14.2492776Z Encountered 1 errors
2019-09-23T05:08:14.2492803Z 
2019-09-23T05:08:14.2493019Z ------------------------------------------
2019-09-23T05:08:14.2493049Z 
2019-09-23T05:08:14.2493049Z 
2019-09-23T05:08:14.2493074Z 
2019-09-23T05:08:14.2493289Z ---- [rustdoc] rustdoc/issue-19055.rs stdout ----
2019-09-23T05:08:14.2493320Z 
2019-09-23T05:08:14.2493368Z error: htmldocck failed!
2019-09-23T05:08:14.2493410Z status: exit code: 1
2019-09-23T05:08:14.2493753Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19055" "/checkout/src/test/rustdoc/issue-19055.rs"
2019-09-23T05:08:14.2494036Z ------------------------------------------
2019-09-23T05:08:14.2494067Z 
2019-09-23T05:08:14.2494278Z ------------------------------------------
2019-09-23T05:08:14.2494323Z stderr:
2019-09-23T05:08:14.2494323Z stderr:
2019-09-23T05:08:14.2494541Z ------------------------------------------
2019-09-23T05:08:14.2494588Z 1: @has check failed
2019-09-23T05:08:14.2495011Z  File does not exist 'issue_19055/trait.Any.html'
2019-09-23T05:08:14.2495137Z  // @has issue_19055/trait.Any.html
2019-09-23T05:08:14.2495175Z 5: @has check failed
2019-09-23T05:08:14.2495383Z  File does not exist 'issue_19055/trait.Any.html'
2019-09-23T05:08:14.2495564Z      // @has - '//*[@id="method.is"]' 'fn is'
2019-09-23T05:08:14.2495602Z 8: @has check failed
2019-09-23T05:08:14.2495789Z  File does not exist 'issue_19055/trait.Any.html'
2019-09-23T05:08:14.2495988Z      // @has - '//*[@id="method.downcast_ref"]' 'fn downcast_ref'
2019-09-23T05:08:14.2496029Z 11: @has check failed
2019-09-23T05:08:14.2496227Z  File does not exist 'issue_19055/trait.Any.html'
2019-09-23T05:08:14.2496426Z      // @has - '//*[@id="method.downcast_mut"]' 'fn downcast_mut'
2019-09-23T05:08:14.2496466Z 19: @has check failed
2019-09-23T05:08:14.2496648Z  File does not exist 'issue_19055/trait.Any.html'
2019-09-23T05:08:14.2496847Z  // @has - '//*[@id="method.foo"]' 'fn foo'
2019-09-23T05:08:14.2496908Z Encountered 5 errors
2019-09-23T05:08:14.2496931Z 
2019-09-23T05:08:14.2497276Z ------------------------------------------
2019-09-23T05:08:14.2497302Z 
2019-09-23T05:08:14.2497302Z 
2019-09-23T05:08:14.2497322Z 
2019-09-23T05:08:14.2497828Z ---- [rustdoc] rustdoc/issue-19190-2.rs stdout ----
2019-09-23T05:08:14.2497859Z 
2019-09-23T05:08:14.2497890Z error: htmldocck failed!
2019-09-23T05:08:14.2497922Z status: exit code: 1
2019-09-23T05:08:14.2498199Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-2" "/checkout/src/test/rustdoc/issue-19190-2.rs"
2019-09-23T05:08:14.2498547Z ------------------------------------------
2019-09-23T05:08:14.2498574Z 
2019-09-23T05:08:14.2498744Z ------------------------------------------
2019-09-23T05:08:14.2498779Z stderr:
2019-09-23T05:08:14.2498779Z stderr:
2019-09-23T05:08:14.2498938Z ------------------------------------------
2019-09-23T05:08:14.2499109Z 10: @has check failed
2019-09-23T05:08:14.2499327Z  File does not exist 'issue_19190_2/struct.Bar.html'
2019-09-23T05:08:14.2499368Z  // @has issue_19190_2/struct.Bar.html
2019-09-23T05:08:14.2499402Z 11: @!has check failed
2019-09-23T05:08:14.2499585Z  File does not exist 'issue_19190_2/struct.Bar.html'
2019-09-23T05:08:14.2499760Z  // @!has - '//*[@id="method.new"]' 'fn new() -> String'
2019-09-23T05:08:14.2499797Z 12: @has check failed
2019-09-23T05:08:14.2500156Z  File does not exist 'issue_19190_2/struct.Bar.html'
2019-09-23T05:08:14.2500342Z  // @has - '//*[@id="method.as_str"]' 'fn as_str(&self) -> &str'
2019-09-23T05:08:14.2500416Z Encountered 3 errors
2019-09-23T05:08:14.2500437Z 
2019-09-23T05:08:14.2500605Z ------------------------------------------
2019-09-23T05:08:14.2500630Z 
2019-09-23T05:08:14.2500630Z 
2019-09-23T05:08:14.2500650Z 
2019-09-23T05:08:14.2500827Z ---- [rustdoc] rustdoc/issue-19190.rs stdout ----
2019-09-23T05:08:14.2500852Z 
2019-09-23T05:08:14.2500892Z error: htmldocck failed!
2019-09-23T05:08:14.2500925Z status: exit code: 1
2019-09-23T05:08:14.2502196Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190" "/checkout/src/test/rustdoc/issue-19190.rs"
2019-09-23T05:08:14.2502525Z ------------------------------------------
2019-09-23T05:08:14.2502570Z 
2019-09-23T05:08:14.2502781Z ------------------------------------------
2019-09-23T05:08:14.2502826Z stderr:
2019-09-23T05:08:14.2502826Z stderr:
2019-09-23T05:08:14.2503039Z ------------------------------------------
2019-09-23T05:08:14.2503085Z 18: @has check failed
2019-09-23T05:08:14.2503317Z  File does not exist 'issue_19190/Bar.t.html'
2019-09-23T05:08:14.2503365Z  // @has issue_19190/Bar.t.html
2019-09-23T05:08:14.2503416Z 19: @has check failed
2019-09-23T05:08:14.2503635Z  File does not exist 'issue_19190/struct.Bar.html'
2019-09-23T05:08:14.2503684Z  // @has issue_19190/struct.Bar.html
2019-09-23T05:08:14.2503743Z 20: @has check failed
2019-09-23T05:08:14.2503965Z  File does not exist 'issue_19190/struct.Bar.html'
2019-09-23T05:08:14.2504181Z  // @has - '//*[@id="foo.v"]' 'fn foo(&self)'
2019-09-23T05:08:14.2504234Z 21: @has check failed
2019-09-23T05:08:14.2504452Z  File does not exist 'issue_19190/struct.Bar.html'
2019-09-23T05:08:14.2504834Z  // @has - '//*[@id="method.foo"]' 'fn foo(&self)'
2019-09-23T05:08:14.2505048Z 22: @!has check failed
2019-09-23T05:08:14.2505392Z  File does not exist 'issue_19190/struct.Bar.html'
2019-09-23T05:08:14.2505573Z  // @!has - '//*[@id="static_foo.v"]' 'fn static_foo()'
2019-09-23T05:08:14.2505611Z 23: @!has check failed
2019-09-23T05:08:14.2505800Z  File does not exist 'issue_19190/struct.Bar.html'
2019-09-23T05:08:14.2505980Z  // @!has - '//*[@id="method.static_foo"]' 'fn static_foo()'
2019-09-23T05:08:14.2506043Z Encountered 6 errors
2019-09-23T05:08:14.2506065Z 
2019-09-23T05:08:14.2506228Z ------------------------------------------
2019-09-23T05:08:14.2506260Z 
2019-09-23T05:08:14.2506260Z 
2019-09-23T05:08:14.2506280Z 
2019-09-23T05:08:14.2506464Z ---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----
2019-09-23T05:08:14.2506489Z 
2019-09-23T05:08:14.2506522Z error: htmldocck failed!
2019-09-23T05:08:14.2506555Z status: exit code: 1
2019-09-23T05:08:14.2506841Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3" "/checkout/src/test/rustdoc/issue-19190-3.rs"
2019-09-23T05:08:14.2507051Z ------------------------------------------
2019-09-23T05:08:14.2507082Z 
2019-09-23T05:08:14.2507246Z ------------------------------------------
2019-09-23T05:08:14.2507585Z stderr:
2019-09-23T05:08:14.2507585Z stderr:
2019-09-23T05:08:14.2507797Z ------------------------------------------
2019-09-23T05:08:14.2507836Z 9: @has check failed
2019-09-23T05:08:14.2508019Z  File does not exist 'issue_19190_3/struct.Foo.html'
2019-09-23T05:08:14.2508059Z  // @has issue_19190_3/struct.Foo.html
2019-09-23T05:08:14.2508424Z 10: @has check failed
2019-09-23T05:08:14.2508872Z  File does not exist 'issue_19190_3/struct.Foo.html'
2019-09-23T05:08:14.2509246Z  // @has - '//*[@id="method.as_str"]' 'fn as_str(&self) -> &str'
2019-09-23T05:08:14.2509300Z 11: @!has check failed
2019-09-23T05:08:14.2509494Z  File does not exist 'issue_19190_3/struct.Foo.html'
2019-09-23T05:08:14.2509688Z  // @!has - '//*[@id="method.new"]' 'fn new() -> String'
2019-09-23T05:08:14.2509736Z 14: @has check failed
2019-09-23T05:08:14.2509928Z  File does not exist 'issue_19190_3/struct.Bar.html'
2019-09-23T05:08:14.2509971Z  // @has issue_19190_3/struct.Bar.html
2019-09-23T05:08:14.2510016Z 15: @has check failed
2019-09-23T05:08:14.2510222Z  File does not exist 'issue_19190_3/struct.Bar.html'
2019-09-23T05:08:14.2510413Z  // @has - '//*[@id="method.baz"]' 'fn baz(&self)'
2019-09-23T05:08:14.2510454Z 16: @!has check failed
2019-09-23T05:08:14.2510655Z  File does not exist 'issue_19190_3/struct.Bar.html'
2019-09-23T05:08:14.2510863Z  // @!has - '//*[@id="method.static_baz"]' 'fn static_baz()'
2019-09-23T05:08:14.2510905Z 19: @has check failed
2019-09-23T05:08:14.2511110Z  File does not exist 'issue_19190_3/struct.MyBar.html'
2019-09-23T05:08:14.2511154Z  // @has issue_19190_3/struct.MyBar.html
2019-09-23T05:08:14.2511192Z 20: @has check failed
2019-09-23T05:08:14.2511394Z  File does not exist 'issue_19190_3/struct.MyBar.html'
2019-09-23T05:08:14.2511958Z  // @has - '//*[@id="method.baz"]' 'fn baz(&self)'
2019-09-23T05:08:14.2512012Z 21: @!has check failed
2019-09-23T05:08:14.2512265Z  File does not exist 'issue_19190_3/struct.MyBar.html'
2019-09-23T05:08:14.2512497Z  // @!has - '//*[@id="method.static_baz"]' 'fn static_baz()'
2019-09-23T05:08:14.2512582Z Encountered 9 errors
2019-09-23T05:08:14.2512617Z 
2019-09-23T05:08:14.2512827Z ------------------------------------------
2019-09-23T05:08:14.2512859Z 
2019-09-23T05:08:14.2512859Z 
2019-09-23T05:08:14.2512884Z 
2019-09-23T05:08:14.2513103Z ---- [rustdoc] rustdoc/issue-20175.rs stdout ----
2019-09-23T05:08:14.2513136Z 
2019-09-23T05:08:14.2513185Z error: htmldocck failed!
2019-09-23T05:08:14.2513227Z status: exit code: 1
2019-09-23T05:08:14.2513586Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20175" "/checkout/src/test/rustdoc/issue-20175.rs"
2019-09-23T05:08:14.2513850Z ------------------------------------------
2019-09-23T05:08:14.2513881Z 
2019-09-23T05:08:14.2514097Z ------------------------------------------
2019-09-23T05:08:14.2514142Z stderr:
2019-09-23T05:08:14.2514142Z stderr:
2019-09-23T05:08:14.2514347Z ------------------------------------------
2019-09-23T05:08:14.2514399Z 7: @has check failed
2019-09-23T05:08:14.2514633Z  File does not exist 'issue_20175/struct.Bar.html'
2019-09-23T05:08:14.2514872Z  // @has issue_20175/struct.Bar.html '//*[@id="method.foo"]' 'fn foo'
2019-09-23T05:08:14.2514956Z Encountered 1 errors
2019-09-23T05:08:14.2514984Z 
2019-09-23T05:08:14.2515202Z ------------------------------------------
2019-09-23T05:08:14.2515394Z 
2019-09-23T05:08:14.2515394Z 
2019-09-23T05:08:14.2515422Z 
2019-09-23T05:08:14.2515606Z ---- [rustdoc] rustdoc/issue-20646.rs stdout ----
2019-09-23T05:08:14.2515633Z 
2019-09-23T05:08:14.2515667Z error: htmldocck failed!
2019-09-23T05:08:14.2515711Z status: exit code: 1
2019-09-23T05:08:14.2515998Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20646" "/checkout/src/test/rustdoc/issue-20646.rs"
2019-09-23T05:08:14.2516225Z ------------------------------------------
2019-09-23T05:08:14.2516251Z 
2019-09-23T05:08:14.2516583Z ------------------------------------------
2019-09-23T05:08:14.2516745Z stderr:
2019-09-23T05:08:14.2516745Z stderr:
2019-09-23T05:08:14.2516960Z ------------------------------------------
2019-09-23T05:08:14.2517000Z 8: @has check failed
2019-09-23T05:08:14.2517356Z  File does not exist 'issue_20646/trait.Trait.html'
2019-09-23T05:08:14.2517669Z  // @has issue_20646/trait.Trait.html '//*[@id="associatedtype.Output"]' 'type Output'
2019-09-23T05:08:14.2517722Z 15: @has check failed
2019-09-23T05:08:14.2517932Z  File does not exist 'issue_20646/fn.fun.html'
2019-09-23T05:08:14.2518163Z  // @has issue_20646/fn.fun.html '//*[@class="rust fn"]' 'where T: Trait<Output = i32>'
2019-09-23T05:08:14.2518207Z 20: @has check failed
2019-09-23T05:08:14.2518397Z  File does not exist 'issue_20646/reexport/trait.Trait.html'
2019-09-23T05:08:14.2518631Z      // @has issue_20646/reexport/trait.Trait.html '//*[@id="associatedtype.Output"]' 'type Output'
2019-09-23T05:08:14.2518675Z 23: @has check failed
2019-09-23T05:08:14.2518862Z  File does not exist 'issue_20646/reexport/fn.fun.html'
2019-09-23T05:08:14.2519106Z      // @has issue_20646/reexport/fn.fun.html '//*[@class="rust fn"]' 'where T: Trait<Output = i32>'
2019-09-23T05:08:14.2519172Z Encountered 4 errors
2019-09-23T05:08:14.2519195Z 
2019-09-23T05:08:14.2519380Z ------------------------------------------
2019-09-23T05:08:14.2519414Z 
2019-09-23T05:08:14.2519414Z 
2019-09-23T05:08:14.2519436Z 
2019-09-23T05:08:14.2519619Z ---- [rustdoc] rustdoc/issue-20727-2.rs stdout ----
2019-09-23T05:08:14.2519646Z 
2019-09-23T05:08:14.2519687Z error: htmldocck failed!
2019-09-23T05:08:14.2519722Z status: exit code: 1
2019-09-23T05:08:14.2520170Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2" "/checkout/src/test/rustdoc/issue-20727-2.rs"
2019-09-23T05:08:14.2520621Z ------------------------------------------
2019-09-23T05:08:14.2520645Z 
2019-09-23T05:08:14.2520807Z ------------------------------------------
2019-09-23T05:08:14.2521254Z stderr:
2019-09-23T05:08:14.2521254Z stderr:
2019-09-23T05:08:14.2521508Z ------------------------------------------
2019-09-23T05:08:14.2521556Z 6: @has check failed
2019-09-23T05:08:14.2521786Z  File does not exist 'issue_20727_2/trait.Add.html'
2019-09-23T05:08:14.2521836Z  // @has issue_20727_2/trait.Add.html
2019-09-23T05:08:14.2521889Z 8: @has check failed
---
2019-09-23T05:08:14.3219891Z test result: FAILED. 60 passed; 258 failed; 2 ignored; 0 measured; 0 filtered out
2019-09-23T05:08:14.3219923Z 
2019-09-23T05:08:14.3219945Z 
2019-09-23T05:08:14.3219966Z 
2019-09-23T05:08:14.3221850Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-23T05:08:14.3222109Z 
2019-09-23T05:08:14.3222263Z 
2019-09-23T05:08:14.3222318Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-23T05:08:14.3222369Z Build completed unsuccessfully in 1:16:11
2019-09-23T05:08:14.3222369Z Build completed unsuccessfully in 1:16:11
2019-09-23T05:08:14.3222416Z == clock drift check ==
2019-09-23T05:08:14.3222470Z   local time: Mon Sep 23 05:08:14 UTC 2019
2019-09-23T05:08:14.3222908Z   network time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-23T05:08:14.3222982Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-23T05:08:14.3453590Z Mon, 23 Sep 2019 05:08:14 GMT
2019-09-23T05:08:14.3453679Z == end clock drift check ==
2019-09-23T05:08:16.4503055Z ##[error]Bash exited with code '1'.
2019-09-23T05:08:16.4539290Z ##[section]Starting: Checkout
2019-09-23T05:08:16.4540887Z ==============================================================================
2019-09-23T05:08:16.4540951Z Task         : Get sources
2019-09-23T05:08:16.4541006Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
