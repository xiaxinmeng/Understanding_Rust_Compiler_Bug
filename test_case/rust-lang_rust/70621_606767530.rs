plain
2020-03-31T16:24:27.5894727Z ========================== Starting Command Output ===========================
2020-03-31T16:24:27.5909642Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/349e2602-c44d-4b62-b75a-ba41d5ee0cdd.sh
2020-03-31T16:24:28.2984114Z 
2020-03-31T16:24:28.3040301Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T16:24:28.3068627Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70621/merge to s
2020-03-31T16:24:28.3079502Z Task         : Get sources
2020-03-31T16:24:28.3079944Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T16:24:28.3080385Z Version      : 1.0.0
2020-03-31T16:24:28.3080714Z Author       : Microsoft
---
2020-03-31T16:24:33.9009807Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T16:24:33.9358152Z ##[command]git config gc.auto 0
2020-03-31T16:24:33.9394827Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T16:24:33.9425638Z ##[command]git config --get-all http.proxy
2020-03-31T16:24:33.9512192Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70621/merge:refs/remotes/pull/70621/merge
---
2020-03-31T16:34:40.9199112Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T16:34:42.2218581Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T16:34:43.6457035Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T16:34:44.0233253Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T16:34:52.3867691Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T16:34:53.7823570Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T16:34:57.6248904Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T16:35:01.3666007Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T16:35:10.7685332Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T16:55:45.2980155Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T16:55:46.9858038Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T16:55:48.9187810Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T16:55:49.5589879Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T16:56:00.5466593Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T16:56:02.6931046Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T16:56:07.7795166Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T16:56:13.0643384Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T16:56:23.9804396Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T17:19:18.1019140Z .........................................i.......................................................... 1200/9856
2020-03-31T17:19:19.6884699Z ...................................6    |
2020-03-31T17:19:19.6885820Z 7    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:19:19.6886212Z 8    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:19:19.6886767Z -            `use Lib::TheTrait;`
2020-03-31T17:19:19.6888856Z +            `use rustc_ast::Lib::TheTrait;`
2020-03-31T17:19:19.6889270Z 11 error: aborting due to previous error
2020-03-31T17:19:19.6889450Z 12 
2020-03-31T17:19:19.6889548Z 
2020-03-31T17:19:19.8286356Z F.6    |
2020-03-31T17:19:19.8286356Z F.6    |
2020-03-31T17:19:19.8286759Z 7    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:19:19.8287212Z 8    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:19:19.8291379Z -            `use coherence_inherent_cc_lib::TheTrait;`
2020-03-31T17:19:19.8301206Z +            `use rustc_ast::coherence_inherent_cc_lib::TheTrait;`
2020-03-31T17:19:19.8301893Z 11 error: aborting due to previous error
2020-03-31T17:19:19.8302078Z 12 
2020-03-31T17:19:19.8302174Z 
2020-03-31T17:19:25.0039959Z F.............................................................. 1300/9856
2020-03-31T17:19:25.0039959Z F.............................................................. 1300/9856
2020-03-31T17:19:28.6983964Z .................................................................................................... 1400/9856
2020-03-31T17:19:34.8521122Z .................................................................................................... 1500/9856
2020-03-31T17:19:39.8736447Z .................................................................................................... 1600/9856
2020-03-31T17:19:45.3488130Z .................................................................................................... 1700/9856
2020-03-31T17:19:49.0337634Z .................................................................................................... 1800/9856
2020-03-31T17:19:58.5454486Z ..........................................................................................i......... 1900/9856
2020-03-31T17:20:05.1331341Z .................................................................................................... 2000/9856
2020-03-31T17:20:11.2201751Z ................................................................................iiiii............... 2100/9856
2020-03-31T17:20:31.1275617Z .................................................................................................... 2300/9856
2020-03-31T17:20:33.0937800Z .................................................................................................... 2400/9856
2020-03-31T17:20:35.2803690Z .................................................................................................... 2500/9856
2020-03-31T17:20:41.1843864Z .................................................................................................... 2600/9856
---
2020-03-31T17:21:33.2902452Z 9    |
2020-03-31T17:21:33.2902870Z - LL | use foo::Bar;
2020-03-31T17:21:33.2903087Z + LL | use rustc_ast::foo::Bar;
2020-03-31T17:21:33.2903269Z 11    |
2020-03-31T17:21:33.2903807Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2907831Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2908464Z 13    |
2020-03-31T17:21:33.2909303Z - LL | use no_method_suggested_traits::qux::PrivPub;
2020-03-31T17:21:33.2910348Z + LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T17:21:33.2915542Z - LL | use no_method_suggested_traits::Reexported;
2020-03-31T17:21:33.2918145Z + LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T17:21:33.2919219Z 17    |
2020-03-31T17:21:33.2921512Z 18 
2020-03-31T17:21:33.2921512Z 18 
2020-03-31T17:21:33.2922632Z 19 error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&u32>>` in the current scope
2020-03-31T17:21:33.2926348Z 25    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:21:33.2929670Z 26 help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2020-03-31T17:21:33.2930008Z 27    |
2020-03-31T17:21:33.2934037Z - LL | use foo::Bar;
2020-03-31T17:21:33.2934037Z - LL | use foo::Bar;
2020-03-31T17:21:33.2934467Z + LL | use rustc_ast::foo::Bar;
2020-03-31T17:21:33.2942692Z 29    |
2020-03-31T17:21:33.2943829Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2944926Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2945867Z 31    |
2020-03-31T17:21:33.2946667Z - LL | use no_method_suggested_traits::qux::PrivPub;
2020-03-31T17:21:33.2947223Z + LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T17:21:33.2948187Z - LL | use no_method_suggested_traits::Reexported;
2020-03-31T17:21:33.2948681Z + LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T17:21:33.2949051Z 35    |
2020-03-31T17:21:33.2949300Z 36 
---
2020-03-31T17:21:33.2952274Z - LL | use foo::Bar;
2020-03-31T17:21:33.2952634Z + LL | use rustc_ast::foo::Bar;
2020-03-31T17:21:33.2952963Z 47    |
2020-03-31T17:21:33.2953218Z 48 
2020-03-31T17:21:33.2953734Z 49 error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&char>>` in the current scope
2020-03-31T17:21:33.2954573Z 63    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:21:33.2955095Z 64 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:21:33.2955519Z 65    |
2020-03-31T17:21:33.2955999Z - LL | use foo::Bar;
---
2020-03-31T17:21:33.2967432Z 
2020-03-31T17:21:33.2967828Z 75    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:21:33.2968352Z 76 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:21:33.2968955Z 77    |
2020-03-31T17:21:33.2969661Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2970178Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2971016Z 80 
2020-03-31T17:21:33.2971016Z 80 
2020-03-31T17:21:33.2972153Z 81 error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&i32>>` in the current scope
2020-03-31T17:21:33.2973240Z 97    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:21:33.2973786Z 98 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:21:33.2974193Z 99    |
2020-03-31T17:21:33.2974193Z 99    |
2020-03-31T17:21:33.2974820Z - LL | use no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2975315Z + LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:21:33.2975962Z 102 
2020-03-31T17:21:33.2976382Z 103 error[E0599]: no method named `method` found for struct `Foo` in the current scope
2020-03-31T17:21:33.2976751Z 
2020-03-31T17:21:33.9103930Z F................ 3300/9856
---
2020-03-31T17:23:05.3013168Z .................................................................................................... 4800/9856
2020-03-31T17:23:05.6757237Z ........7    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:23:05.6757927Z 8 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:23:05.6761342Z 9    |
2020-03-31T17:23:05.6765014Z - LL | use xcrate_issue_43189_b::xcrate_issue_43189_a::A;
2020-03-31T17:23:05.6768593Z + LL | use rustc_ast::xcrate_issue_43189_b::xcrate_issue_43189_a::A;
2020-03-31T17:23:05.6775534Z 12 
2020-03-31T17:23:05.6779216Z 13 error: aborting due to previous error
2020-03-31T17:23:05.6782281Z 
2020-03-31T17:23:12.1712018Z F........................................................................................... 4900/9856
2020-03-31T17:23:12.1712018Z F........................................................................................... 4900/9856
2020-03-31T17:23:16.7502442Z ......................................................i...............i............................. 5000/9856
2020-03-31T17:23:24.0252425Z .................................................................................................... 5100/9856
2020-03-31T17:23:30.7528678Z ...................................................................................................i 5200/9856
2020-03-31T17:23:35.4460781Z .................................................................................................... 5300/9856
2020-03-31T17:23:45.2657046Z .....................................................................................ii.ii........i. 5400/9856
2020-03-31T17:23:48.5078593Z ..i................................................................................................. 5500/9856
2020-03-31T17:23:56.7420395Z ..............................i..................................................................... 5700/9856
2020-03-31T17:24:05.6751758Z ................................................ii....................................i............. 5800/9856
2020-03-31T17:24:12.4702872Z .................................................................................................... 5900/9856
2020-03-31T17:24:17.1521350Z .................................................................................................... 6000/9856
2020-03-31T17:24:17.1521350Z .................................................................................................... 6000/9856
2020-03-31T17:24:25.8228217Z ................................................................................ii...i..ii.......... 6100/9856
2020-03-31T17:24:37.0649134Z .i.................................................................................................. 6200/9856
2020-03-31T17:24:49.4169712Z .................................................................................................... 6400/9856
2020-03-31T17:24:52.4998022Z .................................................................................................... 6500/9856
2020-03-31T17:24:52.4998022Z .................................................................................................... 6500/9856
2020-03-31T17:25:03.8996573Z ..........i..ii..................................................................................... 6600/9856
2020-03-31T17:25:23.3212169Z .................................................................................................... 6800/9856
2020-03-31T17:25:25.2991937Z ..........i......................................................................................... 6900/9856
2020-03-31T17:25:27.2494428Z .................................................................................................... 7000/9856
2020-03-31T17:25:29.3603811Z ...............................................i.................................................... 7100/9856
---
2020-03-31T17:27:21.6368904Z 
2020-03-31T17:27:21.9528403Z F.6    |
2020-03-31T17:27:21.9529293Z 7    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:27:21.9530096Z 8    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:27:21.9531776Z -            `use crate::foo::foobar::Foobar;`
2020-03-31T17:27:21.9532634Z +            `use rustc_ast::crate::foo::foobar::Foobar;`
2020-03-31T17:27:21.9537654Z 11 error[E0599]: no method named `bar` found for type `u32` in the current scope
2020-03-31T17:27:21.9538696Z 12   --> $DIR/trait-import-suggestions.rs:28:7
2020-03-31T17:27:21.9539266Z 
2020-03-31T17:27:21.9540885Z 17    = help: items from traits can only be used if the trait is in scope
---
2020-03-31T17:27:21.9558733Z 39    |
2020-03-31T17:27:21.9560038Z 40 
2020-03-31T17:27:21.9561317Z 41 error: aborting due to 4 previous errors
2020-03-31T17:27:21.9562523Z 
2020-03-31T17:27:26.5690345Z F............................iiiiiiiiii.i................................ 8200/9856
2020-03-31T17:27:31.7043778Z 8 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:27:31.7046766Z 9    |
2020-03-31T17:27:31.7050100Z - LL | use foo::T;
2020-03-31T17:27:31.7053446Z + LL | use rustc_ast::foo::T;
---
2020-03-31T17:29:50.2038506Z 
2020-03-31T17:29:50.2038739Z 
2020-03-31T17:29:50.2039318Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2040268Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent/coherence_inherent.stderr
2020-03-31T17:29:50.2041052Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2041863Z To only update this specific test, also pass `--test-args coherence/coherence_inherent.rs`
2020-03-31T17:29:50.2042672Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2043003Z status: exit code: 1
2020-03-31T17:29:50.2043003Z status: exit code: 1
2020-03-31T17:29:50.2044969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence_inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent/auxiliary"
2020-03-31T17:29:50.2046698Z ------------------------------------------
2020-03-31T17:29:50.2047158Z 
2020-03-31T17:29:50.2047634Z ------------------------------------------
2020-03-31T17:29:50.2047952Z stderr:
2020-03-31T17:29:50.2047952Z stderr:
2020-03-31T17:29:50.2048433Z ------------------------------------------
2020-03-31T17:29:50.2048919Z error[E0599]: no method named `the_fn` found for reference `&Lib::TheStruct` in the current scope
2020-03-31T17:29:50.2049998Z    |
2020-03-31T17:29:50.2049998Z    |
2020-03-31T17:29:50.2050259Z LL |         s.the_fn();
2020-03-31T17:29:50.2050645Z    |           ^^^^^^ method not found in `&Lib::TheStruct`
2020-03-31T17:29:50.2051326Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2051824Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2051824Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2052274Z            `use rustc_ast::Lib::TheTrait;`
2020-03-31T17:29:50.2053068Z error: aborting due to previous error
2020-03-31T17:29:50.2057254Z 
2020-03-31T17:29:50.2057929Z For more information about this error, try `rustc --explain E0599`.
2020-03-31T17:29:50.2058145Z 
---
2020-03-31T17:29:50.2059347Z diff of stderr:
2020-03-31T17:29:50.2059461Z 
2020-03-31T17:29:50.2059552Z 
2020-03-31T17:29:50.2059756Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2060414Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent_cc/coherence_inherent_cc.stderr
2020-03-31T17:29:50.2061018Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2061603Z To only update this specific test, also pass `--test-args coherence/coherence_inherent_cc.rs`
2020-03-31T17:29:50.2062020Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2062254Z status: exit code: 1
2020-03-31T17:29:50.2062254Z status: exit code: 1
2020-03-31T17:29:50.2064090Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence_inherent_cc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent_cc" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence_inherent_cc/auxiliary"
2020-03-31T17:29:50.2065721Z ------------------------------------------
2020-03-31T17:29:50.2065880Z 
2020-03-31T17:29:50.2066282Z ------------------------------------------
2020-03-31T17:29:50.2066474Z stderr:
2020-03-31T17:29:50.2066474Z stderr:
2020-03-31T17:29:50.2066822Z ------------------------------------------
2020-03-31T17:29:50.2067242Z error[E0599]: no method named `the_fn` found for reference `&coherence_inherent_cc_lib::TheStruct` in the current scope
2020-03-31T17:29:50.2068126Z    |
2020-03-31T17:29:50.2068126Z    |
2020-03-31T17:29:50.2068293Z LL |         s.the_fn();
2020-03-31T17:29:50.2068600Z    |           ^^^^^^ method not found in `&coherence_inherent_cc_lib::TheStruct`
2020-03-31T17:29:50.2069119Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2069491Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2069491Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2069872Z            `use rustc_ast::coherence_inherent_cc_lib::TheTrait;`
2020-03-31T17:29:50.2070275Z error: aborting due to previous error
2020-03-31T17:29:50.2070427Z 
2020-03-31T17:29:50.2070824Z For more information about this error, try `rustc --explain E0599`.
2020-03-31T17:29:50.2071783Z 
---
2020-03-31T17:29:50.2073112Z 
2020-03-31T17:29:50.2073202Z 
2020-03-31T17:29:50.2073392Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2074036Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/no_implicit_prelude.stderr
2020-03-31T17:29:50.2074632Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2075196Z To only update this specific test, also pass `--test-args hygiene/no_implicit_prelude.rs`
2020-03-31T17:29:50.2075608Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2075826Z status: exit code: 1
2020-03-31T17:29:50.2075826Z status: exit code: 1
2020-03-31T17:29:50.2077651Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/no_implicit_prelude.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/auxiliary"
2020-03-31T17:29:50.2080943Z ------------------------------------------
2020-03-31T17:29:50.2081122Z 
2020-03-31T17:29:50.2081504Z ------------------------------------------
2020-03-31T17:29:50.2081730Z stderr:
---
2020-03-31T17:29:50.2084760Z 
2020-03-31T17:29:50.2085016Z error[E0433]: failed to resolve: use of undeclared type or module `Vec`
2020-03-31T17:29:50.2085696Z   --> /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:11:9
2020-03-31T17:29:50.2085959Z    |
2020-03-31T17:29:50.2086152Z LL |     fn f() { ::bar::m!(); }
2020-03-31T17:29:50.2086903Z ...
2020-03-31T17:29:50.2087129Z LL |         Vec::new(); //~ ERROR failed to resolve
2020-03-31T17:29:50.2087438Z    |         ^^^ use of undeclared type or module `Vec`
2020-03-31T17:29:50.2087651Z    |
2020-03-31T17:29:50.2087651Z    |
2020-03-31T17:29:50.2088215Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-31T17:29:50.2088501Z 
2020-03-31T17:29:50.2088768Z error[E0599]: no method named `clone` found for unit type `()` in the current scope
2020-03-31T17:29:50.2089364Z   --> /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:12:12
2020-03-31T17:29:50.2089609Z    |
2020-03-31T17:29:50.2089800Z LL |     fn f() { ::bar::m!(); }
2020-03-31T17:29:50.2090496Z ...
2020-03-31T17:29:50.2090496Z ...
2020-03-31T17:29:50.2090726Z LL |         ().clone() //~ ERROR no method named `clone` found
2020-03-31T17:29:50.2091046Z    |            ^^^^^ method not found in `()`
2020-03-31T17:29:50.2091498Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2091915Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2092395Z            `use rustc_ast::std::clone::Clone;`
2020-03-31T17:29:50.2093047Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
2020-03-31T17:29:50.2095378Z diff of stderr:
2020-03-31T17:29:50.2095480Z 
2020-03-31T17:29:50.2095557Z 
2020-03-31T17:29:50.2095737Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2096247Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items/trait_items.stderr
2020-03-31T17:29:50.2096740Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2097215Z To only update this specific test, also pass `--test-args hygiene/trait_items.rs`
2020-03-31T17:29:50.2097561Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2097934Z status: exit code: 1
2020-03-31T17:29:50.2097934Z status: exit code: 1
2020-03-31T17:29:50.2100686Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/trait_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items/auxiliary"
2020-03-31T17:29:50.2102355Z ------------------------------------------
2020-03-31T17:29:50.2102517Z 
2020-03-31T17:29:50.2102870Z ------------------------------------------
2020-03-31T17:29:50.2103056Z stderr:
2020-03-31T17:29:50.2103056Z stderr:
2020-03-31T17:29:50.2103551Z ------------------------------------------
2020-03-31T17:29:50.2104153Z error[E0599]: no method named `f` found for unit type `()` in the current scope
2020-03-31T17:29:50.2104943Z   --> /checkout/src/test/ui/hygiene/trait_items.rs:17:24
2020-03-31T17:29:50.2105267Z    |
2020-03-31T17:29:50.2105475Z LL |     fn f() { ::baz::m!(); }
2020-03-31T17:29:50.2106332Z ...
2020-03-31T17:29:50.2106332Z ...
2020-03-31T17:29:50.2106645Z LL |     pub macro m() { ().f() } //~ ERROR no method named `f` found
2020-03-31T17:29:50.2107088Z    |                        ^ method not found in `()`
2020-03-31T17:29:50.2107537Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2107911Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2108239Z            `use rustc_ast::foo::T;`
2020-03-31T17:29:50.2108855Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
2020-03-31T17:29:50.2111464Z diff of stderr:
2020-03-31T17:29:50.2111577Z 
2020-03-31T17:29:50.2111683Z 
2020-03-31T17:29:50.2111873Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2112554Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
2020-03-31T17:29:50.2113192Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2113764Z To only update this specific test, also pass `--test-args impl-trait/no-method-suggested-traits.rs`
2020-03-31T17:29:50.2114210Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2115030Z status: exit code: 1
2020-03-31T17:29:50.2115030Z status: exit code: 1
2020-03-31T17:29:50.2116962Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/auxiliary"
2020-03-31T17:29:50.2118495Z ------------------------------------------
2020-03-31T17:29:50.2118673Z 
2020-03-31T17:29:50.2119007Z ------------------------------------------
2020-03-31T17:29:50.2119197Z stderr:
2020-03-31T17:29:50.2119197Z stderr:
2020-03-31T17:29:50.2119552Z ------------------------------------------
2020-03-31T17:29:50.2119865Z error[E0599]: no method named `method` found for type `u32` in the current scope
2020-03-31T17:29:50.2120426Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:23:10
2020-03-31T17:29:50.2120854Z    |
2020-03-31T17:29:50.2121013Z LL |     1u32.method();
2020-03-31T17:29:50.2121695Z    |
2020-03-31T17:29:50.2122046Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2122436Z help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2020-03-31T17:29:50.2122845Z    |
2020-03-31T17:29:50.2122845Z    |
2020-03-31T17:29:50.2123021Z LL | use rustc_ast::foo::Bar;
2020-03-31T17:29:50.2123191Z    |
2020-03-31T17:29:50.2123562Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:29:50.2123905Z    |
2020-03-31T17:29:50.2124295Z LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T17:29:50.2124891Z LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T17:29:50.2125115Z    |
2020-03-31T17:29:50.2125211Z 
2020-03-31T17:29:50.2125211Z 
2020-03-31T17:29:50.2125740Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&u32>>` in the current scope
2020-03-31T17:29:50.2126827Z    |
2020-03-31T17:29:50.2126827Z    |
2020-03-31T17:29:50.2127071Z LL |     std::rc::Rc::new(&mut Box::new(&1u32)).method();
2020-03-31T17:29:50.2127518Z    |                                            ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&u32>>`
2020-03-31T17:29:50.2128225Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2128605Z help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
2020-03-31T17:29:50.2128877Z    |
2020-03-31T17:29:50.2129065Z LL | use rustc_ast::foo::Bar;
2020-03-31T17:29:50.2129065Z LL | use rustc_ast::foo::Bar;
2020-03-31T17:29:50.2129234Z    |
2020-03-31T17:29:50.2129465Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:29:50.2129709Z    |
2020-03-31T17:29:50.2129941Z LL | use rustc_ast::no_method_suggested_traits::qux::PrivPub;
2020-03-31T17:29:50.2130417Z LL | use rustc_ast::no_method_suggested_traits::Reexported;
2020-03-31T17:29:50.2130640Z    |
2020-03-31T17:29:50.2130735Z 
2020-03-31T17:29:50.2130979Z error[E0599]: no method named `method` found for type `char` in the current scope
2020-03-31T17:29:50.2130979Z error[E0599]: no method named `method` found for type `char` in the current scope
2020-03-31T17:29:50.2131586Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:30:9
2020-03-31T17:29:50.2131833Z    |
2020-03-31T17:29:50.2132126Z LL |     'a'.method();
2020-03-31T17:29:50.2132547Z    |
2020-03-31T17:29:50.2132778Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2133161Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2133412Z    |
2020-03-31T17:29:50.2133412Z    |
2020-03-31T17:29:50.2133582Z LL | use rustc_ast::foo::Bar;
2020-03-31T17:29:50.2133767Z    |
2020-03-31T17:29:50.2133863Z 
2020-03-31T17:29:50.2139968Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&char>>` in the current scope
2020-03-31T17:29:50.2141142Z    |
2020-03-31T17:29:50.2141308Z LL |         fn method(&self) {}
2020-03-31T17:29:50.2141652Z    |            ------
2020-03-31T17:29:50.2141845Z    |            |
2020-03-31T17:29:50.2141845Z    |            |
2020-03-31T17:29:50.2142197Z    |            the method is available for `std::boxed::Box<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T17:29:50.2142727Z    |            the method is available for `std::pin::Pin<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T17:29:50.2143677Z    |            the method is available for `std::sync::Arc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T17:29:50.2144213Z    |            the method is available for `std::rc::Rc<std::rc::Rc<&mut std::boxed::Box<&char>>>` here
2020-03-31T17:29:50.2144529Z ...
2020-03-31T17:29:50.2145002Z LL |     std::rc::Rc::new(&mut Box::new(&'a')).method();
2020-03-31T17:29:50.2145452Z    |                                           ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&char>>`
2020-03-31T17:29:50.2146039Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2146403Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2146657Z    |
2020-03-31T17:29:50.2146844Z LL | use rustc_ast::foo::Bar;
2020-03-31T17:29:50.2146844Z LL | use rustc_ast::foo::Bar;
2020-03-31T17:29:50.2147014Z    |
2020-03-31T17:29:50.2147112Z 
2020-03-31T17:29:50.2147622Z error[E0599]: no method named `method` found for type `i32` in the current scope
2020-03-31T17:29:50.2148338Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:35:10
2020-03-31T17:29:50.2148587Z    |
2020-03-31T17:29:50.2148755Z LL |     1i32.method();
2020-03-31T17:29:50.2149159Z    |
2020-03-31T17:29:50.2149452Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2149837Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2150089Z    |
2020-03-31T17:29:50.2150089Z    |
2020-03-31T17:29:50.2150322Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:29:50.2150665Z 
2020-03-31T17:29:50.2150665Z 
2020-03-31T17:29:50.2151809Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&i32>>` in the current scope
2020-03-31T17:29:50.2152816Z    |
2020-03-31T17:29:50.2152816Z    |
2020-03-31T17:29:50.2153051Z LL |     std::rc::Rc::new(&mut Box::new(&1i32)).method();
2020-03-31T17:29:50.2153514Z    |                                            ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&i32>>`
2020-03-31T17:29:50.2154320Z   ::: /checkout/src/test/ui/impl-trait/auxiliary/no_method_suggested_traits.rs:8:12
2020-03-31T17:29:50.2154605Z    |
2020-03-31T17:29:50.2154771Z LL |         fn method(&self) {}
2020-03-31T17:29:50.2155108Z    |            ------
2020-03-31T17:29:50.2155108Z    |            ------
2020-03-31T17:29:50.2155296Z    |            |
2020-03-31T17:29:50.2155644Z    |            the method is available for `std::boxed::Box<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T17:29:50.2156170Z    |            the method is available for `std::pin::Pin<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T17:29:50.2156708Z    |            the method is available for `std::sync::Arc<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T17:29:50.2157226Z    |            the method is available for `std::rc::Rc<std::rc::Rc<&mut std::boxed::Box<&i32>>>` here
2020-03-31T17:29:50.2157792Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2158156Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2158407Z    |
2020-03-31T17:29:50.2158407Z    |
2020-03-31T17:29:50.2158659Z LL | use rustc_ast::no_method_suggested_traits::foo::PubPub;
2020-03-31T17:29:50.2158984Z 
2020-03-31T17:29:50.2159229Z error[E0599]: no method named `method` found for struct `Foo` in the current scope
2020-03-31T17:29:50.2159810Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:40:9
2020-03-31T17:29:50.2160057Z    |
2020-03-31T17:29:50.2160057Z    |
2020-03-31T17:29:50.2160195Z LL | struct Foo;
2020-03-31T17:29:50.2160594Z    | ----------- method `method` not found for this
2020-03-31T17:29:50.2160789Z ...
2020-03-31T17:29:50.2160937Z LL |     Foo.method();
2020-03-31T17:29:50.2161355Z    |
2020-03-31T17:29:50.2161796Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2161796Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2162256Z    = note: the following traits define an item `method`, perhaps you need to implement one of them:
2020-03-31T17:29:50.2162618Z            candidate #1: `foo::Bar`
2020-03-31T17:29:50.2162933Z            candidate #2: `no_method_suggested_traits::foo::PubPub`
2020-03-31T17:29:50.2163328Z            candidate #3: `no_method_suggested_traits::qux::PrivPub`
2020-03-31T17:29:50.2163702Z            candidate #4: `no_method_suggested_traits::Reexported`
2020-03-31T17:29:50.2163924Z 
2020-03-31T17:29:50.2164286Z error[E0599]: no method named `method` found for struct `std::rc::Rc<&mut std::boxed::Box<&Foo>>` in the current scope
2020-03-31T17:29:50.2165275Z    |
2020-03-31T17:29:50.2165275Z    |
2020-03-31T17:29:50.2165516Z LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method();
2020-03-31T17:29:50.2166089Z    |                                           ^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Foo>>`
2020-03-31T17:29:50.2166733Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2166733Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2167237Z    = note: the following traits define an item `method`, perhaps you need to implement one of them:
2020-03-31T17:29:50.2167597Z            candidate #1: `foo::Bar`
2020-03-31T17:29:50.2167928Z            candidate #2: `no_method_suggested_traits::foo::PubPub`
2020-03-31T17:29:50.2168309Z            candidate #3: `no_method_suggested_traits::qux::PrivPub`
2020-03-31T17:29:50.2168682Z            candidate #4: `no_method_suggested_traits::Reexported`
2020-03-31T17:29:50.2169186Z error[E0599]: no method named `method2` found for type `u64` in the current scope
2020-03-31T17:29:50.2169807Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:45:10
2020-03-31T17:29:50.2170079Z    |
2020-03-31T17:29:50.2170079Z    |
2020-03-31T17:29:50.2170261Z LL |     1u64.method2();
2020-03-31T17:29:50.2170704Z    |
2020-03-31T17:29:50.2170996Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2170996Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2171510Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T17:29:50.2172319Z    |
2020-03-31T17:29:50.2172471Z LL |     pub trait Bar {
2020-03-31T17:29:50.2172654Z    |     ^^^^^^^^^^^^^
2020-03-31T17:29:50.2172778Z 
2020-03-31T17:29:50.2172778Z 
2020-03-31T17:29:50.2173132Z error[E0599]: no method named `method2` found for struct `std::rc::Rc<&mut std::boxed::Box<&u64>>` in the current scope
2020-03-31T17:29:50.2174319Z    |
2020-03-31T17:29:50.2174319Z    |
2020-03-31T17:29:50.2174565Z LL |     std::rc::Rc::new(&mut Box::new(&1u64)).method2();
2020-03-31T17:29:50.2175014Z    |                                            ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&u64>>`
2020-03-31T17:29:50.2175625Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2175625Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2176002Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T17:29:50.2176806Z    |
2020-03-31T17:29:50.2176958Z LL |     pub trait Bar {
2020-03-31T17:29:50.2177139Z    |     ^^^^^^^^^^^^^
2020-03-31T17:29:50.2177279Z 
2020-03-31T17:29:50.2177279Z 
2020-03-31T17:29:50.2177600Z error[E0599]: no method named `method2` found for struct `no_method_suggested_traits::Foo` in the current scope
2020-03-31T17:29:50.2178235Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:50:37
2020-03-31T17:29:50.2178498Z    |
2020-03-31T17:29:50.2178713Z LL |     no_method_suggested_traits::Foo.method2();
2020-03-31T17:29:50.2179115Z    |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`
2020-03-31T17:29:50.2179702Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2179702Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2180076Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T17:29:50.2180876Z    |
2020-03-31T17:29:50.2181028Z LL |     pub trait Bar {
2020-03-31T17:29:50.2181224Z    |     ^^^^^^^^^^^^^
2020-03-31T17:29:50.2181348Z 
2020-03-31T17:29:50.2181348Z 
2020-03-31T17:29:50.2181927Z error[E0599]: no method named `method2` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>` in the current scope
2020-03-31T17:29:50.2183035Z    |
2020-03-31T17:29:50.2183035Z    |
2020-03-31T17:29:50.2183336Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method2();
2020-03-31T17:29:50.2183997Z    |                                                                       ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>`
2020-03-31T17:29:50.2184831Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2184831Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2185251Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T17:29:50.2186112Z    |
2020-03-31T17:29:50.2186293Z LL |     pub trait Bar {
2020-03-31T17:29:50.2186487Z    |     ^^^^^^^^^^^^^
2020-03-31T17:29:50.2186620Z 
2020-03-31T17:29:50.2186620Z 
2020-03-31T17:29:50.2186960Z error[E0599]: no method named `method2` found for enum `no_method_suggested_traits::Bar` in the current scope
2020-03-31T17:29:50.2187667Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:54:40
2020-03-31T17:29:50.2187932Z    |
2020-03-31T17:29:50.2188167Z LL |     no_method_suggested_traits::Bar::X.method2();
2020-03-31T17:29:50.2188634Z    |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`
2020-03-31T17:29:50.2189251Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2189251Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2189669Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T17:29:50.2190517Z    |
2020-03-31T17:29:50.2190698Z LL |     pub trait Bar {
2020-03-31T17:29:50.2190892Z    |     ^^^^^^^^^^^^^
2020-03-31T17:29:50.2191237Z 
2020-03-31T17:29:50.2191237Z 
2020-03-31T17:29:50.2191682Z error[E0599]: no method named `method2` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>` in the current scope
2020-03-31T17:29:50.2192739Z    |
2020-03-31T17:29:50.2192739Z    |
2020-03-31T17:29:50.2193067Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method2();
2020-03-31T17:29:50.2193747Z    |                                                                          ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>`
2020-03-31T17:29:50.2194534Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2194534Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2194938Z note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
2020-03-31T17:29:50.2195805Z    |
2020-03-31T17:29:50.2195970Z LL |     pub trait Bar {
2020-03-31T17:29:50.2196274Z    |     ^^^^^^^^^^^^^
2020-03-31T17:29:50.2196398Z 
2020-03-31T17:29:50.2196398Z 
2020-03-31T17:29:50.2196665Z error[E0599]: no method named `method3` found for struct `Foo` in the current scope
2020-03-31T17:29:50.2197224Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:59:9
2020-03-31T17:29:50.2197470Z    |
2020-03-31T17:29:50.2197626Z LL | struct Foo;
2020-03-31T17:29:50.2198014Z    | ----------- method `method3` not found for this
2020-03-31T17:29:50.2198210Z ...
2020-03-31T17:29:50.2198379Z LL |     Foo.method3();
2020-03-31T17:29:50.2198780Z    |
2020-03-31T17:29:50.2199049Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2199049Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2199447Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T17:29:50.2199827Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T17:29:50.2200038Z 
2020-03-31T17:29:50.2200393Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&Foo>>` in the current scope
2020-03-31T17:29:50.2201385Z    |
2020-03-31T17:29:50.2201385Z    |
2020-03-31T17:29:50.2201612Z LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method3();
2020-03-31T17:29:50.2202107Z    |                                           ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Foo>>`
2020-03-31T17:29:50.2202720Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2202720Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2203117Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T17:29:50.2203510Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T17:29:50.2203964Z error[E0599]: no method named `method3` found for enum `Bar` in the current scope
2020-03-31T17:29:50.2204535Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:63:12
2020-03-31T17:29:50.2204839Z    |
2020-03-31T17:29:50.2204839Z    |
2020-03-31T17:29:50.2204982Z LL | enum Bar { X }
2020-03-31T17:29:50.2205364Z    | -------- method `method3` not found for this
2020-03-31T17:29:50.2205572Z ...
2020-03-31T17:29:50.2205735Z LL |     Bar::X.method3();
2020-03-31T17:29:50.2206180Z    |
2020-03-31T17:29:50.2206604Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2206604Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2207033Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T17:29:50.2207459Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T17:29:50.2207687Z 
2020-03-31T17:29:50.2208050Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&Bar>>` in the current scope
2020-03-31T17:29:50.2209043Z    |
2020-03-31T17:29:50.2209043Z    |
2020-03-31T17:29:50.2209298Z LL |     std::rc::Rc::new(&mut Box::new(&Bar::X)).method3();
2020-03-31T17:29:50.2209795Z    |                                              ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&Bar>>`
2020-03-31T17:29:50.2210462Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2210462Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2210905Z    = note: the following trait defines an item `method3`, perhaps you need to implement it:
2020-03-31T17:29:50.2211315Z            candidate #1: `no_method_suggested_traits::foo::PubPub`
2020-03-31T17:29:50.2211806Z error[E0599]: no method named `method3` found for type `usize` in the current scope
2020-03-31T17:29:50.2212427Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:69:13
2020-03-31T17:29:50.2212693Z    |
2020-03-31T17:29:50.2212693Z    |
2020-03-31T17:29:50.2212911Z LL |     1_usize.method3(); //~ ERROR no method named
2020-03-31T17:29:50.2213414Z 
2020-03-31T17:29:50.2213414Z 
2020-03-31T17:29:50.2213779Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&usize>>` in the current scope
2020-03-31T17:29:50.2214771Z    |
2020-03-31T17:29:50.2214771Z    |
2020-03-31T17:29:50.2215065Z LL |     std::rc::Rc::new(&mut Box::new(&1_usize)).method3(); //~ ERROR no method named
2020-03-31T17:29:50.2215626Z    |                                               ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&usize>>`
2020-03-31T17:29:50.2216323Z error[E0599]: no method named `method3` found for struct `no_method_suggested_traits::Foo` in the current scope
2020-03-31T17:29:50.2217009Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:71:37
2020-03-31T17:29:50.2217292Z    |
2020-03-31T17:29:50.2217292Z    |
2020-03-31T17:29:50.2217568Z LL |     no_method_suggested_traits::Foo.method3();  //~ ERROR no method named
2020-03-31T17:29:50.2218143Z    |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`
2020-03-31T17:29:50.2218471Z 
2020-03-31T17:29:50.2218895Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>` in the current scope
2020-03-31T17:29:50.2220009Z    |
2020-03-31T17:29:50.2220009Z    |
2020-03-31T17:29:50.2220313Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method3();
2020-03-31T17:29:50.2220976Z    |                                                                       ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Foo>>`
2020-03-31T17:29:50.2221796Z error[E0599]: no method named `method3` found for enum `no_method_suggested_traits::Bar` in the current scope
2020-03-31T17:29:50.2222486Z   --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:74:40
2020-03-31T17:29:50.2222774Z    |
2020-03-31T17:29:50.2222774Z    |
2020-03-31T17:29:50.2223057Z LL |     no_method_suggested_traits::Bar::X.method3();  //~ ERROR no method named
2020-03-31T17:29:50.2223554Z    |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`
2020-03-31T17:29:50.2223893Z 
2020-03-31T17:29:50.2224316Z error[E0599]: no method named `method3` found for struct `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>` in the current scope
2020-03-31T17:29:50.2225367Z    |
2020-03-31T17:29:50.2225367Z    |
2020-03-31T17:29:50.2225679Z LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method3();
2020-03-31T17:29:50.2226353Z    |                                                                          ^^^^^^^ method not found in `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>`
2020-03-31T17:29:50.2227034Z error: aborting due to 24 previous errors
2020-03-31T17:29:50.2227206Z 
2020-03-31T17:29:50.2227632Z For more information about this error, try `rustc --explain E0599`.
2020-03-31T17:29:50.2227861Z 
---
2020-03-31T17:29:50.2229221Z 
2020-03-31T17:29:50.2229317Z 
2020-03-31T17:29:50.2229538Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2230271Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10465/issue-10465.stderr
2020-03-31T17:29:50.2230837Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2231624Z To only update this specific test, also pass `--test-args issues/issue-10465.rs`
2020-03-31T17:29:50.2232031Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2232250Z status: exit code: 1
2020-03-31T17:29:50.2232250Z status: exit code: 1
2020-03-31T17:29:50.2234007Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10465.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10465" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10465/auxiliary"
2020-03-31T17:29:50.2235965Z ------------------------------------------
2020-03-31T17:29:50.2236141Z 
2020-03-31T17:29:50.2236502Z ------------------------------------------
2020-03-31T17:29:50.2236722Z stderr:
2020-03-31T17:29:50.2236722Z stderr:
2020-03-31T17:29:50.2237084Z ------------------------------------------
2020-03-31T17:29:50.2237531Z error[E0599]: no method named `foo` found for reference `&b::B` in the current scope
2020-03-31T17:29:50.2238371Z    |
2020-03-31T17:29:50.2238371Z    |
2020-03-31T17:29:50.2238687Z LL |             b.foo(); //~ ERROR: no method named `foo` found
2020-03-31T17:29:50.2239068Z    |               ^^^ method not found in `&b::B`
2020-03-31T17:29:50.2239548Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2239968Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2240315Z            `use rustc_ast::a::A;`
2020-03-31T17:29:50.2240484Z 
---
2020-03-31T17:29:50.2242869Z 
2020-03-31T17:29:50.2242965Z 
2020-03-31T17:29:50.2243174Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2243830Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35976/issue-35976.stderr
2020-03-31T17:29:50.2244446Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2245016Z To only update this specific test, also pass `--test-args issues/issue-35976.rs`
2020-03-31T17:29:50.2245459Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2245695Z status: exit code: 1
2020-03-31T17:29:50.2245695Z status: exit code: 1
2020-03-31T17:29:50.2247586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35976.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35976" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35976/auxiliary"
2020-03-31T17:29:50.2249221Z ------------------------------------------
2020-03-31T17:29:50.2249381Z 
2020-03-31T17:29:50.2249711Z ------------------------------------------
2020-03-31T17:29:50.2249914Z stderr:
2020-03-31T17:29:50.2249914Z stderr:
2020-03-31T17:29:50.2250251Z ------------------------------------------
2020-03-31T17:29:50.2250524Z error: the `wait` method cannot be invoked on a trait object
2020-03-31T17:29:50.2251013Z   --> /checkout/src/test/ui/issues/issue-35976.rs:14:9
2020-03-31T17:29:50.2251225Z    |
2020-03-31T17:29:50.2251433Z LL |         fn wait(&self) where Self: Sized;
2020-03-31T17:29:50.2252183Z ...
2020-03-31T17:29:50.2252329Z LL |     arg.wait();
2020-03-31T17:29:50.2252495Z    |         ^^^^
2020-03-31T17:29:50.2252650Z    |
---
2020-03-31T17:29:50.2255359Z 
2020-03-31T17:29:50.2255464Z 
2020-03-31T17:29:50.2255655Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2256251Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/issue-39175.stderr
2020-03-31T17:29:50.2256875Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2257665Z To only update this specific test, also pass `--test-args issues/issue-39175.rs`
2020-03-31T17:29:50.2258084Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2258302Z status: exit code: 1
2020-03-31T17:29:50.2258302Z status: exit code: 1
2020-03-31T17:29:50.2260036Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39175.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/auxiliary"
2020-03-31T17:29:50.2261467Z ------------------------------------------
2020-03-31T17:29:50.2261648Z 
2020-03-31T17:29:50.2261980Z ------------------------------------------
2020-03-31T17:29:50.2262165Z stderr:
2020-03-31T17:29:50.2262165Z stderr:
2020-03-31T17:29:50.2262518Z ------------------------------------------
2020-03-31T17:29:50.2262911Z error[E0599]: no method named `exec` found for mutable reference `&mut std::process::Command` in the current scope
2020-03-31T17:29:50.2263504Z   --> /checkout/src/test/ui/issues/issue-39175.rs:15:39
2020-03-31T17:29:50.2263735Z    |
2020-03-31T17:29:50.2263939Z LL |     Command::new("echo").arg("hello").exec();
2020-03-31T17:29:50.2264326Z    |                                       ^^^^ method not found in `&mut std::process::Command`
2020-03-31T17:29:50.2264878Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2265242Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2265508Z    |
2020-03-31T17:29:50.2265734Z LL | use rustc_ast::std::os::unix::process::CommandExt;
---
2020-03-31T17:29:50.2268251Z 
2020-03-31T17:29:50.2268340Z 
2020-03-31T17:29:50.2268529Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2269140Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/issue-43189.stderr
2020-03-31T17:29:50.2269702Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2270230Z To only update this specific test, also pass `--test-args issues/issue-43189.rs`
2020-03-31T17:29:50.2270643Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2270859Z status: exit code: 1
2020-03-31T17:29:50.2270859Z status: exit code: 1
2020-03-31T17:29:50.2275592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43189.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/auxiliary"
2020-03-31T17:29:50.2277221Z ------------------------------------------
2020-03-31T17:29:50.2277383Z 
2020-03-31T17:29:50.2277720Z ------------------------------------------
2020-03-31T17:29:50.2277984Z stderr:
2020-03-31T17:29:50.2277984Z stderr:
2020-03-31T17:29:50.2278337Z ------------------------------------------
2020-03-31T17:29:50.2278646Z error[E0599]: no method named `a` found for unit type `()` in the current scope
2020-03-31T17:29:50.2279169Z   --> /checkout/src/test/ui/issues/issue-43189.rs:10:8
2020-03-31T17:29:50.2279385Z    |
2020-03-31T17:29:50.2279526Z LL |     ().a();
2020-03-31T17:29:50.2279736Z    |        ^ method not found in `()`
2020-03-31T17:29:50.2280140Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2280520Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2280774Z    |
2020-03-31T17:29:50.2280774Z    |
2020-03-31T17:29:50.2281023Z LL | use rustc_ast::xcrate_issue_43189_b::xcrate_issue_43189_a::A;
2020-03-31T17:29:50.2281379Z 
2020-03-31T17:29:50.2281548Z error: aborting due to previous error
2020-03-31T17:29:50.2281699Z 
2020-03-31T17:29:50.2282294Z For more information about this error, try `rustc --explain E0599`.
---
2020-03-31T17:29:50.2283805Z diff of fixed:
2020-03-31T17:29:50.2283927Z 
2020-03-31T17:29:50.2284023Z 
2020-03-31T17:29:50.2284240Z The actual fixed differed from the expected fixed.
2020-03-31T17:29:50.2284918Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.fixed
2020-03-31T17:29:50.2285560Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2286178Z To only update this specific test, also pass `--test-args rust-2018/remove-extern-crate.rs`
2020-03-31T17:29:50.2286623Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2286875Z status: exit code: 0
2020-03-31T17:29:50.2286875Z status: exit code: 0
2020-03-31T17:29:50.2288912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/remove-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "--extern" "remove_extern_crate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/auxiliary"
2020-03-31T17:29:50.2290574Z ------------------------------------------
2020-03-31T17:29:50.2290746Z 
2020-03-31T17:29:50.2291119Z ------------------------------------------
2020-03-31T17:29:50.2291319Z stderr:
---
2020-03-31T17:29:50.2298593Z 
2020-03-31T17:29:50.2298681Z 
2020-03-31T17:29:50.2298871Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2299766Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/trait-import-suggestions.stderr
2020-03-31T17:29:50.2300427Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2301056Z To only update this specific test, also pass `--test-args rust-2018/trait-import-suggestions.rs`
2020-03-31T17:29:50.2301510Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2301743Z status: exit code: 1
2020-03-31T17:29:50.2301743Z status: exit code: 1
2020-03-31T17:29:50.2303899Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "--extern" "trait-import-suggestions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/auxiliary"
2020-03-31T17:29:50.2305687Z ------------------------------------------
2020-03-31T17:29:50.2305861Z 
2020-03-31T17:29:50.2306219Z ------------------------------------------
2020-03-31T17:29:50.2306438Z stderr:
2020-03-31T17:29:50.2306438Z stderr:
2020-03-31T17:29:50.2306801Z ------------------------------------------
2020-03-31T17:29:50.2307136Z error[E0599]: no method named `foobar` found for type `u32` in the current scope
2020-03-31T17:29:50.2307751Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:22:11
2020-03-31T17:29:50.2308015Z    |
2020-03-31T17:29:50.2308241Z LL |         x.foobar(); //~ ERROR no method named `foobar`
2020-03-31T17:29:50.2308757Z    |
2020-03-31T17:29:50.2309011Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2309429Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2309429Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2309826Z            `use rustc_ast::crate::foo::foobar::Foobar;`
2020-03-31T17:29:50.2310316Z error[E0599]: no method named `bar` found for type `u32` in the current scope
2020-03-31T17:29:50.2310905Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:28:7
2020-03-31T17:29:50.2311406Z    |
2020-03-31T17:29:50.2311635Z LL |     x.bar(); //~ ERROR no method named `bar`
---
2020-03-31T17:29:50.2313620Z 
2020-03-31T17:29:50.2313872Z error[E0599]: no method named `baz` found for type `u32` in the current scope
2020-03-31T17:29:50.2314500Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:29:7
2020-03-31T17:29:50.2314758Z    |
2020-03-31T17:29:50.2315019Z LL |     x.baz(); //~ ERROR no method named `baz`
2020-03-31T17:29:50.2315480Z 
2020-03-31T17:29:50.2315778Z error[E0599]: no function or associated item named `from_str` found for type `u32` in the current scope
2020-03-31T17:29:50.2316442Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:30:18
2020-03-31T17:29:50.2316706Z    |
2020-03-31T17:29:50.2316706Z    |
2020-03-31T17:29:50.2317008Z LL |     let y = u32::from_str("33"); //~ ERROR no function or associated item named `from_str`
2020-03-31T17:29:50.2317448Z    |                  ^^^^^^^^ function or associated item not found in `u32`
2020-03-31T17:29:50.2317953Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2318359Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2318631Z    |
2020-03-31T17:29:50.2318833Z LL | use rustc_ast::std::str::FromStr;
---
2020-03-31T17:29:50.2321461Z diff of stderr:
2020-03-31T17:29:50.2321582Z 
2020-03-31T17:29:50.2321678Z 
2020-03-31T17:29:50.2321899Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2322600Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods/shadowed-trait-methods.stderr
2020-03-31T17:29:50.2323254Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2323878Z To only update this specific test, also pass `--test-args shadowed/shadowed-trait-methods.rs`
2020-03-31T17:29:50.2324327Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2324579Z status: exit code: 1
2020-03-31T17:29:50.2324579Z status: exit code: 1
2020-03-31T17:29:50.2326550Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/shadowed/shadowed-trait-methods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods/auxiliary"
2020-03-31T17:29:50.2328169Z ------------------------------------------
2020-03-31T17:29:50.2328341Z 
2020-03-31T17:29:50.2328719Z ------------------------------------------
2020-03-31T17:29:50.2328922Z stderr:
2020-03-31T17:29:50.2328922Z stderr:
2020-03-31T17:29:50.2329285Z ------------------------------------------
2020-03-31T17:29:50.2329636Z error[E0599]: no method named `f` found for unit type `()` in the current scope
2020-03-31T17:29:50.2330220Z   --> /checkout/src/test/ui/shadowed/shadowed-trait-methods.rs:13:8
2020-03-31T17:29:50.2330475Z    |
2020-03-31T17:29:50.2330677Z LL |     ().f() //~ ERROR no method
2020-03-31T17:29:50.2330919Z    |        ^ method not found in `()`
2020-03-31T17:29:50.2331369Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2331820Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2332093Z    |
2020-03-31T17:29:50.2332293Z LL | use rustc_ast::foo::T;
---
2020-03-31T17:29:50.2335031Z diff of stderr:
2020-03-31T17:29:50.2335170Z 
2020-03-31T17:29:50.2335265Z 
2020-03-31T17:29:50.2335468Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2336135Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy/trait-item-privacy.stderr
2020-03-31T17:29:50.2336793Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2337380Z To only update this specific test, also pass `--test-args traits/trait-item-privacy.rs`
2020-03-31T17:29:50.2337840Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2338073Z status: exit code: 1
2020-03-31T17:29:50.2338073Z status: exit code: 1
2020-03-31T17:29:50.2340012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-item-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy/auxiliary"
2020-03-31T17:29:50.2341604Z ------------------------------------------
2020-03-31T17:29:50.2341777Z 
2020-03-31T17:29:50.2342137Z ------------------------------------------
2020-03-31T17:29:50.2342339Z stderr:
2020-03-31T17:29:50.2342339Z stderr:
2020-03-31T17:29:50.2342724Z ------------------------------------------
2020-03-31T17:29:50.2343052Z error[E0599]: no method named `a` found for struct `S` in the current scope
2020-03-31T17:29:50.2343870Z    |
2020-03-31T17:29:50.2344018Z LL | struct S;
2020-03-31T17:29:50.2344404Z    | --------- method `a` not found for this
2020-03-31T17:29:50.2344621Z ...
2020-03-31T17:29:50.2344621Z ...
2020-03-31T17:29:50.2344833Z LL |     S.a(); //~ ERROR no method named `a` found
2020-03-31T17:29:50.2345100Z    |       ^ method not found in `S`
2020-03-31T17:29:50.2345579Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2345579Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2345983Z note: `method::A` defines an item `a`, perhaps you need to implement it
2020-03-31T17:29:50.2346791Z    |
2020-03-31T17:29:50.2346946Z LL |     trait A {
2020-03-31T17:29:50.2347142Z    |     ^^^^^^^
2020-03-31T17:29:50.2347271Z 
2020-03-31T17:29:50.2347271Z 
2020-03-31T17:29:50.2347522Z error[E0599]: no method named `b` found for struct `S` in the current scope
2020-03-31T17:29:50.2348080Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:68:7
2020-03-31T17:29:50.2348338Z    |
2020-03-31T17:29:50.2348484Z LL | struct S;
2020-03-31T17:29:50.2348870Z    | --------- method `b` not found for this
2020-03-31T17:29:50.2349086Z ...
2020-03-31T17:29:50.2349257Z LL |         fn b(&self) { }
2020-03-31T17:29:50.2349797Z    |            |
2020-03-31T17:29:50.2349797Z    |            |
2020-03-31T17:29:50.2350090Z    |            the method is available for `std::boxed::Box<S>` here
2020-03-31T17:29:50.2350497Z    |            the method is available for `std::sync::Arc<S>` here
2020-03-31T17:29:50.2350974Z    |            the method is available for `std::rc::Rc<S>` here
2020-03-31T17:29:50.2351430Z ...
2020-03-31T17:29:50.2351643Z LL |     S.b(); //~ ERROR no method named `b` found
2020-03-31T17:29:50.2351926Z    |       ^ method not found in `S`
2020-03-31T17:29:50.2352415Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2352813Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2353106Z    |
2020-03-31T17:29:50.2353294Z LL | use rustc_ast::method::B;
2020-03-31T17:29:50.2353294Z LL | use rustc_ast::method::B;
2020-03-31T17:29:50.2353481Z    |
2020-03-31T17:29:50.2353720Z 
2020-03-31T17:29:50.2353909Z error[E0624]: associated function `a` is private
2020-03-31T17:29:50.2354420Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:72:7
2020-03-31T17:29:50.2354659Z    |
2020-03-31T17:29:50.2354871Z LL |     c.a(); //~ ERROR associated function `a` is private
2020-03-31T17:29:50.2355142Z    |       ^ private associated function
2020-03-31T17:29:50.2355294Z 
2020-03-31T17:29:50.2355575Z error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
2020-03-31T17:29:50.2356352Z    |
2020-03-31T17:29:50.2356503Z LL | struct S;
2020-03-31T17:29:50.2356911Z    | --------- function or associated item `a` not found for this
2020-03-31T17:29:50.2357126Z ...
2020-03-31T17:29:50.2357126Z ...
2020-03-31T17:29:50.2357294Z LL |     S::a(&S);
2020-03-31T17:29:50.2357530Z    |        ^ function or associated item not found in `S`
2020-03-31T17:29:50.2358006Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2358006Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2358373Z note: `method::A` defines an item `a`, perhaps you need to implement it
2020-03-31T17:29:50.2359121Z    |
2020-03-31T17:29:50.2359264Z LL |     trait A {
2020-03-31T17:29:50.2359426Z    |     ^^^^^^^
2020-03-31T17:29:50.2359541Z 
2020-03-31T17:29:50.2359541Z 
2020-03-31T17:29:50.2359820Z error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
2020-03-31T17:29:50.2360371Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:80:8
2020-03-31T17:29:50.2360594Z    |
2020-03-31T17:29:50.2360745Z LL | struct S;
2020-03-31T17:29:50.2361154Z    | --------- function or associated item `b` not found for this
2020-03-31T17:29:50.2361370Z ...
2020-03-31T17:29:50.2361531Z LL |     S::b(&S);
2020-03-31T17:29:50.2361764Z    |        ^ function or associated item not found in `S`
2020-03-31T17:29:50.2362213Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2362575Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2362826Z    |
2020-03-31T17:29:50.2363015Z LL | use rustc_ast::method::B;
2020-03-31T17:29:50.2363015Z LL | use rustc_ast::method::B;
2020-03-31T17:29:50.2363193Z    |
2020-03-31T17:29:50.2363288Z 
2020-03-31T17:29:50.2363478Z error[E0624]: associated function `a` is private
2020-03-31T17:29:50.2363970Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:84:8
2020-03-31T17:29:50.2364192Z    |
2020-03-31T17:29:50.2364416Z LL |     C::a(&S); //~ ERROR associated function `a` is private
2020-03-31T17:29:50.2364710Z    |        ^ private associated function
2020-03-31T17:29:50.2364865Z 
2020-03-31T17:29:50.2365110Z error[E0599]: no associated item named `A` found for struct `S` in the current scope
2020-03-31T17:29:50.2365877Z    |
2020-03-31T17:29:50.2366013Z LL | struct S;
2020-03-31T17:29:50.2366013Z LL | struct S;
2020-03-31T17:29:50.2366395Z    | --------- associated item `A` not found for this
2020-03-31T17:29:50.2366608Z ...
2020-03-31T17:29:50.2366820Z LL |     S::A; //~ ERROR no associated item named `A` found
2020-03-31T17:29:50.2367102Z    |        ^ associated item not found in `S`
2020-03-31T17:29:50.2367626Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2367626Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-03-31T17:29:50.2368004Z note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
2020-03-31T17:29:50.2368883Z    |
2020-03-31T17:29:50.2369007Z LL |     trait A {
2020-03-31T17:29:50.2369162Z    |     ^^^^^^^
2020-03-31T17:29:50.2369264Z 
2020-03-31T17:29:50.2369264Z 
2020-03-31T17:29:50.2369478Z error[E0599]: no associated item named `B` found for struct `S` in the current scope
2020-03-31T17:29:50.2369947Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:98:8
2020-03-31T17:29:50.2370155Z    |
2020-03-31T17:29:50.2370274Z LL | struct S;
2020-03-31T17:29:50.2370800Z    | --------- associated item `B` not found for this
2020-03-31T17:29:50.2371014Z ...
2020-03-31T17:29:50.2371229Z LL |     S::B; //~ ERROR no associated item named `B` found
2020-03-31T17:29:50.2371512Z    |        ^ associated item not found in `S`
2020-03-31T17:29:50.2371944Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2372307Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2372577Z    |
2020-03-31T17:29:50.2372763Z LL | use rustc_ast::assoc_const::B;
2020-03-31T17:29:50.2372763Z LL | use rustc_ast::assoc_const::B;
2020-03-31T17:29:50.2372946Z    |
2020-03-31T17:29:50.2373042Z 
2020-03-31T17:29:50.2373246Z error[E0624]: associated constant `A` is private
2020-03-31T17:29:50.2373726Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:101:8
2020-03-31T17:29:50.2373950Z    |
2020-03-31T17:29:50.2374180Z LL |     C::A; //~ ERROR associated constant `A` is private
2020-03-31T17:29:50.2374452Z    |        ^ private associated constant
2020-03-31T17:29:50.2374841Z error[E0038]: the trait `assoc_const::C` cannot be made into an object
2020-03-31T17:29:50.2375384Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:101:5
2020-03-31T17:29:50.2375613Z    |
2020-03-31T17:29:50.2375784Z LL |         const A: u8 = 0;
2020-03-31T17:29:50.2375784Z LL |         const A: u8 = 0;
2020-03-31T17:29:50.2376250Z    |               - ...because it contains this associated `const`
2020-03-31T17:29:50.2376469Z ...
2020-03-31T17:29:50.2376637Z LL |         const B: u8 = 0;
2020-03-31T17:29:50.2377099Z    |               - ...because it contains this associated `const`
2020-03-31T17:29:50.2377316Z ...
2020-03-31T17:29:50.2377484Z LL |     pub trait C: A + B {
2020-03-31T17:29:50.2378402Z LL |         const C: u8 = 0;
2020-03-31T17:29:50.2378881Z    |               - ...because it contains this associated `const`
2020-03-31T17:29:50.2379134Z ...
2020-03-31T17:29:50.2379134Z ...
2020-03-31T17:29:50.2379365Z LL |     C::A; //~ ERROR associated constant `A` is private
2020-03-31T17:29:50.2379730Z    |     ^^^^ the trait `assoc_const::C` cannot be made into an object
2020-03-31T17:29:50.2380218Z    = help: consider moving `C` to another trait
2020-03-31T17:29:50.2380504Z    = help: consider moving `B` to another trait
2020-03-31T17:29:50.2380806Z    = help: consider moving `A` to another trait
2020-03-31T17:29:50.2380990Z 
2020-03-31T17:29:50.2380990Z 
2020-03-31T17:29:50.2381180Z error[E0223]: ambiguous associated type
2020-03-31T17:29:50.2381687Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:115:12
2020-03-31T17:29:50.2381947Z    |
2020-03-31T17:29:50.2382181Z LL |     let _: S::A; //~ ERROR ambiguous associated type
2020-03-31T17:29:50.2382760Z    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
2020-03-31T17:29:50.2383225Z error[E0223]: ambiguous associated type
2020-03-31T17:29:50.2383722Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:116:12
2020-03-31T17:29:50.2383967Z    |
2020-03-31T17:29:50.2383967Z    |
2020-03-31T17:29:50.2384321Z LL |     let _: S::B; //~ ERROR ambiguous associated type
2020-03-31T17:29:50.2384856Z    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::B`
2020-03-31T17:29:50.2385340Z error[E0223]: ambiguous associated type
2020-03-31T17:29:50.2385811Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:117:12
2020-03-31T17:29:50.2386037Z    |
2020-03-31T17:29:50.2386037Z    |
2020-03-31T17:29:50.2386267Z LL |     let _: S::C; //~ ERROR ambiguous associated type
2020-03-31T17:29:50.2386845Z    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::C`
2020-03-31T17:29:50.2387255Z error: associated type `A` is private
2020-03-31T17:29:50.2387735Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:119:12
2020-03-31T17:29:50.2387961Z    |
2020-03-31T17:29:50.2387961Z    |
2020-03-31T17:29:50.2388184Z LL |     let _: T::A; //~ ERROR associated type `A` is private
2020-03-31T17:29:50.2388642Z 
2020-03-31T17:29:50.2388809Z error: associated type `A` is private
2020-03-31T17:29:50.2389277Z   --> /checkout/src/test/ui/traits/trait-item-privacy.rs:128:9
2020-03-31T17:29:50.2389504Z    |
2020-03-31T17:29:50.2389504Z    |
2020-03-31T17:29:50.2389900Z LL |         A = u8, //~ ERROR associated type `A` is private
2020-03-31T17:29:50.2390484Z 
2020-03-31T17:29:50.2390658Z error: aborting due to 15 previous errors
2020-03-31T17:29:50.2390815Z 
2020-03-31T17:29:50.2391199Z Some errors have detailed explanations: E0038, E0223, E0599, E0624.
---
2020-03-31T17:29:50.2394499Z 
2020-03-31T17:29:50.2394595Z 
2020-03-31T17:29:50.2394800Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2395479Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-method-private/trait-method-private.stderr
2020-03-31T17:29:50.2396146Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2396738Z To only update this specific test, also pass `--test-args traits/trait-method-private.rs`
2020-03-31T17:29:50.2397201Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2397439Z status: exit code: 1
2020-03-31T17:29:50.2397439Z status: exit code: 1
2020-03-31T17:29:50.2399377Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-method-private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-method-private" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-method-private/auxiliary"
2020-03-31T17:29:50.2401001Z ------------------------------------------
2020-03-31T17:29:50.2401175Z 
2020-03-31T17:29:50.2401530Z ------------------------------------------
2020-03-31T17:29:50.2401731Z stderr:
2020-03-31T17:29:50.2401731Z stderr:
2020-03-31T17:29:50.2402115Z ------------------------------------------
2020-03-31T17:29:50.2402401Z error[E0624]: associated function `method` is private
2020-03-31T17:29:50.2402924Z   --> /checkout/src/test/ui/traits/trait-method-private.rs:19:9
2020-03-31T17:29:50.2403189Z    |
2020-03-31T17:29:50.2403388Z LL |     foo.method(); //~ ERROR is private
2020-03-31T17:29:50.2403876Z    |
2020-03-31T17:29:50.2404126Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2404549Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2404932Z    |
---
2020-03-31T17:29:50.2407780Z diff of stderr:
2020-03-31T17:29:50.2407903Z 
2020-03-31T17:29:50.2407999Z 
2020-03-31T17:29:50.2408203Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2408973Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene/hygiene.stderr
2020-03-31T17:29:50.2409546Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2410116Z To only update this specific test, also pass `--test-args underscore-imports/hygiene.rs`
2020-03-31T17:29:50.2410523Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2410739Z status: exit code: 1
2020-03-31T17:29:50.2410739Z status: exit code: 1
2020-03-31T17:29:50.2412545Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene/auxiliary"
2020-03-31T17:29:50.2414014Z ------------------------------------------
2020-03-31T17:29:50.2414179Z 
2020-03-31T17:29:50.2414526Z ------------------------------------------
2020-03-31T17:29:50.2414713Z stderr:
2020-03-31T17:29:50.2414713Z stderr:
2020-03-31T17:29:50.2415049Z ------------------------------------------
2020-03-31T17:29:50.2415366Z error[E0599]: no method named `deref` found for reference `&()` in the current scope
2020-03-31T17:29:50.2415917Z   --> /checkout/src/test/ui/underscore-imports/hygiene.rs:38:11
2020-03-31T17:29:50.2416139Z    |
2020-03-31T17:29:50.2416381Z LL |     (&()).deref();              //~ ERROR no method named `deref`
2020-03-31T17:29:50.2416675Z    |           ^^^^^ method not found in `&()`
2020-03-31T17:29:50.2417155Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2417485Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2417702Z    |
2020-03-31T17:29:50.2417862Z LL | use rustc_ast::std::ops::Deref;
2020-03-31T17:29:50.2417862Z LL | use rustc_ast::std::ops::Deref;
2020-03-31T17:29:50.2418039Z    |
2020-03-31T17:29:50.2418121Z 
2020-03-31T17:29:50.2418359Z error[E0599]: no method named `deref_mut` found for mutable reference `&mut ()` in the current scope
2020-03-31T17:29:50.2418858Z   --> /checkout/src/test/ui/underscore-imports/hygiene.rs:39:15
2020-03-31T17:29:50.2419051Z    |
2020-03-31T17:29:50.2419257Z LL |     (&mut ()).deref_mut();      //~ ERROR no method named `deref_mut`
2020-03-31T17:29:50.2419550Z    |               ^^^^^^^^^ method not found in `&mut ()`
2020-03-31T17:29:50.2419927Z    = help: items from traits can only be used if the trait is in scope
2020-03-31T17:29:50.2420257Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-31T17:29:50.2420474Z    |
2020-03-31T17:29:50.2420639Z LL | use rustc_ast::std::ops::DerefMut;
---
2020-03-31T17:29:50.2422826Z diff of stderr:
2020-03-31T17:29:50.2422928Z 
2020-03-31T17:29:50.2423004Z 
2020-03-31T17:29:50.2423184Z The actual stderr differed from the expected stderr.
2020-03-31T17:29:50.2423705Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/shadow/shadow.stderr
2020-03-31T17:29:50.2424199Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T17:29:50.2424685Z To only update this specific test, also pass `--test-args underscore-imports/shadow.rs`
2020-03-31T17:29:50.2425037Z error: 1 errors occurred comparing output.
2020-03-31T17:29:50.2425241Z status: exit code: 1
2020-03-31T17:29:50.2425241Z status: exit code: 1
2020-03-31T17:29:50.2426794Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/shadow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/shadow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/shadow/auxiliary"
2020-03-31T17:29:50.2428077Z ------------------------------------------
2020-03-31T17:29:50.2428215Z 
2020-03-31T17:29:50.2428517Z ------------------------------------------
2020-03-31T17:29:50.2428680Z stderr:
---
2020-03-31T17:29:50.2440107Z test result: FAILED. 9780 passed; 16 failed; 60 ignored; 0 measured; 0 filtered out
2020-03-31T17:29:50.2440375Z 
2020-03-31T17:29:50.2440465Z 
2020-03-31T17:29:50.2440553Z 
2020-03-31T17:29:50.2443985Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-31T17:29:50.2446439Z 
2020-03-31T17:29:50.2446531Z 
2020-03-31T17:29:50.2446983Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-31T17:29:50.2447384Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T17:29:50.2447384Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T17:29:50.2451999Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T17:29:50.2452401Z Build completed unsuccessfully in 0:58:20
2020-03-31T17:29:50.2453153Z == clock drift check ==
2020-03-31T17:29:50.2453417Z   local time: Tue Mar 31 17:29:50 UTC 2020
2020-03-31T17:29:50.5118451Z   network time: Tue, 31 Mar 2020 17:29:50 GMT
2020-03-31T17:29:50.5121775Z == end clock drift check ==
2020-03-31T17:29:51.0981644Z 
2020-03-31T17:29:51.1051449Z ##[error]Bash exited with code '1'.
2020-03-31T17:29:51.1065070Z ##[section]Finishing: Run build
2020-03-31T17:29:51.1112087Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70621/merge to s
2020-03-31T17:29:51.1117209Z Task         : Get sources
2020-03-31T17:29:51.1117552Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T17:29:51.1117875Z Version      : 1.0.0
2020-03-31T17:29:51.1118115Z Author       : Microsoft
2020-03-31T17:29:51.1118115Z Author       : Microsoft
2020-03-31T17:29:51.1118471Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T17:29:51.1118874Z ==============================================================================
2020-03-31T17:29:51.4502623Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T17:29:51.4549499Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70621/merge to s
2020-03-31T17:29:51.4646981Z Cleaning up task key
2020-03-31T17:29:51.4648503Z Start cleaning up orphan processes.
2020-03-31T17:29:51.4827207Z Terminate orphan process: pid (4003) (python)
2020-03-31T17:29:51.4994733Z ##[section]Finishing: Finalize Job
