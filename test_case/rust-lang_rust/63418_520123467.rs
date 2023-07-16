plain
2019-08-10T05:03:00.9320035Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T05:03:00.9505469Z ##[command]git config gc.auto 0
2019-08-10T05:03:00.9581416Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T05:03:00.9634564Z ##[command]git config --get-all http.proxy
2019-08-10T05:03:00.9761993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63418/merge:refs/remotes/pull/63418/merge
---
2019-08-10T05:03:12.3805754Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T05:03:12.3805783Z 
2019-08-10T05:03:12.3805975Z   git checkout -b <new-branch-name>
2019-08-10T05:03:12.3806003Z 
2019-08-10T05:03:12.3806253Z HEAD is now at 2a458af39 Merge b46f8288554f2e8b68f5a3b57fd18b8e8f4986e0 into be8bbb06976c8065425b18e9cbe24a6d1d4e7515
2019-08-10T05:03:12.3960569Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T05:03:12.3963478Z ==============================================================================
2019-08-10T05:03:12.3963557Z Task         : Bash
2019-08-10T05:03:12.3963596Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T06:04:29.9677373Z .................................................................................................... 1300/8860
2019-08-10T06:04:36.6755250Z .................................................................................................... 1400/8860
2019-08-10T06:04:43.2878561Z .................................................................................................... 1500/8860
2019-08-10T06:04:54.6829590Z ....................................................................................i............... 1600/8860
2019-08-10T06:05:02.8525381Z i................................................................................................... 1700/8860
2019-08-10T06:05:10.0906950Z ...........................................................................iiiii.................... 1800/8860
2019-08-10T06:05:33.5692294Z .................................................................................................... 2000/8860
2019-08-10T06:05:36.1720949Z .................................................................................................... 2100/8860
2019-08-10T06:05:39.1279077Z .................................................................................................... 2200/8860
2019-08-10T06:05:47.4054937Z .................................................................................................... 2300/8860
---
2019-08-10T06:09:43.0639616Z .................................................................................................... 5200/8860
2019-08-10T06:09:54.1602894Z .................................................................................................... 5300/8860
2019-08-10T06:10:01.9108987Z .i.................................................................................................. 5400/8860
2019-08-10T06:10:07.3731634Z .................................................................................................... 5500/8860
2019-08-10T06:10:20.3150600Z ...............................................................................................ii... 5600/8860
2019-08-10T06:10:36.7213109Z i..ii...........i................................................................................... 5700/8860
2019-08-10T06:10:51.6863342Z .................................................................................................... 5900/8860
2019-08-10T06:10:56.6706339Z ................................................................................................i..i 6000/8860
2019-08-10T06:11:11.7289256Z i................................................................................................... 6100/8860
2019-08-10T06:11:28.8515912Z .................................................................................................... 6200/8860
---
2019-08-10T06:16:30.2276147Z  finished in 21.110
2019-08-10T06:16:30.2486760Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-10T06:16:30.4163109Z 
2019-08-10T06:16:30.4164489Z running 146 tests
2019-08-10T06:16:33.9809492Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-10T06:16:35.8924373Z iii..............i.........iii.i......ii......
2019-08-10T06:16:35.8924970Z 
2019-08-10T06:16:35.8928923Z  finished in 5.644
2019-08-10T06:16:35.9117309Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-10T06:16:36.0761495Z 
---
2019-08-10T06:16:38.2039265Z  finished in 2.291
2019-08-10T06:16:38.2254288Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-10T06:16:38.3900602Z 
2019-08-10T06:16:38.3901170Z running 9 tests
2019-08-10T06:16:38.3902539Z iiiiiiiii
2019-08-10T06:16:38.3903022Z 
2019-08-10T06:16:38.3906182Z  finished in 0.165
2019-08-10T06:16:38.4089421Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-10T06:16:38.5915540Z 
---
2019-08-10T06:16:57.4292714Z  finished in 19.019
2019-08-10T06:16:57.4482533Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-10T06:16:57.6152809Z 
2019-08-10T06:16:57.6153894Z running 122 tests
2019-08-10T06:17:21.9487268Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-10T06:17:26.6897961Z .i.i......iii.i.....ii
2019-08-10T06:17:26.6898854Z 
2019-08-10T06:17:26.6898899Z  finished in 29.241
2019-08-10T06:17:26.6905137Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-10T06:17:26.6905538Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-10T06:32:12.3861866Z 
2019-08-10T06:32:12.3870107Z    Doc-tests core
2019-08-10T06:32:16.6425758Z 
2019-08-10T06:32:16.6426575Z running 2379 tests
2019-08-10T06:32:29.2820646Z ......iiiii......................................................................................... 100/2379
2019-08-10T06:32:41.7660788Z .........................................................................ii......................... 200/2379
2019-08-10T06:33:14.6470867Z .................................................................................................... 400/2379
2019-08-10T06:33:14.6470867Z .................................................................................................... 400/2379
2019-08-10T06:33:26.4629779Z ..............................i..i.................iiii............................................. 500/2379
2019-08-10T06:33:51.6874747Z .................................................................................................... 700/2379
2019-08-10T06:34:04.0264503Z .................................................................................................... 800/2379
2019-08-10T06:34:16.7355710Z .................................................................................................... 900/2379
2019-08-10T06:34:29.5787737Z .................................................................................................... 1000/2379
---
2019-08-10T06:39:58.0245600Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-10T06:39:58.0443296Z ................. 600/762
2019-08-10T06:40:00.0731526Z .......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-08-10T06:40:00.0733759Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-08-10T06:40:00.0737797Z thread '...<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-08-10T06:40:00.0742882Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
2019-08-10T06:40:00.0750858Z ...thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-08-10T06:40:00.0752015Z  right: `2`', src/libstd/sync/mutex.rs:653:13
2019-08-10T06:40:00.0790570Z ..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:791:13
2019-08-10T06:40:00.0799255Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:768:13
2019-08-10T06:40:00.0812401Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:705:13
2019-08-10T06:40:00.0812401Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:705:13
2019-08-10T06:40:00.0817685Z thread '<unnamed>.' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:635:13
2019-08-10T06:40:00.0822335Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:646:13
2019-08-10T06:40:00.0828200Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:611:13
2019-08-10T06:40:00.0832607Z thread '.<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:623:13
2019-08-10T06:40:02.1472995Z ...........................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:233:13
2019-08-10T06:40:02.1572768Z ..................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1535:13
2019-08-10T06:40:02.7662874Z ............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1667:13
2019-08-10T06:40:02.7666919Z ..thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1653:13
2019-08-10T06:40:02.7671973Z thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1639:13
---
2019-08-10T06:40:08.2706012Z 
2019-08-10T06:40:08.2709686Z    Doc-tests std
2019-08-10T06:40:09.4700402Z 
2019-08-10T06:40:09.4708060Z running 991 tests
2019-08-10T06:40:22.3963958Z i...F.FFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF.FFF........................................ 100/991
2019-08-10T06:40:35.5494079Z .................................................................................................... 200/991
2019-08-10T06:40:44.3277814Z .................iii......i......i...i......i....................................................... 300/991
2019-08-10T06:40:49.3226208Z .................................................................................................... 400/991
2019-08-10T06:40:57.3577322Z ..................................i..i.................................ii........................... 500/991
2019-08-10T06:41:12.0675617Z .................................................................................................... 700/991
2019-08-10T06:41:12.0675617Z .................................................................................................... 700/991
2019-08-10T06:41:20.2680927Z .................iiii..........................................F.................................... 800/991
2019-08-10T06:41:35.2657636Z .................................................................................................... 900/991
2019-08-10T06:41:42.9181633Z .......................................iiii................................................
2019-08-10T06:41:42.9185228Z 
2019-08-10T06:41:42.9186520Z ---- collections/hash/map.rs - collections::hash::map::Entry::and_modify (line 2010) stdout ----
2019-08-10T06:41:42.9186520Z ---- collections/hash/map.rs - collections::hash::map::Entry::and_modify (line 2010) stdout ----
2019-08-10T06:41:42.9186834Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9187763Z  --> collections/hash/map.rs:2011:5
2019-08-10T06:41:42.9188157Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9188283Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9188396Z   |
2019-08-10T06:41:42.9188531Z note: lint level defined here
---
2019-08-10T06:41:42.9189716Z error: aborting due to previous error
2019-08-10T06:41:42.9189816Z 
2019-08-10T06:41:42.9190129Z Couldn't compile the test.
2019-08-10T06:41:42.9190795Z ---- collections/hash/map.rs - collections::hash::map::Entry::key (line 1990) stdout ----
2019-08-10T06:41:42.9190961Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9191279Z  --> collections/hash/map.rs:1991:5
2019-08-10T06:41:42.9191545Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9191680Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9192427Z   |
2019-08-10T06:41:42.9192569Z note: lint level defined here
---
2019-08-10T06:41:42.9193898Z error: aborting due to previous error
2019-08-10T06:41:42.9194018Z 
2019-08-10T06:41:42.9194353Z Couldn't compile the test.
2019-08-10T06:41:42.9194805Z ---- collections/hash/map.rs - collections::hash::map::Entry::or_default (line 2048) stdout ----
2019-08-10T06:41:42.9195003Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9195497Z  --> collections/hash/map.rs:2050:5
2019-08-10T06:41:42.9195782Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9195898Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9196028Z   |
2019-08-10T06:41:42.9196143Z note: lint level defined here
---
2019-08-10T06:41:42.9197411Z error: aborting due to previous error
2019-08-10T06:41:42.9197670Z 
2019-08-10T06:41:42.9197975Z Couldn't compile the test.
2019-08-10T06:41:42.9198330Z ---- collections/hash/map.rs - collections::hash::map::Entry::or_insert (line 1943) stdout ----
2019-08-10T06:41:42.9198512Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9198797Z  --> collections/hash/map.rs:1944:5
2019-08-10T06:41:42.9199092Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9199207Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9199334Z   |
2019-08-10T06:41:42.9199446Z note: lint level defined here
---
2019-08-10T06:41:42.9200470Z error: aborting due to previous error
2019-08-10T06:41:42.9200565Z 
2019-08-10T06:41:42.9200857Z Couldn't compile the test.
2019-08-10T06:41:42.9201217Z ---- collections/hash/map.rs - collections::hash::map::Entry::or_insert_with (line 1968) stdout ----
2019-08-10T06:41:42.9201399Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9201728Z  --> collections/hash/map.rs:1969:5
2019-08-10T06:41:42.9202284Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9202423Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9202557Z   |
2019-08-10T06:41:42.9202711Z note: lint level defined here
---
2019-08-10T06:41:42.9203967Z error: aborting due to previous error
2019-08-10T06:41:42.9204085Z 
2019-08-10T06:41:42.9204432Z Couldn't compile the test.
2019-08-10T06:41:42.9204850Z ---- collections/hash/map.rs - collections::hash::map::HashMap (line 123) stdout ----
2019-08-10T06:41:42.9205185Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9205721Z  --> collections/hash/map.rs:124:5
2019-08-10T06:41:42.9206187Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9206329Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9206449Z   |
2019-08-10T06:41:42.9206571Z note: lint level defined here
---
2019-08-10T06:41:42.9207875Z error: aborting due to previous error
2019-08-10T06:41:42.9208139Z 
2019-08-10T06:41:42.9208419Z Couldn't compile the test.
2019-08-10T06:41:42.9208788Z ---- collections/hash/map.rs - collections::hash::map::HashMap (line 161) stdout ----
2019-08-10T06:41:42.9209117Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9209403Z  --> collections/hash/map.rs:162:5
2019-08-10T06:41:42.9209872Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9210010Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9210140Z   |
2019-08-10T06:41:42.9210332Z note: lint level defined here
---
2019-08-10T06:41:42.9211605Z error: aborting due to previous error
2019-08-10T06:41:42.9211863Z 
2019-08-10T06:41:42.9212442Z Couldn't compile the test.
2019-08-10T06:41:42.9212912Z ---- collections/hash/map.rs - collections::hash::map::HashMap (line 65) stdout ----
2019-08-10T06:41:42.9213121Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9213486Z  --> collections/hash/map.rs:66:5
2019-08-10T06:41:42.9213828Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9213987Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9214127Z   |
2019-08-10T06:41:42.9214266Z note: lint level defined here
---
2019-08-10T06:41:42.9215904Z error: aborting due to previous error
2019-08-10T06:41:42.9216212Z 
2019-08-10T06:41:42.9216514Z Couldn't compile the test.
2019-08-10T06:41:42.9217023Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, RandomState>::new (line 219) stdout ----
2019-08-10T06:41:42.9217231Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9217558Z  --> collections/hash/map.rs:220:5
2019-08-10T06:41:42.9217875Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9217999Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9218138Z   |
2019-08-10T06:41:42.9218260Z note: lint level defined here
---
2019-08-10T06:41:42.9220038Z error: aborting due to previous error
2019-08-10T06:41:42.9220146Z 
2019-08-10T06:41:42.9220452Z Couldn't compile the test.
2019-08-10T06:41:42.9220845Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, RandomState>::with_capacity (line 236) stdout ----
2019-08-10T06:41:42.9221006Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9221774Z  --> collections/hash/map.rs:237:5
2019-08-10T06:41:42.9222228Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9222369Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9222502Z   |
2019-08-10T06:41:42.9222659Z note: lint level defined here
---
2019-08-10T06:41:42.9223919Z error: aborting due to previous error
2019-08-10T06:41:42.9224052Z 
2019-08-10T06:41:42.9224391Z Couldn't compile the test.
2019-08-10T06:41:42.9224868Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::capacity (line 255) stdout ----
2019-08-10T06:41:42.9225062Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9225845Z  --> collections/hash/map.rs:256:5
2019-08-10T06:41:42.9226153Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9226277Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9226560Z   |
2019-08-10T06:41:42.9226689Z note: lint level defined here
---
2019-08-10T06:41:42.9227949Z error: aborting due to previous error
2019-08-10T06:41:42.9228048Z 
2019-08-10T06:41:42.9228421Z Couldn't compile the test.
2019-08-10T06:41:42.9228854Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::clear (line 452) stdout ----
2019-08-10T06:41:42.9229035Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9229352Z  --> collections/hash/map.rs:453:5
2019-08-10T06:41:42.9229633Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9229774Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9229889Z   |
2019-08-10T06:41:42.9230004Z note: lint level defined here
---
2019-08-10T06:41:42.9231083Z error: aborting due to previous error
2019-08-10T06:41:42.9231183Z 
2019-08-10T06:41:42.9231562Z Couldn't compile the test.
2019-08-10T06:41:42.9232269Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::contains_key (line 746) stdout ----
2019-08-10T06:41:42.9232461Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9232852Z  --> collections/hash/map.rs:747:5
2019-08-10T06:41:42.9233184Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9233349Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9233507Z   |
2019-08-10T06:41:42.9233648Z note: lint level defined here
---
2019-08-10T06:41:42.9234937Z error: aborting due to previous error
2019-08-10T06:41:42.9235058Z 
2019-08-10T06:41:42.9235566Z Couldn't compile the test.
2019-08-10T06:41:42.9236003Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::drain (line 427) stdout ----
2019-08-10T06:41:42.9236383Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9236732Z  --> collections/hash/map.rs:428:5
2019-08-10T06:41:42.9237190Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9237321Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9237454Z   |
2019-08-10T06:41:42.9237605Z note: lint level defined here
---
2019-08-10T06:41:42.9239091Z error: aborting due to previous error
2019-08-10T06:41:42.9239200Z 
2019-08-10T06:41:42.9239514Z Couldn't compile the test.
2019-08-10T06:41:42.9243135Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::entry (line 655) stdout ----
2019-08-10T06:41:42.9243945Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9245358Z  --> collections/hash/map.rs:656:5
2019-08-10T06:41:42.9247122Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9247409Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9248786Z   |
2019-08-10T06:41:42.9248832Z note: lint level defined here
---
2019-08-10T06:41:42.9271413Z error: aborting due to previous error
2019-08-10T06:41:42.9271511Z 
2019-08-10T06:41:42.9273353Z Couldn't compile the test.
2019-08-10T06:41:42.9276006Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::get (line 687) stdout ----
2019-08-10T06:41:42.9276436Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9276837Z  --> collections/hash/map.rs:688:5
2019-08-10T06:41:42.9277421Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9277542Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9277680Z   |
2019-08-10T06:41:42.9277799Z note: lint level defined here
---
2019-08-10T06:41:42.9278903Z error: aborting due to previous error
2019-08-10T06:41:42.9279003Z 
2019-08-10T06:41:42.9279275Z Couldn't compile the test.
2019-08-10T06:41:42.9279936Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::get_key_value (line 716) stdout ----
2019-08-10T06:41:42.9280107Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9280410Z  --> collections/hash/map.rs:718:5
2019-08-10T06:41:42.9280701Z 5 | use std::collections::HashMap;
2019-08-10T06:41:42.9280815Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9280923Z   |
2019-08-10T06:41:42.9281057Z note: lint level defined here
---
2019-08-10T06:41:42.9282850Z error: aborting due to previous error
2019-08-10T06:41:42.9282969Z 
2019-08-10T06:41:42.9283351Z Couldn't compile the test.
2019-08-10T06:41:42.9283842Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::get_mut (line 775) stdout ----
2019-08-10T06:41:42.9284031Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9284370Z  --> collections/hash/map.rs:776:5
2019-08-10T06:41:42.9287426Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9287817Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9288302Z   |
2019-08-10T06:41:42.9288470Z note: lint level defined here
---
2019-08-10T06:41:42.9312498Z error: aborting due to previous error
2019-08-10T06:41:42.9312528Z 
2019-08-10T06:41:42.9313562Z Couldn't compile the test.
2019-08-10T06:41:42.9314074Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::hasher (line 535) stdout ----
2019-08-10T06:41:42.9314142Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9314386Z  --> collections/hash/map.rs:536:5
2019-08-10T06:41:42.9314476Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9314531Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9314591Z   |
2019-08-10T06:41:42.9314634Z note: lint level defined here
---
2019-08-10T06:41:42.9315135Z error: aborting due to previous error
2019-08-10T06:41:42.9315165Z 
2019-08-10T06:41:42.9315376Z Couldn't compile the test.
2019-08-10T06:41:42.9316530Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::into_iter (line 1715) stdout ----
2019-08-10T06:41:42.9316604Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9316863Z  --> collections/hash/map.rs:1716:5
2019-08-10T06:41:42.9316949Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9317001Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9317058Z   |
2019-08-10T06:41:42.9317099Z note: lint level defined here
---
2019-08-10T06:41:42.9317582Z error: aborting due to previous error
2019-08-10T06:41:42.9317609Z 
2019-08-10T06:41:42.9317796Z Couldn't compile the test.
2019-08-10T06:41:42.9318081Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::insert (line 809) stdout ----
2019-08-10T06:41:42.9318135Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9318335Z  --> collections/hash/map.rs:810:5
2019-08-10T06:41:42.9318433Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9318477Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9318539Z   |
2019-08-10T06:41:42.9318738Z note: lint level defined here
---
2019-08-10T06:41:42.9319200Z error: aborting due to previous error
2019-08-10T06:41:42.9319227Z 
2019-08-10T06:41:42.9319409Z Couldn't compile the test.
2019-08-10T06:41:42.9319684Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::is_empty (line 408) stdout ----
2019-08-10T06:41:42.9319737Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9319931Z  --> collections/hash/map.rs:409:5
2019-08-10T06:41:42.9320027Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9320067Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9320192Z   |
2019-08-10T06:41:42.9320249Z note: lint level defined here
---
2019-08-10T06:41:42.9320727Z error: aborting due to previous error
2019-08-10T06:41:42.9320754Z 
2019-08-10T06:41:42.9321112Z Couldn't compile the test.
2019-08-10T06:41:42.9321386Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::iter (line 342) stdout ----
2019-08-10T06:41:42.9321450Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9324636Z  --> collections/hash/map.rs:343:5
2019-08-10T06:41:42.9324769Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9324815Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9324874Z   |
2019-08-10T06:41:42.9324937Z note: lint level defined here
---
2019-08-10T06:41:42.9326200Z error: aborting due to previous error
2019-08-10T06:41:42.9326229Z 
2019-08-10T06:41:42.9326499Z Couldn't compile the test.
2019-08-10T06:41:42.9326924Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::keys (line 271) stdout ----
2019-08-10T06:41:42.9327148Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9327398Z  --> collections/hash/map.rs:272:5
2019-08-10T06:41:42.9327499Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9327542Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9327592Z   |
2019-08-10T06:41:42.9327649Z note: lint level defined here
---
2019-08-10T06:41:42.9328132Z error: aborting due to previous error
2019-08-10T06:41:42.9328160Z 
2019-08-10T06:41:42.9328348Z Couldn't compile the test.
2019-08-10T06:41:42.9328766Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::iter_mut (line 365) stdout ----
2019-08-10T06:41:42.9328840Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9329032Z  --> collections/hash/map.rs:366:5
2019-08-10T06:41:42.9329131Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9329172Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9329210Z   |
2019-08-10T06:41:42.9329257Z note: lint level defined here
---
2019-08-10T06:41:42.9329719Z error: aborting due to previous error
2019-08-10T06:41:42.9329761Z 
2019-08-10T06:41:42.9329944Z Couldn't compile the test.
2019-08-10T06:41:42.9330193Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::len (line 391) stdout ----
2019-08-10T06:41:42.9330269Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9330464Z  --> collections/hash/map.rs:392:5
2019-08-10T06:41:42.9330543Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9330602Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9330640Z   |
2019-08-10T06:41:42.9330680Z note: lint level defined here
---
2019-08-10T06:41:42.9331433Z error: aborting due to previous error
2019-08-10T06:41:42.9331477Z 
2019-08-10T06:41:42.9331666Z Couldn't compile the test.
2019-08-10T06:41:42.9332381Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::remove (line 838) stdout ----
2019-08-10T06:41:42.9332447Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9332683Z  --> collections/hash/map.rs:839:5
2019-08-10T06:41:42.9335259Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9335404Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9335446Z   |
2019-08-10T06:41:42.9335488Z note: lint level defined here
---
2019-08-10T06:41:42.9336146Z error: aborting due to previous error
2019-08-10T06:41:42.9336336Z 
2019-08-10T06:41:42.9336546Z Couldn't compile the test.
2019-08-10T06:41:42.9336811Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::reserve (line 561) stdout ----
2019-08-10T06:41:42.9336865Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9337228Z  --> collections/hash/map.rs:562:5
2019-08-10T06:41:42.9337323Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9337381Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9337421Z   |
2019-08-10T06:41:42.9337461Z note: lint level defined here
---
2019-08-10T06:41:42.9338134Z error: aborting due to previous error
2019-08-10T06:41:42.9338161Z 
2019-08-10T06:41:42.9338533Z Couldn't compile the test.
2019-08-10T06:41:42.9338793Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::retain (line 894) stdout ----
2019-08-10T06:41:42.9338846Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9339387Z  --> collections/hash/map.rs:895:5
2019-08-10T06:41:42.9339811Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9339869Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9339909Z   |
2019-08-10T06:41:42.9339949Z note: lint level defined here
---
2019-08-10T06:41:42.9342700Z error: aborting due to previous error
2019-08-10T06:41:42.9342729Z 
2019-08-10T06:41:42.9343598Z Couldn't compile the test.
2019-08-10T06:41:42.9343895Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::shrink_to (line 628) stdout ----
2019-08-10T06:41:42.9343954Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9344177Z  --> collections/hash/map.rs:630:5
2019-08-10T06:41:42.9344277Z 5 | use std::collections::HashMap;
2019-08-10T06:41:42.9344322Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9344378Z   |
2019-08-10T06:41:42.9344421Z note: lint level defined here
---
2019-08-10T06:41:42.9345078Z error: aborting due to previous error
2019-08-10T06:41:42.9345107Z 
2019-08-10T06:41:42.9345334Z Couldn't compile the test.
2019-08-10T06:41:42.9345632Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::shrink_to_fit (line 603) stdout ----
2019-08-10T06:41:42.9345689Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9345893Z  --> collections/hash/map.rs:604:5
2019-08-10T06:41:42.9346004Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9346048Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9346104Z   |
2019-08-10T06:41:42.9346146Z note: lint level defined here
---
2019-08-10T06:41:42.9346643Z error: aborting due to previous error
2019-08-10T06:41:42.9346671Z 
2019-08-10T06:41:42.9346866Z Couldn't compile the test.
2019-08-10T06:41:42.9347159Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::try_reserve (line 583) stdout ----
2019-08-10T06:41:42.9347216Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9350935Z  --> collections/hash/map.rs:585:5
2019-08-10T06:41:42.9351222Z 5 | use std::collections::HashMap;
2019-08-10T06:41:42.9351262Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9351316Z   |
2019-08-10T06:41:42.9351355Z note: lint level defined here
---
2019-08-10T06:41:42.9353101Z error: aborting due to previous error
2019-08-10T06:41:42.9353131Z 
2019-08-10T06:41:42.9353365Z Couldn't compile the test.
2019-08-10T06:41:42.9353659Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::values (line 293) stdout ----
2019-08-10T06:41:42.9353717Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9353923Z  --> collections/hash/map.rs:294:5
2019-08-10T06:41:42.9354035Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9354080Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9354121Z   |
2019-08-10T06:41:42.9357710Z note: lint level defined here
---
2019-08-10T06:41:42.9358821Z error: aborting due to previous error
2019-08-10T06:41:42.9358848Z 
2019-08-10T06:41:42.9359035Z Couldn't compile the test.
2019-08-10T06:41:42.9359303Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::with_capacity_and_hasher (line 513) stdout ----
2019-08-10T06:41:42.9359374Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9359567Z  --> collections/hash/map.rs:514:5
2019-08-10T06:41:42.9359675Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9359717Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9359756Z   |
2019-08-10T06:41:42.9359950Z note: lint level defined here
---
2019-08-10T06:41:42.9360789Z error: aborting due to previous error
2019-08-10T06:41:42.9360817Z 
2019-08-10T06:41:42.9361039Z Couldn't compile the test.
2019-08-10T06:41:42.9361306Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::values_mut (line 315) stdout ----
2019-08-10T06:41:42.9361375Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9361573Z  --> collections/hash/map.rs:316:5
2019-08-10T06:41:42.9361707Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9361750Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9361790Z   |
2019-08-10T06:41:42.9361847Z note: lint level defined here
---
2019-08-10T06:41:42.9364156Z error: aborting due to previous error
2019-08-10T06:41:42.9364186Z 
2019-08-10T06:41:42.9364467Z Couldn't compile the test.
2019-08-10T06:41:42.9368376Z ---- collections/hash/map.rs - collections::hash::map::HashMap<K, V, S>::with_hasher (line 484) stdout ----
2019-08-10T06:41:42.9368479Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9369020Z  --> collections/hash/map.rs:485:5
2019-08-10T06:41:42.9369339Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9371058Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9371119Z   |
2019-08-10T06:41:42.9371160Z note: lint level defined here
---
2019-08-10T06:41:42.9373037Z error: aborting due to previous error
2019-08-10T06:41:42.9373083Z 
2019-08-10T06:41:42.9373883Z Couldn't compile the test.
2019-08-10T06:41:42.9374376Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::get (line 2113) stdout ----
2019-08-10T06:41:42.9374456Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9374698Z  --> collections/hash/map.rs:2114:5
2019-08-10T06:41:42.9374787Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9374860Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9374901Z   |
2019-08-10T06:41:42.9374943Z note: lint level defined here
---
2019-08-10T06:41:42.9375608Z error: aborting due to previous error
2019-08-10T06:41:42.9375636Z 
2019-08-10T06:41:42.9375844Z Couldn't compile the test.
2019-08-10T06:41:42.9376105Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::get_mut (line 2139) stdout ----
2019-08-10T06:41:42.9376316Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9376526Z  --> collections/hash/map.rs:2140:5
2019-08-10T06:41:42.9376607Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9376673Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9376713Z   |
2019-08-10T06:41:42.9376752Z note: lint level defined here
---
2019-08-10T06:41:42.9380204Z error: aborting due to previous error
2019-08-10T06:41:42.9380231Z 
2019-08-10T06:41:42.9380550Z Couldn't compile the test.
2019-08-10T06:41:42.9380800Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::insert (line 2196) stdout ----
2019-08-10T06:41:42.9380852Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9381056Z  --> collections/hash/map.rs:2197:5
2019-08-10T06:41:42.9381135Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9381203Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9381241Z   |
2019-08-10T06:41:42.9381440Z note: lint level defined here
---
2019-08-10T06:41:42.9382697Z error: aborting due to previous error
2019-08-10T06:41:42.9382726Z 
2019-08-10T06:41:42.9383011Z Couldn't compile the test.
2019-08-10T06:41:42.9383288Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::into_mut (line 2172) stdout ----
2019-08-10T06:41:42.9383345Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9383569Z  --> collections/hash/map.rs:2173:5
2019-08-10T06:41:42.9383655Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9383847Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9383919Z   |
2019-08-10T06:41:42.9383961Z note: lint level defined here
---
2019-08-10T06:41:42.9384502Z error: aborting due to previous error
2019-08-10T06:41:42.9384530Z 
2019-08-10T06:41:42.9384726Z Couldn't compile the test.
2019-08-10T06:41:42.9385009Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::key (line 2072) stdout ----
2019-08-10T06:41:42.9385066Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9385473Z  --> collections/hash/map.rs:2073:5
2019-08-10T06:41:42.9385563Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9385601Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9385660Z   |
2019-08-10T06:41:42.9385697Z note: lint level defined here
---
2019-08-10T06:41:42.9386133Z error: aborting due to previous error
2019-08-10T06:41:42.9386158Z 
2019-08-10T06:41:42.9386329Z Couldn't compile the test.
2019-08-10T06:41:42.9386742Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::remove (line 2219) stdout ----
2019-08-10T06:41:42.9386789Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9386963Z  --> collections/hash/map.rs:2220:5
2019-08-10T06:41:42.9387051Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9387088Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9387138Z   |
2019-08-10T06:41:42.9387180Z note: lint level defined here
---
2019-08-10T06:41:42.9387724Z error: aborting due to previous error
2019-08-10T06:41:42.9387749Z 
2019-08-10T06:41:42.9387954Z Couldn't compile the test.
2019-08-10T06:41:42.9388202Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::remove_entry (line 2089) stdout ----
2019-08-10T06:41:42.9388251Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9388427Z  --> collections/hash/map.rs:2090:5
2019-08-10T06:41:42.9388514Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9388552Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9388586Z   |
2019-08-10T06:41:42.9388648Z note: lint level defined here
---
2019-08-10T06:41:42.9389072Z error: aborting due to previous error
2019-08-10T06:41:42.9389096Z 
2019-08-10T06:41:42.9389261Z Couldn't compile the test.
2019-08-10T06:41:42.9389506Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::replace_entry (line 2243) stdout ----
2019-08-10T06:41:42.9389555Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9389730Z  --> collections/hash/map.rs:2245:41
2019-08-10T06:41:42.9389769Z   |
2019-08-10T06:41:42.9389823Z 5 | use std::collections::hash_map::{Entry, HashMap};
2019-08-10T06:41:42.9389966Z   |
2019-08-10T06:41:42.9390025Z note: lint level defined here
2019-08-10T06:41:42.9390222Z  --> collections/hash/map.rs:2241:9
2019-08-10T06:41:42.9390259Z   |
---
2019-08-10T06:41:42.9390471Z error: aborting due to previous error
2019-08-10T06:41:42.9390495Z 
2019-08-10T06:41:42.9390661Z Couldn't compile the test.
2019-08-10T06:41:42.9390889Z ---- collections/hash/map.rs - collections::hash::map::OccupiedEntry::replace_key (line 2269) stdout ----
2019-08-10T06:41:42.9390952Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9391128Z  --> collections/hash/map.rs:2271:41
2019-08-10T06:41:42.9391165Z   |
2019-08-10T06:41:42.9446148Z 5 | use std::collections::hash_map::{Entry, HashMap};
2019-08-10T06:41:42.9446316Z   |
2019-08-10T06:41:42.9446354Z note: lint level defined here
2019-08-10T06:41:42.9446769Z  --> collections/hash/map.rs:2267:9
2019-08-10T06:41:42.9446814Z   |
---
2019-08-10T06:41:42.9447016Z error: aborting due to previous error
2019-08-10T06:41:42.9447049Z 
2019-08-10T06:41:42.9447230Z Couldn't compile the test.
2019-08-10T06:41:42.9447449Z ---- collections/hash/map.rs - collections::hash::map::RandomState (line 2407) stdout ----
2019-08-10T06:41:42.9447551Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9447730Z  --> collections/hash/map.rs:2408:5
2019-08-10T06:41:42.9447813Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9447851Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9447893Z   |
2019-08-10T06:41:42.9447928Z note: lint level defined here
---
2019-08-10T06:41:42.9448519Z error: aborting due to previous error
2019-08-10T06:41:42.9448552Z 
2019-08-10T06:41:42.9448931Z Couldn't compile the test.
2019-08-10T06:41:42.9449164Z ---- collections/hash/map.rs - collections::hash::map::RawEntryMut::and_modify (line 1420) stdout ----
2019-08-10T06:41:42.9449218Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9449393Z  --> collections/hash/map.rs:1422:5
2019-08-10T06:41:42.9449467Z 5 | use std::collections::HashMap;
2019-08-10T06:41:42.9449511Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9449546Z   |
2019-08-10T06:41:42.9449590Z note: lint level defined here
---
2019-08-10T06:41:42.9450002Z error: aborting due to previous error
2019-08-10T06:41:42.9450033Z 
2019-08-10T06:41:42.9450199Z Couldn't compile the test.
2019-08-10T06:41:42.9453084Z ---- collections/hash/map.rs - collections::hash::map::RawEntryMut::or_insert (line 1356) stdout ----
2019-08-10T06:41:42.9453168Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9453426Z  --> collections/hash/map.rs:1358:5
2019-08-10T06:41:42.9453515Z 5 | use std::collections::HashMap;
2019-08-10T06:41:42.9453568Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9453609Z   |
2019-08-10T06:41:42.9453813Z note: lint level defined here
---
2019-08-10T06:41:42.9454365Z error: aborting due to previous error
2019-08-10T06:41:42.9454394Z 
2019-08-10T06:41:42.9454605Z Couldn't compile the test.
2019-08-10T06:41:42.9455045Z ---- collections/hash/map.rs - collections::hash::map::RawEntryMut::or_insert_with (line 1386) stdout ----
2019-08-10T06:41:42.9455094Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9455284Z  --> collections/hash/map.rs:1388:5
2019-08-10T06:41:42.9500692Z 5 | use std::collections::HashMap;
2019-08-10T06:41:42.9500750Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9500786Z   |
2019-08-10T06:41:42.9500838Z note: lint level defined here
---
2019-08-10T06:41:42.9501884Z error: aborting due to previous error
2019-08-10T06:41:42.9502183Z 
2019-08-10T06:41:42.9502744Z Couldn't compile the test.
2019-08-10T06:41:42.9503023Z ---- collections/hash/map.rs - collections::hash::map::VacantEntry::into_key (line 2319) stdout ----
2019-08-10T06:41:42.9503082Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9503303Z  --> collections/hash/map.rs:2320:5
2019-08-10T06:41:42.9503394Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9503439Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9503494Z   |
2019-08-10T06:41:42.9503536Z note: lint level defined here
---
2019-08-10T06:41:42.9504217Z error: aborting due to previous error
2019-08-10T06:41:42.9504246Z 
2019-08-10T06:41:42.9504501Z Couldn't compile the test.
2019-08-10T06:41:42.9504772Z ---- collections/hash/map.rs - collections::hash::map::VacantEntry::insert (line 2340) stdout ----
2019-08-10T06:41:42.9504828Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9505036Z  --> collections/hash/map.rs:2341:5
2019-08-10T06:41:42.9505141Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9505187Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9505240Z   |
2019-08-10T06:41:42.9505283Z note: lint level defined here
---
2019-08-10T06:41:42.9505948Z error: aborting due to previous error
2019-08-10T06:41:42.9505973Z 
2019-08-10T06:41:42.9506305Z Couldn't compile the test.
2019-08-10T06:41:42.9506709Z ---- collections/hash/map.rs - collections::hash::map::VacantEntry::key (line 2303) stdout ----
2019-08-10T06:41:42.9506758Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9506937Z  --> collections/hash/map.rs:2304:5
2019-08-10T06:41:42.9507025Z 4 | use std::collections::HashMap;
2019-08-10T06:41:42.9507064Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9507113Z   |
2019-08-10T06:41:42.9507150Z note: lint level defined here
---
2019-08-10T06:41:42.9507634Z 
2019-08-10T06:41:42.9507678Z error: aborting due to previous error
2019-08-10T06:41:42.9507703Z 
2019-08-10T06:41:42.9507898Z Couldn't compile the test.
2019-08-10T06:41:42.9508117Z ---- process.rs - process::Command::envs (line 575) stdout ----
2019-08-10T06:41:42.9508163Z error: the item `HashMap` is imported redundantly
2019-08-10T06:41:42.9508331Z  --> process.rs:578:5
2019-08-10T06:41:42.9508423Z 6 | use std::collections::HashMap;
2019-08-10T06:41:42.9508461Z   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:41:42.9508497Z   |
2019-08-10T06:41:42.9508545Z note: lint level defined here
---
2019-08-10T06:41:42.9534230Z 
2019-08-10T06:41:42.9534541Z error: test failed, to rerun pass '--doc'
2019-08-10T06:41:42.9534576Z 
2019-08-10T06:41:42.9534602Z 
2019-08-10T06:41:42.9535197Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
2019-08-10T06:41:42.9535322Z 
2019-08-10T06:41:42.9535537Z 
2019-08-10T06:41:42.9535578Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-10T06:41:42.9535624Z Build completed unsuccessfully in 1:32:33
2019-08-10T06:41:42.9535624Z Build completed unsuccessfully in 1:32:33
2019-08-10T06:41:43.5390189Z ##[error]Bash exited with code '1'.
2019-08-10T06:41:43.5429324Z ##[section]Starting: Checkout
2019-08-10T06:41:43.5430898Z ==============================================================================
2019-08-10T06:41:43.5430943Z Task         : Get sources
2019-08-10T06:41:43.5430981Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
