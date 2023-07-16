plain
2020-04-28T07:10:23.7510669Z ========================== Starting Command Output ===========================
2020-04-28T07:10:23.7514438Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e730dcb9-c0f4-43c7-bb93-9c33cc5665c2.sh
2020-04-28T07:10:23.7514720Z 
2020-04-28T07:10:23.7518444Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T07:10:23.7537718Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-28T07:10:23.7541224Z Task         : Get sources
2020-04-28T07:10:23.7541500Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T07:10:23.7541755Z Version      : 1.0.0
2020-04-28T07:10:23.7541929Z Author       : Microsoft
---
2020-04-28T07:10:26.4293475Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T07:10:26.4303419Z ##[command]git config gc.auto 0
2020-04-28T07:10:26.4309668Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T07:10:26.4315292Z ##[command]git config --get-all http.proxy
2020-04-28T07:10:26.4327500Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71158/merge:refs/remotes/pull/71158/merge
---
2020-04-28T07:14:13.1656995Z  ---> cb2676f08729
2020-04-28T07:14:13.1657735Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-28T07:14:13.1658560Z  ---> Using cache
2020-04-28T07:14:13.1658875Z  ---> df25ce111862
2020-04-28T07:14:13.1660006Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-28T07:14:13.1661102Z  ---> 599b9ac96b27
2020-04-28T07:14:13.1661316Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-28T07:14:13.1661654Z  ---> Using cache
2020-04-28T07:14:13.1661963Z  ---> 091087e35a36
---
2020-04-28T07:14:13.2307209Z Looks like docker image is the same as before, not uploading
2020-04-28T07:14:21.8121493Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T07:14:21.8436013Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T07:14:21.8460571Z == clock drift check ==
2020-04-28T07:14:21.8467450Z   local time: Tue Apr 28 07:14:21 UTC 2020
2020-04-28T07:14:21.9076313Z   network time: Tue, 28 Apr 2020 07:14:21 GMT
2020-04-28T07:14:21.9102949Z Starting sccache server...
2020-04-28T07:14:21.9882335Z configure: processing command line
2020-04-28T07:14:21.9894646Z configure: 
2020-04-28T07:14:21.9896244Z configure: rust.dist-src        := False
---
2020-04-28T07:16:36.3331842Z    Compiling unicode-width v0.1.6
2020-04-28T07:16:36.4152081Z    Compiling getopts v0.2.21
2020-04-28T07:16:45.7859787Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T07:16:53.1626280Z     Finished release [optimized] target(s) in 55.96s
2020-04-28T07:16:53.1627874Z {"reason":"build-finished","success":true}
2020-04-28T07:16:53.1854654Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T07:16:54.0299207Z    Compiling cfg-if v0.1.10
2020-04-28T07:16:54.0300563Z    Compiling libc v0.2.69
2020-04-28T07:16:54.0301537Z    Compiling semver-parser v0.7.0
---
2020-04-28T07:19:09.9121558Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T07:19:11.2698792Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T07:19:12.6355521Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T07:19:12.9264912Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T07:19:21.3085987Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T07:19:22.7713372Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T07:19:26.7384041Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T07:19:30.3827127Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T07:19:39.8987893Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T07:32:35.8031264Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T07:32:42.8615315Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T07:36:32.5701502Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T07:36:33.1817647Z     Finished release [optimized] target(s) in 19m 39s
2020-04-28T07:36:33.1818816Z {"reason":"build-finished","success":true}
2020-04-28T07:36:33.2377750Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-28T07:36:33.2396974Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T07:36:33.5300355Z    Compiling cc v1.0.50
2020-04-28T07:36:33.5301012Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-28T07:37:18.4375895Z    Compiling unicode-width v0.1.6
2020-04-28T07:37:18.5287478Z    Compiling getopts v0.2.21
2020-04-28T07:37:29.0608754Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T07:37:37.8869997Z     Finished release [optimized] target(s) in 1m 04s
2020-04-28T07:37:37.8871162Z {"reason":"build-finished","success":true}
2020-04-28T07:37:37.8995615Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T07:37:38.4544191Z    Compiling cfg-if v0.1.10
2020-04-28T07:37:38.4973302Z    Compiling libc v0.2.69
2020-04-28T07:37:38.4973841Z    Compiling semver-parser v0.7.0
---
2020-04-28T07:40:15.7167101Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T07:40:17.1919381Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T07:40:18.7945428Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T07:40:18.7967307Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T07:40:28.8502563Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T07:40:30.6164435Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T07:40:34.8525354Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T07:40:38.8331128Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T07:40:48.8248105Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T07:53:45.6055380Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-04-28T07:55:28.4379462Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T07:55:32.3423567Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T07:58:05.3313335Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T07:58:05.9689777Z {"reason":"build-finished","success":true}
2020-04-28T07:58:06.0157182Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-28T07:58:06.0197007Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-28T07:58:06.0216196Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T07:58:06.0217241Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-04-28T07:58:44.8235695Z    Compiling serde_derive v1.0.81
2020-04-28T07:59:10.4868665Z    Compiling serde_json v1.0.40
2020-04-28T07:59:11.8265302Z    Compiling rustfix v0.5.0
2020-04-28T07:59:14.7538484Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-28T07:59:27.9845164Z {"reason":"build-finished","success":true}
2020-04-28T07:59:28.0101029Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T07:59:31.2607095Z 
2020-04-28T07:59:31.2607406Z running 9928 tests
2020-04-28T07:59:44.1341183Z .................................................................................................... 100/9928
---
2020-04-28T08:01:29.5550290Z .................................................................................................... 1700/9928
2020-04-28T08:01:34.2014293Z .................................................................................................... 1800/9928
2020-04-28T08:01:41.9792416Z .................................................................................................... 1900/9928
2020-04-28T08:01:50.6254017Z .........i.......................................................................................... 2000/9928
2020-04-28T08:01:57.0814450Z ...................................................................................................i 2100/9928
2020-04-28T08:02:09.8117471Z iiii................................................................................................ 2200/9928
2020-04-28T08:02:18.4020587Z .................................................................................................... 2400/9928
2020-04-28T08:02:20.8000101Z .................................................................................................... 2500/9928
2020-04-28T08:02:26.3782935Z .................................................................................................... 2600/9928
2020-04-28T08:02:44.2532456Z .................................................................................................... 2700/9928
---
2020-04-28T08:05:26.4079862Z ......................................................................F............................. 5100/9928
2020-04-28T08:05:33.5612737Z .................................................................................................... 5200/9928
2020-04-28T08:05:38.2287613Z ......................i............................................................................. 5300/9928
2020-04-28T08:05:47.8480487Z .............i...................................................................................... 5400/9928
2020-04-28T08:05:53.4168316Z ..............ii.ii........i...i.................................................................... 5500/9928
2020-04-28T08:06:01.2253188Z .............................................................i...................................... 5700/9928
2020-04-28T08:06:09.7027856Z ..............................................................................................ii.... 5800/9928
2020-04-28T08:06:16.4296878Z .................................i.................................................................. 5900/9928
2020-04-28T08:06:22.0886975Z .................................................................................................... 6000/9928
2020-04-28T08:06:22.0886975Z .................................................................................................... 6000/9928
2020-04-28T08:06:32.2199050Z .................................................................................................... 6100/9928
2020-04-28T08:06:42.3153346Z ...........................ii...i..ii...........i................................................... 6200/9928
2020-04-28T08:06:58.7970908Z .................................................................................................... 6400/9928
2020-04-28T08:07:02.2975992Z .................................................................................................... 6500/9928
2020-04-28T08:07:02.2975992Z .................................................................................................... 6500/9928
2020-04-28T08:07:08.3451147Z .........................................................i..ii...................................... 6600/9928
2020-04-28T08:07:31.3948055Z .................................................................................................... 6800/9928
2020-04-28T08:07:35.6736729Z ..........................................................i......................................... 6900/9928
2020-04-28T08:07:37.7155630Z .................................................................................................... 7000/9928
2020-04-28T08:07:39.8041747Z .................................................................................................... 7100/9928
---
2020-04-28T08:09:13.2689513Z .................................................................................................... 7900/9928
2020-04-28T08:09:18.6750768Z .................................................................................................... 8000/9928
2020-04-28T08:09:24.9019836Z ....................................................................i............................... 8100/9928
2020-04-28T08:09:35.0936043Z .................................................................................................... 8200/9928
2020-04-28T08:09:40.3476271Z .................iiiiii.iiiii.i..................................................................... 8300/9928
2020-04-28T08:09:53.2079499Z .................................................................................................... 8500/9928
2020-04-28T08:09:58.7012463Z .................................................................................................... 8600/9928
2020-04-28T08:10:12.9340771Z .................................................................................................... 8700/9928
2020-04-28T08:10:19.5203476Z .................................................................................................... 8800/9928
---
2020-04-28T08:12:00.5956737Z ---- [ui] ui/fmt/format-string-error-2.rs stdout ----
2020-04-28T08:12:00.5956974Z diff of stderr:
2020-04-28T08:12:00.5957102Z 
2020-04-28T08:12:00.5957258Z 6    |                      |
2020-04-28T08:12:00.5957601Z 7    |                      help: format of unicode escape sequences uses braces: `\u{8}`
2020-04-28T08:12:00.5957884Z 8 
2020-04-28T08:12:00.5958318Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-28T08:12:00.5958805Z -   --> $DIR/format-string-error-2.rs:5:5
2020-04-28T08:12:00.5959499Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.5960270Z 11    |
2020-04-28T08:12:00.5960426Z 12 LL |     format!("{
2020-04-28T08:12:00.5960837Z -    |              - because of this opening brace
2020-04-28T08:12:00.5961210Z - LL |     a");
2020-04-28T08:12:00.5961210Z - LL |     a");
2020-04-28T08:12:00.5961572Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.5961999Z +    |              -^ expected `}` in format string
2020-04-28T08:12:00.5962450Z +    |              because of this opening brace
2020-04-28T08:12:00.5962641Z 16    |
2020-04-28T08:12:00.5962641Z 16    |
2020-04-28T08:12:00.5962897Z 17    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.5963223Z 
2020-04-28T08:12:00.5963223Z 
2020-04-28T08:12:00.5963622Z - error: invalid format string: expected `'}'`, found `'b'`
2020-04-28T08:12:00.5964096Z -   --> $DIR/format-string-error-2.rs:9:5
2020-04-28T08:12:00.5964567Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.5965241Z 21    |
2020-04-28T08:12:00.5965399Z 22 LL |     format!("{ \
2020-04-28T08:12:00.5965799Z -    |              - because of this opening brace
2020-04-28T08:12:00.5966146Z - LL | 
2020-04-28T08:12:00.5966146Z - LL | 
2020-04-28T08:12:00.5966431Z - LL |     b");
2020-04-28T08:12:00.5966789Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.5967230Z +    |              -^ expected `}` in format string
2020-04-28T08:12:00.5967663Z +    |              because of this opening brace
2020-04-28T08:12:00.5967870Z 27    |
2020-04-28T08:12:00.5967870Z 27    |
2020-04-28T08:12:00.5968103Z 28    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.5968424Z 
2020-04-28T08:12:00.5968424Z 
2020-04-28T08:12:00.5968838Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-28T08:12:00.5969309Z -   --> $DIR/format-string-error-2.rs:11:18
2020-04-28T08:12:00.5969781Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6014798Z 32    |
2020-04-28T08:12:00.6014798Z 32    |
2020-04-28T08:12:00.6014963Z 33 LL |     format!(r#"{ \
2020-04-28T08:12:00.6015415Z -    |                - ^ expected `}` in format string
2020-04-28T08:12:00.6015872Z +    |                -^ expected `}` in format string
2020-04-28T08:12:00.6016343Z 36    |                because of this opening brace
2020-04-28T08:12:00.6016540Z 37    |
2020-04-28T08:12:00.6016646Z 
2020-04-28T08:12:00.6016646Z 
2020-04-28T08:12:00.6016877Z 38    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6017121Z 39 
2020-04-28T08:12:00.6017533Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-28T08:12:00.6017999Z -   --> $DIR/format-string-error-2.rs:15:18
2020-04-28T08:12:00.6018503Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6019169Z 42    |
2020-04-28T08:12:00.6019169Z 42    |
2020-04-28T08:12:00.6019348Z 43 LL |     format!(r#"{ \n
2020-04-28T08:12:00.6019761Z -    |                - ^ expected `}` in format string
2020-04-28T08:12:00.6020210Z +    |                -^ expected `}` in format string
2020-04-28T08:12:00.6020677Z 46    |                because of this opening brace
2020-04-28T08:12:00.6020872Z 47    |
2020-04-28T08:12:00.6020974Z 
2020-04-28T08:12:00.6020974Z 
2020-04-28T08:12:00.6021212Z 48    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6021437Z 49 
2020-04-28T08:12:00.6021839Z - error: invalid format string: expected `'}'`, found `'e'`
2020-04-28T08:12:00.6022315Z -   --> $DIR/format-string-error-2.rs:21:5
2020-04-28T08:12:00.6022785Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6023594Z 52    |
2020-04-28T08:12:00.6023753Z 53 LL |     format!("{ \n
2020-04-28T08:12:00.6024234Z -    |              - because of this opening brace
2020-04-28T08:12:00.6024608Z - LL | \n
2020-04-28T08:12:00.6024608Z - LL | \n
2020-04-28T08:12:00.6024901Z - LL |     e");
2020-04-28T08:12:00.6025262Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.6025688Z +    |              -^ expected `}` in format string
2020-04-28T08:12:00.6026140Z +    |              because of this opening brace
2020-04-28T08:12:00.6026333Z 58    |
2020-04-28T08:12:00.6026333Z 58    |
2020-04-28T08:12:00.6026582Z 59    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6026905Z 
2020-04-28T08:12:00.6026905Z 
2020-04-28T08:12:00.6027319Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-28T08:12:00.6027781Z -   --> $DIR/format-string-error-2.rs:25:5
2020-04-28T08:12:00.6028255Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6028932Z 63    |
2020-04-28T08:12:00.6029074Z 64 LL |     {
2020-04-28T08:12:00.6029437Z -    |     - because of this opening brace
2020-04-28T08:12:00.6029794Z - LL |     a");
2020-04-28T08:12:00.6029794Z - LL |     a");
2020-04-28T08:12:00.6030157Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.6030559Z +    |     -^ expected `}` in format string
2020-04-28T08:12:00.6030956Z +    |     because of this opening brace
2020-04-28T08:12:00.6031135Z 68    |
2020-04-28T08:12:00.6031135Z 68    |
2020-04-28T08:12:00.6031381Z 69    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6031704Z 
2020-04-28T08:12:00.6031704Z 
2020-04-28T08:12:00.6032103Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-28T08:12:00.6032582Z -   --> $DIR/format-string-error-2.rs:29:5
2020-04-28T08:12:00.6033052Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6033732Z 73    |
2020-04-28T08:12:00.6033871Z 74 LL |     {
2020-04-28T08:12:00.6034232Z -    |     - because of this opening brace
2020-04-28T08:12:00.6034579Z - LL |     a
2020-04-28T08:12:00.6034579Z - LL |     a
2020-04-28T08:12:00.6034936Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.6035336Z +    |     -^ expected `}` in format string
2020-04-28T08:12:00.6035734Z +    |     because of this opening brace
2020-04-28T08:12:00.6035911Z 78    |
2020-04-28T08:12:00.6035911Z 78    |
2020-04-28T08:12:00.6036159Z 79    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6036480Z 
2020-04-28T08:12:00.6036480Z 
2020-04-28T08:12:00.6036873Z - error: invalid format string: expected `'}'`, found `'b'`
2020-04-28T08:12:00.6037348Z -   --> $DIR/format-string-error-2.rs:35:5
2020-04-28T08:12:00.6037816Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6038485Z 83    |
2020-04-28T08:12:00.6038628Z 84 LL |     { \
2020-04-28T08:12:00.6038988Z -    |     - because of this opening brace
2020-04-28T08:12:00.6039346Z - LL |         \
2020-04-28T08:12:00.6039346Z - LL |         \
2020-04-28T08:12:00.6039648Z - LL |     b");
2020-04-28T08:12:00.6040008Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.6040410Z +    |     -^ expected `}` in format string
2020-04-28T08:12:00.6040806Z +    |     because of this opening brace
2020-04-28T08:12:00.6040984Z 89    |
2020-04-28T08:12:00.6040984Z 89    |
2020-04-28T08:12:00.6041232Z 90    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6041553Z 
2020-04-28T08:12:00.6041553Z 
2020-04-28T08:12:00.6041964Z - error: invalid format string: expected `'}'`, found `'b'`
2020-04-28T08:12:00.6042425Z -   --> $DIR/format-string-error-2.rs:40:5
2020-04-28T08:12:00.6042892Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6043610Z 94    |
2020-04-28T08:12:00.6043751Z 95 LL |     { \
2020-04-28T08:12:00.6044124Z -    |     - because of this opening brace
2020-04-28T08:12:00.6044525Z - LL |         \
2020-04-28T08:12:00.6044525Z - LL |         \
2020-04-28T08:12:00.6044837Z - LL |     b \
2020-04-28T08:12:00.6045197Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.6045616Z +    |     -^ expected `}` in format string
2020-04-28T08:12:00.6045994Z +    |     because of this opening brace
2020-04-28T08:12:00.6046190Z 100    |
2020-04-28T08:12:00.6046190Z 100    |
2020-04-28T08:12:00.6046428Z 101    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6046756Z 
2020-04-28T08:12:00.6046756Z 
2020-04-28T08:12:00.6047169Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-28T08:12:00.6047627Z -   --> $DIR/format-string-error-2.rs:45:8
2020-04-28T08:12:00.6048092Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6048769Z 105    |
2020-04-28T08:12:00.6048769Z 105    |
2020-04-28T08:12:00.6048916Z 106 LL | raw  { \
2020-04-28T08:12:00.6049305Z -    |      - ^ expected `}` in format string
2020-04-28T08:12:00.6049716Z +    |      -^ expected `}` in format string
2020-04-28T08:12:00.6050441Z 109    |      because of this opening brace
2020-04-28T08:12:00.6050626Z 110    |
2020-04-28T08:12:00.6050730Z 
2020-04-28T08:12:00.6050730Z 
2020-04-28T08:12:00.6050955Z 111    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6051199Z 112 
2020-04-28T08:12:00.6051615Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-28T08:12:00.6052072Z -   --> $DIR/format-string-error-2.rs:50:8
2020-04-28T08:12:00.6052555Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6053205Z 115    |
2020-04-28T08:12:00.6053205Z 115    |
2020-04-28T08:12:00.6053371Z 116 LL | raw  { \n
2020-04-28T08:12:00.6053743Z -    |      - ^ expected `}` in format string
2020-04-28T08:12:00.6054154Z +    |      -^ expected `}` in format string
2020-04-28T08:12:00.6054568Z 119    |      because of this opening brace
2020-04-28T08:12:00.6054752Z 120    |
2020-04-28T08:12:00.6054855Z 
2020-04-28T08:12:00.6054855Z 
2020-04-28T08:12:00.6055096Z 121    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6055323Z 122 
2020-04-28T08:12:00.6055726Z - error: invalid format string: expected `'}'`, found `'e'`
2020-04-28T08:12:00.6056201Z -   --> $DIR/format-string-error-2.rs:57:5
2020-04-28T08:12:00.6056667Z + error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6057338Z 125    |
2020-04-28T08:12:00.6057481Z 126 LL |   { \n
2020-04-28T08:12:00.6057835Z -    |   - because of this opening brace
2020-04-28T08:12:00.6058170Z - LL | \n
2020-04-28T08:12:00.6058170Z - LL | \n
2020-04-28T08:12:00.6058461Z - LL |     e");
2020-04-28T08:12:00.6058820Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.6059218Z +    |   -^ expected `}` in format string
2020-04-28T08:12:00.6059604Z +    |   because of this opening brace
2020-04-28T08:12:00.6059780Z 131    |
2020-04-28T08:12:00.6059780Z 131    |
2020-04-28T08:12:00.6060032Z 132    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6060359Z 
2020-04-28T08:12:00.6060359Z 
2020-04-28T08:12:00.6060771Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-28T08:12:00.6061230Z -   --> $DIR/format-string-error-2.rs:67:5
2020-04-28T08:12:00.6061698Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6062371Z 136    |
2020-04-28T08:12:00.6062516Z + LL |     {asdf
2020-04-28T08:12:00.6062890Z +    |     -    ^ expected `}` in format string
2020-04-28T08:12:00.6063104Z +    |     |
2020-04-28T08:12:00.6063104Z +    |     |
2020-04-28T08:12:00.6063290Z +    |     because of this opening brace
2020-04-28T08:12:00.6063466Z +    |
2020-04-28T08:12:00.6063779Z +    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6064000Z + 
2020-04-28T08:12:00.6064458Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6065142Z +    |
2020-04-28T08:12:00.6065281Z 137 LL |     {
2020-04-28T08:12:00.6065655Z -    |     - because of this opening brace
2020-04-28T08:12:00.6065995Z - LL |     asdf}
2020-04-28T08:12:00.6065995Z - LL |     asdf}
2020-04-28T08:12:00.6066356Z -    |     ^ expected `}` in format string
2020-04-28T08:12:00.6066773Z +    |     -^ expected `}` in format string
2020-04-28T08:12:00.6067152Z +    |     because of this opening brace
2020-04-28T08:12:00.6067332Z 141    |
2020-04-28T08:12:00.6067332Z 141    |
2020-04-28T08:12:00.6067584Z 142    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6067910Z 
2020-04-28T08:12:00.6067910Z 
2020-04-28T08:12:00.6068079Z 147 LL |     println!("\t{}");
2020-04-28T08:12:00.6068432Z 149 
2020-04-28T08:12:00.6068432Z 149 
2020-04-28T08:12:00.6068873Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-28T08:12:00.6069406Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6070076Z 152    |
2020-04-28T08:12:00.6070076Z 152    |
2020-04-28T08:12:00.6070274Z 153 LL |     println!("\x7B}\u{8} {", 1);
2020-04-28T08:12:00.6070425Z 
2020-04-28T08:12:00.6070820Z -    |                          -^ expected `'}'` in format string
2020-04-28T08:12:00.6071320Z +    |                          -^ expected `}` in format string
2020-04-28T08:12:00.6071835Z 156    |                          because of this opening brace
2020-04-28T08:12:00.6072062Z 157    |
2020-04-28T08:12:00.6072166Z 
2020-04-28T08:12:00.6072287Z 173    |
2020-04-28T08:12:00.6072287Z 173    |
2020-04-28T08:12:00.6072522Z 174    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-28T08:12:00.6073123Z - error: aborting due to 18 previous errors
2020-04-28T08:12:00.6073374Z + error: aborting due to 19 previous errors
2020-04-28T08:12:00.6073573Z 177 
2020-04-28T08:12:00.6073693Z 178 
2020-04-28T08:12:00.6073693Z 178 
2020-04-28T08:12:00.6073789Z 
2020-04-28T08:12:00.6073880Z 
2020-04-28T08:12:00.6074086Z The actual stderr differed from the expected stderr.
2020-04-28T08:12:00.6074718Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/format-string-error-2.stderr
2020-04-28T08:12:00.6075308Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:12:00.6075877Z To only update this specific test, also pass `--test-args fmt/format-string-error-2.rs`
2020-04-28T08:12:00.6076637Z error: 1 errors occurred comparing output.
2020-04-28T08:12:00.6076869Z status: exit code: 1
2020-04-28T08:12:00.6076869Z status: exit code: 1
2020-04-28T08:12:00.6079087Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/auxiliary"
2020-04-28T08:12:00.6080760Z ------------------------------------------
2020-04-28T08:12:00.6080920Z 
2020-04-28T08:12:00.6081279Z ------------------------------------------
2020-04-28T08:12:00.6081465Z stderr:
2020-04-28T08:12:00.6081465Z stderr:
2020-04-28T08:12:00.6082007Z ------------------------------------------
2020-04-28T08:12:00.6082264Z error: incorrect unicode escape sequence
2020-04-28T08:12:00.6082746Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:77:20
2020-04-28T08:12:00.6083076Z    |
2020-04-28T08:12:00.6083264Z LL |     println!("\x7B}\u8 {", 1);
2020-04-28T08:12:00.6083880Z    |                      |
2020-04-28T08:12:00.6083880Z    |                      |
2020-04-28T08:12:00.6084212Z    |                      help: format of unicode escape sequences uses braces: `\u{8}`
2020-04-28T08:12:00.6084473Z 
2020-04-28T08:12:00.6084873Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6085615Z    |
2020-04-28T08:12:00.6085760Z LL |     format!("{
2020-04-28T08:12:00.6085760Z LL |     format!("{
2020-04-28T08:12:00.6086146Z    |              -^ expected `}` in format string
2020-04-28T08:12:00.6086584Z    |              because of this opening brace
2020-04-28T08:12:00.6086769Z    |
2020-04-28T08:12:00.6086769Z    |
2020-04-28T08:12:00.6087008Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6087204Z 
2020-04-28T08:12:00.6087588Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6088329Z    |
2020-04-28T08:12:00.6088476Z LL |     format!("{ \
2020-04-28T08:12:00.6088476Z LL |     format!("{ \
2020-04-28T08:12:00.6088865Z    |              -^ expected `}` in format string
2020-04-28T08:12:00.6089304Z    |              because of this opening brace
2020-04-28T08:12:00.6089487Z    |
2020-04-28T08:12:00.6089487Z    |
2020-04-28T08:12:00.6089723Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6103404Z 
2020-04-28T08:12:00.6104089Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6104870Z    |
2020-04-28T08:12:00.6104870Z    |
2020-04-28T08:12:00.6105024Z LL |     format!(r#"{ \
2020-04-28T08:12:00.6105435Z    |                -^ expected `}` in format string
2020-04-28T08:12:00.6105892Z    |                because of this opening brace
2020-04-28T08:12:00.6107926Z    |
2020-04-28T08:12:00.6107926Z    |
2020-04-28T08:12:00.6109178Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6109383Z 
2020-04-28T08:12:00.6109893Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6110643Z    |
2020-04-28T08:12:00.6110643Z    |
2020-04-28T08:12:00.6110799Z LL |     format!(r#"{ \n
2020-04-28T08:12:00.6111220Z    |                -^ expected `}` in format string
2020-04-28T08:12:00.6111658Z    |                because of this opening brace
2020-04-28T08:12:00.6111845Z    |
2020-04-28T08:12:00.6111845Z    |
2020-04-28T08:12:00.6112083Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6112281Z 
2020-04-28T08:12:00.6112667Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6113413Z    |
2020-04-28T08:12:00.6113564Z LL |     format!("{ \n
2020-04-28T08:12:00.6113564Z LL |     format!("{ \n
2020-04-28T08:12:00.6113976Z    |              -^ expected `}` in format string
2020-04-28T08:12:00.6114407Z    |              because of this opening brace
2020-04-28T08:12:00.6114605Z    |
2020-04-28T08:12:00.6114605Z    |
2020-04-28T08:12:00.6114828Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6115025Z 
2020-04-28T08:12:00.6115413Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6116150Z    |
2020-04-28T08:12:00.6116282Z LL |     {
2020-04-28T08:12:00.6116282Z LL |     {
2020-04-28T08:12:00.6116927Z    |     -^ expected `}` in format string
2020-04-28T08:12:00.6117300Z    |     because of this opening brace
2020-04-28T08:12:00.6117485Z    |
2020-04-28T08:12:00.6117485Z    |
2020-04-28T08:12:00.6117709Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6118045Z 
2020-04-28T08:12:00.6118446Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6119264Z    |
2020-04-28T08:12:00.6119395Z LL |     {
2020-04-28T08:12:00.6119395Z LL |     {
2020-04-28T08:12:00.6119774Z    |     -^ expected `}` in format string
2020-04-28T08:12:00.6120143Z    |     because of this opening brace
2020-04-28T08:12:00.6120332Z    |
2020-04-28T08:12:00.6120332Z    |
2020-04-28T08:12:00.6120555Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6120754Z 
2020-04-28T08:12:00.6121141Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6121878Z    |
2020-04-28T08:12:00.6122013Z LL |     { \
2020-04-28T08:12:00.6122013Z LL |     { \
2020-04-28T08:12:00.6122543Z    |     -^ expected `}` in format string
2020-04-28T08:12:00.6122917Z    |     because of this opening brace
2020-04-28T08:12:00.6123114Z    |
2020-04-28T08:12:00.6123114Z    |
2020-04-28T08:12:00.6123341Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6123539Z 
2020-04-28T08:12:00.6123926Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6124661Z    |
2020-04-28T08:12:00.6124794Z LL |     { \
2020-04-28T08:12:00.6124794Z LL |     { \
2020-04-28T08:12:00.6126582Z    |     -^ expected `}` in format string
2020-04-28T08:12:00.6126980Z    |     because of this opening brace
2020-04-28T08:12:00.6129151Z    |
2020-04-28T08:12:00.6129151Z    |
2020-04-28T08:12:00.6129385Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6129586Z 
2020-04-28T08:12:00.6130302Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6131049Z    |
2020-04-28T08:12:00.6131049Z    |
2020-04-28T08:12:00.6131186Z LL | raw  { \
2020-04-28T08:12:00.6131569Z    |      -^ expected `}` in format string
2020-04-28T08:12:00.6131950Z    |      because of this opening brace
2020-04-28T08:12:00.6139133Z    |
2020-04-28T08:12:00.6139133Z    |
2020-04-28T08:12:00.6139378Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6139580Z 
2020-04-28T08:12:00.6140159Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6140921Z    |
2020-04-28T08:12:00.6140921Z    |
2020-04-28T08:12:00.6141060Z LL | raw  { \n
2020-04-28T08:12:00.6141573Z    |      -^ expected `}` in format string
2020-04-28T08:12:00.6141951Z    |      because of this opening brace
2020-04-28T08:12:00.6142141Z    |
2020-04-28T08:12:00.6142141Z    |
2020-04-28T08:12:00.6143549Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6143757Z 
2020-04-28T08:12:00.6144226Z error: invalid format string: expected `}`, found `' '`
2020-04-28T08:12:00.6144987Z    |
2020-04-28T08:12:00.6145125Z LL |   { \n
2020-04-28T08:12:00.6145125Z LL |   { \n
2020-04-28T08:12:00.6145495Z    |   -^ expected `}` in format string
2020-04-28T08:12:00.6146521Z    |   because of this opening brace
2020-04-28T08:12:00.6146707Z    |
2020-04-28T08:12:00.6146707Z    |
2020-04-28T08:12:00.6146934Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6147134Z 
2020-04-28T08:12:00.6147646Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6154053Z    |
2020-04-28T08:12:00.6154201Z LL |     {asdf
2020-04-28T08:12:00.6154632Z    |     -    ^ expected `}` in format string
2020-04-28T08:12:00.6154827Z    |     |
2020-04-28T08:12:00.6154827Z    |     |
2020-04-28T08:12:00.6155007Z    |     because of this opening brace
2020-04-28T08:12:00.6155196Z    |
2020-04-28T08:12:00.6155422Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6155777Z 
2020-04-28T08:12:00.6156252Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6157013Z    |
2020-04-28T08:12:00.6157144Z LL |     {
2020-04-28T08:12:00.6157144Z LL |     {
2020-04-28T08:12:00.6157514Z    |     -^ expected `}` in format string
2020-04-28T08:12:00.6157884Z    |     because of this opening brace
2020-04-28T08:12:00.6158071Z    |
2020-04-28T08:12:00.6158071Z    |
2020-04-28T08:12:00.6158295Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6158718Z error: 1 positional argument in format string, but no arguments were given
2020-04-28T08:12:00.6159253Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:70:17
2020-04-28T08:12:00.6159474Z    |
2020-04-28T08:12:00.6159474Z    |
2020-04-28T08:12:00.6159783Z LL |     println!("\t{}");
2020-04-28T08:12:00.6160126Z 
2020-04-28T08:12:00.6160126Z 
2020-04-28T08:12:00.6160526Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6161276Z    |
2020-04-28T08:12:00.6161276Z    |
2020-04-28T08:12:00.6161449Z LL |     println!("\x7B}\u{8} {", 1);
2020-04-28T08:12:00.6161889Z    |                          -^ expected `}` in format string
2020-04-28T08:12:00.6162398Z    |                          because of this opening brace
2020-04-28T08:12:00.6162599Z    |
2020-04-28T08:12:00.6162599Z    |
2020-04-28T08:12:00.6162838Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6163233Z error: invalid format string: unmatched `}` found
2020-04-28T08:12:00.6163882Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:81:21
2020-04-28T08:12:00.6164105Z    |
2020-04-28T08:12:00.6164105Z    |
2020-04-28T08:12:00.6164282Z LL |     println!(r#"\x7B}\u{8} {"#, 1);
2020-04-28T08:12:00.6164556Z    |                     ^ unmatched `}` in format string
2020-04-28T08:12:00.6164754Z    |
2020-04-28T08:12:00.6164978Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-28T08:12:00.6165388Z error: invalid format string: unmatched `}` found
2020-04-28T08:12:00.6165879Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:84:21
2020-04-28T08:12:00.6166100Z    |
2020-04-28T08:12:00.6166100Z    |
2020-04-28T08:12:00.6166293Z LL |     println!(r#"\x7B}\u8 {"#, 1);
2020-04-28T08:12:00.6166543Z    |                     ^ unmatched `}` in format string
2020-04-28T08:12:00.6166740Z    |
2020-04-28T08:12:00.6166978Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-28T08:12:00.6167349Z error: aborting due to 19 previous errors
2020-04-28T08:12:00.6167630Z 
2020-04-28T08:12:00.6167724Z 
2020-04-28T08:12:00.6168092Z ------------------------------------------
2020-04-28T08:12:00.6168092Z ------------------------------------------
2020-04-28T08:12:00.6168248Z 
2020-04-28T08:12:00.6168338Z 
2020-04-28T08:12:00.6168703Z ---- [ui] ui/fmt/format-string-error.rs stdout ----
2020-04-28T08:12:00.6168938Z diff of stderr:
2020-04-28T08:12:00.6169060Z 
2020-04-28T08:12:00.6169496Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-28T08:12:00.6170308Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6170983Z 3    |
2020-04-28T08:12:00.6171158Z 4 LL |     println!("{");
2020-04-28T08:12:00.6171287Z 
2020-04-28T08:12:00.6171287Z 
2020-04-28T08:12:00.6171659Z -    |               -^ expected `'}'` in format string
2020-04-28T08:12:00.6172106Z +    |               -^ expected `}` in format string
2020-04-28T08:12:00.6172562Z 7    |               because of this opening brace
2020-04-28T08:12:00.6172753Z 8    |
2020-04-28T08:12:00.6172869Z 
2020-04-28T08:12:00.6172989Z 40    |
2020-04-28T08:12:00.6172989Z 40    |
2020-04-28T08:12:00.6173202Z 41    = note: argument name cannot be a single underscore
2020-04-28T08:12:00.6173491Z 42 
2020-04-28T08:12:00.6173946Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-28T08:12:00.6174370Z + error: invalid format string: expected `}` but string was terminated
2020-04-28T08:12:00.6175072Z 45    |
2020-04-28T08:12:00.6175371Z 46 LL |     let _ = format!("{");
2020-04-28T08:12:00.6175512Z 
2020-04-28T08:12:00.6175512Z 
2020-04-28T08:12:00.6175927Z -    |                      -^ expected `'}'` in format string
2020-04-28T08:12:00.6176401Z +    |                      -^ expected `}` in format string
2020-04-28T08:12:00.6176902Z 49    |                      because of this opening brace
2020-04-28T08:12:00.6177107Z 50    |
2020-04-28T08:12:00.6177209Z 
2020-04-28T08:12:00.6177328Z 58    |
2020-04-28T08:12:00.6177328Z 58    |
2020-04-28T08:12:00.6177576Z 59    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-28T08:12:00.6177801Z 60 
2020-04-28T08:12:00.6178203Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-28T08:12:00.6178841Z + error: invalid format string: expected `}`, found `'\'`
2020-04-28T08:12:00.6179508Z 63    |
2020-04-28T08:12:00.6179508Z 63    |
2020-04-28T08:12:00.6179695Z 64 LL |     let _ = format!("{\}");
2020-04-28T08:12:00.6179959Z 68    |
2020-04-28T08:12:00.6179959Z 68    |
2020-04-28T08:12:00.6180191Z 69    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6180431Z 70 
2020-04-28T08:12:00.6180860Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-28T08:12:00.6181334Z -   --> $DIR/format-string-error.rs:21:35
2020-04-28T08:12:00.6181815Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6182463Z 73    |
2020-04-28T08:12:00.6182463Z 73    |
2020-04-28T08:12:00.6182666Z 74 LL |     let _ = format!("\n\n\n{\n\n\n");
2020-04-28T08:12:00.6183147Z -    |                            -      ^ expected `'}'` in format string
2020-04-28T08:12:00.6183654Z +    |                            -^ expected `}` in format string
2020-04-28T08:12:00.6184193Z 77    |                            because of this opening brace
2020-04-28T08:12:00.6184537Z 78    |
2020-04-28T08:12:00.6184640Z 
2020-04-28T08:12:00.6184640Z 
2020-04-28T08:12:00.6184881Z 79    = note: if you intended to print `{`, you can escape it using `{{`
---
2020-04-28T08:12:00.6235248Z 
2020-04-28T08:12:00.6235391Z 164    |             |
2020-04-28T08:12:00.6235611Z 165    |             formatting specifier missing
2020-04-28T08:12:00.6235814Z 166 
2020-04-28T08:12:00.6236250Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-28T08:12:00.6236600Z + error: invalid format string: expected `}` but string was terminated
2020-04-28T08:12:00.6237077Z 168   --> $DIR/ifmt-bad-arg.rs:51:15
2020-04-28T08:12:00.6237430Z 170 LL |     format!("{");
2020-04-28T08:12:00.6237560Z 
2020-04-28T08:12:00.6237560Z 
2020-04-28T08:12:00.6237943Z -    |              -^ expected `'}'` in format string
2020-04-28T08:12:00.6238390Z +    |              -^ expected `}` in format string
2020-04-28T08:12:00.6238849Z 173    |              because of this opening brace
2020-04-28T08:12:00.6239045Z 174    |
2020-04-28T08:12:00.6239149Z 
2020-04-28T08:12:00.6239292Z 206 LL |         {foo}
2020-04-28T08:12:00.6239292Z 206 LL |         {foo}
2020-04-28T08:12:00.6239481Z 207    |         ^^^^^
2020-04-28T08:12:00.6239628Z 208 
2020-04-28T08:12:00.6240029Z - error: invalid format string: expected `'}'`, found `'t'`
2020-04-28T08:12:00.6240484Z -   --> $DIR/ifmt-bad-arg.rs:75:1
2020-04-28T08:12:00.6240938Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6241372Z +   --> $DIR/ifmt-bad-arg.rs:74:16
2020-04-28T08:12:00.6241571Z 211    |
2020-04-28T08:12:00.6241740Z 212 LL | ninth number: {
2020-04-28T08:12:00.6242152Z -    |               - because of this opening brace
2020-04-28T08:12:00.6242552Z - LL | tenth number: {}",
2020-04-28T08:12:00.6242936Z -    | ^ expected `}` in format string
2020-04-28T08:12:00.6243359Z +    |               -^ expected `}` in format string
2020-04-28T08:12:00.6243813Z +    |               because of this opening brace
2020-04-28T08:12:00.6244007Z 216    |
2020-04-28T08:12:00.6244007Z 216    |
2020-04-28T08:12:00.6244245Z 217    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6244588Z 
2020-04-28T08:12:00.6244678Z 
2020-04-28T08:12:00.6244939Z The actual stderr differed from the expected stderr.
2020-04-28T08:12:00.6245705Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/ifmt-bad-arg.stderr
2020-04-28T08:12:00.6245705Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/ifmt-bad-arg.stderr
2020-04-28T08:12:00.6246241Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:12:00.6246745Z To only update this specific test, also pass `--test-args if/ifmt-bad-arg.rs`
2020-04-28T08:12:00.6247101Z error: 1 errors occurred comparing output.
2020-04-28T08:12:00.6247299Z status: exit code: 1
2020-04-28T08:12:00.6247299Z status: exit code: 1
2020-04-28T08:12:00.6248906Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/ifmt-bad-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/auxiliary"
2020-04-28T08:12:00.6250406Z ------------------------------------------
2020-04-28T08:12:00.6250553Z 
2020-04-28T08:12:00.6250866Z ------------------------------------------
2020-04-28T08:12:00.6251054Z stderr:
---
2020-04-28T08:12:00.6264573Z 
2020-04-28T08:12:00.6264797Z error: invalid reference to positional argument 2 (there are 2 arguments)
2020-04-28T08:12:00.6265315Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:22:28
2020-04-28T08:12:00.6265509Z    |
2020-04-28T08:12:00.6265685Z LL |     format!("{} {value} {} {}", 1, value=2);
2020-04-28T08:12:00.6266068Z    |
2020-04-28T08:12:00.6266411Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6266580Z 
2020-04-28T08:12:00.6266580Z 
2020-04-28T08:12:00.6266799Z error: invalid reference to positional arguments 3, 4 and 5 (there are 3 arguments)
2020-04-28T08:12:00.6267467Z    |
2020-04-28T08:12:00.6267467Z    |
2020-04-28T08:12:00.6267674Z LL |     format!("{name} {value} {} {} {} {} {} {}", 0, name=1, value=2);
2020-04-28T08:12:00.6267941Z    |                                      ^^ ^^ ^^
2020-04-28T08:12:00.6268462Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6268619Z 
2020-04-28T08:12:00.6268777Z error: there is no argument named `foo`
2020-04-28T08:12:00.6269203Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:17
2020-04-28T08:12:00.6269203Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:17
2020-04-28T08:12:00.6269391Z    |
2020-04-28T08:12:00.6269567Z LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
2020-04-28T08:12:00.6270127Z 
2020-04-28T08:12:00.6270300Z error: there is no argument named `bar`
2020-04-28T08:12:00.6270741Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:26
2020-04-28T08:12:00.6270958Z    |
2020-04-28T08:12:00.6270958Z    |
2020-04-28T08:12:00.6271149Z LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
2020-04-28T08:12:00.6271545Z 
2020-04-28T08:12:00.6271717Z error: there is no argument named `foo`
2020-04-28T08:12:00.6272153Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:31:14
2020-04-28T08:12:00.6272356Z    |
2020-04-28T08:12:00.6272356Z    |
2020-04-28T08:12:00.6272629Z LL |     format!("{foo}");                //~ ERROR: no argument named `foo`
2020-04-28T08:12:00.6273127Z 
2020-04-28T08:12:00.6273309Z error: multiple unused formatting arguments
2020-04-28T08:12:00.6273722Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:32:17
2020-04-28T08:12:00.6273912Z    |
2020-04-28T08:12:00.6273912Z    |
2020-04-28T08:12:00.6274185Z LL |     format!("", 1, 2);               //~ ERROR: multiple unused formatting arguments
2020-04-28T08:12:00.6274657Z    |             --  ^  ^ argument never used
2020-04-28T08:12:00.6275055Z    |             |   argument never used
2020-04-28T08:12:00.6275286Z    |             multiple missing formatting specifiers
2020-04-28T08:12:00.6275444Z 
2020-04-28T08:12:00.6275587Z error: argument never used
---
2020-04-28T08:12:00.6279340Z 
2020-04-28T08:12:00.6279488Z error: named argument never used
2020-04-28T08:12:00.6279899Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:35:26
2020-04-28T08:12:00.6280086Z    |
2020-04-28T08:12:00.6280321Z LL |     format!("{}", 1, foo=2);         //~ ERROR: named argument never used
2020-04-28T08:12:00.6280795Z    |             ----         ^ named argument never used
2020-04-28T08:12:00.6281284Z    |             formatting specifier missing
2020-04-28T08:12:00.6281433Z 
2020-04-28T08:12:00.6281591Z error: argument never used
2020-04-28T08:12:00.6281988Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:36:22
2020-04-28T08:12:00.6281988Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:36:22
2020-04-28T08:12:00.6282177Z    |
2020-04-28T08:12:00.6282419Z LL |     format!("{foo}", 1, foo=2);      //~ ERROR: argument never used
2020-04-28T08:12:00.6282861Z    |             -------  ^ argument never used
2020-04-28T08:12:00.6283252Z    |             formatting specifier missing
2020-04-28T08:12:00.6283397Z 
2020-04-28T08:12:00.6283546Z error: named argument never used
2020-04-28T08:12:00.6283942Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:37:21
2020-04-28T08:12:00.6283942Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:37:21
2020-04-28T08:12:00.6284144Z    |
2020-04-28T08:12:00.6284379Z LL |     format!("", foo=2);              //~ ERROR: named argument never used
2020-04-28T08:12:00.6284845Z    |             --      ^ named argument never used
2020-04-28T08:12:00.6285250Z    |             formatting specifier missing
2020-04-28T08:12:00.6285396Z 
2020-04-28T08:12:00.6285559Z error: multiple unused formatting arguments
2020-04-28T08:12:00.6285988Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:38:32
2020-04-28T08:12:00.6285988Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:38:32
2020-04-28T08:12:00.6286175Z    |
2020-04-28T08:12:00.6286441Z LL |     format!("{} {}", 1, 2, foo=1, bar=2);  //~ ERROR: multiple unused formatting arguments
2020-04-28T08:12:00.6286989Z    |             -------            ^      ^ named argument never used
2020-04-28T08:12:00.6287478Z    |             |                  named argument never used
2020-04-28T08:12:00.6287753Z    |             multiple missing formatting specifiers
2020-04-28T08:12:00.6287911Z 
2020-04-28T08:12:00.6288066Z error: duplicate argument named `foo`
2020-04-28T08:12:00.6288066Z error: duplicate argument named `foo`
2020-04-28T08:12:00.6288484Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:40:33
2020-04-28T08:12:00.6288675Z    |
2020-04-28T08:12:00.6288896Z LL |     format!("{foo}", foo=1, foo=2);  //~ ERROR: duplicate argument
2020-04-28T08:12:00.6289374Z    |                          -      ^ duplicate argument
2020-04-28T08:12:00.6289804Z    |                          previously here
2020-04-28T08:12:00.6290392Z 
2020-04-28T08:12:00.6290584Z error: positional arguments cannot follow named arguments
2020-04-28T08:12:00.6291032Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:41:35
2020-04-28T08:12:00.6291032Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:41:35
2020-04-28T08:12:00.6291221Z    |
2020-04-28T08:12:00.6291499Z LL |     format!("{foo} {} {}", foo=1, 2);   //~ ERROR: positional arguments cannot follow
2020-04-28T08:12:00.6292078Z    |                                -  ^ positional arguments must be before named arguments
2020-04-28T08:12:00.6292781Z    |                                named argument
2020-04-28T08:12:00.6292949Z 
2020-04-28T08:12:00.6293132Z error: there is no argument named `valueb`
2020-04-28T08:12:00.6293575Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:23
2020-04-28T08:12:00.6293575Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:23
2020-04-28T08:12:00.6293797Z    |
2020-04-28T08:12:00.6293999Z LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
2020-04-28T08:12:00.6294408Z 
2020-04-28T08:12:00.6294569Z error: named argument never used
2020-04-28T08:12:00.6294997Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:51
2020-04-28T08:12:00.6295214Z    |
2020-04-28T08:12:00.6295214Z    |
2020-04-28T08:12:00.6295416Z LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
2020-04-28T08:12:00.6295936Z    |             -------------------                   ^ named argument never used
2020-04-28T08:12:00.6296411Z    |             formatting specifier missing
2020-04-28T08:12:00.6296569Z 
2020-04-28T08:12:00.6296795Z error: invalid format string: expected `}` but string was terminated
2020-04-28T08:12:00.6297380Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:51:15
2020-04-28T08:12:00.6297380Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:51:15
2020-04-28T08:12:00.6297617Z    |
2020-04-28T08:12:00.6298131Z LL |     format!("{"); //~ ERROR: expected `'}'` but string was terminated
2020-04-28T08:12:00.6298664Z    |              -^ expected `}` in format string
2020-04-28T08:12:00.6299085Z    |              because of this opening brace
2020-04-28T08:12:00.6299284Z    |
2020-04-28T08:12:00.6299284Z    |
2020-04-28T08:12:00.6299510Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6299906Z error: invalid format string: unmatched `}` found
2020-04-28T08:12:00.6300382Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:53:18
2020-04-28T08:12:00.6300587Z    |
2020-04-28T08:12:00.6300587Z    |
2020-04-28T08:12:00.6300815Z LL |     format!("foo } bar"); //~ ERROR: unmatched `}` found
2020-04-28T08:12:00.6301130Z    |                  ^ unmatched `}` in format string
2020-04-28T08:12:00.6301323Z    |
2020-04-28T08:12:00.6301548Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-28T08:12:00.6301970Z error: invalid format string: unmatched `}` found
2020-04-28T08:12:00.6302438Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:54:18
2020-04-28T08:12:00.6302656Z    |
2020-04-28T08:12:00.6302656Z    |
2020-04-28T08:12:00.6302877Z LL |     format!("foo }"); //~ ERROR: unmatched `}` found
2020-04-28T08:12:00.6303168Z    |                  ^ unmatched `}` in format string
2020-04-28T08:12:00.6303361Z    |
2020-04-28T08:12:00.6303599Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-28T08:12:00.6303950Z error: argument never used
2020-04-28T08:12:00.6304383Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:56:27
2020-04-28T08:12:00.6304586Z    |
2020-04-28T08:12:00.6304586Z    |
2020-04-28T08:12:00.6304824Z LL |     format!("foo %s baz", "bar"); //~ ERROR: argument never used
2020-04-28T08:12:00.6305564Z    |                  |
2020-04-28T08:12:00.6305564Z    |                  |
2020-04-28T08:12:00.6305844Z    |                  help: format specifiers use curly braces: `{}`
2020-04-28T08:12:00.6306377Z    = note: printf formatting not supported; see the documentation for `std::fmt`
2020-04-28T08:12:00.6306622Z 
2020-04-28T08:12:00.6306794Z error: there is no argument named `foo`
2020-04-28T08:12:00.6307243Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:60:9
2020-04-28T08:12:00.6307243Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:60:9
2020-04-28T08:12:00.6307449Z    |
2020-04-28T08:12:00.6307591Z LL |         {foo}
2020-04-28T08:12:00.6307769Z    |         ^^^^^
2020-04-28T08:12:00.6307888Z 
2020-04-28T08:12:00.6308276Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6308964Z    |
2020-04-28T08:12:00.6308964Z    |
2020-04-28T08:12:00.6309121Z LL | ninth number: {
2020-04-28T08:12:00.6309521Z    |               -^ expected `}` in format string
2020-04-28T08:12:00.6309966Z    |               because of this opening brace
2020-04-28T08:12:00.6310155Z    |
2020-04-28T08:12:00.6310155Z    |
2020-04-28T08:12:00.6310393Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6310818Z error: 4 positional arguments in format string, but there are 3 arguments
2020-04-28T08:12:00.6311453Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:78:15
2020-04-28T08:12:00.6311677Z    |
2020-04-28T08:12:00.6311677Z    |
2020-04-28T08:12:00.6311875Z LL |     println!("{} {:.*} {}", 1, 3.2, 4);
2020-04-28T08:12:00.6312305Z    |               ^^ ^^--^ ^^   -  ---  -
2020-04-28T08:12:00.6312841Z    |                    |           this parameter corresponds to the precision flag
2020-04-28T08:12:00.6312841Z    |                    |           this parameter corresponds to the precision flag
2020-04-28T08:12:00.6313274Z    |                    this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
2020-04-28T08:12:00.6313990Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6313990Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6314577Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-28T08:12:00.6315188Z error: 4 positional arguments in format string, but there are 3 arguments
2020-04-28T08:12:00.6315691Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:81:15
2020-04-28T08:12:00.6315895Z    |
2020-04-28T08:12:00.6315895Z    |
2020-04-28T08:12:00.6316110Z LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
2020-04-28T08:12:00.6316535Z    |               ^^ ^^^----^ ^^   -  ---  -
2020-04-28T08:12:00.6317086Z    |                     | |           this parameter corresponds to the precision flag
2020-04-28T08:12:00.6317086Z    |                     | |           this parameter corresponds to the precision flag
2020-04-28T08:12:00.6317525Z    |                     | this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
2020-04-28T08:12:00.6317998Z    |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
2020-04-28T08:12:00.6318653Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6318653Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6319237Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-28T08:12:00.6319746Z error: invalid reference to positional argument 7 (there are 3 arguments)
2020-04-28T08:12:00.6320232Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:84:18
2020-04-28T08:12:00.6320451Z    |
2020-04-28T08:12:00.6320451Z    |
2020-04-28T08:12:00.6320647Z LL |     println!("{} {:07$} {}", 1, 3.2, 4);
2020-04-28T08:12:00.6321031Z    |                  ^^^--^
2020-04-28T08:12:00.6321226Z    |                     |
2020-04-28T08:12:00.6321557Z    |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
2020-04-28T08:12:00.6322191Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6322191Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6322790Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-28T08:12:00.6323226Z error: unknown format trait `foo`
2020-04-28T08:12:00.6323671Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:86:17
2020-04-28T08:12:00.6323873Z    |
2020-04-28T08:12:00.6323873Z    |
2020-04-28T08:12:00.6324103Z LL |     println!("{:foo}", 1); //~ ERROR unknown format trait `foo`
2020-04-28T08:12:00.6324528Z    |
2020-04-28T08:12:00.6324751Z    = note: the only appropriate formatting traits are:
2020-04-28T08:12:00.6325206Z            - ``, which uses the `Display` trait
2020-04-28T08:12:00.6325641Z            - `?`, which uses the `Debug` trait
2020-04-28T08:12:00.6325641Z            - `?`, which uses the `Debug` trait
2020-04-28T08:12:00.6326066Z            - `e`, which uses the `LowerExp` trait
2020-04-28T08:12:00.6326492Z            - `E`, which uses the `UpperExp` trait
2020-04-28T08:12:00.6326929Z            - `o`, which uses the `Octal` trait
2020-04-28T08:12:00.6327349Z            - `p`, which uses the `Pointer` trait
2020-04-28T08:12:00.6327770Z            - `b`, which uses the `Binary` trait
2020-04-28T08:12:00.6328214Z            - `x`, which uses the `LowerHex` trait
2020-04-28T08:12:00.6328646Z            - `X`, which uses the `UpperHex` trait
2020-04-28T08:12:00.6328811Z 
2020-04-28T08:12:00.6329064Z error: invalid reference to positional arguments 4, 5, 6 and 7 (there is 1 argument)
2020-04-28T08:12:00.6329768Z    |
2020-04-28T08:12:00.6329768Z    |
2020-04-28T08:12:00.6330076Z LL |     println!("{5} {:4$} {6:7$}", 1);
2020-04-28T08:12:00.6330488Z    |               ^^^ ^^--^ ^^^--^
2020-04-28T08:12:00.6330702Z    |                     |      |
2020-04-28T08:12:00.6331045Z    |                     |      this width flag expects an `usize` argument at position 7, but there is 1 argument
2020-04-28T08:12:00.6331483Z    |                     this width flag expects an `usize` argument at position 4, but there is 1 argument
2020-04-28T08:12:00.6332130Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6332130Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6332789Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-28T08:12:00.6333340Z error: 2 positional arguments in format string, but no arguments were given
2020-04-28T08:12:00.6333861Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:92:15
2020-04-28T08:12:00.6334065Z    |
2020-04-28T08:12:00.6334065Z    |
2020-04-28T08:12:00.6334234Z LL |     println!("{:.*}");
2020-04-28T08:12:00.6334604Z    |               ^^--^
2020-04-28T08:12:00.6334786Z    |                 |
2020-04-28T08:12:00.6335124Z    |                 this precision flag adds an extra required argument at position 0, which is why there are 2 arguments expected
2020-04-28T08:12:00.6335803Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6335803Z    = note: positional arguments are zero-based
2020-04-28T08:12:00.6336383Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-28T08:12:00.6336834Z error[E0308]: mismatched types
2020-04-28T08:12:00.6337267Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:78:32
2020-04-28T08:12:00.6337483Z    |
2020-04-28T08:12:00.6337483Z    |
2020-04-28T08:12:00.6337682Z LL |     println!("{} {:.*} {}", 1, 3.2, 4);
2020-04-28T08:12:00.6338203Z    |                                ^^^ expected `usize`, found floating-point number
2020-04-28T08:12:00.6338637Z    = note: expected reference `&usize`
2020-04-28T08:12:00.6338637Z    = note: expected reference `&usize`
2020-04-28T08:12:00.6338868Z               found reference `&{float}`
2020-04-28T08:12:00.6339695Z 
2020-04-28T08:12:00.6339858Z error[E0308]: mismatched types
2020-04-28T08:12:00.6340286Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:81:35
2020-04-28T08:12:00.6340505Z    |
2020-04-28T08:12:00.6340505Z    |
2020-04-28T08:12:00.6340702Z LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
2020-04-28T08:12:00.6341234Z    |                                   ^^^ expected `usize`, found floating-point number
2020-04-28T08:12:00.6341679Z    = note: expected reference `&usize`
2020-04-28T08:12:00.6341679Z    = note: expected reference `&usize`
2020-04-28T08:12:00.6341914Z               found reference `&{float}`
2020-04-28T08:12:00.6342742Z 
2020-04-28T08:12:00.6342915Z error: aborting due to 36 previous errors
2020-04-28T08:12:00.6343072Z 
2020-04-28T08:12:00.6343490Z For more information about this error, try `rustc --explain E0308`.
2020-04-28T08:12:00.6343490Z For more information about this error, try `rustc --explain E0308`.
2020-04-28T08:12:00.6343683Z 
2020-04-28T08:12:00.6344022Z ------------------------------------------
2020-04-28T08:12:00.6344177Z 
2020-04-28T08:12:00.6344269Z 
2020-04-28T08:12:00.6344636Z ---- [ui] ui/issues/issue-51848.rs stdout ----
2020-04-28T08:12:00.6344847Z diff of stderr:
2020-04-28T08:12:00.6344970Z 
2020-04-28T08:12:00.6345403Z - error: invalid format string: expected `}` but string was terminated
2020-04-28T08:12:00.6345914Z + error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6346545Z 3    |
2020-04-28T08:12:00.6346712Z 4 LL |         println!("{");
2020-04-28T08:12:00.6346847Z 
2020-04-28T08:12:00.6346938Z 
2020-04-28T08:12:00.6346938Z 
2020-04-28T08:12:00.6347123Z The actual stderr differed from the expected stderr.
2020-04-28T08:12:00.6347730Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848/issue-51848.stderr
2020-04-28T08:12:00.6348288Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:12:00.6348831Z To only update this specific test, also pass `--test-args issues/issue-51848.rs`
2020-04-28T08:12:00.6349220Z error: 1 errors occurred comparing output.
2020-04-28T08:12:00.6349436Z status: exit code: 1
2020-04-28T08:12:00.6349436Z status: exit code: 1
2020-04-28T08:12:00.6351224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848/auxiliary"
2020-04-28T08:12:00.6352673Z ------------------------------------------
2020-04-28T08:12:00.6352833Z 
2020-04-28T08:12:00.6353173Z ------------------------------------------
2020-04-28T08:12:00.6353376Z stderr:
2020-04-28T08:12:00.6353376Z stderr:
2020-04-28T08:12:00.6353729Z ------------------------------------------
2020-04-28T08:12:00.6354176Z error: invalid format string: expected `}`, found `'\n'`
2020-04-28T08:12:00.6354881Z    |
2020-04-28T08:12:00.6355064Z LL |         println!("{"); //~ ERROR invalid
2020-04-28T08:12:00.6355064Z LL |         println!("{"); //~ ERROR invalid
2020-04-28T08:12:00.6355521Z    |                   -^ expected `}` in format string
2020-04-28T08:12:00.6355975Z    |                   because of this opening brace
2020-04-28T08:12:00.6356179Z ...
2020-04-28T08:12:00.6356337Z LL |     macro_with_error!();
2020-04-28T08:12:00.6356746Z    |     -------------------- in this macro invocation
2020-04-28T08:12:00.6356746Z    |     -------------------- in this macro invocation
2020-04-28T08:12:00.6356958Z    |
2020-04-28T08:12:00.6357185Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-28T08:12:00.6358042Z 
2020-04-28T08:12:00.6358258Z error: invalid format string: unmatched `}` found
2020-04-28T08:12:00.6358731Z   --> /checkout/src/test/ui/issues/issue-51848.rs:18:15
2020-04-28T08:12:00.6358938Z    |
2020-04-28T08:12:00.6358938Z    |
2020-04-28T08:12:00.6359131Z LL |     println!("}"); //~ ERROR invalid
2020-04-28T08:12:00.6359378Z    |               ^ unmatched `}` in format string
2020-04-28T08:12:00.6359571Z    |
2020-04-28T08:12:00.6359807Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-28T08:12:00.6360183Z error: aborting due to 2 previous errors
2020-04-28T08:12:00.6360337Z 
2020-04-28T08:12:00.6360442Z 
2020-04-28T08:12:00.6360785Z ------------------------------------------
---
2020-04-28T08:12:00.6364145Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-28T08:12:00.6364521Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-28T08:12:00.6364747Z 
2020-04-28T08:12:00.6364838Z 
2020-04-28T08:12:00.6368240Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-28T08:12:00.6370659Z 
2020-04-28T08:12:00.6370753Z 
2020-04-28T08:12:00.6371240Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-28T08:12:00.6371590Z Build completed unsuccessfully in 0:56:04
2020-04-28T08:12:00.6371590Z Build completed unsuccessfully in 0:56:04
2020-04-28T08:12:00.6371816Z == clock drift check ==
2020-04-28T08:12:00.6372055Z   local time: Tue Apr 28 08:12:00 UTC 2020
2020-04-28T08:12:00.6839935Z   network time: Tue, 28 Apr 2020 08:12:00 GMT
2020-04-28T08:12:01.2983031Z 
2020-04-28T08:12:01.2983031Z 
2020-04-28T08:12:01.3051155Z ##[error]Bash exited with code '1'.
2020-04-28T08:12:01.3064646Z ##[section]Finishing: Run build
2020-04-28T08:12:01.3106128Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-28T08:12:01.3111906Z Task         : Get sources
2020-04-28T08:12:01.3112217Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T08:12:01.3112499Z Version      : 1.0.0
2020-04-28T08:12:01.3112717Z Author       : Microsoft
2020-04-28T08:12:01.3112717Z Author       : Microsoft
2020-04-28T08:12:01.3113035Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T08:12:01.3113396Z ==============================================================================
2020-04-28T08:12:01.6197183Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T08:12:01.6257188Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-28T08:12:01.6344017Z Cleaning up task key
2020-04-28T08:12:01.6345111Z Start cleaning up orphan processes.
2020-04-28T08:12:01.6510517Z Terminate orphan process: pid (4328) (python)
2020-04-28T08:12:01.6791971Z ##[section]Finishing: Finalize Job
