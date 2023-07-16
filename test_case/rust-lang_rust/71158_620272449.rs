plain
2020-04-27T21:36:10.5849539Z ========================== Starting Command Output ===========================
2020-04-27T21:36:10.5854305Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/70670c4f-2b1e-4599-97d0-6dcb0fe707b6.sh
2020-04-27T21:36:10.5854721Z 
2020-04-27T21:36:10.5858961Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T21:36:10.5876532Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T21:36:10.5879557Z Task         : Get sources
2020-04-27T21:36:10.5879845Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T21:36:10.5880108Z Version      : 1.0.0
2020-04-27T21:36:10.5880294Z Author       : Microsoft
---
2020-04-27T21:36:11.7886178Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T21:36:11.7894623Z ##[command]git config gc.auto 0
2020-04-27T21:36:11.7898759Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T21:36:11.7902027Z ##[command]git config --get-all http.proxy
2020-04-27T21:36:11.7909626Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71158/merge:refs/remotes/pull/71158/merge
---
2020-04-27T21:38:16.6798473Z  ---> cb2676f08729
2020-04-27T21:38:16.6799547Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-27T21:38:16.6804569Z  ---> Using cache
2020-04-27T21:38:16.6805163Z  ---> df25ce111862
2020-04-27T21:38:16.6806547Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-27T21:38:16.6812296Z  ---> 599b9ac96b27
2020-04-27T21:38:16.6812629Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-27T21:38:16.6817180Z  ---> Using cache
2020-04-27T21:38:16.6817909Z  ---> 091087e35a36
---
2020-04-27T21:38:16.7437203Z Looks like docker image is the same as before, not uploading
2020-04-27T21:38:25.0087912Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T21:38:25.0312218Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T21:38:25.0340960Z == clock drift check ==
2020-04-27T21:38:25.0350492Z   local time: Mon Apr 27 21:38:25 UTC 2020
2020-04-27T21:38:25.0771641Z   network time: Mon, 27 Apr 2020 21:38:25 GMT
2020-04-27T21:38:25.0808992Z Starting sccache server...
2020-04-27T21:38:25.1606422Z configure: processing command line
2020-04-27T21:38:25.1607091Z configure: 
2020-04-27T21:38:25.1608250Z configure: rust.dist-src        := False
---
2020-04-27T21:40:42.8654090Z    Compiling unicode-width v0.1.6
2020-04-27T21:40:42.9510694Z    Compiling getopts v0.2.21
2020-04-27T21:40:52.6785624Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-27T21:41:00.9640138Z     Finished release [optimized] target(s) in 1m 00s
2020-04-27T21:41:00.9641146Z {"reason":"build-finished","success":true}
2020-04-27T21:41:00.9910296Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T21:41:01.5434320Z    Compiling cfg-if v0.1.10
2020-04-27T21:41:01.5434767Z    Compiling libc v0.2.69
2020-04-27T21:41:01.5918995Z    Compiling semver-parser v0.7.0
---
2020-04-27T21:43:33.7809961Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T21:43:35.2358406Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T21:43:36.6622430Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T21:43:37.7087973Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T21:43:46.4711559Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T21:43:48.8989308Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T21:43:53.3118865Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T21:43:57.3847377Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T21:44:07.0103827Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T21:57:27.5820837Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T21:57:57.3935799Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T22:02:00.4631815Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T22:02:01.0432045Z     Finished release [optimized] target(s) in 21m 00s
2020-04-27T22:02:01.0433291Z {"reason":"build-finished","success":true}
2020-04-27T22:02:01.0960254Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-27T22:02:01.0977952Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T22:02:01.3797069Z    Compiling cc v1.0.50
2020-04-27T22:02:01.3814798Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-27T22:02:45.7718258Z    Compiling term v0.0.0 (/checkout/src/libterm)
2020-04-27T22:02:50.0419802Z    Compiling unicode-width v0.1.6
2020-04-27T22:02:50.1376321Z    Compiling getopts v0.2.21
2020-04-27T22:03:01.7836992Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-27T22:03:10.8529772Z     Finished release [optimized] target(s) in 1m 09s{"reason":"build-finished","success":true}
2020-04-27T22:03:10.8627679Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-27T22:03:10.8643501Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T22:03:11.3897938Z    Compiling cfg-if v0.1.10
2020-04-27T22:03:11.3898496Z    Compiling libc v0.2.69
---
2020-04-27T22:05:58.1959498Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T22:05:59.7887450Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T22:06:01.5341597Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T22:06:02.5325202Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T22:06:12.4976930Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T22:06:15.4217451Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T22:06:20.0164877Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T22:06:24.3147669Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T22:06:33.9048076Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T22:22:24.0714105Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T22:22:31.7941374Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T22:25:26.5902285Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T22:25:27.1853818Z     Finished release [optimized] target(s) in 22m 16s
2020-04-27T22:25:27.1856558Z {"reason":"build-finished","success":true}
2020-04-27T22:25:27.2367725Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-27T22:25:27.2383769Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T22:25:27.2384616Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-27T22:25:27.2393883Z Building test helpers
---
2020-04-27T22:26:07.9577876Z    Compiling serde_derive v1.0.81
2020-04-27T22:26:34.0789924Z    Compiling serde_json v1.0.40
2020-04-27T22:26:35.4668243Z    Compiling rustfix v0.5.0
2020-04-27T22:26:38.6098999Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-27T22:26:52.0482325Z    {"reason":"build-finished","success":true}
2020-04-27T22:26:52.0727717Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T22:26:54.8254101Z 
2020-04-27T22:26:54.8254428Z running 9927 tests
2020-04-27T22:27:07.1660724Z .................................................................................................... 100/9927
---
2020-04-27T22:28:47.8880196Z .................................................................................................... 1700/9927
2020-04-27T22:28:51.9261333Z .................................................................................................... 1800/9927
2020-04-27T22:28:59.2447615Z .................................................................................................... 1900/9927
2020-04-27T22:29:07.1108056Z ........i........................................................................................... 2000/9927
2020-04-27T22:29:13.0383790Z ..................................................................................................ii 2100/9927
2020-04-27T22:29:25.7094430Z iii................................................................................................. 2200/9927
2020-04-27T22:29:33.7115345Z .................................................................................................... 2400/9927
2020-04-27T22:29:35.8692037Z .................................................................................................... 2500/9927
2020-04-27T22:29:41.0726823Z .................................................................................................... 2600/9927
2020-04-27T22:29:58.3176519Z .................................................................................................... 2700/9927
---
2020-04-27T22:32:26.1275923Z .....................................................................F.............................. 5100/9927
2020-04-27T22:32:32.6589318Z .................................................................................................... 5200/9927
2020-04-27T22:32:36.8974684Z .....................i.............................................................................. 5300/9927
2020-04-27T22:32:45.9618569Z ............i....................................................................................... 5400/9927
2020-04-27T22:32:51.3315648Z .............ii.ii........i...i..................................................................... 5500/9927
2020-04-27T22:32:58.6518512Z ............................................................i....................................... 5700/9927
2020-04-27T22:33:06.7636025Z .............................................................................................ii..... 5800/9927
2020-04-27T22:33:12.9328760Z ................................i................................................................... 5900/9927
2020-04-27T22:33:18.2928442Z .................................................................................................... 6000/9927
2020-04-27T22:33:18.2928442Z .................................................................................................... 6000/9927
2020-04-27T22:33:28.0431082Z .................................................................................................... 6100/9927
2020-04-27T22:33:37.5370645Z ..........................ii...i..ii...........i.................................................... 6200/9927
2020-04-27T22:33:52.6920582Z .................................................................................................... 6400/9927
2020-04-27T22:33:59.0360995Z .................................................................................................... 6500/9927
2020-04-27T22:33:59.0360995Z .................................................................................................... 6500/9927
2020-04-27T22:34:08.3232467Z ........................................................i..ii....................................... 6600/9927
2020-04-27T22:34:30.4139311Z .................................................................................................... 6800/9927
2020-04-27T22:34:34.5614524Z .........................................................i.......................................... 6900/9927
2020-04-27T22:34:36.4787338Z .................................................................................................... 7000/9927
2020-04-27T22:34:38.4429216Z ...................................................................................................i 7100/9927
---
2020-04-27T22:36:11.0155971Z .................................................................................................... 7900/9927
2020-04-27T22:36:15.8629753Z .................................................................................................... 8000/9927
2020-04-27T22:36:21.7857214Z ...................................................................i................................ 8100/9927
2020-04-27T22:36:30.9089605Z .................................................................................................... 8200/9927
2020-04-27T22:36:35.9167904Z ................iiiiii.iiiii.i...................................................................... 8300/9927
2020-04-27T22:36:48.6908517Z .................................................................................................... 8500/9927
2020-04-27T22:36:53.9579588Z .................................................................................................... 8600/9927
2020-04-27T22:37:07.2820878Z .................................................................................................... 8700/9927
2020-04-27T22:37:13.6872305Z .................................................................................................... 8800/9927
---
2020-04-27T22:38:55.8664064Z ---- [ui] ui/fmt/format-string-error-2.rs stdout ----
2020-04-27T22:38:55.8664425Z diff of stderr:
2020-04-27T22:38:55.8664638Z 
2020-04-27T22:38:55.8664899Z 6    |                      |
2020-04-27T22:38:55.8665298Z 7    |                      help: format of unicode escape sequences uses braces: `\u{8}`
2020-04-27T22:38:55.8665877Z 8 
2020-04-27T22:38:55.8666469Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-27T22:38:55.8667064Z -   --> $DIR/format-string-error-2.rs:5:5
2020-04-27T22:38:55.8667659Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8668887Z 11    |
2020-04-27T22:38:55.8669807Z 12 LL |     format!("{
2020-04-27T22:38:55.8671409Z -    |              - because of this opening brace
2020-04-27T22:38:55.8672895Z - LL |     a");
2020-04-27T22:38:55.8672895Z - LL |     a");
2020-04-27T22:38:55.8676668Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8679026Z +    |              -^ expected `}` in format string
2020-04-27T22:38:55.8681782Z +    |              because of this opening brace
2020-04-27T22:38:55.8683034Z 16    |
2020-04-27T22:38:55.8683034Z 16    |
2020-04-27T22:38:55.8683450Z 17    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8684959Z 
2020-04-27T22:38:55.8684959Z 
2020-04-27T22:38:55.8685851Z - error: invalid format string: expected `'}'`, found `'b'`
2020-04-27T22:38:55.8688170Z -   --> $DIR/format-string-error-2.rs:9:5
2020-04-27T22:38:55.8689931Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8692840Z 21    |
2020-04-27T22:38:55.8693125Z 22 LL |     format!("{ \
2020-04-27T22:38:55.8694504Z -    |              - because of this opening brace
2020-04-27T22:38:55.8695948Z - LL | 
2020-04-27T22:38:55.8695948Z - LL | 
2020-04-27T22:38:55.8696570Z - LL |     b");
2020-04-27T22:38:55.8698248Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8699769Z +    |              -^ expected `}` in format string
2020-04-27T22:38:55.8701351Z +    |              because of this opening brace
2020-04-27T22:38:55.8701814Z 27    |
2020-04-27T22:38:55.8701814Z 27    |
2020-04-27T22:38:55.8702150Z 28    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8702928Z 
2020-04-27T22:38:55.8702928Z 
2020-04-27T22:38:55.8703746Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-27T22:38:55.8704396Z -   --> $DIR/format-string-error-2.rs:11:18
2020-04-27T22:38:55.8704989Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8705907Z 32    |
2020-04-27T22:38:55.8705907Z 32    |
2020-04-27T22:38:55.8706153Z 33 LL |     format!(r#"{ \
2020-04-27T22:38:55.8706686Z -    |                - ^ expected `}` in format string
2020-04-27T22:38:55.8707258Z +    |                -^ expected `}` in format string
2020-04-27T22:38:55.8707917Z 36    |                because of this opening brace
2020-04-27T22:38:55.8708193Z 37    |
2020-04-27T22:38:55.8708383Z 
2020-04-27T22:38:55.8708383Z 
2020-04-27T22:38:55.8708706Z 38    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8709019Z 39 
2020-04-27T22:38:55.8709521Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-27T22:38:55.8710131Z -   --> $DIR/format-string-error-2.rs:15:18
2020-04-27T22:38:55.8710718Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8711622Z 42    |
2020-04-27T22:38:55.8711622Z 42    |
2020-04-27T22:38:55.8711869Z 43 LL |     format!(r#"{ \n
2020-04-27T22:38:55.8712390Z -    |                - ^ expected `}` in format string
2020-04-27T22:38:55.8712963Z +    |                -^ expected `}` in format string
2020-04-27T22:38:55.8713711Z 46    |                because of this opening brace
2020-04-27T22:38:55.8717575Z 47    |
2020-04-27T22:38:55.8717678Z 
2020-04-27T22:38:55.8717678Z 
2020-04-27T22:38:55.8717893Z 48    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8718095Z 49 
2020-04-27T22:38:55.8719821Z - error: invalid format string: expected `'}'`, found `'e'`
2020-04-27T22:38:55.8720543Z -   --> $DIR/format-string-error-2.rs:21:5
2020-04-27T22:38:55.8720975Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8721576Z 52    |
2020-04-27T22:38:55.8723454Z 53 LL |     format!("{ \n
2020-04-27T22:38:55.8723934Z -    |              - because of this opening brace
2020-04-27T22:38:55.8724259Z - LL | \n
2020-04-27T22:38:55.8724259Z - LL | \n
2020-04-27T22:38:55.8724517Z - LL |     e");
2020-04-27T22:38:55.8724834Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8725340Z +    |              -^ expected `}` in format string
2020-04-27T22:38:55.8725756Z +    |              because of this opening brace
2020-04-27T22:38:55.8725925Z 58    |
2020-04-27T22:38:55.8725925Z 58    |
2020-04-27T22:38:55.8726150Z 59    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8726438Z 
2020-04-27T22:38:55.8726438Z 
2020-04-27T22:38:55.8726844Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-27T22:38:55.8727266Z -   --> $DIR/format-string-error-2.rs:25:5
2020-04-27T22:38:55.8727685Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8728279Z 63    |
2020-04-27T22:38:55.8728401Z 64 LL |     {
2020-04-27T22:38:55.8728717Z -    |     - because of this opening brace
2020-04-27T22:38:55.8729030Z - LL |     a");
2020-04-27T22:38:55.8729030Z - LL |     a");
2020-04-27T22:38:55.8729345Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8729703Z +    |     -^ expected `}` in format string
2020-04-27T22:38:55.8730055Z +    |     because of this opening brace
2020-04-27T22:38:55.8730211Z 68    |
2020-04-27T22:38:55.8730211Z 68    |
2020-04-27T22:38:55.8730668Z 69    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8730959Z 
2020-04-27T22:38:55.8730959Z 
2020-04-27T22:38:55.8731354Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-27T22:38:55.8731789Z -   --> $DIR/format-string-error-2.rs:29:5
2020-04-27T22:38:55.8732208Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8732800Z 73    |
2020-04-27T22:38:55.8732921Z 74 LL |     {
2020-04-27T22:38:55.8733236Z -    |     - because of this opening brace
2020-04-27T22:38:55.8733539Z - LL |     a
2020-04-27T22:38:55.8733539Z - LL |     a
2020-04-27T22:38:55.8733851Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8734204Z +    |     -^ expected `}` in format string
2020-04-27T22:38:55.8734559Z +    |     because of this opening brace
2020-04-27T22:38:55.8734715Z 78    |
2020-04-27T22:38:55.8734715Z 78    |
2020-04-27T22:38:55.8734938Z 79    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8735222Z 
2020-04-27T22:38:55.8735222Z 
2020-04-27T22:38:55.8735720Z - error: invalid format string: expected `'}'`, found `'b'`
2020-04-27T22:38:55.8736149Z -   --> $DIR/format-string-error-2.rs:35:5
2020-04-27T22:38:55.8736575Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8737168Z 83    |
2020-04-27T22:38:55.8737292Z 84 LL |     { \
2020-04-27T22:38:55.8737928Z -    |     - because of this opening brace
2020-04-27T22:38:55.8738406Z - LL |         \
2020-04-27T22:38:55.8738406Z - LL |         \
2020-04-27T22:38:55.8739458Z - LL |     b");
2020-04-27T22:38:55.8739788Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8740142Z +    |     -^ expected `}` in format string
2020-04-27T22:38:55.8741108Z +    |     because of this opening brace
2020-04-27T22:38:55.8741439Z 89    |
2020-04-27T22:38:55.8741439Z 89    |
2020-04-27T22:38:55.8741671Z 90    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8741953Z 
2020-04-27T22:38:55.8741953Z 
2020-04-27T22:38:55.8742424Z - error: invalid format string: expected `'}'`, found `'b'`
2020-04-27T22:38:55.8743025Z -   --> $DIR/format-string-error-2.rs:40:5
2020-04-27T22:38:55.8743648Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8744252Z 94    |
2020-04-27T22:38:55.8749932Z 95 LL |     { \
2020-04-27T22:38:55.8752180Z -    |     - because of this opening brace
2020-04-27T22:38:55.8752607Z - LL |         \
2020-04-27T22:38:55.8752607Z - LL |         \
2020-04-27T22:38:55.8752913Z - LL |     b \
2020-04-27T22:38:55.8753278Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8753708Z +    |     -^ expected `}` in format string
2020-04-27T22:38:55.8754285Z +    |     because of this opening brace
2020-04-27T22:38:55.8754488Z 100    |
2020-04-27T22:38:55.8754488Z 100    |
2020-04-27T22:38:55.8754731Z 101    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8755067Z 
2020-04-27T22:38:55.8755067Z 
2020-04-27T22:38:55.8755788Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-27T22:38:55.8756310Z -   --> $DIR/format-string-error-2.rs:45:8
2020-04-27T22:38:55.8756735Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8757335Z 105    |
2020-04-27T22:38:55.8757335Z 105    |
2020-04-27T22:38:55.8757463Z 106 LL | raw  { \
2020-04-27T22:38:55.8757805Z -    |      - ^ expected `}` in format string
2020-04-27T22:38:55.8758168Z +    |      -^ expected `}` in format string
2020-04-27T22:38:55.8758530Z 109    |      because of this opening brace
2020-04-27T22:38:55.8758691Z 110    |
2020-04-27T22:38:55.8758786Z 
2020-04-27T22:38:55.8758786Z 
2020-04-27T22:38:55.8758988Z 111    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8759208Z 112 
2020-04-27T22:38:55.8759567Z - error: invalid format string: expected `'}'`, found `'\'`
2020-04-27T22:38:55.8759973Z -   --> $DIR/format-string-error-2.rs:50:8
2020-04-27T22:38:55.8760407Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8760991Z 115    |
2020-04-27T22:38:55.8760991Z 115    |
2020-04-27T22:38:55.8761136Z 116 LL | raw  { \n
2020-04-27T22:38:55.8761466Z -    |      - ^ expected `}` in format string
2020-04-27T22:38:55.8761826Z +    |      -^ expected `}` in format string
2020-04-27T22:38:55.8762190Z 119    |      because of this opening brace
2020-04-27T22:38:55.8762352Z 120    |
2020-04-27T22:38:55.8762442Z 
2020-04-27T22:38:55.8762442Z 
2020-04-27T22:38:55.8762658Z 121    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8762862Z 122 
2020-04-27T22:38:55.8763221Z - error: invalid format string: expected `'}'`, found `'e'`
2020-04-27T22:38:55.8763643Z -   --> $DIR/format-string-error-2.rs:57:5
2020-04-27T22:38:55.8764060Z + error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8764653Z 125    |
2020-04-27T22:38:55.8764778Z 126 LL |   { \n
2020-04-27T22:38:55.8765099Z -    |   - because of this opening brace
2020-04-27T22:38:55.8765394Z - LL | \n
2020-04-27T22:38:55.8765394Z - LL | \n
2020-04-27T22:38:55.8765648Z - LL |     e");
2020-04-27T22:38:55.8765965Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8766316Z +    |   -^ expected `}` in format string
2020-04-27T22:38:55.8766658Z +    |   because of this opening brace
2020-04-27T22:38:55.8766814Z 131    |
2020-04-27T22:38:55.8766814Z 131    |
2020-04-27T22:38:55.8767039Z 132    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8767329Z 
2020-04-27T22:38:55.8767329Z 
2020-04-27T22:38:55.8767702Z - error: invalid format string: expected `'}'`, found `'a'`
2020-04-27T22:38:55.8768111Z -   --> $DIR/format-string-error-2.rs:67:5
2020-04-27T22:38:55.8768528Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8769126Z 136    |
2020-04-27T22:38:55.8769254Z + LL |     {asdf
2020-04-27T22:38:55.8769703Z +    |     -    ^ expected `}` in format string
2020-04-27T22:38:55.8769895Z +    |     |
2020-04-27T22:38:55.8769895Z +    |     |
2020-04-27T22:38:55.8770060Z +    |     because of this opening brace
2020-04-27T22:38:55.8770215Z +    |
2020-04-27T22:38:55.8770437Z +    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8770667Z + 
2020-04-27T22:38:55.8771036Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8771634Z +    |
2020-04-27T22:38:55.8771757Z 137 LL |     {
2020-04-27T22:38:55.8772155Z -    |     - because of this opening brace
2020-04-27T22:38:55.8772488Z - LL |     asdf}
2020-04-27T22:38:55.8772488Z - LL |     asdf}
2020-04-27T22:38:55.8772807Z -    |     ^ expected `}` in format string
2020-04-27T22:38:55.8773163Z +    |     -^ expected `}` in format string
2020-04-27T22:38:55.8773516Z +    |     because of this opening brace
2020-04-27T22:38:55.8773675Z 141    |
2020-04-27T22:38:55.8773675Z 141    |
2020-04-27T22:38:55.8773902Z 142    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8774198Z 
2020-04-27T22:38:55.8774198Z 
2020-04-27T22:38:55.8774351Z 147 LL |     println!("\t{}");
2020-04-27T22:38:55.8774659Z 149 
2020-04-27T22:38:55.8774659Z 149 
2020-04-27T22:38:55.8775049Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-27T22:38:55.8775528Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8776123Z 152    |
2020-04-27T22:38:55.8776123Z 152    |
2020-04-27T22:38:55.8776302Z 153 LL |     println!("\x7B}\u{8} {", 1);
2020-04-27T22:38:55.8776435Z 
2020-04-27T22:38:55.8776789Z -    |                          -^ expected `'}'` in format string
2020-04-27T22:38:55.8777237Z +    |                          -^ expected `}` in format string
2020-04-27T22:38:55.8777693Z 156    |                          because of this opening brace
2020-04-27T22:38:55.8777903Z 157    |
2020-04-27T22:38:55.8777993Z 
2020-04-27T22:38:55.8778097Z 173    |
2020-04-27T22:38:55.8778097Z 173    |
2020-04-27T22:38:55.8778306Z 174    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-27T22:38:55.8778843Z - error: aborting due to 18 previous errors
2020-04-27T22:38:55.8779062Z + error: aborting due to 19 previous errors
2020-04-27T22:38:55.8779238Z 177 
2020-04-27T22:38:55.8779342Z 178 
2020-04-27T22:38:55.8779342Z 178 
2020-04-27T22:38:55.8779427Z 
2020-04-27T22:38:55.8779506Z 
2020-04-27T22:38:55.8779695Z The actual stderr differed from the expected stderr.
2020-04-27T22:38:55.8780262Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/format-string-error-2.stderr
2020-04-27T22:38:55.8780786Z To update references, rerun the tests and pass the `--bless` flag
2020-04-27T22:38:55.8781282Z To only update this specific test, also pass `--test-args fmt/format-string-error-2.rs`
2020-04-27T22:38:55.8781641Z error: 1 errors occurred comparing output.
2020-04-27T22:38:55.8781831Z status: exit code: 1
2020-04-27T22:38:55.8781831Z status: exit code: 1
2020-04-27T22:38:55.8784100Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/auxiliary"
2020-04-27T22:38:55.8785586Z ------------------------------------------
2020-04-27T22:38:55.8785738Z 
2020-04-27T22:38:55.8786079Z ------------------------------------------
2020-04-27T22:38:55.8786255Z stderr:
2020-04-27T22:38:55.8786255Z stderr:
2020-04-27T22:38:55.8786728Z ------------------------------------------
2020-04-27T22:38:55.8786953Z error: incorrect unicode escape sequence
2020-04-27T22:38:55.8787417Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:77:20
2020-04-27T22:38:55.8787630Z    |
2020-04-27T22:38:55.8787791Z LL |     println!("\x7B}\u8 {", 1);
2020-04-27T22:38:55.8788321Z    |                      |
2020-04-27T22:38:55.8788321Z    |                      |
2020-04-27T22:38:55.8788624Z    |                      help: format of unicode escape sequences uses braces: `\u{8}`
2020-04-27T22:38:55.8788885Z 
2020-04-27T22:38:55.8789324Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8790127Z    |
2020-04-27T22:38:55.8790254Z LL |     format!("{
2020-04-27T22:38:55.8790254Z LL |     format!("{
2020-04-27T22:38:55.8790596Z    |              -^ expected `}` in format string
2020-04-27T22:38:55.8790984Z    |              because of this opening brace
2020-04-27T22:38:55.8791151Z    |
2020-04-27T22:38:55.8791151Z    |
2020-04-27T22:38:55.8791365Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8791541Z 
2020-04-27T22:38:55.8791887Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8792536Z    |
2020-04-27T22:38:55.8792665Z LL |     format!("{ \
2020-04-27T22:38:55.8792665Z LL |     format!("{ \
2020-04-27T22:38:55.8793007Z    |              -^ expected `}` in format string
2020-04-27T22:38:55.8793401Z    |              because of this opening brace
2020-04-27T22:38:55.8793562Z    |
2020-04-27T22:38:55.8793562Z    |
2020-04-27T22:38:55.8793776Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8793951Z 
2020-04-27T22:38:55.8794297Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8794961Z    |
2020-04-27T22:38:55.8794961Z    |
2020-04-27T22:38:55.8795094Z LL |     format!(r#"{ \
2020-04-27T22:38:55.8795446Z    |                -^ expected `}` in format string
2020-04-27T22:38:55.8795847Z    |                because of this opening brace
2020-04-27T22:38:55.8796011Z    |
2020-04-27T22:38:55.8796011Z    |
2020-04-27T22:38:55.8796224Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8796398Z 
2020-04-27T22:38:55.8796740Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8797404Z    |
2020-04-27T22:38:55.8797404Z    |
2020-04-27T22:38:55.8797538Z LL |     format!(r#"{ \n
2020-04-27T22:38:55.8797891Z    |                -^ expected `}` in format string
2020-04-27T22:38:55.8798289Z    |                because of this opening brace
2020-04-27T22:38:55.8798452Z    |
2020-04-27T22:38:55.8798452Z    |
2020-04-27T22:38:55.8798665Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8798844Z 
2020-04-27T22:38:55.8799189Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8799845Z    |
2020-04-27T22:38:55.8799976Z LL |     format!("{ \n
2020-04-27T22:38:55.8799976Z LL |     format!("{ \n
2020-04-27T22:38:55.8800320Z    |              -^ expected `}` in format string
2020-04-27T22:38:55.8800708Z    |              because of this opening brace
2020-04-27T22:38:55.8800869Z    |
2020-04-27T22:38:55.8800869Z    |
2020-04-27T22:38:55.8801086Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8801260Z 
2020-04-27T22:38:55.8801608Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8802261Z    |
2020-04-27T22:38:55.8802376Z LL |     {
2020-04-27T22:38:55.8802376Z LL |     {
2020-04-27T22:38:55.8802684Z    |     -^ expected `}` in format string
2020-04-27T22:38:55.8803095Z    |     because of this opening brace
2020-04-27T22:38:55.8803245Z    |
2020-04-27T22:38:55.8803245Z    |
2020-04-27T22:38:55.8803458Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8803633Z 
2020-04-27T22:38:55.8804000Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8804659Z    |
2020-04-27T22:38:55.8804772Z LL |     {
2020-04-27T22:38:55.8804772Z LL |     {
2020-04-27T22:38:55.8805078Z    |     -^ expected `}` in format string
2020-04-27T22:38:55.8805477Z    |     because of this opening brace
2020-04-27T22:38:55.8805626Z    |
2020-04-27T22:38:55.8805626Z    |
2020-04-27T22:38:55.8805836Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8806010Z 
2020-04-27T22:38:55.8806375Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8807036Z    |
2020-04-27T22:38:55.8807153Z LL |     { \
2020-04-27T22:38:55.8807153Z LL |     { \
2020-04-27T22:38:55.8807466Z    |     -^ expected `}` in format string
2020-04-27T22:38:55.8807810Z    |     because of this opening brace
2020-04-27T22:38:55.8807960Z    |
2020-04-27T22:38:55.8807960Z    |
2020-04-27T22:38:55.8808174Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8808350Z 
2020-04-27T22:38:55.8808695Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8809350Z    |
2020-04-27T22:38:55.8809465Z LL |     { \
2020-04-27T22:38:55.8809465Z LL |     { \
2020-04-27T22:38:55.8809797Z    |     -^ expected `}` in format string
2020-04-27T22:38:55.8810122Z    |     because of this opening brace
2020-04-27T22:38:55.8810272Z    |
2020-04-27T22:38:55.8810272Z    |
2020-04-27T22:38:55.8810486Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8810666Z 
2020-04-27T22:38:55.8811011Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8811664Z    |
2020-04-27T22:38:55.8811664Z    |
2020-04-27T22:38:55.8811781Z LL | raw  { \
2020-04-27T22:38:55.8812110Z    |      -^ expected `}` in format string
2020-04-27T22:38:55.8812440Z    |      because of this opening brace
2020-04-27T22:38:55.8812589Z    |
2020-04-27T22:38:55.8812589Z    |
2020-04-27T22:38:55.8812804Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8812978Z 
2020-04-27T22:38:55.8813325Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8813978Z    |
2020-04-27T22:38:55.8813978Z    |
2020-04-27T22:38:55.8814097Z LL | raw  { \n
2020-04-27T22:38:55.8814430Z    |      -^ expected `}` in format string
2020-04-27T22:38:55.8814761Z    |      because of this opening brace
2020-04-27T22:38:55.8814916Z    |
2020-04-27T22:38:55.8814916Z    |
2020-04-27T22:38:55.8815129Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8815304Z 
2020-04-27T22:38:55.8815647Z error: invalid format string: expected `'}'`, found `' '`
2020-04-27T22:38:55.8816296Z    |
2020-04-27T22:38:55.8816410Z LL |   { \n
2020-04-27T22:38:55.8816410Z LL |   { \n
2020-04-27T22:38:55.8816736Z    |   -^ expected `}` in format string
2020-04-27T22:38:55.8817053Z    |   because of this opening brace
2020-04-27T22:38:55.8817200Z    |
2020-04-27T22:38:55.8817200Z    |
2020-04-27T22:38:55.8817413Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8817587Z 
2020-04-27T22:38:55.8817936Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8818591Z    |
2020-04-27T22:38:55.8818709Z LL |     {asdf
2020-04-27T22:38:55.8819133Z    |     -    ^ expected `}` in format string
2020-04-27T22:38:55.8819305Z    |     |
2020-04-27T22:38:55.8819305Z    |     |
2020-04-27T22:38:55.8819463Z    |     because of this opening brace
2020-04-27T22:38:55.8819611Z    |
2020-04-27T22:38:55.8819826Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8820001Z 
2020-04-27T22:38:55.8820349Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8821000Z    |
2020-04-27T22:38:55.8821113Z LL |     {
2020-04-27T22:38:55.8821113Z LL |     {
2020-04-27T22:38:55.8821507Z    |     -^ expected `}` in format string
2020-04-27T22:38:55.8821838Z    |     because of this opening brace
2020-04-27T22:38:55.8821987Z    |
2020-04-27T22:38:55.8821987Z    |
2020-04-27T22:38:55.8822197Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8822568Z error: 1 positional argument in format string, but no arguments were given
2020-04-27T22:38:55.8823425Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:70:17
2020-04-27T22:38:55.8823628Z    |
2020-04-27T22:38:55.8823628Z    |
2020-04-27T22:38:55.8823762Z LL |     println!("\t{}");
2020-04-27T22:38:55.8824050Z 
2020-04-27T22:38:55.8824050Z 
2020-04-27T22:38:55.8824405Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8825064Z    |
2020-04-27T22:38:55.8825064Z    |
2020-04-27T22:38:55.8825214Z LL |     println!("\x7B}\u{8} {", 1);
2020-04-27T22:38:55.8825610Z    |                          -^ expected `}` in format string
2020-04-27T22:38:55.8826066Z    |                          because of this opening brace
2020-04-27T22:38:55.8826243Z    |
2020-04-27T22:38:55.8826243Z    |
2020-04-27T22:38:55.8826456Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8826804Z error: invalid format string: unmatched `}` found
2020-04-27T22:38:55.8827249Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:81:21
2020-04-27T22:38:55.8827461Z    |
2020-04-27T22:38:55.8827461Z    |
2020-04-27T22:38:55.8827617Z LL |     println!(r#"\x7B}\u{8} {"#, 1);
2020-04-27T22:38:55.8827842Z    |                     ^ unmatched `}` in format string
2020-04-27T22:38:55.8828033Z    |
2020-04-27T22:38:55.8828230Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-27T22:38:55.8828595Z error: invalid format string: unmatched `}` found
2020-04-27T22:38:55.8829036Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:84:21
2020-04-27T22:38:55.8829232Z    |
2020-04-27T22:38:55.8829232Z    |
2020-04-27T22:38:55.8829399Z LL |     println!(r#"\x7B}\u8 {"#, 1);
2020-04-27T22:38:55.8829621Z    |                     ^ unmatched `}` in format string
2020-04-27T22:38:55.8829794Z    |
2020-04-27T22:38:55.8829990Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-27T22:38:55.8830336Z error: aborting due to 19 previous errors
2020-04-27T22:38:55.8830473Z 
2020-04-27T22:38:55.8830551Z 
2020-04-27T22:38:55.8830873Z ------------------------------------------
2020-04-27T22:38:55.8830873Z ------------------------------------------
2020-04-27T22:38:55.8831010Z 
2020-04-27T22:38:55.8831088Z 
2020-04-27T22:38:55.8831403Z ---- [ui] ui/fmt/format-string-error.rs stdout ----
2020-04-27T22:38:55.8831613Z diff of stderr:
2020-04-27T22:38:55.8831720Z 
2020-04-27T22:38:55.8832094Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-27T22:38:55.8832568Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8833148Z 3    |
2020-04-27T22:38:55.8833284Z 4 LL |     println!("{");
2020-04-27T22:38:55.8833412Z 
2020-04-27T22:38:55.8833412Z 
2020-04-27T22:38:55.8833742Z -    |               -^ expected `'}'` in format string
2020-04-27T22:38:55.8834135Z +    |               -^ expected `}` in format string
2020-04-27T22:38:55.8834631Z 7    |               because of this opening brace
2020-04-27T22:38:55.8834800Z 8    |
2020-04-27T22:38:55.8834887Z 
2020-04-27T22:38:55.8835005Z 40    |
2020-04-27T22:38:55.8835005Z 40    |
2020-04-27T22:38:55.8835193Z 41    = note: argument name cannot be a single underscore
2020-04-27T22:38:55.8835372Z 42 
2020-04-27T22:38:55.8835801Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-27T22:38:55.8836115Z + error: invalid format string: expected `}` but string was terminated
2020-04-27T22:38:55.8836852Z 45    |
2020-04-27T22:38:55.8837205Z 46 LL |     let _ = format!("{");
2020-04-27T22:38:55.8837347Z 
2020-04-27T22:38:55.8837347Z 
2020-04-27T22:38:55.8837748Z -    |                      -^ expected `'}'` in format string
2020-04-27T22:38:55.8838228Z +    |                      -^ expected `}` in format string
2020-04-27T22:38:55.8838704Z 49    |                      because of this opening brace
2020-04-27T22:38:55.8838925Z 50    |
2020-04-27T22:38:55.8839023Z 
2020-04-27T22:38:55.8839136Z 68    |
2020-04-27T22:38:55.8839136Z 68    |
2020-04-27T22:38:55.8839382Z 69    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8839602Z 70 
2020-04-27T22:38:55.8840251Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-27T22:38:55.8840672Z -   --> $DIR/format-string-error.rs:21:35
2020-04-27T22:38:55.8841105Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8841680Z 73    |
2020-04-27T22:38:55.8841680Z 73    |
2020-04-27T22:38:55.8841864Z 74 LL |     let _ = format!("\n\n\n{\n\n\n");
2020-04-27T22:38:55.8842289Z -    |                            -      ^ expected `'}'` in format string
2020-04-27T22:38:55.8842736Z +    |                            -^ expected `}` in format string
2020-04-27T22:38:55.8843214Z 77    |                            because of this opening brace
2020-04-27T22:38:55.8843405Z 78    |
2020-04-27T22:38:55.8843510Z 
2020-04-27T22:38:55.8843510Z 
2020-04-27T22:38:55.8843707Z 79    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8843905Z 80 
2020-04-27T22:38:55.8844303Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-27T22:38:55.8844617Z + error: invalid format string: expected `}` but string was terminated
2020-04-27T22:38:55.8845219Z 83    |
2020-04-27T22:38:55.8845219Z 83    |
2020-04-27T22:38:55.8845349Z 84 LL |     {"###);
2020-04-27T22:38:55.8845454Z 
2020-04-27T22:38:55.8845763Z -    |     -^ expected `'}'` in format string
2020-04-27T22:38:55.8846139Z +    |     -^ expected `}` in format string
---
2020-04-27T22:38:55.8897206Z 
2020-04-27T22:38:55.8897329Z 164    |             |
2020-04-27T22:38:55.8897519Z 165    |             formatting specifier missing
2020-04-27T22:38:55.8897701Z 166 
2020-04-27T22:38:55.8898086Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-27T22:38:55.8898398Z + error: invalid format string: expected `}` but string was terminated
2020-04-27T22:38:55.8898823Z 168   --> $DIR/ifmt-bad-arg.rs:51:15
2020-04-27T22:38:55.8899127Z 170 LL |     format!("{");
2020-04-27T22:38:55.8899257Z 
2020-04-27T22:38:55.8899257Z 
2020-04-27T22:38:55.8899582Z -    |              -^ expected `'}'` in format string
2020-04-27T22:38:55.8899972Z +    |              -^ expected `}` in format string
2020-04-27T22:38:55.8900379Z 173    |              because of this opening brace
2020-04-27T22:38:55.8900552Z 174    |
2020-04-27T22:38:55.8900642Z 
2020-04-27T22:38:55.8900787Z 206 LL |         {foo}
2020-04-27T22:38:55.8900787Z 206 LL |         {foo}
2020-04-27T22:38:55.8900940Z 207    |         ^^^^^
2020-04-27T22:38:55.8901068Z 208 
2020-04-27T22:38:55.8901441Z - error: invalid format string: expected `'}'`, found `'t'`
2020-04-27T22:38:55.8901824Z -   --> $DIR/ifmt-bad-arg.rs:75:1
2020-04-27T22:38:55.8902226Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8902625Z +   --> $DIR/ifmt-bad-arg.rs:74:16
2020-04-27T22:38:55.8902792Z 211    |
2020-04-27T22:38:55.8903468Z 212 LL | ninth number: {
2020-04-27T22:38:55.8903887Z -    |               - because of this opening brace
2020-04-27T22:38:55.8904251Z - LL | tenth number: {}",
2020-04-27T22:38:55.8905091Z -    | ^ expected `}` in format string
2020-04-27T22:38:55.8905526Z +    |               -^ expected `}` in format string
2020-04-27T22:38:55.8905990Z +    |               because of this opening brace
2020-04-27T22:38:55.8906187Z 216    |
2020-04-27T22:38:55.8906187Z 216    |
2020-04-27T22:38:55.8906454Z 217    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8906787Z 
2020-04-27T22:38:55.8906877Z 
2020-04-27T22:38:55.8907084Z The actual stderr differed from the expected stderr.
2020-04-27T22:38:55.8907690Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/ifmt-bad-arg.stderr
2020-04-27T22:38:55.8907690Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/ifmt-bad-arg.stderr
2020-04-27T22:38:55.8908264Z To update references, rerun the tests and pass the `--bless` flag
2020-04-27T22:38:55.8908977Z To only update this specific test, also pass `--test-args if/ifmt-bad-arg.rs`
2020-04-27T22:38:55.8909368Z error: 1 errors occurred comparing output.
2020-04-27T22:38:55.8909601Z status: exit code: 1
2020-04-27T22:38:55.8909601Z status: exit code: 1
2020-04-27T22:38:55.8911408Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/ifmt-bad-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/auxiliary"
2020-04-27T22:38:55.8913052Z ------------------------------------------
2020-04-27T22:38:55.8913215Z 
2020-04-27T22:38:55.8913567Z ------------------------------------------
2020-04-27T22:38:55.8913751Z stderr:
---
2020-04-27T22:38:55.8925210Z 
2020-04-27T22:38:55.8925423Z error: invalid reference to positional argument 2 (there are 2 arguments)
2020-04-27T22:38:55.8925852Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:22:28
2020-04-27T22:38:55.8926121Z    |
2020-04-27T22:38:55.8926302Z LL |     format!("{} {value} {} {}", 1, value=2);
2020-04-27T22:38:55.8926735Z    |
2020-04-27T22:38:55.8927107Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8927256Z 
2020-04-27T22:38:55.8927256Z 
2020-04-27T22:38:55.8927466Z error: invalid reference to positional arguments 3, 4 and 5 (there are 3 arguments)
2020-04-27T22:38:55.8928109Z    |
2020-04-27T22:38:55.8928109Z    |
2020-04-27T22:38:55.8928307Z LL |     format!("{name} {value} {} {} {} {} {} {}", 0, name=1, value=2);
2020-04-27T22:38:55.8928625Z    |                                      ^^ ^^ ^^
2020-04-27T22:38:55.8929153Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8929302Z 
2020-04-27T22:38:55.8929469Z error: there is no argument named `foo`
2020-04-27T22:38:55.8929858Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:17
2020-04-27T22:38:55.8929858Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:17
2020-04-27T22:38:55.8930037Z    |
2020-04-27T22:38:55.8930203Z LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
2020-04-27T22:38:55.8930533Z 
2020-04-27T22:38:55.8930683Z error: there is no argument named `bar`
2020-04-27T22:38:55.8931091Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:26
2020-04-27T22:38:55.8931270Z    |
2020-04-27T22:38:55.8931270Z    |
2020-04-27T22:38:55.8931437Z LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
2020-04-27T22:38:55.8931784Z 
2020-04-27T22:38:55.8931932Z error: there is no argument named `foo`
2020-04-27T22:38:55.8932325Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:31:14
2020-04-27T22:38:55.8932518Z    |
2020-04-27T22:38:55.8932518Z    |
2020-04-27T22:38:55.8932745Z LL |     format!("{foo}");                //~ ERROR: no argument named `foo`
2020-04-27T22:38:55.8933250Z 
2020-04-27T22:38:55.8933416Z error: multiple unused formatting arguments
2020-04-27T22:38:55.8933839Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:32:17
2020-04-27T22:38:55.8934030Z    |
2020-04-27T22:38:55.8934030Z    |
2020-04-27T22:38:55.8934316Z LL |     format!("", 1, 2);               //~ ERROR: multiple unused formatting arguments
2020-04-27T22:38:55.8934796Z    |             --  ^  ^ argument never used
2020-04-27T22:38:55.8935202Z    |             |   argument never used
2020-04-27T22:38:55.8935541Z    |             multiple missing formatting specifiers
2020-04-27T22:38:55.8935692Z 
2020-04-27T22:38:55.8935838Z error: argument never used
---
2020-04-27T22:38:55.8939414Z 
2020-04-27T22:38:55.8939554Z error: named argument never used
2020-04-27T22:38:55.8939946Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:35:26
2020-04-27T22:38:55.8940125Z    |
2020-04-27T22:38:55.8940356Z LL |     format!("{}", 1, foo=2);         //~ ERROR: named argument never used
2020-04-27T22:38:55.8940829Z    |             ----         ^ named argument never used
2020-04-27T22:38:55.8941205Z    |             formatting specifier missing
2020-04-27T22:38:55.8941358Z 
2020-04-27T22:38:55.8941490Z error: argument never used
2020-04-27T22:38:55.8941954Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:36:22
2020-04-27T22:38:55.8941954Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:36:22
2020-04-27T22:38:55.8942134Z    |
2020-04-27T22:38:55.8942361Z LL |     format!("{foo}", 1, foo=2);      //~ ERROR: argument never used
2020-04-27T22:38:55.8942782Z    |             -------  ^ argument never used
2020-04-27T22:38:55.8943336Z    |             formatting specifier missing
2020-04-27T22:38:55.8943475Z 
2020-04-27T22:38:55.8943615Z error: named argument never used
2020-04-27T22:38:55.8944056Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:37:21
2020-04-27T22:38:55.8944056Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:37:21
2020-04-27T22:38:55.8944236Z    |
2020-04-27T22:38:55.8944555Z LL |     format!("", foo=2);              //~ ERROR: named argument never used
2020-04-27T22:38:55.8945048Z    |             --      ^ named argument never used
2020-04-27T22:38:55.8945416Z    |             formatting specifier missing
2020-04-27T22:38:55.8945554Z 
2020-04-27T22:38:55.8945724Z error: multiple unused formatting arguments
2020-04-27T22:38:55.8946125Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:38:32
2020-04-27T22:38:55.8946125Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:38:32
2020-04-27T22:38:55.8946304Z    |
2020-04-27T22:38:55.8946576Z LL |     format!("{} {}", 1, 2, foo=1, bar=2);  //~ ERROR: multiple unused formatting arguments
2020-04-27T22:38:55.8947090Z    |             -------            ^      ^ named argument never used
2020-04-27T22:38:55.8947571Z    |             |                  named argument never used
2020-04-27T22:38:55.8947822Z    |             multiple missing formatting specifiers
2020-04-27T22:38:55.8947978Z 
2020-04-27T22:38:55.8948125Z error: duplicate argument named `foo`
2020-04-27T22:38:55.8948125Z error: duplicate argument named `foo`
2020-04-27T22:38:55.8948528Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:40:33
2020-04-27T22:38:55.8948708Z    |
2020-04-27T22:38:55.8948920Z LL |     format!("{foo}", foo=1, foo=2);  //~ ERROR: duplicate argument
2020-04-27T22:38:55.8949377Z    |                          -      ^ duplicate argument
2020-04-27T22:38:55.8949794Z    |                          previously here
2020-04-27T22:38:55.8949948Z 
2020-04-27T22:38:55.8950121Z error: positional arguments cannot follow named arguments
2020-04-27T22:38:55.8950535Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:41:35
2020-04-27T22:38:55.8950535Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:41:35
2020-04-27T22:38:55.8950730Z    |
2020-04-27T22:38:55.8950978Z LL |     format!("{foo} {} {}", foo=1, 2);   //~ ERROR: positional arguments cannot follow
2020-04-27T22:38:55.8951529Z    |                                -  ^ positional arguments must be before named arguments
2020-04-27T22:38:55.8952031Z    |                                named argument
2020-04-27T22:38:55.8952176Z 
2020-04-27T22:38:55.8952329Z error: there is no argument named `valueb`
2020-04-27T22:38:55.8952739Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:23
2020-04-27T22:38:55.8952739Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:23
2020-04-27T22:38:55.8952918Z    |
2020-04-27T22:38:55.8953096Z LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
2020-04-27T22:38:55.8953461Z 
2020-04-27T22:38:55.8953601Z error: named argument never used
2020-04-27T22:38:55.8953984Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:51
2020-04-27T22:38:55.8954179Z    |
2020-04-27T22:38:55.8954179Z    |
2020-04-27T22:38:55.8954356Z LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
2020-04-27T22:38:55.8954816Z    |             -------------------                   ^ named argument never used
2020-04-27T22:38:55.8955240Z    |             formatting specifier missing
2020-04-27T22:38:55.8955383Z 
2020-04-27T22:38:55.8955597Z error: invalid format string: expected `}` but string was terminated
2020-04-27T22:38:55.8956033Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:51:15
2020-04-27T22:38:55.8956033Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:51:15
2020-04-27T22:38:55.8956213Z    |
2020-04-27T22:38:55.8956630Z LL |     format!("{"); //~ ERROR: expected `'}'` but string was terminated
2020-04-27T22:38:55.8957078Z    |              -^ expected `}` in format string
2020-04-27T22:38:55.8957545Z    |              because of this opening brace
2020-04-27T22:38:55.8957708Z    |
2020-04-27T22:38:55.8957708Z    |
2020-04-27T22:38:55.8957906Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8958272Z error: invalid format string: unmatched `}` found
2020-04-27T22:38:55.8958706Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:53:18
2020-04-27T22:38:55.8958885Z    |
2020-04-27T22:38:55.8958885Z    |
2020-04-27T22:38:55.8959099Z LL |     format!("foo } bar"); //~ ERROR: unmatched `}` found
2020-04-27T22:38:55.8959418Z    |                  ^ unmatched `}` in format string
2020-04-27T22:38:55.8959593Z    |
2020-04-27T22:38:55.8959804Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-27T22:38:55.8960151Z error: invalid format string: unmatched `}` found
2020-04-27T22:38:55.8960585Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:54:18
2020-04-27T22:38:55.8960780Z    |
2020-04-27T22:38:55.8960780Z    |
2020-04-27T22:38:55.8960980Z LL |     format!("foo }"); //~ ERROR: unmatched `}` found
2020-04-27T22:38:55.8961237Z    |                  ^ unmatched `}` in format string
2020-04-27T22:38:55.8961422Z    |
2020-04-27T22:38:55.8961618Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-27T22:38:55.8961924Z error: argument never used
2020-04-27T22:38:55.8962322Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:56:27
2020-04-27T22:38:55.8962503Z    |
2020-04-27T22:38:55.8962503Z    |
2020-04-27T22:38:55.8962712Z LL |     format!("foo %s baz", "bar"); //~ ERROR: argument never used
2020-04-27T22:38:55.8963376Z    |                  |
2020-04-27T22:38:55.8963376Z    |                  |
2020-04-27T22:38:55.8963621Z    |                  help: format specifiers use curly braces: `{}`
2020-04-27T22:38:55.8964092Z    = note: printf formatting not supported; see the documentation for `std::fmt`
2020-04-27T22:38:55.8964309Z 
2020-04-27T22:38:55.8964477Z error: there is no argument named `foo`
2020-04-27T22:38:55.8964866Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:60:9
2020-04-27T22:38:55.8964866Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:60:9
2020-04-27T22:38:55.8965045Z    |
2020-04-27T22:38:55.8965169Z LL |         {foo}
2020-04-27T22:38:55.8965326Z    |         ^^^^^
2020-04-27T22:38:55.8965429Z 
2020-04-27T22:38:55.8965775Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.8966389Z    |
2020-04-27T22:38:55.8966389Z    |
2020-04-27T22:38:55.8966523Z LL | ninth number: {
2020-04-27T22:38:55.8966896Z    |               -^ expected `}` in format string
2020-04-27T22:38:55.8967275Z    |               because of this opening brace
2020-04-27T22:38:55.8967437Z    |
2020-04-27T22:38:55.8967437Z    |
2020-04-27T22:38:55.8967649Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.8968018Z error: 4 positional arguments in format string, but there are 3 arguments
2020-04-27T22:38:55.8968514Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:78:15
2020-04-27T22:38:55.8968691Z    |
2020-04-27T22:38:55.8968691Z    |
2020-04-27T22:38:55.8968861Z LL |     println!("{} {:.*} {}", 1, 3.2, 4);
2020-04-27T22:38:55.8969244Z    |               ^^ ^^--^ ^^   -  ---  -
2020-04-27T22:38:55.8969704Z    |                    |           this parameter corresponds to the precision flag
2020-04-27T22:38:55.8969704Z    |                    |           this parameter corresponds to the precision flag
2020-04-27T22:38:55.8970107Z    |                    this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
2020-04-27T22:38:55.8970721Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8970721Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8971256Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-27T22:38:55.8971693Z error: 4 positional arguments in format string, but there are 3 arguments
2020-04-27T22:38:55.8972139Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:81:15
2020-04-27T22:38:55.8972399Z    |
2020-04-27T22:38:55.8972399Z    |
2020-04-27T22:38:55.8972573Z LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
2020-04-27T22:38:55.8972992Z    |               ^^ ^^^----^ ^^   -  ---  -
2020-04-27T22:38:55.8973468Z    |                     | |           this parameter corresponds to the precision flag
2020-04-27T22:38:55.8973468Z    |                     | |           this parameter corresponds to the precision flag
2020-04-27T22:38:55.8973878Z    |                     | this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
2020-04-27T22:38:55.8974364Z    |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
2020-04-27T22:38:55.8974970Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8974970Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8975490Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-27T22:38:55.8975926Z error: invalid reference to positional argument 7 (there are 3 arguments)
2020-04-27T22:38:55.8976381Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:84:18
2020-04-27T22:38:55.8976561Z    |
2020-04-27T22:38:55.8976561Z    |
2020-04-27T22:38:55.8976733Z LL |     println!("{} {:07$} {}", 1, 3.2, 4);
2020-04-27T22:38:55.8977090Z    |                  ^^^--^
2020-04-27T22:38:55.8977260Z    |                     |
2020-04-27T22:38:55.8977536Z    |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
2020-04-27T22:38:55.8978119Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8978119Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8978637Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-27T22:38:55.8979037Z error: unknown format trait `foo`
2020-04-27T22:38:55.8979414Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:86:17
2020-04-27T22:38:55.8979608Z    |
2020-04-27T22:38:55.8979608Z    |
2020-04-27T22:38:55.8979810Z LL |     println!("{:foo}", 1); //~ ERROR unknown format trait `foo`
2020-04-27T22:38:55.8980173Z    |
2020-04-27T22:38:55.8980383Z    = note: the only appropriate formatting traits are:
2020-04-27T22:38:55.8980791Z            - ``, which uses the `Display` trait
2020-04-27T22:38:55.8981161Z            - `?`, which uses the `Debug` trait
2020-04-27T22:38:55.8981161Z            - `?`, which uses the `Debug` trait
2020-04-27T22:38:55.8981553Z            - `e`, which uses the `LowerExp` trait
2020-04-27T22:38:55.8981930Z            - `E`, which uses the `UpperExp` trait
2020-04-27T22:38:55.8982303Z            - `o`, which uses the `Octal` trait
2020-04-27T22:38:55.8982698Z            - `p`, which uses the `Pointer` trait
2020-04-27T22:38:55.8983208Z            - `b`, which uses the `Binary` trait
2020-04-27T22:38:55.8983585Z            - `x`, which uses the `LowerHex` trait
2020-04-27T22:38:55.8983981Z            - `X`, which uses the `UpperHex` trait
2020-04-27T22:38:55.8984127Z 
2020-04-27T22:38:55.8984338Z error: invalid reference to positional arguments 4, 5, 6 and 7 (there is 1 argument)
2020-04-27T22:38:55.8984988Z    |
2020-04-27T22:38:55.8984988Z    |
2020-04-27T22:38:55.8985159Z LL |     println!("{5} {:4$} {6:7$}", 1);
2020-04-27T22:38:55.8985513Z    |               ^^^ ^^--^ ^^^--^
2020-04-27T22:38:55.8985718Z    |                     |      |
2020-04-27T22:38:55.8986013Z    |                     |      this width flag expects an `usize` argument at position 7, but there is 1 argument
2020-04-27T22:38:55.8986424Z    |                     this width flag expects an `usize` argument at position 4, but there is 1 argument
2020-04-27T22:38:55.8986989Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8986989Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8987519Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-27T22:38:55.8987960Z error: 2 positional arguments in format string, but no arguments were given
2020-04-27T22:38:55.8988393Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:92:15
2020-04-27T22:38:55.8988674Z    |
2020-04-27T22:38:55.8988674Z    |
2020-04-27T22:38:55.8988820Z LL |     println!("{:.*}");
2020-04-27T22:38:55.8989150Z    |               ^^--^
2020-04-27T22:38:55.8989328Z    |                 |
2020-04-27T22:38:55.8989629Z    |                 this precision flag adds an extra required argument at position 0, which is why there are 2 arguments expected
2020-04-27T22:38:55.8990236Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8990236Z    = note: positional arguments are zero-based
2020-04-27T22:38:55.8990818Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-27T22:38:55.8991207Z error[E0308]: mismatched types
2020-04-27T22:38:55.8991625Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:78:32
2020-04-27T22:38:55.8991804Z    |
2020-04-27T22:38:55.8991804Z    |
2020-04-27T22:38:55.8991974Z LL |     println!("{} {:.*} {}", 1, 3.2, 4);
2020-04-27T22:38:55.8992449Z    |                                ^^^ expected `usize`, found floating-point number
2020-04-27T22:38:55.8993082Z    = note: expected reference `&usize`
2020-04-27T22:38:55.8993082Z    = note: expected reference `&usize`
2020-04-27T22:38:55.8993441Z               found reference `&{float}`
2020-04-27T22:38:55.8994273Z 
2020-04-27T22:38:55.8994450Z error[E0308]: mismatched types
2020-04-27T22:38:55.8994884Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:81:35
2020-04-27T22:38:55.8995089Z    |
2020-04-27T22:38:55.8995089Z    |
2020-04-27T22:38:55.8995295Z LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
2020-04-27T22:38:55.8995849Z    |                                   ^^^ expected `usize`, found floating-point number
2020-04-27T22:38:55.8996279Z    = note: expected reference `&usize`
2020-04-27T22:38:55.8996279Z    = note: expected reference `&usize`
2020-04-27T22:38:55.8996528Z               found reference `&{float}`
2020-04-27T22:38:55.8997356Z 
2020-04-27T22:38:55.8997547Z error: aborting due to 36 previous errors
2020-04-27T22:38:55.8997704Z 
2020-04-27T22:38:55.8998351Z For more information about this error, try `rustc --explain E0308`.
2020-04-27T22:38:55.8998351Z For more information about this error, try `rustc --explain E0308`.
2020-04-27T22:38:55.8998526Z 
2020-04-27T22:38:55.8998857Z ------------------------------------------
2020-04-27T22:38:55.8999003Z 
2020-04-27T22:38:55.8999087Z 
2020-04-27T22:38:55.8999414Z ---- [ui] ui/issues/issue-51848.rs stdout ----
2020-04-27T22:38:55.8999627Z diff of stderr:
2020-04-27T22:38:55.8999742Z 
2020-04-27T22:38:55.9000148Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-27T22:38:55.9000640Z + error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.9001235Z 3    |
2020-04-27T22:38:55.9001388Z 4 LL |         println!("{");
2020-04-27T22:38:55.9001530Z 
2020-04-27T22:38:55.9001530Z 
2020-04-27T22:38:55.9001887Z -    |                   -^ expected `'}'` in format string
2020-04-27T22:38:55.9002331Z +    |                   -^ expected `}` in format string
2020-04-27T22:38:55.9002785Z 7    |                   because of this opening brace
2020-04-27T22:38:55.9002970Z 8 ...
2020-04-27T22:38:55.9003062Z 
2020-04-27T22:38:55.9003161Z 
2020-04-27T22:38:55.9003161Z 
2020-04-27T22:38:55.9003337Z The actual stderr differed from the expected stderr.
2020-04-27T22:38:55.9003897Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848/issue-51848.stderr
2020-04-27T22:38:55.9004451Z To update references, rerun the tests and pass the `--bless` flag
2020-04-27T22:38:55.9004956Z To only update this specific test, also pass `--test-args issues/issue-51848.rs`
2020-04-27T22:38:55.9005322Z error: 1 errors occurred comparing output.
2020-04-27T22:38:55.9005539Z status: exit code: 1
2020-04-27T22:38:55.9005539Z status: exit code: 1
2020-04-27T22:38:55.9007175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848/auxiliary"
2020-04-27T22:38:55.9008652Z ------------------------------------------
2020-04-27T22:38:55.9008808Z 
2020-04-27T22:38:55.9009168Z ------------------------------------------
2020-04-27T22:38:55.9009345Z stderr:
2020-04-27T22:38:55.9009345Z stderr:
2020-04-27T22:38:55.9009674Z ------------------------------------------
2020-04-27T22:38:55.9010119Z error: invalid format string: expected `'}'`, found `'\n'`
2020-04-27T22:38:55.9010782Z    |
2020-04-27T22:38:55.9010972Z LL |         println!("{"); //~ ERROR invalid
2020-04-27T22:38:55.9010972Z LL |         println!("{"); //~ ERROR invalid
2020-04-27T22:38:55.9011388Z    |                   -^ expected `}` in format string
2020-04-27T22:38:55.9011835Z    |                   because of this opening brace
2020-04-27T22:38:55.9012013Z ...
2020-04-27T22:38:55.9012160Z LL |     macro_with_error!();
2020-04-27T22:38:55.9012563Z    |     -------------------- in this macro invocation
2020-04-27T22:38:55.9012563Z    |     -------------------- in this macro invocation
2020-04-27T22:38:55.9012748Z    |
2020-04-27T22:38:55.9012964Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-27T22:38:55.9013801Z 
2020-04-27T22:38:55.9013988Z error: invalid format string: unmatched `}` found
2020-04-27T22:38:55.9014454Z   --> /checkout/src/test/ui/issues/issue-51848.rs:18:15
2020-04-27T22:38:55.9014649Z    |
2020-04-27T22:38:55.9014649Z    |
2020-04-27T22:38:55.9014816Z LL |     println!("}"); //~ ERROR invalid
2020-04-27T22:38:55.9015054Z    |               ^ unmatched `}` in format string
2020-04-27T22:38:55.9015248Z    |
2020-04-27T22:38:55.9015459Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-27T22:38:55.9015822Z error: aborting due to 2 previous errors
2020-04-27T22:38:55.9015967Z 
2020-04-27T22:38:55.9016051Z 
2020-04-27T22:38:55.9016373Z ------------------------------------------
---
2020-04-27T22:38:55.9018835Z test result: FAILED. 9862 passed; 4 failed; 61 ignored; 0 measured; 0 filtered out
2020-04-27T22:38:55.9019064Z 
2020-04-27T22:38:55.9019148Z 
2020-04-27T22:38:55.9019233Z 
2020-04-27T22:38:55.9022462Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-27T22:38:55.9025285Z 
2020-04-27T22:38:55.9025379Z 
2020-04-27T22:38:55.9026008Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-27T22:38:55.9026362Z Build completed unsuccessfully in 0:58:56
2020-04-27T22:38:55.9026362Z Build completed unsuccessfully in 0:58:56
2020-04-27T22:38:55.9030434Z == clock drift check ==
2020-04-27T22:38:55.9030875Z   local time: Mon Apr 27 22:38:55 UTC 2020
2020-04-27T22:38:55.9525371Z   network time: Mon, 27 Apr 2020 22:38:55 GMT
2020-04-27T22:38:56.4850974Z 
2020-04-27T22:38:56.4850974Z 
2020-04-27T22:38:56.4934097Z ##[error]Bash exited with code '1'.
2020-04-27T22:38:56.4947415Z ##[section]Finishing: Run build
2020-04-27T22:38:56.4988006Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T22:38:56.4992742Z Task         : Get sources
2020-04-27T22:38:56.4993043Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T22:38:56.4993337Z Version      : 1.0.0
2020-04-27T22:38:56.4993532Z Author       : Microsoft
2020-04-27T22:38:56.4993532Z Author       : Microsoft
2020-04-27T22:38:56.4993976Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T22:38:56.4994360Z ==============================================================================
2020-04-27T22:38:56.8213356Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T22:38:56.8256122Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T22:38:56.8339910Z Cleaning up task key
2020-04-27T22:38:56.8341016Z Start cleaning up orphan processes.
2020-04-27T22:38:56.8522922Z Terminate orphan process: pid (4375) (python)
2020-04-27T22:38:56.8671263Z ##[section]Finishing: Finalize Job
