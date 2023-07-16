plain
2019-12-07T06:26:09.4877126Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-07T06:26:09.4904553Z ##[command]git config gc.auto 0
2019-12-07T06:26:09.4909462Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-07T06:26:09.4914499Z ##[command]git config --get-all http.proxy
2019-12-07T06:26:09.4920608Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67077/merge:refs/remotes/pull/67077/merge
---
2019-12-07T07:24:52.8452680Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-07T07:25:41.8918426Z    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-12-07T07:28:21.8847609Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2019-12-07T07:28:22.4905685Z     Finished release [optimized] target(s) in 21m 43s
2019-12-07T07:28:22.5604292Z Installing libLLVM.so to stage 0 (x86_64-unknown-linux-gnu)
2019-12-07T07:28:23.9810991Z Building LLD for x86_64-unknown-linux-gnu
2019-12-07T07:28:23.9839058Z running: "cmake" "/checkout/src/llvm-project/lld" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm -static-libstdc++" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DLLVM_CONFIG_PATH=/checkout/obj/build/bootstrap/debug/llvm-config-wrapper" "-DLLVM_INCLUDE_TESTS=OFF" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/lld" "-DCMAKE_BUILD_TYPE=Release"
2019-12-07T07:28:24.1068785Z -- The C compiler identification is Clang 9.0.0
2019-12-07T07:28:24.2283737Z -- The CXX compiler identification is Clang 9.0.0
---
2019-12-07T08:04:17.2318384Z     Checking once_cell v1.1.0
2019-12-07T08:04:17.2761112Z  Documenting rustc_fs_util v0.0.0 (/checkout/src/librustc_fs_util)
2019-12-07T08:04:17.3017877Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:17.3018176Z   |
2019-12-07T08:04:17.3019111Z   = warning: please see ***/issues/44136
2019-12-07T08:04:17.3032184Z  Documenting graphviz v0.0.0 (/checkout/src/libgraphviz)
2019-12-07T08:04:17.3268860Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:17.3269139Z   |
2019-12-07T08:04:17.3269139Z   |
2019-12-07T08:04:17.3269573Z   = warning: please see ***/issues/44136
2019-12-07T08:04:17.3280587Z  Documenting rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2019-12-07T08:04:17.3491990Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:17.3492248Z   |
2019-12-07T08:04:17.3492248Z   |
2019-12-07T08:04:17.3492641Z   = warning: please see ***/issues/44136
2019-12-07T08:04:17.3503900Z  Documenting build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-07T08:04:17.3717143Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:17.3717474Z   |
2019-12-07T08:04:17.3717474Z   |
2019-12-07T08:04:17.3717862Z   = warning: please see ***/issues/44136
2019-12-07T08:04:17.3730936Z     Checking crossbeam-utils v0.6.5
2019-12-07T08:04:17.3985430Z     Checking log_settings v0.1.2
2019-12-07T08:04:17.4226464Z     Checking lock_api v0.3.1
2019-12-07T08:04:17.4487462Z     Checking arrayvec v0.4.7
2019-12-07T08:04:17.4487462Z     Checking arrayvec v0.4.7
2019-12-07T08:04:17.7630733Z     Checking serialize v0.0.0 (/checkout/src/libserialize)
2019-12-07T08:04:17.7883626Z  Documenting serialize v0.0.0 (/checkout/src/libserialize)
2019-12-07T08:04:17.8301480Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:17.8301791Z   |
2019-12-07T08:04:17.8302220Z   = warning: please see ***/issues/44136
2019-12-07T08:04:17.8337644Z     Checking itertools v0.8.0
2019-12-07T08:04:17.8575052Z    Compiling rustc_target v0.0.0 (/checkout/src/librustc_target)
2019-12-07T08:04:17.8665895Z    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
2019-12-07T08:04:17.8729666Z    Compiling rustc v0.0.0 (/checkout/src/librustc)
---
2019-12-07T08:04:33.2586123Z     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2019-12-07T08:04:33.2879580Z  Documenting rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2019-12-07T08:04:33.3109277Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:33.3110587Z   |
2019-12-07T08:04:33.3111428Z   = warning: please see ***/issues/44136
2019-12-07T08:04:33.3168619Z     Checking rustc_index v0.0.0 (/checkout/src/librustc_index)
2019-12-07T08:04:34.8867112Z  Documenting rustc_index v0.0.0 (/checkout/src/librustc_index)
2019-12-07T08:04:34.9128825Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:34.9129794Z   |
2019-12-07T08:04:34.9129794Z   |
2019-12-07T08:04:34.9130573Z   = warning: please see ***/issues/44136
2019-12-07T08:04:36.1969016Z     Checking memoffset v0.5.1
2019-12-07T08:04:39.6540962Z     Checking rls-span v0.5.1
2019-12-07T08:04:39.6884344Z     Checking ena v0.13.1
2019-12-07T08:04:39.7158776Z     Checking env_logger v0.7.1
2019-12-07T08:04:39.7158776Z     Checking env_logger v0.7.1
2019-12-07T08:04:40.6026950Z     Checking rls-data v0.19.0
2019-12-07T08:04:42.0885360Z     Checking rustc-hash v1.0.1
2019-12-07T08:04:42.1213200Z     Checking rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
2019-12-07T08:04:42.5091381Z  Documenting rustc_lexer v0.1.0 (/checkout/src/librustc_lexer)
2019-12-07T08:04:42.5426306Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:42.5426662Z   |
2019-12-07T08:04:42.5427082Z   = warning: please see ***/issues/44136
2019-12-07T08:04:42.5564126Z     Checking polonius-engine v0.10.0
2019-12-07T08:04:43.3013465Z  Documenting rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-12-07T08:04:43.3303990Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:43.3304442Z   |
2019-12-07T08:04:43.3304442Z   |
2019-12-07T08:04:43.3304954Z   = warning: please see ***/issues/44136
2019-12-07T08:04:43.3395205Z     Checking crossbeam-epoch v0.7.2
2019-12-07T08:04:43.3673821Z     Checking serde_json v1.0.40
2019-12-07T08:04:43.6272460Z     Checking miniz_oxide v0.3.5
2019-12-07T08:04:44.0511094Z     Checking num_cpus v1.10.1
---
2019-12-07T08:04:51.6199648Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-07T08:04:51.6428779Z  Documenting rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-07T08:04:51.6702845Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:51.6703131Z   |
2019-12-07T08:04:51.6703576Z   = warning: please see ***/issues/44136
2019-12-07T08:04:54.1181843Z warning: `[0]` cannot be resolved, ignoring it.
2019-12-07T08:04:54.1182228Z    --> src/librustc_data_structures/obligation_forest/mod.rs:178:29
2019-12-07T08:04:54.1182439Z     |
2019-12-07T08:04:54.1182740Z 178 |     /// If true, dependents[0] points to a "parent" node, which requires
---
2019-12-07T08:04:56.3450033Z     Checking arena v0.0.0 (/checkout/src/libarena)
2019-12-07T08:04:56.3746455Z  Documenting arena v0.0.0 (/checkout/src/libarena)
2019-12-07T08:04:56.4022374Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:56.4022717Z   |
2019-12-07T08:04:56.4023239Z   = warning: please see ***/issues/44136
2019-12-07T08:04:57.7367719Z     Checking rand v0.7.0
2019-12-07T08:04:57.7645108Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-07T08:04:59.3124700Z  Documenting syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-12-07T08:04:59.3353715Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:59.3353715Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:04:59.3354062Z   |
2019-12-07T08:04:59.3354456Z   = warning: please see ***/issues/44136
2019-12-07T08:04:59.3423588Z     Checking tempfile v3.1.0
2019-12-07T08:04:59.8873775Z     Checking synstructure v0.12.1
2019-12-07T08:05:00.5631541Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2019-12-07T08:05:01.0730727Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
---
2019-12-07T08:05:01.5709379Z 
2019-12-07T08:05:01.9172610Z  Documenting rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2019-12-07T08:05:01.9426985Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:01.9428166Z   |
2019-12-07T08:05:01.9428857Z   = warning: please see ***/issues/44136
2019-12-07T08:05:02.1815917Z  Documenting rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-12-07T08:05:02.2067362Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:02.2068649Z   |
2019-12-07T08:05:02.2068649Z   |
2019-12-07T08:05:02.2069288Z   = warning: please see ***/issues/44136
2019-12-07T08:05:02.5378252Z  Documenting fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-12-07T08:05:02.5623115Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:02.5623420Z   |
2019-12-07T08:05:02.5623420Z   |
2019-12-07T08:05:02.5623865Z   = warning: please see ***/issues/44136
2019-12-07T08:05:02.5713098Z  Documenting rustc_target v0.0.0 (/checkout/src/librustc_target)
2019-12-07T08:05:02.5939438Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:02.5939741Z   |
2019-12-07T08:05:02.5939741Z   |
2019-12-07T08:05:02.5940185Z   = warning: please see ***/issues/44136
2019-12-07T08:05:04.1725660Z  Documenting rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-12-07T08:05:08.1177883Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2019-12-07T08:05:08.1685552Z  Documenting rustc_session v0.0.0 (/checkout/src/librustc_session)
2019-12-07T08:05:08.1919114Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:08.1919114Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:08.1919469Z   |
2019-12-07T08:05:08.1919897Z   = warning: please see ***/issues/44136
2019-12-07T08:05:09.8930361Z  Documenting syntax v0.0.0 (/checkout/src/libsyntax)
2019-12-07T08:05:09.9174002Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:09.9174378Z   |
2019-12-07T08:05:09.9174378Z   |
2019-12-07T08:05:09.9174893Z   = warning: please see ***/issues/44136
2019-12-07T08:05:16.4276902Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-12-07T08:05:17.1200185Z warning: `[MacDelimeter]` cannot be resolved, ignoring it.
2019-12-07T08:05:17.1200648Z   --> src/libsyntax/ast.rs:17:49
2019-12-07T08:05:17.1200901Z    |
---
2019-12-07T08:05:17.1617210Z 
2019-12-07T08:05:20.7485624Z  Documenting rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-12-07T08:05:20.8174102Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:20.8174464Z   |
2019-12-07T08:05:20.8174864Z   = warning: please see ***/issues/44136
2019-12-07T08:05:20.8401040Z  Documenting rustc v0.0.0 (/checkout/src/librustc)
2019-12-07T08:05:20.8660521Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:05:20.8660946Z   |
2019-12-07T08:05:20.8660946Z   |
2019-12-07T08:05:20.8661369Z   = warning: please see ***/issues/44136
2019-12-07T08:05:23.3132805Z error: unknown start of token: `
2019-12-07T08:05:23.3133594Z  --> <doctest>:1:13
2019-12-07T08:05:23.3133683Z   |
2019-12-07T08:05:23.3133800Z 1 | USE_TREE = [`::`] `*` |
---
2019-12-07T08:05:23.6091245Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-07T08:06:02.9738659Z  Documenting syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-07T08:06:03.0002259Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:06:03.0003224Z   |
2019-12-07T08:06:03.0003923Z   = warning: please see ***/issues/44136
2019-12-07T08:06:06.1793646Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-12-07T08:06:06.2270353Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-07T08:06:14.6933498Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-12-07T08:06:14.7253710Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
---
2019-12-07T08:06:20.0674141Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-12-07T08:06:20.0983161Z  Documenting rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-12-07T08:06:20.1252210Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:06:20.1252510Z   |
2019-12-07T08:06:20.1252917Z   = warning: please see ***/issues/44136
2019-12-07T08:06:20.1283648Z  Documenting rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
2019-12-07T08:06:20.1625770Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:06:20.1626100Z   |
2019-12-07T08:06:20.1626100Z   |
2019-12-07T08:06:20.1626495Z   = warning: please see ***/issues/44136
2019-12-07T08:06:34.8558780Z  Documenting rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-07T08:06:34.8803115Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:06:34.8804175Z   |
2019-12-07T08:06:34.8804175Z   |
2019-12-07T08:06:34.8804975Z   = warning: please see ***/issues/44136
2019-12-07T08:06:34.8871888Z  Documenting rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-12-07T08:06:34.9413196Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:06:34.9414107Z   |
2019-12-07T08:06:34.9414107Z   |
2019-12-07T08:06:34.9415617Z   = warning: please see ***/issues/44136
2019-12-07T08:06:34.9562671Z  Documenting rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-12-07T08:06:35.0421596Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:06:35.0424301Z   |
2019-12-07T08:06:35.0424301Z   |
2019-12-07T08:06:35.0424757Z   = warning: please see ***/issues/44136
2019-12-07T08:06:50.2014787Z warning: `[link_reborrowed_region]` cannot be resolved, ignoring it.
2019-12-07T08:06:50.2015163Z     --> src/librustc_typeck/check/regionck.rs:1281:45
2019-12-07T08:06:50.2015781Z      |
2019-12-07T08:06:50.2019928Z 1281 |     /// of the borrow that's provided. See [link_reborrowed_region] for some
---
2019-12-07T08:06:50.2411826Z 
2019-12-07T08:06:50.9879803Z  Documenting rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-12-07T08:06:51.0136354Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:06:51.0136738Z   |
2019-12-07T08:06:51.0137183Z   = warning: please see ***/issues/44136
2019-12-07T08:07:02.5276179Z warning: `[rustc::traits::query::implied_outlives_bounds]` cannot be resolved, ignoring it.
2019-12-07T08:07:02.5276565Z  --> src/librustc_traits/implied_outlives_bounds.rs:2:44
2019-12-07T08:07:02.5276771Z   |
2019-12-07T08:07:02.5277064Z 2 | //! Do not call this query directory. See [`rustc::traits::query::implied_outlives_bounds`].
---
2019-12-07T08:07:02.8267392Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-07T08:07:02.8587909Z  Documenting rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
2019-12-07T08:07:02.9027342Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:02.9027681Z   |
2019-12-07T08:07:02.9028152Z   = warning: please see ***/issues/44136
2019-12-07T08:07:11.7786149Z  Documenting syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-07T08:07:11.8044696Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:11.8045014Z   |
2019-12-07T08:07:11.8045014Z   |
2019-12-07T08:07:11.8045495Z   = warning: please see ***/issues/44136
2019-12-07T08:07:11.8069999Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-12-07T08:07:11.8408445Z  Documenting rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-12-07T08:07:11.8660251Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:11.8660654Z   |
2019-12-07T08:07:11.8660654Z   |
2019-12-07T08:07:11.8661127Z   = warning: please see ***/issues/44136
2019-12-07T08:07:24.8747344Z warning: `[lifetime]` cannot be resolved, ignoring it.
2019-12-07T08:07:24.8748627Z    --> src/libsyntax_ext/deriving/generic/ty.rs:103:25
2019-12-07T08:07:24.8749387Z     |
2019-12-07T08:07:24.8750473Z 103 |     /// mod::mod::Type<[lifetime], [Params...]>, including a plain type
---
2019-12-07T08:07:28.8077760Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-12-07T08:07:28.8430980Z  Documenting rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-12-07T08:07:28.9342803Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:28.9343265Z   |
2019-12-07T08:07:28.9343682Z   = warning: please see ***/issues/44136
2019-12-07T08:07:29.1450341Z  Documenting rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-12-07T08:07:29.1709306Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:29.1709637Z   |
2019-12-07T08:07:29.1709637Z   |
2019-12-07T08:07:29.1710122Z   = warning: please see ***/issues/44136
2019-12-07T08:07:29.1722091Z  Documenting rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-12-07T08:07:29.2024273Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:29.2024605Z   |
2019-12-07T08:07:29.2024605Z   |
2019-12-07T08:07:29.2025160Z   = warning: please see ***/issues/44136
2019-12-07T08:07:29.9538871Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-12-07T08:07:29.9856348Z  Documenting rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-12-07T08:07:30.0107918Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:30.0108322Z   |
2019-12-07T08:07:30.0108322Z   |
2019-12-07T08:07:30.0108784Z   = warning: please see ***/issues/44136
2019-12-07T08:07:34.2010920Z warning: `[rustc_error]` cannot be resolved, ignoring it.
2019-12-07T08:07:34.2011636Z   --> src/librustc_codegen_utils/lib.rs:38:21
2019-12-07T08:07:34.2011859Z    |
2019-12-07T08:07:34.2012173Z 38 | /// check for the #[rustc_error] annotation, which forces an
---
2019-12-07T08:07:35.7333219Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-07T08:07:35.7616266Z  Documenting rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-07T08:07:35.7880496Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:35.7880870Z   |
2019-12-07T08:07:35.7881289Z   = warning: please see ***/issues/44136
2019-12-07T08:07:38.6875753Z error: unknown start of token: `
2019-12-07T08:07:38.6877673Z  --> <doctest>:3:27
2019-12-07T08:07:38.6877937Z   |
2019-12-07T08:07:38.6883794Z 3 |    |     ^^^ did you mean `self::foo`?
---
2019-12-07T08:07:38.7490629Z 
2019-12-07T08:07:40.1713032Z  Documenting rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
2019-12-07T08:07:40.2015957Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:40.2016975Z   |
2019-12-07T08:07:40.2028470Z   = warning: please see ***/issues/44136
2019-12-07T08:07:43.3659552Z warning: `[no_mangle]` cannot be resolved, ignoring it.
2019-12-07T08:07:43.3660300Z   --> src/librustc_codegen_ssa/traits/declare.rs:34:44
2019-12-07T08:07:43.3660704Z    |
2019-12-07T08:07:43.3661027Z 34 |     /// to user’s fault (e.g., misuse of #[no_mangle] or #[export_name] attributes).
---
2019-12-07T08:07:43.3745573Z 
2019-12-07T08:07:44.8748001Z  Documenting rustc_interface v0.0.0 (/checkout/src/librustc_interface)
2019-12-07T08:07:44.9058442Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:44.9058788Z   |
2019-12-07T08:07:44.9059224Z   = warning: please see ***/issues/44136
2019-12-07T08:07:48.0191222Z warning: `[T]` cannot be resolved, ignoring it.
2019-12-07T08:07:48.0191666Z   --> src/librustc_codegen_llvm/context.rs:61:36
2019-12-07T08:07:48.0191915Z    |
2019-12-07T08:07:48.0192238Z 61 |     /// Val is a Value holding a *[T].
---
2019-12-07T08:07:48.0204844Z 
2019-12-07T08:07:48.8061547Z  Documenting rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2019-12-07T08:07:48.8605104Z warning: the 'passes' flag is considered deprecated
2019-12-07T08:07:48.8605476Z   |
2019-12-07T08:07:48.8605896Z   = warning: please see ***/issues/44136
2019-12-07T08:07:53.8757667Z     Finished release [optimized] target(s) in 3m 40s
2019-12-07T08:07:53.8980960Z Documenting stage2 rustdoc (x86_64-unknown-linux-gnu)
2019-12-07T08:07:54.4959216Z     Checking cfg-if v0.1.8
2019-12-07T08:07:54.4959658Z     Checking lazy_static v1.3.0
---
2019-12-07T08:57:17.3796826Z -rw-r--r-- 1 vsts docker  18M Dec  7 08:57 rust-std-nightly-x86_64-unknown-linux-gnu.tar.xz
2019-12-07T08:57:17.3797009Z 
2019-12-07T08:57:17.3797510Z src/ci/scripts/upload-artifacts.sh: line 39: DEPLOY_BUCKET: unbound variable
2019-12-07T08:57:17.3802778Z 
2019-12-07T08:57:17.3939756Z ##[error]Bash exited with code '1'.
2019-12-07T08:57:17.3964731Z ##[section]Starting: Checkout
2019-12-07T08:57:17.3966766Z ==============================================================================
2019-12-07T08:57:17.3966852Z Task         : Get sources
2019-12-07T08:57:17.3966905Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
