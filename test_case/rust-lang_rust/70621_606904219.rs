plain
2020-03-31T21:07:02.7688564Z ========================== Starting Command Output ===========================
2020-03-31T21:07:02.7691514Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1e5e8f7e-776b-4a43-8b51-06ca91dfa34c.sh
2020-03-31T21:07:02.7691838Z 
2020-03-31T21:07:02.7696468Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T21:07:02.7719540Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70621/merge to s
2020-03-31T21:07:02.7723371Z Task         : Get sources
2020-03-31T21:07:02.7723745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T21:07:02.7724108Z Version      : 1.0.0
2020-03-31T21:07:02.7724372Z Author       : Microsoft
---
2020-03-31T21:07:03.7845102Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T21:07:03.7850347Z ##[command]git config gc.auto 0
2020-03-31T21:07:03.7853925Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T21:07:03.7857238Z ##[command]git config --get-all http.proxy
2020-03-31T21:07:03.7863463Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70621/merge:refs/remotes/pull/70621/merge
---
2020-03-31T21:14:25.4369178Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T21:14:26.7987117Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T21:14:28.3381671Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T21:14:28.6598218Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T21:14:37.5779877Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T21:14:39.1444029Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T21:14:43.3119383Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T21:14:47.2213814Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T21:14:56.6512737Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T21:35:59.7089764Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T21:36:01.4377572Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T21:36:03.3888984Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T21:36:04.1528933Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T21:36:15.1886502Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T21:36:17.2743099Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T21:36:22.3345774Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T21:36:27.5680280Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T21:36:38.9580230Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T22:00:39.4774184Z ...........................................i........................................................ 1200/9861
2020-03-31T22:00:41.2796801Z .....................................6    |
2020-03-31T22:00:41.2800277Z 7    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:00:41.2801862Z 8    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:00:41.2803070Z -            `use Lib::TheTrait;`
2020-03-31T22:00:41.2803519Z +            `use rustc_ast::Lib::TheTrait;`
2020-03-31T22:00:41.2804207Z 11 error: aborting due to previous error
2020-03-31T22:00:41.2804516Z 12 
2020-03-31T22:00:41.2804727Z 
2020-03-31T22:00:41.4324799Z F..6    |
2020-03-31T22:00:41.4324799Z F..6    |
2020-03-31T22:00:41.4325289Z 7    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:00:41.4325711Z 8    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:00:41.4326355Z -            `use coherence_inherent_cc_lib::TheTrait;`
2020-03-31T22:00:41.4328275Z +            `use rustc_ast::coherence_inherent_cc_lib::TheTrait;`
2020-03-31T22:00:41.4328864Z 11 error: aborting due to previous error
2020-03-31T22:00:41.4329079Z 12 
2020-03-31T22:00:41.4329184Z 
2020-03-31T22:00:46.5053973Z F........................................................... 1300/9861
2020-03-31T22:00:46.5053973Z F........................................................... 1300/9861
2020-03-31T22:00:50.4308072Z .................................................................................................... 1400/9861
2020-03-31T22:00:56.7082436Z .................................................................................................... 1500/9861
2020-03-31T22:01:01.6265199Z .................................................................................................... 1600/9861
2020-03-31T22:01:07.6722432Z .................................................................................................... 1700/9861
2020-03-31T22:01:11.4893070Z .................................................................................................... 1800/9861
2020-03-31T22:01:20.0137710Z ..............................................................................................i..... 1900/9861
2020-03-31T22:01:27.4742034Z .................................................................................................... 2000/9861
2020-03-31T22:01:33.5547468Z ....................................................................................iiiii........... 2100/9861
2020-03-31T22:01:53.8001449Z .................................................................................................... 2300/9861
2020-03-31T22:01:55.9853147Z .................................................................................................... 2400/9861
2020-03-31T22:01:58.2282858Z .................................................................................................... 2500/9861
2020-03-31T22:02:04.4042658Z .................................................................................................... 2600/9861
---
2020-03-31T22:03:00.8648489Z 9    |
2020-03-31T22:03:00.8649083Z - LL | use foo::Bar;
2020-03-31T22:03:00.8654255Z + LL | use rustc_ast::foo::Bar;
2020-03-31T22:03:00.8654467Z 11    |
2020-03-31T22:03:00.8659887Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8660265Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8665624Z 13    |
2020-03-31T22:03:00.8666378Z - LL | use no_method_suggested_traits::qux::PrivPub;
2020-03-31T22:03:00.8683078Z + LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T22:03:00.8684056Z - LL | use no_method_suggested_traits::Reexported;
2020-03-31T22:03:00.8684409Z + LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T22:03:00.8684681Z 17    |
2020-03-31T22:03:00.8684813Z 18 
2020-03-31T22:03:00.8684813Z 18 
2020-03-31T22:03:00.8685194Z 19 error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&u32>>` in the current scope
2020-03-31T22:03:00.8685841Z 25    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:03:00.8686273Z 26 help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2020-03-31T22:03:00.8686578Z 27    |
2020-03-31T22:03:00.8686938Z - LL | use foo::Bar;
2020-03-31T22:03:00.8686938Z - LL | use foo::Bar;
2020-03-31T22:03:00.8687171Z + LL | use rustc_ast::foo::Bar;
2020-03-31T22:03:00.8687366Z 29    |
2020-03-31T22:03:00.8687813Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8688173Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8688433Z 31    |
2020-03-31T22:03:00.8688868Z - LL | use no_method_suggested_traits::qux::PrivPub;
2020-03-31T22:03:00.8689230Z + LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T22:03:00.8689915Z - LL | use no_method_suggested_traits::Reexported;
2020-03-31T22:03:00.8690268Z + LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T22:03:00.8690521Z 35    |
2020-03-31T22:03:00.8690664Z 36 
---
2020-03-31T22:03:00.8692482Z - LL | use foo::Bar;
2020-03-31T22:03:00.8692724Z + LL | use rustc_ast::foo::Bar;
2020-03-31T22:03:00.8692918Z 47    |
2020-03-31T22:03:00.8693046Z 48 
2020-03-31T22:03:00.8693435Z 49 error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&char>>` in the current scope
2020-03-31T22:03:00.8694033Z 63    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:03:00.8694455Z 64 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:03:00.8694737Z 65    |
2020-03-31T22:03:00.8695063Z - LL | use foo::Bar;
---
2020-03-31T22:03:00.8696167Z 
2020-03-31T22:03:00.8696420Z 75    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:03:00.8696827Z 76 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:03:00.8697124Z 77    |
2020-03-31T22:03:00.8697545Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8697905Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8698308Z 80 
2020-03-31T22:03:00.8698308Z 80 
2020-03-31T22:03:00.8698818Z 81 error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&i32>>` in the current scope
2020-03-31T22:03:00.8699429Z 97    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:03:00.8699833Z 98 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:03:00.8700116Z 99    |
2020-03-31T22:03:00.8700116Z 99    |
2020-03-31T22:03:00.8700704Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8701137Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:03:00.8701555Z 102 
2020-03-31T22:03:00.8701838Z 103 error[E0599]: no method named `method` found for struct `Foo` in the current scope
2020-03-31T22:03:00.8702093Z 
2020-03-31T22:03:01.3032489Z F............ 3300/9861
---
2020-03-31T22:04:38.7584607Z .................................................................................................... 4800/9861
2020-03-31T22:04:39.2499604Z ............7    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:04:39.2501842Z 8 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:04:39.2504785Z 9    |
2020-03-31T22:04:39.2505969Z - LL | use xcrate_issue_43189_b::xcrate_issue_43189_a::A;
2020-03-31T22:04:39.2506465Z + LL | use rustc_ast::xcrate_issue_43189_b::xcrate_issue_43189_a::A;
2020-03-31T22:04:39.2506984Z 12 
2020-03-31T22:04:39.2508801Z 13 error: aborting due to previous error
2020-03-31T22:04:39.2509147Z 
2020-03-31T22:04:45.7308199Z F....................................................................................... 4900/9861
2020-03-31T22:04:45.7308199Z F....................................................................................... 4900/9861
2020-03-31T22:04:50.4071345Z ..........................................................i...............i......................... 5000/9861
2020-03-31T22:04:57.6699795Z .................................................................................................... 5100/9861
2020-03-31T22:05:04.9795228Z .................................................................................................... 5200/9861
2020-03-31T22:05:09.7790266Z ...i................................................................................................ 5300/9861
2020-03-31T22:05:19.6715537Z .........................................................................................ii.ii...... 5400/9861
2020-03-31T22:05:23.7052502Z ..i...i............................................................................................. 5500/9861
2020-03-31T22:05:32.2272991Z ..................................i................................................................. 5700/9861
2020-03-31T22:05:41.5968923Z ....................................................ii....................................i......... 5800/9861
2020-03-31T22:05:48.8771610Z .................................................................................................... 5900/9861
2020-03-31T22:05:53.5325950Z .................................................................................................... 6000/9861
2020-03-31T22:05:53.5325950Z .................................................................................................... 6000/9861
2020-03-31T22:06:02.3316439Z ....................................................................................ii...i..ii...... 6100/9861
2020-03-31T22:06:22.2898906Z .................................................................................................... 6300/9861
2020-03-31T22:06:29.0298365Z .................................................................................................... 6400/9861
2020-03-31T22:06:31.8944144Z .................................................................................................... 6500/9861
2020-03-31T22:06:31.8944144Z .................................................................................................... 6500/9861
2020-03-31T22:06:43.7845640Z ..............i..ii................................................................................. 6600/9861
2020-03-31T22:07:03.3055233Z .................................................................................................... 6800/9861
2020-03-31T22:07:05.3204476Z ..............i..................................................................................... 6900/9861
2020-03-31T22:07:07.3376302Z .................................................................................................... 7000/9861
2020-03-31T22:07:09.4350821Z ....................................................i............................................... 7100/9861
---
2020-03-31T22:09:08.0276798Z 
2020-03-31T22:09:08.3405846Z F.6    |
2020-03-31T22:09:08.3406228Z 7    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:09:08.3406691Z 8    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:09:08.3412973Z -            `use crate::foo::foobar::Foobar;`
2020-03-31T22:09:08.3414206Z +            `use rustc_ast::crate::foo::foobar::Foobar;`
2020-03-31T22:09:08.3417768Z 11 error[E0599]: no method named `bar` found for type `u32` in the current scope
2020-03-31T22:09:08.3418734Z 12   --> $DIR/trait-import-suggestions.rs:28:7
2020-03-31T22:09:08.3419274Z 
2020-03-31T22:09:08.3425748Z 17    = help: items from traits can only be used if the trait is in scope
---
2020-03-31T22:09:08.3465960Z 39    |
2020-03-31T22:09:08.3466109Z 40 
2020-03-31T22:09:08.3466311Z 41 error: aborting due to 4 previous errors
2020-03-31T22:09:08.3466491Z 
2020-03-31T22:09:12.5157739Z F............................iiiiiiiiii.i........................... 8200/9861
2020-03-31T22:09:18.4247028Z 8 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:09:18.4247335Z 9    |
2020-03-31T22:09:18.4253411Z - LL | use foo::T;
2020-03-31T22:09:18.4253702Z + LL | use rustc_ast::foo::T;
---
2020-03-31T22:11:45.2228656Z 
2020-03-31T22:11:45.2228755Z 
2020-03-31T22:11:45.2229113Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2230008Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent/coherence_inherent.stderr
2020-03-31T22:11:45.2230832Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2231602Z To only update this specific test, also pass `--test-args coherence/coherence_inherent.rs`
2020-03-31T22:11:45.2232066Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2232620Z status: exit code: 1
2020-03-31T22:11:45.2232620Z status: exit code: 1
2020-03-31T22:11:45.2235023Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence_inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent/auxiliary"
2020-03-31T22:11:45.2237064Z ------------------------------------------
2020-03-31T22:11:45.2237242Z 
2020-03-31T22:11:45.2237772Z ------------------------------------------
2020-03-31T22:11:45.2237990Z stderr:
2020-03-31T22:11:45.2237990Z stderr:
2020-03-31T22:11:45.2238508Z ------------------------------------------
2020-03-31T22:11:45.2238925Z error[E0599]: no method named `the_fn` found for reference `&Lib::TheStruct` in the current scope
2020-03-31T22:11:45.2240111Z    |
2020-03-31T22:11:45.2240111Z    |
2020-03-31T22:11:45.2240299Z LL |         s.the_fn();
2020-03-31T22:11:45.2240701Z    |           ^^^^^^ method not found in `&Lib::TheStruct`
2020-03-31T22:11:45.2241235Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2241779Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2241779Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2242268Z            `use rustc_ast::Lib::TheTrait;`
2020-03-31T22:11:45.2242664Z error: aborting due to previous error
2020-03-31T22:11:45.2242828Z 
2020-03-31T22:11:45.2243425Z For more information about this error, try `rustc --explain E0599`.
2020-03-31T22:11:45.2243831Z 
---
2020-03-31T22:11:45.2245149Z diff of stderr:
2020-03-31T22:11:45.2245271Z 
2020-03-31T22:11:45.2245368Z 
2020-03-31T22:11:45.2245590Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2246360Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent_cc/coherence_inherent_cc.stderr
2020-03-31T22:11:45.2247029Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2247658Z To only update this specific test, also pass `--test-args coherence/coherence_inherent_cc.rs`
2020-03-31T22:11:45.2248109Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2248359Z status: exit code: 1
2020-03-31T22:11:45.2248359Z status: exit code: 1
2020-03-31T22:11:45.2250347Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence_inherent_cc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent_cc" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent_cc/auxiliary"
2020-03-31T22:11:45.2252730Z ------------------------------------------
2020-03-31T22:11:45.2252907Z 
2020-03-31T22:11:45.2253291Z ------------------------------------------
2020-03-31T22:11:45.2253491Z stderr:
2020-03-31T22:11:45.2253491Z stderr:
2020-03-31T22:11:45.2253865Z ------------------------------------------
2020-03-31T22:11:45.2254321Z error[E0599]: no method named `the_fn` found for reference `&coherence_inherent_cc_lib::TheStruct` in the current scope
2020-03-31T22:11:45.2255287Z    |
2020-03-31T22:11:45.2255287Z    |
2020-03-31T22:11:45.2255470Z LL |         s.the_fn();
2020-03-31T22:11:45.2255802Z    |           ^^^^^^ method not found in `&coherence_inherent_cc_lib::TheStruct`
2020-03-31T22:11:45.2256361Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2256770Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2256770Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2257180Z            `use rustc_ast::coherence_inherent_cc_lib::TheTrait;`
2020-03-31T22:11:45.2257612Z error: aborting due to previous error
2020-03-31T22:11:45.2257778Z 
2020-03-31T22:11:45.2258213Z For more information about this error, try `rustc --explain E0599`.
2020-03-31T22:11:45.2258441Z 
---
2020-03-31T22:11:45.2259849Z 
2020-03-31T22:11:45.2259947Z 
2020-03-31T22:11:45.2260151Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2261068Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/no_implicit_prelude.stderr
2020-03-31T22:11:45.2261728Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2262339Z To only update this specific test, also pass `--test-args hygiene/no_implicit_prelude.rs`
2020-03-31T22:11:45.2262781Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2263016Z status: exit code: 1
2020-03-31T22:11:45.2263016Z status: exit code: 1
2020-03-31T22:11:45.2264990Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/no_implicit_prelude.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/auxiliary"
2020-03-31T22:11:45.2266751Z ------------------------------------------
2020-03-31T22:11:45.2266925Z 
2020-03-31T22:11:45.2267297Z ------------------------------------------
2020-03-31T22:11:45.2267517Z stderr:
---
2020-03-31T22:11:45.2270569Z 
2020-03-31T22:11:45.2270827Z error[E0433]: failed to resolve: use of undeclared type or module `Vec`
2020-03-31T22:11:45.2271404Z   --> /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:11:9
2020-03-31T22:11:45.2271662Z    |
2020-03-31T22:11:45.2271856Z LL |     fn f() { ::bar::m!(); }
2020-03-31T22:11:45.2272558Z ...
2020-03-31T22:11:45.2272783Z LL |         Vec::new(); //~ ERROR failed to resolve
2020-03-31T22:11:45.2273096Z    |         ^^^ use of undeclared type or module `Vec`
2020-03-31T22:11:45.2273325Z    |
2020-03-31T22:11:45.2273325Z    |
2020-03-31T22:11:45.2273876Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-31T22:11:45.2274164Z 
2020-03-31T22:11:45.2274431Z error[E0599]: no method named `clone` found for unit type `()` in the current scope
2020-03-31T22:11:45.2275035Z   --> /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:12:12
2020-03-31T22:11:45.2275280Z    |
2020-03-31T22:11:45.2275473Z LL |     fn f() { ::bar::m!(); }
2020-03-31T22:11:45.2276168Z ...
2020-03-31T22:11:45.2276168Z ...
2020-03-31T22:11:45.2276406Z LL |         ().clone() //~ ERROR no method named `clone` found
2020-03-31T22:11:45.2276729Z    |            ^^^^^ method not found in `()`
2020-03-31T22:11:45.2277180Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2277603Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2277982Z            `use rustc_ast::std::clone::Clone;`
2020-03-31T22:11:45.2278624Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
2020-03-31T22:11:45.2281526Z diff of stderr:
2020-03-31T22:11:45.2281649Z 
2020-03-31T22:11:45.2281761Z 
2020-03-31T22:11:45.2281967Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2282609Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items/trait_items.stderr
2020-03-31T22:11:45.2283308Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2283886Z To only update this specific test, also pass `--test-args hygiene/trait_items.rs`
2020-03-31T22:11:45.2284312Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2284567Z status: exit code: 1
2020-03-31T22:11:45.2284567Z status: exit code: 1
2020-03-31T22:11:45.2286508Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/trait_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items/auxiliary"
2020-03-31T22:11:45.2288086Z ------------------------------------------
2020-03-31T22:11:45.2288257Z 
2020-03-31T22:11:45.2288636Z ------------------------------------------
2020-03-31T22:11:45.2288835Z stderr:
2020-03-31T22:11:45.2288835Z stderr:
2020-03-31T22:11:45.2289205Z ------------------------------------------
2020-03-31T22:11:45.2289555Z error[E0599]: no method named `f` found for unit type `()` in the current scope
2020-03-31T22:11:45.2290117Z   --> /checkout/src/test/ui/hygiene/trait_items.rs:17:24
2020-03-31T22:11:45.2290551Z    |
2020-03-31T22:11:45.2296235Z LL |     fn f() { ::baz::m!(); }
2020-03-31T22:11:45.2297375Z ...
2020-03-31T22:11:45.2297375Z ...
2020-03-31T22:11:45.2297645Z LL |     pub macro m() { ().f() } //~ ERROR no method named `f` found
2020-03-31T22:11:45.2297976Z    |                        ^ method not found in `()`
2020-03-31T22:11:45.2298456Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2298876Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2299233Z            `use rustc_ast::foo::T;`
2020-03-31T22:11:45.2299876Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
2020-03-31T22:11:45.2314110Z diff of stderr:
2020-03-31T22:11:45.2314241Z 
2020-03-31T22:11:45.2314372Z 
2020-03-31T22:11:45.2314581Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2315390Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
2020-03-31T22:11:45.2316089Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2316719Z To only update this specific test, also pass `--test-args impl-trait/no-method-suggested-traits.rs`
2020-03-31T22:11:45.2317198Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2317438Z status: exit code: 1
2020-03-31T22:11:45.2317438Z status: exit code: 1
2020-03-31T22:11:45.2319479Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/auxiliary"
2020-03-31T22:11:45.2321290Z ------------------------------------------
2020-03-31T22:11:45.2321480Z 
2020-03-31T22:11:45.2321847Z ------------------------------------------
2020-03-31T22:11:45.2322048Z stderr:
2020-03-31T22:11:45.2322048Z stderr:
2020-03-31T22:11:45.2322438Z ------------------------------------------
2020-03-31T22:11:45.2322852Z error[E0599]: no method named `method` found for type `u32` in the current scope
2020-03-31T22:11:45.2323487Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:23:10
2020-03-31T22:11:45.2323775Z    |
2020-03-31T22:11:45.2323940Z LL |     1u32.method();
2020-03-31T22:11:45.2324391Z    |
2020-03-31T22:11:45.2324642Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2325067Z help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2020-03-31T22:11:45.2325374Z    |
2020-03-31T22:11:45.2325374Z    |
2020-03-31T22:11:45.2325562Z LL | use rustc_ast::foo::Bar;
2020-03-31T22:11:45.2325748Z    |
2020-03-31T22:11:45.2326018Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:11:45.2326269Z    |
2020-03-31T22:11:45.2326518Z LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T22:11:45.2327032Z LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T22:11:45.2327276Z    |
2020-03-31T22:11:45.2327380Z 
2020-03-31T22:11:45.2327380Z 
2020-03-31T22:11:45.2327759Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&u32>>` in the current scope
2020-03-31T22:11:45.2328750Z    |
2020-03-31T22:11:45.2328750Z    |
2020-03-31T22:11:45.2329009Z LL |     std::rc::Rc::new(&mut Box::new(&1u32)).method();
2020-03-31T22:11:45.2329502Z    |                                            ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&u32>>`
2020-03-31T22:11:45.2330130Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2343044Z help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2020-03-31T22:11:45.2343401Z    |
2020-03-31T22:11:45.2343596Z LL | use rustc_ast::foo::Bar;
2020-03-31T22:11:45.2343596Z LL | use rustc_ast::foo::Bar;
2020-03-31T22:11:45.2343785Z    |
2020-03-31T22:11:45.2344071Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:11:45.2344322Z    |
2020-03-31T22:11:45.2344577Z LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T22:11:45.2345090Z LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T22:11:45.2345333Z    |
2020-03-31T22:11:45.2345453Z 
2020-03-31T22:11:45.2345718Z error[E0599]: no method named `method` found for type `char` in the current scope
2020-03-31T22:11:45.2345718Z error[E0599]: no method named `method` found for type `char` in the current scope
2020-03-31T22:11:45.2346529Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:30:9
2020-03-31T22:11:45.2346816Z    |
2020-03-31T22:11:45.2347144Z LL |     'a'.method();
2020-03-31T22:11:45.2347582Z    |
2020-03-31T22:11:45.2347850Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2348246Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2348523Z    |
2020-03-31T22:11:45.2348523Z    |
2020-03-31T22:11:45.2348727Z LL | use rustc_ast::foo::Bar;
2020-03-31T22:11:45.2348911Z    |
2020-03-31T22:11:45.2349016Z 
2020-03-31T22:11:45.2349401Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&char>>` in the current scope
2020-03-31T22:11:45.2350389Z    |
2020-03-31T22:11:45.2350738Z LL |         fn method(&self) {}
2020-03-31T22:11:45.2351112Z    |            ------
2020-03-31T22:11:45.2351297Z    |            |
2020-03-31T22:11:45.2351297Z    |            |
2020-03-31T22:11:45.2351678Z    |            the method is available for `std::boxed::Box<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T22:11:45.2352274Z    |            the method is available for `std::pin::Pin<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T22:11:45.2352849Z    |            the method is available for `std::sync::Arc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T22:11:45.2353496Z    |            the method is available for `std::rc::Rc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T22:11:45.2353842Z ...
2020-03-31T22:11:45.2354282Z LL |     std::rc::Rc::new(&mut Box::new(&'a')).method();
2020-03-31T22:11:45.2354789Z    |                                           ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&char>>`
2020-03-31T22:11:45.2355719Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2356149Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2356545Z    |
2020-03-31T22:11:45.2356733Z LL | use rustc_ast::foo::Bar;
2020-03-31T22:11:45.2356733Z LL | use rustc_ast::foo::Bar;
2020-03-31T22:11:45.2356935Z    |
2020-03-31T22:11:45.2357040Z 
2020-03-31T22:11:45.2357558Z error[E0599]: no method named `method` found for type `i32` in the current scope
2020-03-31T22:11:45.2358250Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:35:10
2020-03-31T22:11:45.2358542Z    |
2020-03-31T22:11:45.2358705Z LL |     1i32.method();
2020-03-31T22:11:45.2359161Z    |
2020-03-31T22:11:45.2359413Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2359809Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2360096Z    |
2020-03-31T22:11:45.2360096Z    |
2020-03-31T22:11:45.2360350Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:11:45.2360710Z 
2020-03-31T22:11:45.2360710Z 
2020-03-31T22:11:45.2361096Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&i32>>` in the current scope
2020-03-31T22:11:45.2362228Z    |
2020-03-31T22:11:45.2362228Z    |
2020-03-31T22:11:45.2362488Z LL |     std::rc::Rc::new(&mut Box::new(&1i32)).method();
2020-03-31T22:11:45.2362979Z    |                                            ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&i32>>`
2020-03-31T22:11:45.2364013Z   ::: /checkout/src/test/ui/impl-trait/auxiliary/no_method_suggested_traits.rs:8:12
2020-03-31T22:11:45.2364300Z    |
2020-03-31T22:11:45.2364476Z LL |         fn method(&self) {}
2020-03-31T22:11:45.2364872Z    |            ------
2020-03-31T22:11:45.2364872Z    |            ------
2020-03-31T22:11:45.2365058Z    |            |
2020-03-31T22:11:45.2365435Z    |            the method is available for `std::boxed::Box<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T22:11:45.2368440Z    |            the method is available for `std::pin::Pin<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T22:11:45.2369025Z    |            the method is available for `std::sync::Arc<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T22:11:45.2369748Z    |            the method is available for `std::rc::Rc<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T22:11:45.2370374Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2370770Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2371056Z    |
2020-03-31T22:11:45.2371056Z    |
2020-03-31T22:11:45.2371308Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T22:11:45.2371682Z 
2020-03-31T22:11:45.2371950Z error[E0599]: no method named `method` found for struct `Foo` in the current scope
2020-03-31T22:11:45.2372840Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:40:9
2020-03-31T22:11:45.2373123Z    |
2020-03-31T22:11:45.2373123Z    |
2020-03-31T22:11:45.2373275Z LL | struct Foo;
2020-03-31T22:11:45.2373702Z    | ----------- method `method` not found for this
2020-03-31T22:11:45.2374195Z ...
2020-03-31T22:11:45.2374363Z LL |     Foo.method();
2020-03-31T22:11:45.2375482Z    |
2020-03-31T22:11:45.2375880Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2375880Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2376332Z    = note: the following traits define an item `method`, perhaps you need to implement one of them:
2020-03-31T22:11:45.2376691Z            candidate #1: `foo::Bar`
2020-03-31T22:11:45.2377026Z            candidate #2: `no_method_suggested_traits::foo::PubPub`
2020-03-31T22:11:45.2377405Z            candidate #3: `no_method_suggested_traits::qux::PrivPub`
2020-03-31T22:11:45.2377779Z            candidate #4: `no_method_suggested_traits::Reexported`
2020-03-31T22:11:45.2378025Z 
2020-03-31T22:11:45.2378393Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&Foo>>` in the current scope
2020-03-31T22:11:45.2379661Z    |
2020-03-31T22:11:45.2379661Z    |
2020-03-31T22:11:45.2379904Z LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method();
2020-03-31T22:11:45.2380392Z    |                                           ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Foo>>`
2020-03-31T22:11:45.2381324Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2381324Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2387372Z    = note: the following traits define an item `method`, perhaps you need to implement one of them:
2020-03-31T22:11:45.2387807Z            candidate #1: `foo::Bar`
2020-03-31T22:11:45.2388125Z            candidate #2: `no_method_suggested_traits::foo::PubPub`
2020-03-31T22:11:45.2388518Z            candidate #3: `no_method_suggested_traits::qux::PrivPub`
2020-03-31T22:11:45.2388911Z            candidate #4: `no_method_suggested_traits::Reexported`
2020-03-31T22:11:45.2389404Z error[E0599]: no method named `method2` found for type `u64` in the current scope
2020-03-31T22:11:45.2390234Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:45:10
2020-03-31T22:11:45.2390502Z    |
2020-03-31T22:11:45.2390502Z    |
2020-03-31T22:11:45.2390670Z LL |     1u64.method2();
2020-03-31T22:11:45.2391138Z    |
2020-03-31T22:11:45.2391415Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2391415Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2391842Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T22:11:45.2392739Z    |
2020-03-31T22:11:45.2392909Z LL |     pub trait Bar {
2020-03-31T22:11:45.2393279Z    |     ^^^^^^^^^^^^^
2020-03-31T22:11:45.2393415Z 
2020-03-31T22:11:45.2393415Z 
2020-03-31T22:11:45.2393785Z error[E0599]: no method named `method2` found for struct `std::rc::Rc<&mut std::boxed::Box<&u64>>` in the current scope
2020-03-31T22:11:45.2395449Z    |
2020-03-31T22:11:45.2395449Z    |
2020-03-31T22:11:45.2395696Z LL |     std::rc::Rc::new(&mut Box::new(&1u64)).method2();
2020-03-31T22:11:45.2396217Z    |                                            ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&u64>>`
2020-03-31T22:11:45.2396873Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2396873Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2397297Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T22:11:45.2398733Z    |
2020-03-31T22:11:45.2398920Z LL |     pub trait Bar {
2020-03-31T22:11:45.2399118Z    |     ^^^^^^^^^^^^^
2020-03-31T22:11:45.2399256Z 
2020-03-31T22:11:45.2399256Z 
2020-03-31T22:11:45.2399789Z error[E0599]: no method named `method2` found for struct `no_method_suggested_traits::Foo` in the current scope
2020-03-31T22:11:45.2400618Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:50:37
2020-03-31T22:11:45.2400886Z    |
2020-03-31T22:11:45.2401116Z LL |     no_method_suggested_traits::Foo.method2();
2020-03-31T22:11:45.2401650Z    |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`
2020-03-31T22:11:45.2402272Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2402272Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2402692Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T22:11:45.2403568Z    |
2020-03-31T22:11:45.2403751Z LL |     pub trait Bar {
2020-03-31T22:11:45.2403947Z    |     ^^^^^^^^^^^^^
2020-03-31T22:11:45.2404083Z 
2020-03-31T22:11:45.2404083Z 
2020-03-31T22:11:45.2404945Z error[E0599]: no method named `method2` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>` in the current scope
2020-03-31T22:11:45.2406033Z    |
2020-03-31T22:11:45.2406033Z    |
2020-03-31T22:11:45.2406364Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method2();
2020-03-31T22:11:45.2407122Z    |                                                                       ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>`
2020-03-31T22:11:45.2408930Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2408930Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2409368Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T22:11:45.2410302Z    |
2020-03-31T22:11:45.2410467Z LL |     pub trait Bar {
2020-03-31T22:11:45.2410664Z    |     ^^^^^^^^^^^^^
2020-03-31T22:11:45.2410815Z 
2020-03-31T22:11:45.2410815Z 
2020-03-31T22:11:45.2411164Z error[E0599]: no method named `method2` found for enum `no_method_suggested_traits::Bar` in the current scope
2020-03-31T22:11:45.2411861Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:54:40
2020-03-31T22:11:45.2412144Z    |
2020-03-31T22:11:45.2412385Z LL |     no_method_suggested_traits::Bar::X.method2();
2020-03-31T22:11:45.2412838Z    |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`
2020-03-31T22:11:45.2413479Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2413479Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2413885Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T22:11:45.2414763Z    |
2020-03-31T22:11:45.2414926Z LL |     pub trait Bar {
2020-03-31T22:11:45.2415137Z    |     ^^^^^^^^^^^^^
2020-03-31T22:11:45.2415271Z 
2020-03-31T22:11:45.2415271Z 
2020-03-31T22:11:45.2415853Z error[E0599]: no method named `method2` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>` in the current scope
2020-03-31T22:11:45.2416938Z    |
2020-03-31T22:11:45.2416938Z    |
2020-03-31T22:11:45.2417254Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method2();
2020-03-31T22:11:45.2417941Z    |                                                                          ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>`
2020-03-31T22:11:45.2418734Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2418734Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2419394Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T22:11:45.2420283Z    |
2020-03-31T22:11:45.2420446Z LL |     pub trait Bar {
2020-03-31T22:11:45.2421494Z    |     ^^^^^^^^^^^^^
2020-03-31T22:11:45.2421729Z 
2020-03-31T22:11:45.2421729Z 
2020-03-31T22:11:45.2421997Z error[E0599]: no method named `method3` found for struct `Foo` in the current scope
2020-03-31T22:11:45.2422767Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:59:9
2020-03-31T22:11:45.2423038Z    |
2020-03-31T22:11:45.2423190Z LL | struct Foo;
2020-03-31T22:11:45.2423639Z    | ----------- method `method3` not found for this
2020-03-31T22:11:45.2423851Z ...
2020-03-31T22:11:45.2424015Z LL |     Foo.method3();
2020-03-31T22:11:45.2424605Z    |
2020-03-31T22:11:45.2424890Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2424890Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2425899Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T22:11:45.2426340Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T22:11:45.2426569Z 
2020-03-31T22:11:45.2426940Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&Foo>>` in the current scope
2020-03-31T22:11:45.2428127Z    |
2020-03-31T22:11:45.2428127Z    |
2020-03-31T22:11:45.2428371Z LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method3();
2020-03-31T22:11:45.2428880Z    |                                           ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Foo>>`
2020-03-31T22:11:45.2429530Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2429530Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2429984Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T22:11:45.2430400Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T22:11:45.2430909Z error[E0599]: no method named `method3` found for enum `Bar` in the current scope
2020-03-31T22:11:45.2431526Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:63:12
2020-03-31T22:11:45.2431793Z    |
2020-03-31T22:11:45.2431793Z    |
2020-03-31T22:11:45.2431969Z LL | enum Bar { X }
2020-03-31T22:11:45.2432519Z    | -------- method `method3` not found for this
2020-03-31T22:11:45.2432726Z ...
2020-03-31T22:11:45.2432902Z LL |     Bar::X.method3();
2020-03-31T22:11:45.2433379Z    |
2020-03-31T22:11:45.2433657Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2433657Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2434102Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T22:11:45.2434518Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T22:11:45.2434746Z 
2020-03-31T22:11:45.2435126Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&Bar>>` in the current scope
2020-03-31T22:11:45.2436126Z    |
2020-03-31T22:11:45.2436126Z    |
2020-03-31T22:11:45.2436399Z LL |     std::rc::Rc::new(&mut Box::new(&Bar::X)).method3();
2020-03-31T22:11:45.2437042Z    |                                              ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Bar>>`
2020-03-31T22:11:45.2437715Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2437715Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2438146Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T22:11:45.2438555Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T22:11:45.2439159Z error[E0599]: no method named `method3` found for type `usize` in the current scope
2020-03-31T22:11:45.2439928Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:69:13
2020-03-31T22:11:45.2440209Z    |
2020-03-31T22:11:45.2440209Z    |
2020-03-31T22:11:45.2440428Z LL |     1_usize.method3(); //~ ERROR no method named
2020-03-31T22:11:45.2440912Z 
2020-03-31T22:11:45.2440912Z 
2020-03-31T22:11:45.2441362Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&usize>>` in the current scope
2020-03-31T22:11:45.2442482Z    |
2020-03-31T22:11:45.2442482Z    |
2020-03-31T22:11:45.2442794Z LL |     std::rc::Rc::new(&mut Box::new(&1_usize)).method3(); //~ ERROR no method named
2020-03-31T22:11:45.2443346Z    |                                               ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&usize>>`
2020-03-31T22:11:45.2444075Z error[E0599]: no method named `method3` found for struct `no_method_suggested_traits::Foo` in the current scope
2020-03-31T22:11:45.2444783Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:71:37
2020-03-31T22:11:45.2445049Z    |
2020-03-31T22:11:45.2445049Z    |
2020-03-31T22:11:45.2445343Z LL |     no_method_suggested_traits::Foo.method3();  //~ ERROR no method named
2020-03-31T22:11:45.2445838Z    |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`
2020-03-31T22:11:45.2446155Z 
2020-03-31T22:11:45.2446600Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>` in the current scope
2020-03-31T22:11:45.2447774Z    |
2020-03-31T22:11:45.2447774Z    |
2020-03-31T22:11:45.2448096Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method3();
2020-03-31T22:11:45.2448773Z    |                                                                       ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>`
2020-03-31T22:11:45.2449600Z error[E0599]: no method named `method3` found for enum `no_method_suggested_traits::Bar` in the current scope
2020-03-31T22:11:45.2450304Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:74:40
2020-03-31T22:11:45.2450571Z    |
2020-03-31T22:11:45.2450571Z    |
2020-03-31T22:11:45.2450876Z LL |     no_method_suggested_traits::Bar::X.method3();  //~ ERROR no method named
2020-03-31T22:11:45.2451379Z    |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`
2020-03-31T22:11:45.2451703Z 
2020-03-31T22:11:45.2452129Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>` in the current scope
2020-03-31T22:11:45.2453193Z    |
2020-03-31T22:11:45.2453193Z    |
2020-03-31T22:11:45.2453507Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method3();
2020-03-31T22:11:45.2454211Z    |                                                                          ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>`
2020-03-31T22:11:45.2454877Z error: aborting due to 24 previous errors
2020-03-31T22:11:45.2455065Z 
2020-03-31T22:11:45.2455503Z For more information about this error, try `rustc --explain E0599`.
2020-03-31T22:11:45.2455717Z 
---
2020-03-31T22:11:45.2457106Z 
2020-03-31T22:11:45.2457205Z 
2020-03-31T22:11:45.2457411Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2458123Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10465/issue-10465.stderr
2020-03-31T22:11:45.2458762Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2459336Z To only update this specific test, also pass `--test-args issues/issue-10465.rs`
2020-03-31T22:11:45.2459779Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2460015Z status: exit code: 1
2020-03-31T22:11:45.2460015Z status: exit code: 1
2020-03-31T22:11:45.2462247Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10465.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10465" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10465/auxiliary"
2020-03-31T22:11:45.2463835Z ------------------------------------------
2020-03-31T22:11:45.2464027Z 
2020-03-31T22:11:45.2464394Z ------------------------------------------
2020-03-31T22:11:45.2464595Z stderr:
2020-03-31T22:11:45.2464595Z stderr:
2020-03-31T22:11:45.2464982Z ------------------------------------------
2020-03-31T22:11:45.2465353Z error[E0599]: no method named `foo` found for reference `&b::B` in the current scope
2020-03-31T22:11:45.2466195Z    |
2020-03-31T22:11:45.2466195Z    |
2020-03-31T22:11:45.2466460Z LL |             b.foo(); //~ ERROR: no method named `foo` found
2020-03-31T22:11:45.2467232Z    |               ^^^ method not found in `&b::B`
2020-03-31T22:11:45.2467735Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2468144Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2468510Z            `use rustc_ast::a::A;`
2020-03-31T22:11:45.2468682Z 
---
2020-03-31T22:11:45.2471099Z 
2020-03-31T22:11:45.2471196Z 
2020-03-31T22:11:45.2471419Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2472065Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35976/issue-35976.stderr
2020-03-31T22:11:45.2472700Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2473286Z To only update this specific test, also pass `--test-args issues/issue-35976.rs`
2020-03-31T22:11:45.2473712Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2473964Z status: exit code: 1
2020-03-31T22:11:45.2473964Z status: exit code: 1
2020-03-31T22:11:45.2475865Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35976.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35976" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35976/auxiliary"
2020-03-31T22:11:45.2477414Z ------------------------------------------
2020-03-31T22:11:45.2477667Z 
2020-03-31T22:11:45.2478054Z ------------------------------------------
2020-03-31T22:11:45.2478255Z stderr:
2020-03-31T22:11:45.2478255Z stderr:
2020-03-31T22:11:45.2478623Z ------------------------------------------
2020-03-31T22:11:45.2478936Z error: the `wait` method cannot be invoked on a trait object
2020-03-31T22:11:45.2479457Z   --> /checkout/src/test/ui/issues/issue-35976.rs:14:9
2020-03-31T22:11:45.2479686Z    |
2020-03-31T22:11:45.2479925Z LL |         fn wait(&self) where Self: Sized;
2020-03-31T22:11:45.2480783Z ...
2020-03-31T22:11:45.2480957Z LL |     arg.wait();
2020-03-31T22:11:45.2481138Z    |         ^^^^
2020-03-31T22:11:45.2481289Z    |
---
2020-03-31T22:11:45.2484225Z 
2020-03-31T22:11:45.2484323Z 
2020-03-31T22:11:45.2484527Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2485191Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/issue-39175.stderr
2020-03-31T22:11:45.2485806Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2486400Z To only update this specific test, also pass `--test-args issues/issue-39175.rs`
2020-03-31T22:11:45.2486827Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2487068Z status: exit code: 1
2020-03-31T22:11:45.2487068Z status: exit code: 1
2020-03-31T22:11:45.2488976Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39175.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/auxiliary"
2020-03-31T22:11:45.2490537Z ------------------------------------------
2020-03-31T22:11:45.2490711Z 
2020-03-31T22:11:45.2491073Z ------------------------------------------
2020-03-31T22:11:45.2491288Z stderr:
2020-03-31T22:11:45.2491288Z stderr:
2020-03-31T22:11:45.2491659Z ------------------------------------------
2020-03-31T22:11:45.2492084Z error[E0599]: no method named `exec` found for mutable reference `&mut std::process::Command` in the current scope
2020-03-31T22:11:45.2492756Z   --> /checkout/src/test/ui/issues/issue-39175.rs:15:39
2020-03-31T22:11:45.2492986Z    |
2020-03-31T22:11:45.2493207Z LL |     Command::new("echo").arg("hello").exec();
2020-03-31T22:11:45.2493645Z    |                                       ^^^^ method not found in `&mut std::process::Command`
2020-03-31T22:11:45.2494225Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2494637Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2494911Z    |
2020-03-31T22:11:45.2495152Z LL | use rustc_ast::std::os::unix::process::CommandExt;
---
2020-03-31T22:11:45.2497973Z 
2020-03-31T22:11:45.2498087Z 
2020-03-31T22:11:45.2498295Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2498982Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/issue-43189.stderr
2020-03-31T22:11:45.2499626Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2500202Z To only update this specific test, also pass `--test-args issues/issue-43189.rs`
2020-03-31T22:11:45.2500794Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2501047Z status: exit code: 1
2020-03-31T22:11:45.2501047Z status: exit code: 1
2020-03-31T22:11:45.2502932Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43189.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/auxiliary"
2020-03-31T22:11:45.2504689Z ------------------------------------------
2020-03-31T22:11:45.2504868Z 
2020-03-31T22:11:45.2505274Z ------------------------------------------
2020-03-31T22:11:45.2505476Z stderr:
2020-03-31T22:11:45.2505476Z stderr:
2020-03-31T22:11:45.2505845Z ------------------------------------------
2020-03-31T22:11:45.2506196Z error[E0599]: no method named `a` found for unit type `()` in the current scope
2020-03-31T22:11:45.2506754Z   --> /checkout/src/test/ui/issues/issue-43189.rs:10:8
2020-03-31T22:11:45.2506988Z    |
2020-03-31T22:11:45.2507161Z LL |     ().a();
2020-03-31T22:11:45.2507371Z    |        ^ method not found in `()`
2020-03-31T22:11:45.2507825Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2508222Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2508496Z    |
2020-03-31T22:11:45.2508496Z    |
2020-03-31T22:11:45.2508825Z LL | use rustc_ast::xcrate_issue_43189_b::xcrate_issue_43189_a::A;
2020-03-31T22:11:45.2509359Z 
2020-03-31T22:11:45.2509542Z error: aborting due to previous error
2020-03-31T22:11:45.2509723Z 
2020-03-31T22:11:45.2510173Z For more information about this error, try `rustc --explain E0599`.
---
2020-03-31T22:11:45.2511691Z diff of fixed:
2020-03-31T22:11:45.2511829Z 
2020-03-31T22:11:45.2511928Z 
2020-03-31T22:11:45.2512131Z The actual fixed differed from the expected fixed.
2020-03-31T22:11:45.2512827Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.fixed
2020-03-31T22:11:45.2513474Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2514085Z To only update this specific test, also pass `--test-args rust-2018/remove-extern-crate.rs`
2020-03-31T22:11:45.2514554Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2514789Z status: exit code: 0
2020-03-31T22:11:45.2514789Z status: exit code: 0
2020-03-31T22:11:45.2516862Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/remove-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "--extern" "remove_extern_crate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/auxiliary"
2020-03-31T22:11:45.2518618Z ------------------------------------------
2020-03-31T22:11:45.2518842Z 
2020-03-31T22:11:45.2519213Z ------------------------------------------
2020-03-31T22:11:45.2519432Z stderr:
---
2020-03-31T22:11:45.2526872Z 
2020-03-31T22:11:45.2526985Z 
2020-03-31T22:11:45.2527192Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2527914Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/trait-import-suggestions.stderr
2020-03-31T22:11:45.2528596Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2529213Z To only update this specific test, also pass `--test-args rust-2018/trait-import-suggestions.rs`
2020-03-31T22:11:45.2529683Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2529921Z status: exit code: 1
2020-03-31T22:11:45.2529921Z status: exit code: 1
2020-03-31T22:11:45.2532083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "--extern" "trait-import-suggestions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/auxiliary"
2020-03-31T22:11:45.2533822Z ------------------------------------------
2020-03-31T22:11:45.2534009Z 
2020-03-31T22:11:45.2534374Z ------------------------------------------
2020-03-31T22:11:45.2534576Z stderr:
2020-03-31T22:11:45.2534576Z stderr:
2020-03-31T22:11:45.2534960Z ------------------------------------------
2020-03-31T22:11:45.2535345Z error[E0599]: no method named `foobar` found for type `u32` in the current scope
2020-03-31T22:11:45.2535965Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:22:11
2020-03-31T22:11:45.2536245Z    |
2020-03-31T22:11:45.2536473Z LL |         x.foobar(); //~ ERROR no method named `foobar`
2020-03-31T22:11:45.2536993Z    |
2020-03-31T22:11:45.2537246Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2537696Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2537696Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2538110Z            `use rustc_ast::crate::foo::foobar::Foobar;`
2020-03-31T22:11:45.2538585Z error[E0599]: no method named `bar` found for type `u32` in the current scope
2020-03-31T22:11:45.2539208Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:28:7
2020-03-31T22:11:45.2539467Z    |
2020-03-31T22:11:45.2539683Z LL |     x.bar(); //~ ERROR no method named `bar`
---
2020-03-31T22:11:45.2541780Z 
2020-03-31T22:11:45.2542042Z error[E0599]: no method named `baz` found for type `u32` in the current scope
2020-03-31T22:11:45.2542657Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:29:7
2020-03-31T22:11:45.2542933Z    |
2020-03-31T22:11:45.2543144Z LL |     x.baz(); //~ ERROR no method named `baz`
2020-03-31T22:11:45.2543582Z 
2020-03-31T22:11:45.2543897Z error[E0599]: no function or associated item named `from_str` found for type `u32` in the current scope
2020-03-31T22:11:45.2544550Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:30:18
2020-03-31T22:11:45.2544815Z    |
2020-03-31T22:11:45.2544815Z    |
2020-03-31T22:11:45.2545133Z LL |     let y = u32::from_str("33"); //~ ERROR no function or associated item named `from_str`
2020-03-31T22:11:45.2545559Z    |                  ^^^^^^^^ function or associated item not found in `u32`
2020-03-31T22:11:45.2546079Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2546474Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2546747Z    |
2020-03-31T22:11:45.2546970Z LL | use rustc_ast::std::str::FromStr;
---
2020-03-31T22:11:45.2549618Z diff of stderr:
2020-03-31T22:11:45.2549743Z 
2020-03-31T22:11:45.2549838Z 
2020-03-31T22:11:45.2550045Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2550768Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods/shadowed-trait-methods.stderr
2020-03-31T22:11:45.2551513Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2552074Z To only update this specific test, also pass `--test-args shadowed/shadowed-trait-methods.rs`
2020-03-31T22:11:45.2552510Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2552727Z status: exit code: 1
2020-03-31T22:11:45.2552727Z status: exit code: 1
2020-03-31T22:11:45.2554713Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/shadowed/shadowed-trait-methods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods/auxiliary"
2020-03-31T22:11:45.2556232Z ------------------------------------------
2020-03-31T22:11:45.2556391Z 
2020-03-31T22:11:45.2556724Z ------------------------------------------
2020-03-31T22:11:45.2556925Z stderr:
2020-03-31T22:11:45.2556925Z stderr:
2020-03-31T22:11:45.2557267Z ------------------------------------------
2020-03-31T22:11:45.2557575Z error[E0599]: no method named `f` found for unit type `()` in the current scope
2020-03-31T22:11:45.2558143Z   --> /checkout/src/test/ui/shadowed/shadowed-trait-methods.rs:13:8
2020-03-31T22:11:45.2558377Z    |
2020-03-31T22:11:45.2558547Z LL |     ().f() //~ ERROR no method
2020-03-31T22:11:45.2558787Z    |        ^ method not found in `()`
2020-03-31T22:11:45.2559190Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2559569Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2559823Z    |
2020-03-31T22:11:45.2559992Z LL | use rustc_ast::foo::T;
---
2020-03-31T22:11:45.2562524Z diff of stderr:
2020-03-31T22:11:45.2562638Z 
2020-03-31T22:11:45.2562728Z 
2020-03-31T22:11:45.2562931Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2563557Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy/trait-item-privacy.stderr
2020-03-31T22:11:45.2564148Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2565196Z To only update this specific test, also pass `--test-args traits/trait-item-privacy.rs`
2020-03-31T22:11:45.2565640Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2565893Z status: exit code: 1
2020-03-31T22:11:45.2565893Z status: exit code: 1
2020-03-31T22:11:45.2567836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-item-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy/auxiliary"
2020-03-31T22:11:45.2569448Z ------------------------------------------
2020-03-31T22:11:45.2569620Z 
2020-03-31T22:11:45.2569996Z ------------------------------------------
2020-03-31T22:11:45.2570197Z stderr:
2020-03-31T22:11:45.2570197Z stderr:
2020-03-31T22:11:45.2570566Z ------------------------------------------
2020-03-31T22:11:45.2570907Z error[E0599]: no method named `a` found for struct `S` in the current scope
2020-03-31T22:11:45.2571720Z    |
2020-03-31T22:11:45.2571953Z LL | struct S;
2020-03-31T22:11:45.2572354Z    | --------- method `a` not found for this
2020-03-31T22:11:45.2572552Z ...
2020-03-31T22:11:45.2572552Z ...
2020-03-31T22:11:45.2572777Z LL |     S.a(); //~ ERROR no method named `a` found
2020-03-31T22:11:45.2573045Z    |       ^ method not found in `S`
2020-03-31T22:11:45.2573521Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2573521Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2573923Z note: `method::A` defines an item `a`, perhaps you need to implement it
2020-03-31T22:11:45.2574786Z    |
2020-03-31T22:11:45.2574939Z LL |     trait A {
2020-03-31T22:11:45.2575115Z    |     ^^^^^^^
2020-03-31T22:11:45.2575243Z 
2020-03-31T22:11:45.2575243Z 
2020-03-31T22:11:45.2575516Z error[E0599]: no method named `b` found for struct `S` in the current scope
2020-03-31T22:11:45.2576090Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:68:7
2020-03-31T22:11:45.2576334Z    |
2020-03-31T22:11:45.2576504Z LL | struct S;
2020-03-31T22:11:45.2576897Z    | --------- method `b` not found for this
2020-03-31T22:11:45.2577095Z ...
2020-03-31T22:11:45.2577279Z LL |         fn b(&self) { }
2020-03-31T22:11:45.2577798Z    |            |
2020-03-31T22:11:45.2577798Z    |            |
2020-03-31T22:11:45.2578094Z    |            the method is available for `std::boxed::Box<S>` here
2020-03-31T22:11:45.2578518Z    |            the method is available for `std::sync::Arc<S>` here
2020-03-31T22:11:45.2578923Z    |            the method is available for `std::rc::Rc<S>` here
2020-03-31T22:11:45.2579180Z ...
2020-03-31T22:11:45.2579407Z LL |     S.b(); //~ ERROR no method named `b` found
2020-03-31T22:11:45.2579673Z    |       ^ method not found in `S`
2020-03-31T22:11:45.2580121Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2580667Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2580931Z    |
2020-03-31T22:11:45.2581133Z LL | use rustc_ast::method::B;
2020-03-31T22:11:45.2581133Z LL | use rustc_ast::method::B;
2020-03-31T22:11:45.2581321Z    |
2020-03-31T22:11:45.2581426Z 
2020-03-31T22:11:45.2581633Z error[E0624]: associated function `a` is private
2020-03-31T22:11:45.2582184Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:72:7
2020-03-31T22:11:45.2582426Z    |
2020-03-31T22:11:45.2582654Z LL |     c.a(); //~ ERROR associated function `a` is private
2020-03-31T22:11:45.2582956Z    |       ^ private associated function
2020-03-31T22:11:45.2583121Z 
2020-03-31T22:11:45.2583411Z error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
2020-03-31T22:11:45.2584267Z    |
2020-03-31T22:11:45.2584413Z LL | struct S;
2020-03-31T22:11:45.2584875Z    | --------- function or associated item `a` not found for this
2020-03-31T22:11:45.2585111Z ...
2020-03-31T22:11:45.2585111Z ...
2020-03-31T22:11:45.2585271Z LL |     S::a(&S);
2020-03-31T22:11:45.2585532Z    |        ^ function or associated item not found in `S`
2020-03-31T22:11:45.2586041Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2586041Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2587338Z note: `method::A` defines an item `a`, perhaps you need to implement it
2020-03-31T22:11:45.2588693Z    |
2020-03-31T22:11:45.2588847Z LL |     trait A {
2020-03-31T22:11:45.2589038Z    |     ^^^^^^^
2020-03-31T22:11:45.2589173Z 
2020-03-31T22:11:45.2589173Z 
2020-03-31T22:11:45.2589463Z error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
2020-03-31T22:11:45.2590077Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:80:8
2020-03-31T22:11:45.2590318Z    |
2020-03-31T22:11:45.2590465Z LL | struct S;
2020-03-31T22:11:45.2590914Z    | --------- function or associated item `b` not found for this
2020-03-31T22:11:45.2591161Z ...
2020-03-31T22:11:45.2591435Z LL |     S::b(&S);
2020-03-31T22:11:45.2591694Z    |        ^ function or associated item not found in `S`
2020-03-31T22:11:45.2592178Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2592572Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2592858Z    |
2020-03-31T22:11:45.2593046Z LL | use rustc_ast::method::B;
2020-03-31T22:11:45.2593046Z LL | use rustc_ast::method::B;
2020-03-31T22:11:45.2593233Z    |
2020-03-31T22:11:45.2593354Z 
2020-03-31T22:11:45.2593611Z error[E0624]: associated function `a` is private
2020-03-31T22:11:45.2594146Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:84:8
2020-03-31T22:11:45.2594388Z    |
2020-03-31T22:11:45.2594644Z LL |     C::a(&S); //~ ERROR associated function `a` is private
2020-03-31T22:11:45.2594950Z    |        ^ private associated function
2020-03-31T22:11:45.2595118Z 
2020-03-31T22:11:45.2595400Z error[E0599]: no associated item named `A` found for struct `S` in the current scope
2020-03-31T22:11:45.2596228Z    |
2020-03-31T22:11:45.2596388Z LL | struct S;
2020-03-31T22:11:45.2596388Z LL | struct S;
2020-03-31T22:11:45.2596805Z    | --------- associated item `A` not found for this
2020-03-31T22:11:45.2597017Z ...
2020-03-31T22:11:45.2597262Z LL |     S::A; //~ ERROR no associated item named `A` found
2020-03-31T22:11:45.2597567Z    |        ^ associated item not found in `S`
2020-03-31T22:11:45.2598044Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2598044Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T22:11:45.2598466Z note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
2020-03-31T22:11:45.2599280Z    |
2020-03-31T22:11:45.2599447Z LL |     trait A {
2020-03-31T22:11:45.2599621Z    |     ^^^^^^^
2020-03-31T22:11:45.2599747Z 
2020-03-31T22:11:45.2599747Z 
2020-03-31T22:11:45.2600029Z error[E0599]: no associated item named `B` found for struct `S` in the current scope
2020-03-31T22:11:45.2600609Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:98:8
2020-03-31T22:11:45.2600851Z    |
2020-03-31T22:11:45.2601012Z LL | struct S;
2020-03-31T22:11:45.2601429Z    | --------- associated item `B` not found for this
2020-03-31T22:11:45.2601640Z ...
2020-03-31T22:11:45.2601871Z LL |     S::B; //~ ERROR no associated item named `B` found
2020-03-31T22:11:45.2602189Z    |        ^ associated item not found in `S`
2020-03-31T22:11:45.2602647Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2603057Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2603330Z    |
2020-03-31T22:11:45.2603527Z LL | use rustc_ast::assoc_const::B;
2020-03-31T22:11:45.2603527Z LL | use rustc_ast::assoc_const::B;
2020-03-31T22:11:45.2603743Z    |
2020-03-31T22:11:45.2603846Z 
2020-03-31T22:11:45.2604051Z error[E0624]: associated constant `A` is private
2020-03-31T22:11:45.2604594Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:101:8
2020-03-31T22:11:45.2604839Z    |
2020-03-31T22:11:45.2605070Z LL |     C::A; //~ ERROR associated constant `A` is private
2020-03-31T22:11:45.2605382Z    |        ^ private associated constant
2020-03-31T22:11:45.2605806Z error[E0038]: the trait `assoc_const::C` cannot be made into an object
2020-03-31T22:11:45.2606378Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:101:5
2020-03-31T22:11:45.2606637Z    |
2020-03-31T22:11:45.2606824Z LL |         const A: u8 = 0;
2020-03-31T22:11:45.2606824Z LL |         const A: u8 = 0;
2020-03-31T22:11:45.2607316Z    |               - ...because it contains this associated `const`
2020-03-31T22:11:45.2607565Z ...
2020-03-31T22:11:45.2607747Z LL |         const B: u8 = 0;
2020-03-31T22:11:45.2608235Z    |               - ...because it contains this associated `const`
2020-03-31T22:11:45.2608481Z ...
2020-03-31T22:11:45.2608663Z LL |     pub trait C: A + B {
2020-03-31T22:11:45.2609531Z LL |         const C: u8 = 0;
2020-03-31T22:11:45.2610028Z    |               - ...because it contains this associated `const`
2020-03-31T22:11:45.2610266Z ...
2020-03-31T22:11:45.2610266Z ...
2020-03-31T22:11:45.2611137Z LL |     C::A; //~ ERROR associated constant `A` is private
2020-03-31T22:11:45.2611730Z    |     ^^^^ the trait `assoc_const::C` cannot be made into an object
2020-03-31T22:11:45.2612206Z    = help: consider moving `C` to another trait
2020-03-31T22:11:45.2612598Z    = help: consider moving `B` to another trait
2020-03-31T22:11:45.2612891Z    = help: consider moving `A` to another trait
2020-03-31T22:11:45.2613077Z 
2020-03-31T22:11:45.2613077Z 
2020-03-31T22:11:45.2613284Z error[E0223]: ambiguous associated type
2020-03-31T22:11:45.2613861Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:115:12
2020-03-31T22:11:45.2614107Z    |
2020-03-31T22:11:45.2614357Z LL |     let _: S::A; //~ ERROR ambiguous associated type
2020-03-31T22:11:45.2614952Z    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
2020-03-31T22:11:45.2615409Z error[E0223]: ambiguous associated type
2020-03-31T22:11:45.2615934Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:116:12
2020-03-31T22:11:45.2616179Z    |
2020-03-31T22:11:45.2616179Z    |
2020-03-31T22:11:45.2616412Z LL |     let _: S::B; //~ ERROR ambiguous associated type
2020-03-31T22:11:45.2617017Z    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::B`
2020-03-31T22:11:45.2617464Z error[E0223]: ambiguous associated type
2020-03-31T22:11:45.2617991Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:117:12
2020-03-31T22:11:45.2618235Z    |
2020-03-31T22:11:45.2618235Z    |
2020-03-31T22:11:45.2618469Z LL |     let _: S::C; //~ ERROR ambiguous associated type
2020-03-31T22:11:45.2619060Z    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::C`
2020-03-31T22:11:45.2619510Z error: associated type `A` is private
2020-03-31T22:11:45.2620015Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:119:12
2020-03-31T22:11:45.2620281Z    |
2020-03-31T22:11:45.2620281Z    |
2020-03-31T22:11:45.2620725Z LL |     let _: T::A; //~ ERROR associated type `A` is private
2020-03-31T22:11:45.2621208Z 
2020-03-31T22:11:45.2621406Z error: associated type `A` is private
2020-03-31T22:11:45.2621925Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:128:9
2020-03-31T22:11:45.2622171Z    |
2020-03-31T22:11:45.2622171Z    |
2020-03-31T22:11:45.2622415Z LL |         A = u8, //~ ERROR associated type `A` is private
2020-03-31T22:11:45.2622890Z 
2020-03-31T22:11:45.2623093Z error: aborting due to 15 previous errors
2020-03-31T22:11:45.2623266Z 
2020-03-31T22:11:45.2623514Z Some errors have detailed explanations: E0038, E0223, E0599, E0624.
---
2020-03-31T22:11:45.2625711Z 
2020-03-31T22:11:45.2625808Z 
2020-03-31T22:11:45.2626026Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2626715Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-method-private/trait-method-private.stderr
2020-03-31T22:11:45.2627369Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2627987Z To only update this specific test, also pass `--test-args traits/trait-method-private.rs`
2020-03-31T22:11:45.2628428Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2628678Z status: exit code: 1
2020-03-31T22:11:45.2628678Z status: exit code: 1
2020-03-31T22:11:45.2630639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-method-private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-method-private" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-method-private/auxiliary"
2020-03-31T22:11:45.2632406Z ------------------------------------------
2020-03-31T22:11:45.2632584Z 
2020-03-31T22:11:45.2632968Z ------------------------------------------
2020-03-31T22:11:45.2633170Z stderr:
2020-03-31T22:11:45.2633170Z stderr:
2020-03-31T22:11:45.2633542Z ------------------------------------------
2020-03-31T22:11:45.2633843Z error[E0624]: associated function `method` is private
2020-03-31T22:11:45.2634379Z   --> /checkout/src/test/ui/traits/trait-method-private.rs:19:9
2020-03-31T22:11:45.2634633Z    |
2020-03-31T22:11:45.2634850Z LL |     foo.method(); //~ ERROR is private
2020-03-31T22:11:45.2635328Z    |
2020-03-31T22:11:45.2635593Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2635987Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2636263Z    |
---
2020-03-31T22:11:45.2639053Z diff of stderr:
2020-03-31T22:11:45.2639194Z 
2020-03-31T22:11:45.2639290Z 
2020-03-31T22:11:45.2639496Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2640148Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene/hygiene.stderr
2020-03-31T22:11:45.2640787Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2641380Z To only update this specific test, also pass `--test-args underscore-imports/hygiene.rs`
2020-03-31T22:11:45.2641841Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2642078Z status: exit code: 1
2020-03-31T22:11:45.2642078Z status: exit code: 1
2020-03-31T22:11:45.2644027Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene/auxiliary"
2020-03-31T22:11:45.2645641Z ------------------------------------------
2020-03-31T22:11:45.2645814Z 
2020-03-31T22:11:45.2646179Z ------------------------------------------
2020-03-31T22:11:45.2646383Z stderr:
2020-03-31T22:11:45.2646383Z stderr:
2020-03-31T22:11:45.2646769Z ------------------------------------------
2020-03-31T22:11:45.2647110Z error[E0599]: no method named `deref` found for reference `&()` in the current scope
2020-03-31T22:11:45.2647693Z   --> /checkout/src/test/ui/underscore-imports/hygiene.rs:38:11
2020-03-31T22:11:45.2647945Z    |
2020-03-31T22:11:45.2648191Z LL |     (&()).deref();              //~ ERROR no method named `deref`
2020-03-31T22:11:45.2648562Z    |           ^^^^^ method not found in `&()`
2020-03-31T22:11:45.2649026Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2649423Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2649708Z    |
2020-03-31T22:11:45.2649908Z LL | use rustc_ast::std::ops::Deref;
2020-03-31T22:11:45.2649908Z LL | use rustc_ast::std::ops::Deref;
2020-03-31T22:11:45.2650109Z    |
2020-03-31T22:11:45.2650212Z 
2020-03-31T22:11:45.2650562Z error[E0599]: no method named `deref_mut` found for mutable reference `&mut ()` in the current scope
2020-03-31T22:11:45.2651186Z   --> /checkout/src/test/ui/underscore-imports/hygiene.rs:39:15
2020-03-31T22:11:45.2651424Z    |
2020-03-31T22:11:45.2651690Z LL |     (&mut ()).deref_mut();      //~ ERROR no method named `deref_mut`
2020-03-31T22:11:45.2652038Z    |               ^^^^^^^^^ method not found in `&mut ()`
2020-03-31T22:11:45.2652520Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T22:11:45.2652919Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T22:11:45.2653190Z    |
2020-03-31T22:11:45.2653408Z LL | use rustc_ast::std::ops::DerefMut;
---
2020-03-31T22:11:45.2656021Z diff of stderr:
2020-03-31T22:11:45.2656145Z 
2020-03-31T22:11:45.2656241Z 
2020-03-31T22:11:45.2656446Z The actual stderr differed from the expected stderr.
2020-03-31T22:11:45.2657106Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/shadow/shadow.stderr
2020-03-31T22:11:45.2657736Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T22:11:45.2658327Z To only update this specific test, also pass `--test-args underscore-imports/shadow.rs`
2020-03-31T22:11:45.2658780Z error: 1 errors occurred comparing output.
2020-03-31T22:11:45.2659015Z status: exit code: 1
2020-03-31T22:11:45.2659015Z status: exit code: 1
2020-03-31T22:11:45.2661084Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/shadow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/shadow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/shadow/auxiliary"
2020-03-31T22:11:45.2662682Z ------------------------------------------
2020-03-31T22:11:45.2662854Z 
2020-03-31T22:11:45.2663219Z ------------------------------------------
2020-03-31T22:11:45.2663433Z stderr:
---
2020-03-31T22:11:45.2676746Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-31T22:11:45.2677171Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:11:45.2677409Z 
2020-03-31T22:11:45.2677506Z 
2020-03-31T22:11:45.2681245Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-31T22:11:45.2684002Z 
2020-03-31T22:11:45.2684102Z 
2020-03-31T22:11:45.2684629Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T22:11:45.2684999Z Build completed unsuccessfully in 1:00:39
2020-03-31T22:11:45.2684999Z Build completed unsuccessfully in 1:00:39
2020-03-31T22:11:45.2685461Z == clock drift check ==
2020-03-31T22:11:45.2685720Z   local time: Tue Mar 31 22:11:45 UTC 2020
2020-03-31T22:11:45.3071854Z   network time: Tue, 31 Mar 2020 22:11:45 GMT
2020-03-31T22:11:45.3078991Z == end clock drift check ==
2020-03-31T22:11:45.7854408Z 
2020-03-31T22:11:45.7949384Z ##[error]Bash exited with code '1'.
2020-03-31T22:11:45.7964107Z ##[section]Finishing: Run build
2020-03-31T22:11:45.8011566Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70621/merge to s
2020-03-31T22:11:45.8016597Z Task         : Get sources
2020-03-31T22:11:45.8016943Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T22:11:45.8017278Z Version      : 1.0.0
2020-03-31T22:11:45.8017509Z Author       : Microsoft
2020-03-31T22:11:45.8017509Z Author       : Microsoft
2020-03-31T22:11:45.8017865Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T22:11:45.8018289Z ==============================================================================
2020-03-31T22:11:46.1218170Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T22:11:46.1259146Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70621/merge to s
2020-03-31T22:11:46.1346006Z Cleaning up task key
2020-03-31T22:11:46.1347266Z Start cleaning up orphan processes.
2020-03-31T22:11:46.1527795Z Terminate orphan process: pid (3438) (python)
2020-03-31T22:11:46.1681115Z ##[section]Finishing: Finalize Job
