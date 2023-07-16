plain
2019-12-10T17:02:16.3365106Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T17:02:17.1882791Z ##[command]git config gc.auto 0
2019-12-10T17:02:17.1888205Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T17:02:17.1892277Z ##[command]git config --get-all http.proxy
2019-12-10T17:02:17.1897013Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65951/merge:refs/remotes/pull/65951/merge
---
2019-12-10T18:01:49.1338986Z .................................................................................................... 1600/9342
2019-12-10T18:01:53.5964803Z .................................................................................................... 1700/9342
2019-12-10T18:02:05.4274093Z ..................................................i................................................. 1800/9342
2019-12-10T18:02:13.6670283Z .................................................................................................... 1900/9342
2019-12-10T18:02:27.6648995Z ...................................iiiii............................................................ 2000/9342
2019-12-10T18:02:37.6389006Z .................................................................................................... 2200/9342
2019-12-10T18:02:40.1553776Z .................................................................................................... 2300/9342
2019-12-10T18:02:44.5144844Z .................................................................................................... 2400/9342
2019-12-10T18:03:06.1356087Z .................................................................................................... 2500/9342
---
2019-12-10T18:05:43.0076038Z ......................................i...............i............................................. 4800/9342
2019-12-10T18:05:52.6770187Z .................................................................................................... 4900/9342
2019-12-10T18:05:59.0868020Z ..................................................................................i................. 5000/9342
2019-12-10T18:06:05.0817557Z .................................................................................................... 5100/9342
2019-12-10T18:06:14.8156719Z ................................................ii.ii...........i................................... 5200/9342
2019-12-10T18:06:24.0604026Z .................................................................................................... 5400/9342
2019-12-10T18:06:34.0385053Z .................................................................................................... 5500/9342
2019-12-10T18:06:40.9834313Z ..............................i..................................................................... 5600/9342
2019-12-10T18:06:47.2534185Z .................................................................................................... 5700/9342
2019-12-10T18:06:47.2534185Z .................................................................................................... 5700/9342
2019-12-10T18:06:58.9530445Z .................................................................................................... 5800/9342
2019-12-10T18:07:09.5703288Z .................ii...i..ii...........i............................................................. 5900/9342
2019-12-10T18:07:27.1849297Z .................................................................................................... 6100/9342
2019-12-10T18:07:35.2036237Z .................................................................................................... 6200/9342
2019-12-10T18:07:35.2036237Z .................................................................................................... 6200/9342
2019-12-10T18:07:51.6559084Z .........................................i..ii...................................................... 6300/9342
2019-12-10T18:08:13.1142558Z .................................................................................................... 6500/9342
2019-12-10T18:08:15.2521093Z .............i...................................................................................... 6600/9342
2019-12-10T18:08:17.5459799Z .................................................................................................... 6700/9342
2019-12-10T18:08:20.1137727Z ....i............................................................................................... 6800/9342
---
2019-12-10T18:09:58.1446792Z .................................................................................................... 7400/9342
2019-12-10T18:10:03.2660434Z .................................................................................................... 7500/9342
2019-12-10T18:10:10.3109752Z .................................................................................................... 7600/9342
2019-12-10T18:10:20.8456355Z .................................................................................................... 7700/9342
2019-12-10T18:10:27.0449205Z ....................iiii............................................................................ 7800/9342
2019-12-10T18:10:41.0821074Z .................................................................................................... 8000/9342
2019-12-10T18:10:51.6713957Z .................................................................................................... 8100/9342
2019-12-10T18:11:04.7730200Z .................................................................................................... 8200/9342
2019-12-10T18:11:11.8196405Z .................................................................................................... 8300/9342
---
2019-12-10T18:13:06.1330952Z 
2019-12-10T18:13:06.1330996Z 52 LL |         fn method(&self) {}
2019-12-10T18:13:06.1331248Z 53    |            ------
2019-12-10T18:13:06.1331316Z 54    |            |
2019-12-10T18:13:06.1331590Z -    |            the method is available for `std::boxed::Box<std::rc::Rc<_>>` here
2019-12-10T18:13:06.1331872Z -    |            the method is available for `std::pin::Pin<std::rc::Rc<_>>` here
2019-12-10T18:13:06.1332137Z -    |            the method is available for `std::sync::Arc<std::rc::Rc<_>>` here
2019-12-10T18:13:06.1332696Z -    |            the method is available for `std::rc::Rc<std::rc::Rc<_>>` here
2019-12-10T18:13:06.1332787Z +    |            the method is available for `std::boxed::Box<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1332864Z +    |            the method is available for `std::pin::Pin<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1332940Z +    |            the method is available for `std::sync::Arc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1332998Z +    |            the method is available for `std::rc::Rc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1333053Z 59 ...
2019-12-10T18:13:06.1333324Z 60 LL |     std::rc::Rc::new(&mut Box::new(&'a')).method();
2019-12-10T18:13:06.1333387Z 61    |                                           ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&char>>`
2019-12-10T18:13:06.1333450Z 
2019-12-10T18:13:06.1333509Z The actual stderr differed from the expected stderr.
2019-12-10T18:13:06.1333845Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
2019-12-10T18:13:06.1333845Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
2019-12-10T18:13:06.1334107Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T18:13:06.1334406Z To only update this specific test, also pass `--test-args impl-trait/no-method-suggested-traits.rs`
2019-12-10T18:13:06.1334486Z error: 1 errors occurred comparing output.
2019-12-10T18:13:06.1334544Z status: exit code: 1
2019-12-10T18:13:06.1334544Z status: exit code: 1
2019-12-10T18:13:06.1335328Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/auxiliary" "-A" "unused"
2019-12-10T18:13:06.1335844Z ------------------------------------------
2019-12-10T18:13:06.1335891Z 
2019-12-10T18:13:06.1336166Z ------------------------------------------
2019-12-10T18:13:06.1336213Z stderr:
2019-12-10T18:13:06.1336213Z stderr:
2019-12-10T18:13:06.1336425Z ------------------------------------------
2019-12-10T18:13:06.1336481Z error[E0599]: no method named `method` found for type `u32` in the current scope
2019-12-10T18:13:06.1336899Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:23:10
2019-12-10T18:13:06.1336952Z    |
2019-12-10T18:13:06.1336993Z LL |     1u32.method();
2019-12-10T18:13:06.1337094Z    |
2019-12-10T18:13:06.1337142Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1337142Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1337214Z help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2019-12-10T18:13:06.1337301Z LL | use foo::Bar;
2019-12-10T18:13:06.1337356Z    |
2019-12-10T18:13:06.1337356Z    |
2019-12-10T18:13:06.1337410Z LL | use no_method_suggested_traits::foo::PubPub;
2019-12-10T18:13:06.1337456Z    |
2019-12-10T18:13:06.1337504Z LL | use no_method_suggested_traits::qux::PrivPub;
2019-12-10T18:13:06.1337609Z LL | use no_method_suggested_traits::Reexported;
2019-12-10T18:13:06.1337653Z    |
2019-12-10T18:13:06.1337697Z 
2019-12-10T18:13:06.1337697Z 
2019-12-10T18:13:06.1337761Z error[E0599]: no method named `method` found for type `std::rc::Rc<&mut std::boxed::Box<&u32>>` in the current scope
2019-12-10T18:13:06.1338139Z    |
2019-12-10T18:13:06.1338139Z    |
2019-12-10T18:13:06.1338186Z LL |     std::rc::Rc::new(&mut Box::new(&1u32)).method();
2019-12-10T18:13:06.1338244Z    |                                            ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&u32>>`
2019-12-10T18:13:06.1338355Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1338355Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1338421Z help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2019-12-10T18:13:06.1338526Z LL | use foo::Bar;
2019-12-10T18:13:06.1338568Z    |
2019-12-10T18:13:06.1338568Z    |
2019-12-10T18:13:06.1338614Z LL | use no_method_suggested_traits::foo::PubPub;
2019-12-10T18:13:06.1338673Z    |
2019-12-10T18:13:06.1338718Z LL | use no_method_suggested_traits::qux::PrivPub;
2019-12-10T18:13:06.1338831Z LL | use no_method_suggested_traits::Reexported;
2019-12-10T18:13:06.1338875Z    |
2019-12-10T18:13:06.1338901Z 
2019-12-10T18:13:06.1338951Z error[E0599]: no method named `method` found for type `char` in the current scope
2019-12-10T18:13:06.1338951Z error[E0599]: no method named `method` found for type `char` in the current scope
2019-12-10T18:13:06.1339251Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:30:9
2019-12-10T18:13:06.1339301Z    |
2019-12-10T18:13:06.1339511Z LL |     'a'.method();
2019-12-10T18:13:06.1339621Z    |
2019-12-10T18:13:06.1339678Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1339678Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1339750Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-12-10T18:13:06.1339839Z LL | use foo::Bar;
2019-12-10T18:13:06.1339895Z    |
2019-12-10T18:13:06.1339922Z 
2019-12-10T18:13:06.1339922Z 
2019-12-10T18:13:06.1339974Z error[E0599]: no method named `method` found for type `std::rc::Rc<&mut std::boxed::Box<&char>>` in the current scope
2019-12-10T18:13:06.1340329Z    |
2019-12-10T18:13:06.1340372Z LL |         fn method(&self) {}
2019-12-10T18:13:06.1340584Z    |            ------
2019-12-10T18:13:06.1340647Z    |            |
2019-12-10T18:13:06.1340647Z    |            |
2019-12-10T18:13:06.1340702Z    |            the method is available for `std::boxed::Box<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1340853Z    |            the method is available for `std::pin::Pin<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1340937Z    |            the method is available for `std::sync::Arc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1340996Z    |            the method is available for `std::rc::Rc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2019-12-10T18:13:06.1341044Z ...
2019-12-10T18:13:06.1341337Z LL |     std::rc::Rc::new(&mut Box::new(&'a')).method();
2019-12-10T18:13:06.1341506Z    |                                           ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&char>>`
2019-12-10T18:13:06.1341615Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1341615Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1341670Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-12-10T18:13:06.1341758Z LL | use foo::Bar;
2019-12-10T18:13:06.1341816Z    |
2019-12-10T18:13:06.1341844Z 
2019-12-10T18:13:06.1341900Z error[E0599]: no method named `method` found for type `i32` in the current scope
2019-12-10T18:13:06.1341900Z error[E0599]: no method named `method` found for type `i32` in the current scope
2019-12-10T18:13:06.1342871Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:35:10
2019-12-10T18:13:06.1342937Z    |
2019-12-10T18:13:06.1342978Z LL |     1i32.method();
2019-12-10T18:13:06.1343080Z    |
2019-12-10T18:13:06.1343140Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1343140Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1343194Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-12-10T18:13:06.1343254Z    |
2019-12-10T18:13:06.1343297Z LL | use no_method_suggested_traits::foo::PubPub;
2019-12-10T18:13:06.1343380Z 
2019-12-10T18:13:06.1343380Z 
2019-12-10T18:13:06.1343432Z error[E0599]: no method named `method` found for type `std::rc::Rc<&mut std::boxed::Box<&i32>>` in the current scope
2019-12-10T18:13:06.1343770Z    |
2019-12-10T18:13:06.1343770Z    |
2019-12-10T18:13:06.1343830Z LL |     std::rc::Rc::new(&mut Box::new(&1i32)).method();
2019-12-10T18:13:06.1343885Z    |                                            ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&i32>>`
2019-12-10T18:13:06.1343995Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1343995Z    = help: items from traits can only be used if the trait is in scope
2019-12-10T18:13:06.1344055Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-12-10T18:13:06.1344100Z    |
2019-12-10T18:13:06.1344157Z LL | use no_method_suggested_traits::foo::PubPub;
2019-12-10T18:13:06.1344224Z 
2019-12-10T18:13:06.1344270Z error[E0599]: no method named `method` found for type `Foo` in the current scope
2019-12-10T18:13:06.1344548Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:40:9
2019-12-10T18:13:06.1344596Z    |
2019-12-10T18:13:06.1344596Z    |
2019-12-10T18:13:06.1344636Z LL | struct Foo;
2019-12-10T18:13:06.1344892Z    | ----------- method `method` not found for this
2019-12-10T18:13:06.1344939Z ...
2019-12-10T18:13:06.1344981Z LL |     Foo.method();
2019-12-10T18:13:06.1345083Z    |
2019-12-10T18:13:06.1345130Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1345130Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1345203Z    = note: the following traits define an item `method`, perhaps you need to implement one of them:
2019-12-10T18:13:06.1345261Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1345310Z            candidate #2: `no_method_suggested_traits::foo::PubPub`
2019-12-10T18:13:06.1345377Z            candidate #3: `no_method_suggested_traits::qux::PrivPub`
2019-12-10T18:13:06.1345429Z            candidate #4: `no_method_suggested_traits::Reexported`
2019-12-10T18:13:06.1345460Z 
2019-12-10T18:13:06.1345512Z error[E0599]: no method named `method` found for type `std::rc::Rc<&mut std::boxed::Box<&Foo>>` in the current scope
2019-12-10T18:13:06.1346004Z    |
2019-12-10T18:13:06.1346004Z    |
2019-12-10T18:13:06.1346051Z LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method();
2019-12-10T18:13:06.1346127Z    |                                           ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Foo>>`
2019-12-10T18:13:06.1346294Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1346294Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1346366Z    = note: the following traits define an item `method`, perhaps you need to implement one of them:
2019-12-10T18:13:06.1346419Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1346468Z            candidate #2: `no_method_suggested_traits::foo::PubPub`
2019-12-10T18:13:06.1346535Z            candidate #3: `no_method_suggested_traits::qux::PrivPub`
2019-12-10T18:13:06.1346587Z            candidate #4: `no_method_suggested_traits::Reexported`
2019-12-10T18:13:06.1346675Z error[E0599]: no method named `method2` found for type `u64` in the current scope
2019-12-10T18:13:06.1347008Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:45:10
2019-12-10T18:13:06.1347059Z    |
2019-12-10T18:13:06.1347059Z    |
2019-12-10T18:13:06.1347101Z LL |     1u64.method2();
2019-12-10T18:13:06.1347218Z    |
2019-12-10T18:13:06.1347268Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1347268Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1347339Z    = note: the following trait defines an item `method2`, perhaps you need to implement it:
2019-12-10T18:13:06.1347390Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1347420Z 
2019-12-10T18:13:06.1347489Z error[E0599]: no method named `method2` found for type `std::rc::Rc<&mut std::boxed::Box<&u64>>` in the current scope
2019-12-10T18:13:06.1347826Z    |
2019-12-10T18:13:06.1347826Z    |
2019-12-10T18:13:06.1347880Z LL |     std::rc::Rc::new(&mut Box::new(&1u64)).method2();
2019-12-10T18:13:06.1347953Z    |                                            ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&u64>>`
2019-12-10T18:13:06.1348050Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1348050Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1348129Z    = note: the following trait defines an item `method2`, perhaps you need to implement it:
2019-12-10T18:13:06.1348180Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1348278Z error[E0599]: no method named `method2` found for type `no_method_suggested_traits::Foo` in the current scope
2019-12-10T18:13:06.1348562Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:50:37
2019-12-10T18:13:06.1348612Z    |
2019-12-10T18:13:06.1348612Z    |
2019-12-10T18:13:06.1348673Z LL |     no_method_suggested_traits::Foo.method2();
2019-12-10T18:13:06.1348738Z    |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`
2019-12-10T18:13:06.1348851Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1348851Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1348908Z    = note: the following trait defines an item `method2`, perhaps you need to implement it:
2019-12-10T18:13:06.1348958Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1348995Z 
2019-12-10T18:13:06.1349065Z error[E0599]: no method named `method2` found for type `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>` in the current scope
2019-12-10T18:13:06.1349407Z    |
2019-12-10T18:13:06.1349407Z    |
2019-12-10T18:13:06.1349472Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method2();
2019-12-10T18:13:06.1349621Z    |                                                                       ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>`
2019-12-10T18:13:06.1349755Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1349755Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1349810Z    = note: the following trait defines an item `method2`, perhaps you need to implement it:
2019-12-10T18:13:06.1349876Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1350020Z error[E0599]: no method named `method2` found for type `no_method_suggested_traits::Bar` in the current scope
2019-12-10T18:13:06.1350330Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:54:40
2019-12-10T18:13:06.1350399Z    |
2019-12-10T18:13:06.1350399Z    |
2019-12-10T18:13:06.1350445Z LL |     no_method_suggested_traits::Bar::X.method2();
2019-12-10T18:13:06.1350501Z    |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`
2019-12-10T18:13:06.1350623Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1350623Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1350679Z    = note: the following trait defines an item `method2`, perhaps you need to implement it:
2019-12-10T18:13:06.1350744Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1350773Z 
2019-12-10T18:13:06.1350827Z error[E0599]: no method named `method2` found for type `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>` in the current scope
2019-12-10T18:13:06.1351188Z    |
2019-12-10T18:13:06.1351188Z    |
2019-12-10T18:13:06.1351238Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method2();
2019-12-10T18:13:06.1351318Z    |                                                                          ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>`
2019-12-10T18:13:06.1351420Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1351420Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1351500Z    = note: the following trait defines an item `method2`, perhaps you need to implement it:
2019-12-10T18:13:06.1351550Z            candidate #1: `foo::Bar`
2019-12-10T18:13:06.1351630Z error[E0599]: no method named `method3` found for type `Foo` in the current scope
2019-12-10T18:13:06.1351922Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:59:9
2019-12-10T18:13:06.1351982Z    |
2019-12-10T18:13:06.1352024Z LL | struct Foo;
2019-12-10T18:13:06.1352024Z LL | struct Foo;
2019-12-10T18:13:06.1352504Z    | ----------- method `method3` not found for this
2019-12-10T18:13:06.1352562Z ...
2019-12-10T18:13:06.1352604Z LL |     Foo.method3();
2019-12-10T18:13:06.1352708Z    |
2019-12-10T18:13:06.1352755Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1352755Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1352810Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2019-12-10T18:13:06.1352889Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2019-12-10T18:13:06.1352921Z 
2019-12-10T18:13:06.1352973Z error[E0599]: no method named `method3` found for type `std::rc::Rc<&mut std::boxed::Box<&Foo>>` in the current scope
2019-12-10T18:13:06.1353387Z    |
2019-12-10T18:13:06.1353387Z    |
2019-12-10T18:13:06.1353433Z LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method3();
2019-12-10T18:13:06.1353509Z    |                                           ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Foo>>`
2019-12-10T18:13:06.1353606Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1353606Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1353677Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2019-12-10T18:13:06.1353732Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2019-12-10T18:13:06.1353917Z error[E0599]: no method named `method3` found for type `Bar` in the current scope
2019-12-10T18:13:06.1354252Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:63:12
2019-12-10T18:13:06.1354302Z    |
2019-12-10T18:13:06.1354302Z    |
2019-12-10T18:13:06.1354344Z LL | enum Bar { X }
2019-12-10T18:13:06.1354606Z    | -------- method `method3` not found for this
2019-12-10T18:13:06.1354740Z ...
2019-12-10T18:13:06.1354783Z LL |     Bar::X.method3();
2019-12-10T18:13:06.1354891Z    |
2019-12-10T18:13:06.1354940Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1354940Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1355012Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2019-12-10T18:13:06.1355067Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2019-12-10T18:13:06.1355099Z 
2019-12-10T18:13:06.1355159Z error[E0599]: no method named `method3` found for type `std::rc::Rc<&mut std::boxed::Box<&Bar>>` in the current scope
2019-12-10T18:13:06.1355534Z    |
2019-12-10T18:13:06.1355534Z    |
2019-12-10T18:13:06.1355581Z LL |     std::rc::Rc::new(&mut Box::new(&Bar::X)).method3();
2019-12-10T18:13:06.1355696Z    |                                              ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Bar>>`
2019-12-10T18:13:06.1355806Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1355806Z    = help: items from traits can only be used if the trait is implemented and in scope
2019-12-10T18:13:06.1355876Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2019-12-10T18:13:06.1355931Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2019-12-10T18:13:06.1356028Z error[E0599]: no method named `method3` found for type `usize` in the current scope
2019-12-10T18:13:06.1356329Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:69:13
2019-12-10T18:13:06.1356379Z    |
2019-12-10T18:13:06.1356379Z    |
2019-12-10T18:13:06.1356427Z LL |     1_usize.method3(); //~ ERROR no method named
2019-12-10T18:13:06.1356524Z 
2019-12-10T18:13:06.1356524Z 
2019-12-10T18:13:06.1356576Z error[E0599]: no method named `method3` found for type `std::rc::Rc<&mut std::boxed::Box<&usize>>` in the current scope
2019-12-10T18:13:06.1356935Z    |
2019-12-10T18:13:06.1356935Z    |
2019-12-10T18:13:06.1356985Z LL |     std::rc::Rc::new(&mut Box::new(&1_usize)).method3(); //~ ERROR no method named
2019-12-10T18:13:06.1357061Z    |                                               ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&usize>>`
2019-12-10T18:13:06.1357148Z error[E0599]: no method named `method3` found for type `no_method_suggested_traits::Foo` in the current scope
2019-12-10T18:13:06.1357445Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:71:37
2019-12-10T18:13:06.1357496Z    |
2019-12-10T18:13:06.1357496Z    |
2019-12-10T18:13:06.1357546Z LL |     no_method_suggested_traits::Foo.method3();  //~ ERROR no method named
2019-12-10T18:13:06.1357619Z    |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`
2019-12-10T18:13:06.1357655Z 
2019-12-10T18:13:06.1357718Z error[E0599]: no method named `method3` found for type `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>` in the current scope
2019-12-10T18:13:06.1358069Z    |
2019-12-10T18:13:06.1358069Z    |
2019-12-10T18:13:06.1358118Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method3();
2019-12-10T18:13:06.1358182Z    |                                                                       ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>`
2019-12-10T18:13:06.1358380Z error[E0599]: no method named `method3` found for type `no_method_suggested_traits::Bar` in the current scope
2019-12-10T18:13:06.1358682Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:74:40
2019-12-10T18:13:06.1358750Z    |
2019-12-10T18:13:06.1358750Z    |
2019-12-10T18:13:06.1358800Z LL |     no_method_suggested_traits::Bar::X.method3();  //~ ERROR no method named
2019-12-10T18:13:06.1358938Z    |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`
2019-12-10T18:13:06.1358972Z 
2019-12-10T18:13:06.1359042Z error[E0599]: no method named `method3` found for type `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>` in the current scope
2019-12-10T18:13:06.1359398Z    |
2019-12-10T18:13:06.1359398Z    |
2019-12-10T18:13:06.1359462Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method3();
2019-12-10T18:13:06.1359536Z    |                                                                          ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>`
2019-12-10T18:13:06.1359637Z error: aborting due to 24 previous errors
2019-12-10T18:13:06.1359668Z 
2019-12-10T18:13:06.1359934Z For more information about this error, try `rustc --explain E0599`.
2019-12-10T18:13:06.1359979Z 
---
2019-12-10T18:13:06.1360985Z test result: FAILED. 9294 passed; 1 failed; 47 ignored; 0 measured; 0 filtered out
2019-12-10T18:13:06.1361038Z 
2019-12-10T18:13:06.1416309Z 
2019-12-10T18:13:06.1416397Z 
2019-12-10T18:13:06.1418338Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-10T18:13:06.1418613Z 
2019-12-10T18:13:06.1418645Z 
2019-12-10T18:13:06.1419002Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-10T18:13:06.1419071Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-10T18:13:06.1419071Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-10T18:13:06.1419136Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-10T18:13:06.1419208Z Build completed unsuccessfully in 1:04:55
2019-12-10T18:13:06.1426363Z == clock drift check ==
2019-12-10T18:13:06.1443661Z   local time: Tue Dec 10 18:13:06 UTC 2019
2019-12-10T18:13:06.4374968Z   network time: Tue, 10 Dec 2019 18:13:06 GMT
2019-12-10T18:13:06.4378496Z == end clock drift check ==
2019-12-10T18:13:07.1865297Z 
2019-12-10T18:13:07.1975533Z ##[error]Bash exited with code '1'.
2019-12-10T18:13:07.2010322Z ##[section]Starting: Checkout
2019-12-10T18:13:07.2012089Z ==============================================================================
2019-12-10T18:13:07.2012148Z Task         : Get sources
2019-12-10T18:13:07.2012375Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
