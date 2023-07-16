plain
2019-08-16T15:37:40.7627727Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T15:37:40.7862187Z ##[command]git config gc.auto 0
2019-08-16T15:37:40.7949623Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T15:37:40.8005459Z ##[command]git config --get-all http.proxy
2019-08-16T15:37:40.8156048Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63637/merge:refs/remotes/pull/63637/merge
---
2019-08-16T15:38:17.1245432Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T15:38:17.1246242Z 
2019-08-16T15:38:17.1247385Z   git checkout -b <new-branch-name>
2019-08-16T15:38:17.1247432Z 
2019-08-16T15:38:17.1247658Z HEAD is now at 3763d0471 Merge ec71d807501512ee48a503e24601769e99bc05ce into 9dd5c191993aab6c2f1538eb8ab69afdc4b6e67a
2019-08-16T15:38:17.1398435Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T15:38:17.1401413Z ==============================================================================
2019-08-16T15:38:17.1401475Z Task         : Bash
2019-08-16T15:38:17.1401522Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T15:42:01.2007904Z    Compiling ryu v1.0.0
2019-08-16T15:42:01.2703904Z    Compiling fixedbitset v0.1.9
2019-08-16T15:42:01.7026542Z    Compiling cfg-if v0.1.8
2019-08-16T15:42:01.7272235Z    Compiling ordermap v0.3.5
2019-08-16T15:42:01.7570426Z    Compiling unicode-width v0.1.5 (https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff43)
2019-08-16T15:42:02.4569564Z    Compiling itoa v0.4.4
2019-08-16T15:42:02.6557835Z    Compiling lazy_static v1.3.0
2019-08-16T15:42:02.7505443Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-16T15:42:02.7505443Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-16T15:42:03.8989868Z    Compiling getopts v0.2.19 (https://github.com/alexcrichton/getopts?branch=rustc-dep-of-std#86b7617c)
2019-08-16T15:42:05.4998234Z    Compiling cmake v0.1.38
2019-08-16T15:42:08.5479333Z    Compiling quote v0.6.12
2019-08-16T15:42:09.1789465Z    Compiling time v0.1.40
2019-08-16T15:42:10.2437625Z    Compiling num_cpus v1.8.0
---
2019-08-16T15:45:00.4315292Z tidy check
2019-08-16T15:45:01.3619835Z * 578 error codes
2019-08-16T15:45:01.3619955Z * highest error code: E0733
2019-08-16T15:45:01.7127047Z * 262 features
2019-08-16T15:45:02.3598004Z invalid source: "git+https://github.com/alexcrichton/getopts?branch=rustc-dep-of-std#86b7617c0874db9d1ff371efb51c3a479af2256d"
2019-08-16T15:45:02.3598453Z invalid source: "git+https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff437d00b39242210d182bc7ec91cf48707b"
2019-08-16T15:45:02.4124305Z + python2.7 ../x.py test
2019-08-16T15:45:02.6628912Z     Finished dev [unoptimized] target(s) in 0.20s
2019-08-16T15:45:03.9251773Z Building stage0 tool tidy (x86_64-unknown-linux-gnu)
2019-08-16T15:45:04.1384340Z     Finished release [optimized] target(s) in 0.21s
2019-08-16T15:45:04.1384340Z     Finished release [optimized] target(s) in 0.21s
2019-08-16T15:45:04.1429367Z tidy check
2019-08-16T15:45:04.9215031Z * 578 error codes
2019-08-16T15:45:04.9215149Z * highest error code: E0733
2019-08-16T15:45:05.2867969Z * 262 features
2019-08-16T15:45:05.9382530Z invalid source: "git+https://github.com/alexcrichton/getopts?branch=rustc-dep-of-std#86b7617c0874db9d1ff371efb51c3a479af2256d"
2019-08-16T15:45:05.9383079Z invalid source: "git+https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff437d00b39242210d182bc7ec91cf48707b"
2019-08-16T15:45:06.4513715Z    Compiling cc v1.0.35
2019-08-16T15:45:06.4514340Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-08-16T15:45:15.8083011Z    Compiling libc v0.2.60
2019-08-16T15:45:16.7334409Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
---
2019-08-16T15:45:40.0617497Z    Compiling hashbrown v0.4.0
2019-08-16T15:45:56.6673886Z    Compiling rustc-std-workspace-std v1.0.0 (/checkout/src/tools/rustc-std-workspace-std)
2019-08-16T15:45:56.6725855Z    Compiling term v0.0.0 (/checkout/src/libterm)
2019-08-16T15:45:56.7201076Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-08-16T15:46:01.7602090Z    Compiling unicode-width v0.1.5 (https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff43)
2019-08-16T15:46:01.8697247Z    Compiling getopts v0.2.19 (https://github.com/alexcrichton/getopts?branch=rustc-dep-of-std#86b7617c)
2019-08-16T15:46:18.4482466Z     Finished release [optimized] target(s) in 1m 12s
2019-08-16T15:46:18.4609176Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-08-16T15:46:18.4641390Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T15:46:18.9729237Z    Compiling semver-parser v0.7.0
---
2019-08-16T15:46:23.8837565Z    Compiling bitflags v1.1.0
2019-08-16T15:46:24.6233964Z    Compiling cc v1.0.35
2019-08-16T15:46:24.6826529Z    Compiling rustc_target v0.0.0 (/checkout/src/librustc_target)
2019-08-16T15:46:24.8986589Z    Compiling scoped-tls v1.0.0
2019-08-16T15:46:25.0540976Z    Compiling unicode-width v0.1.5 (https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff43)
2019-08-16T15:46:25.2458171Z    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
2019-08-16T15:46:25.4582798Z    Compiling crc32fast v1.1.2
2019-08-16T15:46:26.3096655Z    Compiling annotate-snippets v0.6.1
2019-08-16T15:46:29.9468208Z    Compiling termcolor v1.0.4
---
2019-08-16T16:10:03.9707731Z    Compiling hashbrown v0.4.0
2019-08-16T16:10:23.3619867Z    Compiling rustc-std-workspace-std v1.0.0 (/checkout/src/tools/rustc-std-workspace-std)
2019-08-16T16:10:23.3628414Z    Compiling term v0.0.0 (/checkout/src/libterm)
2019-08-16T16:10:23.4095313Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-08-16T16:10:28.7569931Z    Compiling unicode-width v0.1.5 (https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff43)
2019-08-16T16:10:28.8608323Z    Compiling getopts v0.2.19 (https://github.com/alexcrichton/getopts?branch=rustc-dep-of-std#86b7617c)
2019-08-16T16:10:48.6335540Z     Finished release [optimized] target(s) in 1m 19s
2019-08-16T16:10:48.6457086Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-08-16T16:10:48.6484891Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T16:10:49.1580495Z    Compiling semver-parser v0.7.0
---
2019-08-16T16:10:54.5390199Z    Compiling bitflags v1.1.0
2019-08-16T16:10:55.4782100Z    Compiling cc v1.0.35
2019-08-16T16:10:55.4878627Z    Compiling scoped-tls v1.0.0
2019-08-16T16:10:55.6569501Z    Compiling rustc_target v0.0.0 (/checkout/src/librustc_target)
2019-08-16T16:10:55.8418259Z    Compiling unicode-width v0.1.5 (https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff43)
2019-08-16T16:10:56.0325109Z    Compiling crc32fast v1.1.2
2019-08-16T16:10:56.9762894Z    Compiling termcolor v1.0.4
2019-08-16T16:10:58.8453524Z    Compiling annotate-snippets v0.6.1
2019-08-16T16:11:03.0493865Z    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
---
2019-08-16T16:38:18.8421443Z    Compiling rustc-demangle v0.1.15
2019-08-16T16:38:21.0541600Z    Compiling cfg-if v0.1.8
2019-08-16T16:38:21.1064697Z    Compiling quick-error v1.2.2
2019-08-16T16:38:21.2176720Z    Compiling termcolor v1.0.4
2019-08-16T16:38:22.8700212Z    Compiling unicode-width v0.1.5 (https://github.com/alexcrichton/unicode-width?branch=rustc-dep-of-std#cb63ff43)
2019-08-16T16:38:25.1129297Z    Compiling log v0.4.6
2019-08-16T16:38:25.3757618Z    Compiling humantime v1.2.0
2019-08-16T16:38:25.3757618Z    Compiling humantime v1.2.0
2019-08-16T16:38:25.6866702Z    Compiling getopts v0.2.19 (https://github.com/alexcrichton/getopts?branch=rustc-dep-of-std#86b7617c)
2019-08-16T16:38:28.6395079Z    Compiling atty v0.2.11
2019-08-16T16:38:28.7531949Z    Compiling env_logger v0.5.13
2019-08-16T16:38:32.7702332Z    Compiling synstructure v0.10.2
2019-08-16T16:38:48.6457002Z    Compiling backtrace v0.3.34
---
2019-08-16T16:42:04.6587369Z .................................................................................................... 1500/8924
2019-08-16T16:42:10.2781255Z .................................................................................................... 1600/8924
2019-08-16T16:42:23.4504158Z ................................i...............i................................................... 1700/8924
2019-08-16T16:42:30.9558856Z .................................................................................................... 1800/8924
2019-08-16T16:42:45.4511586Z .......................iiiii........................................................................ 1900/8924
2019-08-16T16:42:56.1930046Z .................................................................................................... 2100/8924
2019-08-16T16:42:58.8042082Z .................................................................................................... 2200/8924
2019-08-16T16:43:03.6966317Z .................................................................................................... 2300/8924
2019-08-16T16:43:10.8040723Z .................................................................................................... 2400/8924
---
2019-08-16T16:46:06.9167461Z .................................................................................................... 4600/8924
2019-08-16T16:46:14.1444183Z ....i...............i............................................................................... 4700/8924
2019-08-16T16:46:25.8024586Z .................................................................................................... 4800/8924
2019-08-16T16:46:31.7017798Z .................................................................................................... 4900/8924
2019-08-16T16:46:44.0015783Z .....................................................................................ii.ii.......... 5000/8924
2019-08-16T16:46:53.5329297Z .................................................................................................... 5200/8924
2019-08-16T16:47:03.6115768Z .................................................................................................... 5300/8924
2019-08-16T16:47:10.9698345Z .........................................i.......................................................... 5400/8924
2019-08-16T16:47:17.4544030Z .................................................................................................... 5500/8924
2019-08-16T16:47:17.4544030Z .................................................................................................... 5500/8924
2019-08-16T16:47:29.4066964Z .................................................................................................... 5600/8924
2019-08-16T16:47:42.1547886Z ..................................ii...i..ii...........i............................................ 5700/8924
2019-08-16T16:47:59.1907541Z .................................................................................................... 5900/8924
2019-08-16T16:48:04.1760912Z .................................................................................................... 6000/8924
2019-08-16T16:48:04.1760912Z .................................................................................................... 6000/8924
2019-08-16T16:48:18.3342119Z ...................................i..ii............................................................ 6100/8924
2019-08-16T16:48:39.8569464Z .............................................................................i...................... 6300/8924
2019-08-16T16:48:42.1387162Z .................................................................................................... 6400/8924
2019-08-16T16:48:44.3604554Z .................................................i.................................................. 6500/8924
2019-08-16T16:48:47.7166957Z .................................................................................................... 6600/8924
---
2019-08-16T16:53:32.6442327Z  finished in 21.057
2019-08-16T16:53:32.6607964Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T16:53:32.8270793Z 
2019-08-16T16:53:32.8270941Z running 148 tests
2019-08-16T16:53:36.2670518Z i....iii......iii..iiii....i............................i..i..................i....i.........ii.i.i. 100/148
2019-08-16T16:53:38.1961654Z .iiii..............i.........iii.i......ii......
2019-08-16T16:53:38.1965891Z 
2019-08-16T16:53:38.1966835Z  finished in 5.535
2019-08-16T16:53:38.2136104Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T16:53:38.3870921Z 
---
2019-08-16T16:53:40.5034621Z  finished in 2.289
2019-08-16T16:53:40.5212359Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T16:53:40.6864472Z 
2019-08-16T16:53:40.6864736Z running 9 tests
2019-08-16T16:53:40.6865862Z iiiiiiiii
2019-08-16T16:53:40.6866232Z 
2019-08-16T16:53:40.6869136Z  finished in 0.165
2019-08-16T16:53:40.7051921Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T16:53:41.7344924Z 
---
2019-08-16T16:53:59.3453327Z  finished in 18.640
2019-08-16T16:53:59.3667447Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T16:53:59.5320103Z 
2019-08-16T16:53:59.5320904Z running 122 tests
2019-08-16T16:54:23.6327908Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-16T16:54:28.4156280Z .i.i......iii.i.....ii
2019-08-16T16:54:28.4157899Z 
2019-08-16T16:54:28.4161593Z  finished in 29.049
2019-08-16T16:54:28.4172307Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T16:54:28.4172956Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-16T17:08:50.8092029Z 
2019-08-16T17:08:50.8093259Z    Doc-tests core
2019-08-16T17:08:55.9472699Z 
2019-08-16T17:08:55.9493152Z running 2380 tests
2019-08-16T17:09:09.0892006Z ......iiiii......................................................................................... 100/2380
2019-08-16T17:09:21.9179030Z .........................................................................ii......................... 200/2380
2019-08-16T17:09:52.7705462Z .................................................................................................... 400/2380
2019-08-16T17:09:52.7705462Z .................................................................................................... 400/2380
2019-08-16T17:10:04.2483338Z ..............................i..i.................iiii............................................. 500/2380
2019-08-16T17:10:28.2564407Z .................................................................................................... 700/2380
2019-08-16T17:10:40.4954968Z .................................................................................................... 800/2380
2019-08-16T17:10:52.7679864Z .................................................................................................... 900/2380
2019-08-16T17:11:04.7677425Z .................................................................................................... 1000/2380
---
2019-08-16T17:16:32.3225841Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:611:13
2019-08-16T17:16:32.3226795Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:623:13
2019-08-16T17:16:34.3734030Z ...........................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:233:13
2019-08-16T17:16:34.3782340Z .......................... 700/762
2019-08-16T17:16:34.3856967Z ..................................thread '<unnamed>..' panicked at 'explicit panic', src/libstd/thread/mod.rs:1535:13
2019-08-16T17:16:34.9920137Z ..........thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1667:13
2019-08-16T17:16:34.9924979Z thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1639:13
2019-08-16T17:16:34.9931817Z ...thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1685:13
2019-08-16T17:16:40.5047705Z .............
2019-08-16T17:16:40.5048528Z test result: ok. 762 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
---
2019-08-16T17:16:41.7270989Z 
2019-08-16T17:16:41.7271922Z running 991 tests
2019-08-16T17:17:06.0687922Z i................................................................................................... 100/991
2019-08-16T17:17:19.7000351Z .................................................................................................... 200/991
2019-08-16T17:17:28.8668184Z .................iii......i......i...i......i....................................................... 300/991
2019-08-16T17:17:34.1073657Z .................................................................................................... 400/991
2019-08-16T17:17:42.4422301Z ..................................i..i.................................ii........................... 500/991
2019-08-16T17:17:57.6180738Z .................................................................................................... 700/991
2019-08-16T17:17:57.6180738Z .................................................................................................... 700/991
2019-08-16T17:18:06.2503561Z .................iiii............................................................................... 800/991
2019-08-16T17:18:21.9091437Z .................................................................................................... 900/991
2019-08-16T17:18:30.0084667Z .......................................iiii................................................
2019-08-16T17:18:30.0085426Z 
2019-08-16T17:18:30.0371375Z  finished in 260.561
2019-08-16T17:18:30.0389184Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T17:18:30.2911267Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-08-16T17:34:22.4908766Z  Documenting rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-08-16T17:34:23.6713778Z     Checking hashbrown v0.4.0
2019-08-16T17:34:24.0021848Z  Documenting unwind v0.0.0 (/checkout/src/libunwind)
2019-08-16T17:34:25.3791202Z  Documenting hashbrown v0.4.0
2019-08-16T17:34:26.7356030Z error: duplicate lang item in crate `core`: `char`.
2019-08-16T17:34:26.7367630Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7370369Z 
2019-08-16T17:34:26.7370369Z 
2019-08-16T17:34:26.7375914Z error: duplicate lang item in crate `core`: `str`.
2019-08-16T17:34:26.7386661Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7389296Z 
2019-08-16T17:34:26.7389296Z 
2019-08-16T17:34:26.7394573Z error: duplicate lang item in crate `core`: `slice`.
2019-08-16T17:34:26.7406245Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7411286Z 
2019-08-16T17:34:26.7411286Z 
2019-08-16T17:34:26.7416652Z error: duplicate lang item in crate `core`: `slice_u8`.
2019-08-16T17:34:26.7427673Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7432497Z 
2019-08-16T17:34:26.7432497Z 
2019-08-16T17:34:26.7437775Z error: duplicate lang item in crate `core`: `const_ptr`.
2019-08-16T17:34:26.7449328Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7454716Z 
2019-08-16T17:34:26.7454716Z 
2019-08-16T17:34:26.7471140Z error: duplicate lang item in crate `core`: `mut_ptr`.
2019-08-16T17:34:26.7482888Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7488750Z 
2019-08-16T17:34:26.7488750Z 
2019-08-16T17:34:26.7494486Z error: duplicate lang item in crate `core`: `i8`.
2019-08-16T17:34:26.7557066Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7557242Z 
2019-08-16T17:34:26.7557242Z 
2019-08-16T17:34:26.7581023Z error: duplicate lang item in crate `core`: `i16`.
2019-08-16T17:34:26.7609091Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7609139Z 
2019-08-16T17:34:26.7609139Z 
2019-08-16T17:34:26.7609401Z error: duplicate lang item in crate `core`: `i32`.
2019-08-16T17:34:26.7609820Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7609868Z 
2019-08-16T17:34:26.7609868Z 
2019-08-16T17:34:26.7610106Z error: duplicate lang item in crate `core`: `i64`.
2019-08-16T17:34:26.7610535Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7610584Z 
2019-08-16T17:34:26.7610584Z 
2019-08-16T17:34:26.7610809Z error: duplicate lang item in crate `core`: `i128`.
2019-08-16T17:34:26.7611435Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7611730Z 
2019-08-16T17:34:26.7611730Z 
2019-08-16T17:34:26.7612702Z error: duplicate lang item in crate `core`: `isize`.
2019-08-16T17:34:26.7613216Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7613250Z 
2019-08-16T17:34:26.7613250Z 
2019-08-16T17:34:26.7613512Z error: duplicate lang item in crate `core`: `u8`.
2019-08-16T17:34:26.7614030Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7614065Z 
2019-08-16T17:34:26.7614065Z 
2019-08-16T17:34:26.7614338Z error: duplicate lang item in crate `core`: `u16`.
2019-08-16T17:34:26.7614813Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7614847Z 
2019-08-16T17:34:26.7614847Z 
2019-08-16T17:34:26.7615123Z error: duplicate lang item in crate `core`: `u32`.
2019-08-16T17:34:26.7615597Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7615835Z 
2019-08-16T17:34:26.7615835Z 
2019-08-16T17:34:26.7616059Z error: duplicate lang item in crate `core`: `u64`.
2019-08-16T17:34:26.7616655Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7616715Z 
2019-08-16T17:34:26.7616715Z 
2019-08-16T17:34:26.7616943Z error: duplicate lang item in crate `core`: `u128`.
2019-08-16T17:34:26.7617374Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7617404Z 
2019-08-16T17:34:26.7617404Z 
2019-08-16T17:34:26.7617628Z error: duplicate lang item in crate `core`: `usize`.
2019-08-16T17:34:26.7618062Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7618092Z 
2019-08-16T17:34:26.7618092Z 
2019-08-16T17:34:26.7618309Z error: duplicate lang item in crate `core`: `f32`.
2019-08-16T17:34:26.7618750Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7618861Z 
2019-08-16T17:34:26.7618861Z 
2019-08-16T17:34:26.7619103Z error: duplicate lang item in crate `core`: `f64`.
2019-08-16T17:34:26.7619540Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7619570Z 
2019-08-16T17:34:26.7619570Z 
2019-08-16T17:34:26.7619813Z error: duplicate lang item in crate `core`: `sized`.
2019-08-16T17:34:26.7620228Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7620275Z 
2019-08-16T17:34:26.7620275Z 
2019-08-16T17:34:26.7620502Z error: duplicate lang item in crate `core`: `unsize`.
2019-08-16T17:34:26.7620921Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7620977Z 
2019-08-16T17:34:26.7620977Z 
2019-08-16T17:34:26.7621204Z error: duplicate lang item in crate `core`: `copy`.
2019-08-16T17:34:26.7621635Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7621665Z 
2019-08-16T17:34:26.7621665Z 
2019-08-16T17:34:26.7622054Z error: duplicate lang item in crate `core`: `clone`.
2019-08-16T17:34:26.7622763Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7622797Z 
2019-08-16T17:34:26.7622797Z 
2019-08-16T17:34:26.7623049Z error: duplicate lang item in crate `core`: `sync`.
2019-08-16T17:34:26.7623553Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7623588Z 
2019-08-16T17:34:26.7623588Z 
2019-08-16T17:34:26.7623871Z error: duplicate lang item in crate `core`: `freeze`.
2019-08-16T17:34:26.7624350Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7624385Z 
2019-08-16T17:34:26.7624385Z 
2019-08-16T17:34:26.7624659Z error: duplicate lang item in crate `core`: `drop`.
2019-08-16T17:34:26.7625128Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7625179Z 
2019-08-16T17:34:26.7625179Z 
2019-08-16T17:34:26.7625447Z error: duplicate lang item in crate `core`: `coerce_unsized`.
2019-08-16T17:34:26.7626549Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7626662Z 
2019-08-16T17:34:26.7626662Z 
2019-08-16T17:34:26.7626927Z error: duplicate lang item in crate `core`: `dispatch_from_dyn`.
2019-08-16T17:34:26.7627373Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7627403Z 
2019-08-16T17:34:26.7627403Z 
2019-08-16T17:34:26.7627622Z error: duplicate lang item in crate `core`: `add`.
2019-08-16T17:34:26.7628056Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7628086Z 
2019-08-16T17:34:26.7628086Z 
2019-08-16T17:34:26.7628305Z error: duplicate lang item in crate `core`: `sub`.
2019-08-16T17:34:26.7628736Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7628766Z 
2019-08-16T17:34:26.7628766Z 
2019-08-16T17:34:26.7629007Z error: duplicate lang item in crate `core`: `mul`.
2019-08-16T17:34:26.7629508Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7629538Z 
2019-08-16T17:34:26.7629538Z 
2019-08-16T17:34:26.7629781Z error: duplicate lang item in crate `core`: `div`.
2019-08-16T17:34:26.7630193Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7630240Z 
2019-08-16T17:34:26.7630240Z 
2019-08-16T17:34:26.7630462Z error: duplicate lang item in crate `core`: `rem`.
2019-08-16T17:34:26.7630894Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7630925Z 
2019-08-16T17:34:26.7630925Z 
2019-08-16T17:34:26.7631155Z error: duplicate lang item in crate `core`: `neg`.
2019-08-16T17:34:26.7631600Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7631630Z 
2019-08-16T17:34:26.7631630Z 
2019-08-16T17:34:26.7631848Z error: duplicate lang item in crate `core`: `not`.
2019-08-16T17:34:26.7632794Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7632830Z 
2019-08-16T17:34:26.7632830Z 
2019-08-16T17:34:26.7633084Z error: duplicate lang item in crate `core`: `bitxor`.
2019-08-16T17:34:26.7633580Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7633613Z 
2019-08-16T17:34:26.7633613Z 
2019-08-16T17:34:26.7633924Z error: duplicate lang item in crate `core`: `bitand`.
2019-08-16T17:34:26.7634421Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7634455Z 
2019-08-16T17:34:26.7634455Z 
2019-08-16T17:34:26.7634730Z error: duplicate lang item in crate `core`: `bitor`.
2019-08-16T17:34:26.7635205Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7635258Z 
2019-08-16T17:34:26.7635258Z 
2019-08-16T17:34:26.7635511Z error: duplicate lang item in crate `core`: `shl`.
2019-08-16T17:34:26.7636136Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7636188Z 
2019-08-16T17:34:26.7636188Z 
2019-08-16T17:34:26.7636591Z error: duplicate lang item in crate `core`: `shr`.
2019-08-16T17:34:26.7637511Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7637542Z 
2019-08-16T17:34:26.7637542Z 
2019-08-16T17:34:26.7637760Z error: duplicate lang item in crate `core`: `add_assign`.
2019-08-16T17:34:26.7638180Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7638209Z 
2019-08-16T17:34:26.7638209Z 
2019-08-16T17:34:26.7638427Z error: duplicate lang item in crate `core`: `sub_assign`.
2019-08-16T17:34:26.7638846Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7638874Z 
2019-08-16T17:34:26.7638874Z 
2019-08-16T17:34:26.7639092Z error: duplicate lang item in crate `core`: `mul_assign`.
2019-08-16T17:34:26.7639522Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7639618Z 
2019-08-16T17:34:26.7639618Z 
2019-08-16T17:34:26.7639872Z error: duplicate lang item in crate `core`: `div_assign`.
2019-08-16T17:34:26.7640268Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7640313Z 
2019-08-16T17:34:26.7640313Z 
2019-08-16T17:34:26.7640532Z error: duplicate lang item in crate `core`: `rem_assign`.
2019-08-16T17:34:26.7640930Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7640976Z 
2019-08-16T17:34:26.7640976Z 
2019-08-16T17:34:26.7641197Z error: duplicate lang item in crate `core`: `bitxor_assign`.
2019-08-16T17:34:26.7641623Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7641659Z 
2019-08-16T17:34:26.7641659Z 
2019-08-16T17:34:26.7641883Z error: duplicate lang item in crate `core`: `bitand_assign`.
2019-08-16T17:34:26.7642677Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7642713Z 
2019-08-16T17:34:26.7642713Z 
2019-08-16T17:34:26.7642974Z error: duplicate lang item in crate `core`: `bitor_assign`.
2019-08-16T17:34:26.7643468Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7643502Z 
2019-08-16T17:34:26.7643502Z 
2019-08-16T17:34:26.7643779Z error: duplicate lang item in crate `core`: `shl_assign`.
2019-08-16T17:34:26.7644267Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7644309Z 
2019-08-16T17:34:26.7644309Z 
2019-08-16T17:34:26.7644591Z error: duplicate lang item in crate `core`: `shr_assign`.
2019-08-16T17:34:26.7645067Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7645119Z 
2019-08-16T17:34:26.7645119Z 
2019-08-16T17:34:26.7645376Z error: duplicate lang item in crate `core`: `index`.
2019-08-16T17:34:26.7646331Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7646390Z 
2019-08-16T17:34:26.7646390Z 
2019-08-16T17:34:26.7646644Z error: duplicate lang item in crate `core`: `index_mut`.
2019-08-16T17:34:26.7647245Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7647296Z 
2019-08-16T17:34:26.7647296Z 
2019-08-16T17:34:26.7648039Z error: duplicate lang item in crate `core`: `unsafe_cell`.
2019-08-16T17:34:26.7648625Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7648654Z 
2019-08-16T17:34:26.7648654Z 
2019-08-16T17:34:26.7648868Z error: duplicate lang item in crate `core`: `va_list`.
2019-08-16T17:34:26.7649287Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7649316Z 
2019-08-16T17:34:26.7649316Z 
2019-08-16T17:34:26.7649546Z error: duplicate lang item in crate `core`: `deref`.
2019-08-16T17:34:26.7649959Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7649989Z 
2019-08-16T17:34:26.7649989Z 
2019-08-16T17:34:26.7650321Z error: duplicate lang item in crate `core`: `deref_mut`.
2019-08-16T17:34:26.7650721Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7650768Z 
2019-08-16T17:34:26.7650768Z 
2019-08-16T17:34:26.7650987Z error: duplicate lang item in crate `core`: `receiver`.
2019-08-16T17:34:26.7651386Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7651432Z 
2019-08-16T17:34:26.7651432Z 
2019-08-16T17:34:26.7651647Z error: duplicate lang item in crate `core`: `fn`.
2019-08-16T17:34:26.7652606Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7652656Z 
2019-08-16T17:34:26.7652656Z 
2019-08-16T17:34:26.7652918Z error: duplicate lang item in crate `core`: `fn_mut`.
2019-08-16T17:34:26.7653425Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7653459Z 
2019-08-16T17:34:26.7653459Z 
2019-08-16T17:34:26.7653712Z error: duplicate lang item in crate `core`: `fn_once`.
2019-08-16T17:34:26.7654205Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7654241Z 
2019-08-16T17:34:26.7654241Z 
2019-08-16T17:34:26.7654519Z error: duplicate lang item in crate `core`: `future_trait`.
2019-08-16T17:34:26.7654997Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7655030Z 
2019-08-16T17:34:26.7655030Z 
2019-08-16T17:34:26.7655323Z error: duplicate lang item in crate `core`: `generator_state`.
2019-08-16T17:34:26.7655811Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7655863Z 
2019-08-16T17:34:26.7655863Z 
2019-08-16T17:34:26.7656125Z error: duplicate lang item in crate `core`: `generator`.
2019-08-16T17:34:26.7656597Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7656649Z 
2019-08-16T17:34:26.7656649Z 
2019-08-16T17:34:26.7656905Z error: duplicate lang item in crate `core`: `unpin`.
2019-08-16T17:34:26.7657458Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7657493Z 
2019-08-16T17:34:26.7657493Z 
2019-08-16T17:34:26.7657833Z error: duplicate lang item in crate `core`: `pin`.
2019-08-16T17:34:26.7658722Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7658754Z 
2019-08-16T17:34:26.7658754Z 
2019-08-16T17:34:26.7659133Z error: duplicate lang item in crate `core`: `eq`.
2019-08-16T17:34:26.7659552Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7659580Z 
2019-08-16T17:34:26.7659580Z 
2019-08-16T17:34:26.7659816Z error: duplicate lang item in crate `core`: `partial_ord`.
2019-08-16T17:34:26.7660213Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7660242Z 
2019-08-16T17:34:26.7660242Z 
2019-08-16T17:34:26.7660480Z error: duplicate lang item in crate `core`: `ord`.
2019-08-16T17:34:26.7660965Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7661013Z 
2019-08-16T17:34:26.7661013Z 
2019-08-16T17:34:26.7661230Z error: duplicate lang item in crate `core`: `panic`.
2019-08-16T17:34:26.7661644Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7661674Z 
2019-08-16T17:34:26.7661674Z 
2019-08-16T17:34:26.7662379Z error: duplicate lang item in crate `core`: `panic_bounds_check`.
2019-08-16T17:34:26.7662905Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7662939Z 
2019-08-16T17:34:26.7662939Z 
2019-08-16T17:34:26.7663203Z error: duplicate lang item in crate `core`: `panic_info`.
2019-08-16T17:34:26.7663711Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7663746Z 
2019-08-16T17:34:26.7663746Z 
2019-08-16T17:34:26.7664006Z error: duplicate lang item in crate `core`: `drop_in_place`.
2019-08-16T17:34:26.7664502Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7664536Z 
2019-08-16T17:34:26.7664536Z 
2019-08-16T17:34:26.7664815Z error: duplicate lang item in crate `core`: `alloc_layout`.
2019-08-16T17:34:26.7665291Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7665325Z 
2019-08-16T17:34:26.7665325Z 
2019-08-16T17:34:26.7665983Z error: duplicate lang item in crate `core`: `phantom_data`.
2019-08-16T17:34:26.7666604Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7666905Z 
2019-08-16T17:34:26.7666905Z 
2019-08-16T17:34:26.7667162Z error: duplicate lang item in crate `core`: `manually_drop`.
2019-08-16T17:34:26.7667635Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7667669Z 
2019-08-16T17:34:26.7667669Z 
2019-08-16T17:34:26.7730775Z error: duplicate lang item in crate `core`: `maybe_uninit`.
2019-08-16T17:34:26.7731315Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7731352Z 
2019-08-16T17:34:26.7731352Z 
2019-08-16T17:34:26.7731600Z error: duplicate lang item in crate `core`: `debug_trait`.
2019-08-16T17:34:26.7732848Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7732886Z 
2019-08-16T17:34:26.7732886Z 
2019-08-16T17:34:26.7733147Z error: duplicate lang item in crate `core`: `align_offset`.
2019-08-16T17:34:26.7733651Z   = note: first defined in crate `core`.
2019-08-16T17:34:26.7733686Z 
2019-08-16T17:34:26.7733686Z 
2019-08-16T17:34:26.7733964Z error: duplicate lang item in crate `alloc`: `str_alloc`.
2019-08-16T17:34:26.7734450Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7734484Z 
2019-08-16T17:34:26.7734484Z 
2019-08-16T17:34:26.7734770Z error: duplicate lang item in crate `alloc`: `slice_alloc`.
2019-08-16T17:34:26.7735259Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7735408Z 
2019-08-16T17:34:26.7735408Z 
2019-08-16T17:34:26.7736056Z error: duplicate lang item in crate `alloc`: `slice_u8_alloc`.
2019-08-16T17:34:26.7736516Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7736548Z 
2019-08-16T17:34:26.7736548Z 
2019-08-16T17:34:26.7736788Z error: duplicate lang item in crate `alloc`: `exchange_malloc`.
2019-08-16T17:34:26.7737237Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7737269Z 
2019-08-16T17:34:26.7737269Z 
2019-08-16T17:34:26.7737500Z error: duplicate lang item in crate `alloc`: `box_free`.
2019-08-16T17:34:26.7737961Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7738001Z 
2019-08-16T17:34:26.7738001Z 
2019-08-16T17:34:26.7738233Z error: duplicate lang item in crate `alloc`: `owned_box`.
2019-08-16T17:34:26.7738681Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7738712Z 
2019-08-16T17:34:26.7738712Z 
2019-08-16T17:34:26.7738958Z error: duplicate lang item in crate `alloc`: `arc`.
2019-08-16T17:34:26.7739392Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7739423Z 
2019-08-16T17:34:26.7739423Z 
2019-08-16T17:34:26.7739674Z error: duplicate lang item in crate `alloc`: `rc`.
2019-08-16T17:34:26.7740109Z   = note: first defined in crate `alloc`.
2019-08-16T17:34:26.7740166Z 
2019-08-16T17:34:26.7740166Z 
2019-08-16T17:34:26.8037968Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8049172Z     |
2019-08-16T17:34:26.8049172Z     |
2019-08-16T17:34:26.8054384Z 111 | pub struct Mutex<T: ?Sized> {
2019-08-16T17:34:26.8062642Z 
2019-08-16T17:34:26.8062642Z 
2019-08-16T17:34:26.8090767Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8101555Z     |
2019-08-16T17:34:26.8101555Z     |
2019-08-16T17:34:26.8107429Z 145 | pub struct MutexGuard<'a, T: ?Sized + 'a> {
2019-08-16T17:34:26.8116435Z 
2019-08-16T17:34:26.8116435Z 
2019-08-16T17:34:26.8136483Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8147262Z    |
2019-08-16T17:34:26.8147262Z    |
2019-08-16T17:34:26.8153563Z 67 | pub struct RwLock<T: ?Sized> {
2019-08-16T17:34:26.8163546Z 
2019-08-16T17:34:26.8173033Z  Documenting std v0.0.0 (/checkout/src/libstd)
2019-08-16T17:34:26.8173033Z  Documenting std v0.0.0 (/checkout/src/libstd)
2019-08-16T17:34:26.8218853Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8231695Z     --> src/libstd/collections/hash/map.rs:1026:9
2019-08-16T17:34:26.8238403Z      |
2019-08-16T17:34:26.8245827Z 1026 | impl<K, Q: ?Sized, V, S> Index<&Q> for HashMap<K, V, S>
2019-08-16T17:34:26.8256092Z 
2019-08-16T17:34:26.8256092Z 
2019-08-16T17:34:26.8262322Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8268670Z    --> src/libstd/collections/hash/map.rs:697:16
2019-08-16T17:34:26.8328077Z     |
2019-08-16T17:34:26.8328458Z 697 |     pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
2019-08-16T17:34:26.8328758Z 
2019-08-16T17:34:26.8328758Z 
2019-08-16T17:34:26.8329110Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8329379Z    --> src/libstd/collections/hash/map.rs:727:26
2019-08-16T17:34:26.8329590Z     |
2019-08-16T17:34:26.8329917Z 727 |     pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
2019-08-16T17:34:26.8330255Z 
2019-08-16T17:34:26.8330255Z 
2019-08-16T17:34:26.8330585Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8330850Z    --> src/libstd/collections/hash/map.rs:756:25
2019-08-16T17:34:26.8331261Z     |
2019-08-16T17:34:26.8331686Z 756 |     pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
2019-08-16T17:34:26.8332356Z 
2019-08-16T17:34:26.8332356Z 
2019-08-16T17:34:26.8332725Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8333150Z    --> src/libstd/collections/hash/map.rs:787:20
2019-08-16T17:34:26.8333446Z     |
2019-08-16T17:34:26.8333761Z 787 |     pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
2019-08-16T17:34:26.8334084Z 
2019-08-16T17:34:26.8334084Z 
2019-08-16T17:34:26.8334422Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8334714Z    --> src/libstd/collections/hash/map.rs:848:19
2019-08-16T17:34:26.8334931Z     |
2019-08-16T17:34:26.8335232Z 848 |     pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
2019-08-16T17:34:26.8335796Z 
2019-08-16T17:34:26.8335796Z 
2019-08-16T17:34:26.8336100Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8336355Z    --> src/libstd/collections/hash/map.rs:880:25
2019-08-16T17:34:26.8336536Z     |
2019-08-16T17:34:26.8336794Z 880 |     pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
2019-08-16T17:34:26.8337072Z 
2019-08-16T17:34:26.8337072Z 
2019-08-16T17:34:26.8337356Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8337615Z     --> src/libstd/collections/hash/map.rs:1278:21
2019-08-16T17:34:26.8337806Z      |
2019-08-16T17:34:26.8338089Z 1278 |     pub fn from_key<Q: ?Sized>(self, k: &Q) -> RawEntryMut<'a, K, V, S>
2019-08-16T17:34:26.8338542Z 
2019-08-16T17:34:26.8338542Z 
2019-08-16T17:34:26.8338855Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8339523Z     --> src/libstd/collections/hash/map.rs:1289:36
2019-08-16T17:34:26.8339983Z      |
2019-08-16T17:34:26.8340338Z 1289 |     pub fn from_key_hashed_nocheck<Q: ?Sized>(self, hash: u64, k: &Q) -> RawEntryMut<'a, K, V, S>
2019-08-16T17:34:26.8340662Z 
2019-08-16T17:34:26.8340662Z 
2019-08-16T17:34:26.8341003Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8341269Z     --> src/libstd/collections/hash/map.rs:1320:21
2019-08-16T17:34:26.8341476Z      |
2019-08-16T17:34:26.8341792Z 1320 |     pub fn from_key<Q: ?Sized>(self, k: &Q) -> Option<(&'a K, &'a V)>
2019-08-16T17:34:26.8342475Z 
2019-08-16T17:34:26.8342475Z 
2019-08-16T17:34:26.8342926Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8343245Z     --> src/libstd/collections/hash/map.rs:1331:36
2019-08-16T17:34:26.8343460Z      |
2019-08-16T17:34:26.8343814Z 1331 |     pub fn from_key_hashed_nocheck<Q: ?Sized>(self, hash: u64, k: &Q) -> Option<(&'a K, &'a V)>
2019-08-16T17:34:26.8344136Z 
2019-08-16T17:34:26.8344136Z 
2019-08-16T17:34:26.8344490Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8344764Z    --> src/libstd/collections/hash/set.rs:587:21
2019-08-16T17:34:26.8348689Z     |
2019-08-16T17:34:26.8349111Z 587 |     pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
2019-08-16T17:34:26.8349533Z 
2019-08-16T17:34:26.8349533Z 
2019-08-16T17:34:26.8350293Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8350551Z    --> src/libstd/collections/hash/set.rs:614:16
2019-08-16T17:34:26.8350978Z     |
2019-08-16T17:34:26.8351281Z 614 |     pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
2019-08-16T17:34:26.8351568Z 
2019-08-16T17:34:26.8351568Z 
2019-08-16T17:34:26.8351900Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8352622Z    --> src/libstd/collections/hash/set.rs:667:31
2019-08-16T17:34:26.8352843Z     |
2019-08-16T17:34:26.8353181Z 667 |     pub fn get_or_insert_with<Q: ?Sized, F>(&mut self, value: &Q, f: F) -> &T
2019-08-16T17:34:26.8353492Z 
2019-08-16T17:34:26.8353492Z 
2019-08-16T17:34:26.8353825Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8354120Z    --> src/libstd/collections/hash/set.rs:828:19
2019-08-16T17:34:26.8354332Z     |
2019-08-16T17:34:26.8354636Z 828 |     pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
2019-08-16T17:34:26.8354970Z 
2019-08-16T17:34:26.8354970Z 
2019-08-16T17:34:26.8355304Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8355745Z    --> src/libstd/collections/hash/set.rs:855:17
2019-08-16T17:34:26.8355931Z     |
2019-08-16T17:34:26.8356187Z 855 |     pub fn take<Q: ?Sized>(&mut self, value: &Q) -> Option<T>
2019-08-16T17:34:26.8359008Z 
2019-08-16T17:34:26.8359008Z 
2019-08-16T17:34:26.8365979Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8366385Z    --> src/libstd/env.rs:408:20
2019-08-16T17:34:26.8366580Z     |
2019-08-16T17:34:26.8366858Z 408 | pub fn split_paths<T: AsRef<OsStr> + ?Sized>(unparsed: &T) -> SplitPaths<'_> {
2019-08-16T17:34:26.8367143Z 
2019-08-16T17:34:26.8367143Z 
2019-08-16T17:34:26.8367435Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8367870Z     |
2019-08-16T17:34:26.8367870Z     |
2019-08-16T17:34:26.8368135Z 368 | impl<T: ?Sized + AsRef<OsStr>> From<&T> for OsString {
2019-08-16T17:34:26.8368525Z 
2019-08-16T17:34:26.8368525Z 
2019-08-16T17:34:26.8368815Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8369252Z     |
2019-08-16T17:34:26.8369252Z     |
2019-08-16T17:34:26.8369512Z 499 |     pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &OsStr {
2019-08-16T17:34:26.8369787Z 
2019-08-16T17:34:26.8369787Z 
2019-08-16T17:34:26.8370163Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8370417Z    --> src/libstd/io/mod.rs:355:16
2019-08-16T17:34:26.8370605Z     |
2019-08-16T17:34:26.8370884Z 355 | fn read_to_end<R: Read + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
2019-08-16T17:34:26.8371329Z 
2019-08-16T17:34:26.8371329Z 
2019-08-16T17:34:26.8371633Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8372282Z    --> src/libstd/io/mod.rs:359:33
2019-08-16T17:34:26.8372565Z     |
2019-08-16T17:34:26.8372874Z 359 | fn read_to_end_with_reservation<R: Read + ?Sized>(r: &mut R,
2019-08-16T17:34:26.8373217Z 
2019-08-16T17:34:26.8373217Z 
2019-08-16T17:34:26.8373557Z warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported
2019-08-16T17:34:26.8373818Z     --> src/libstd/io/mod.rs:1381:28
2019-08-16T17:34:26.8374067Z      |
