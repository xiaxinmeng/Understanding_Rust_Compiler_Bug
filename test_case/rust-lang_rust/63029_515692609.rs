plain
2019-07-27T13:11:06.2310534Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T13:11:06.2310611Z 
2019-07-27T13:11:06.2310836Z   git checkout -b <new-branch-name>
2019-07-27T13:11:06.2310877Z 
2019-07-27T13:11:06.2311113Z HEAD is now at 5f61b16d6 Auto merge of #63029 - petrochenkov:rpass, r=Centril
2019-07-27T13:11:06.2471358Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T13:11:06.2474117Z ==============================================================================
2019-07-27T13:11:06.2474199Z Task         : Bash
2019-07-27T13:11:06.2474280Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T13:15:22.9950400Z  ---> 8858e4cda6d1
2019-07-27T13:15:22.9950680Z Step 4/18 : RUN bash /scripts/emscripten.sh
2019-07-27T13:15:23.1191564Z  ---> Running in 93db3dc5b620
2019-07-27T13:15:23.6088228Z + cd /
2019-07-27T13:15:23.6089000Z + curl -fL https://mozilla-games.s3.amazonaws.com/emscripten/releases/emsdk-portable.tar.gz
2019-07-27T13:15:23.6089238Z + tar -xz
2019-07-27T13:15:23.6112828Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-27T13:15:23.6112892Z 
2019-07-27T13:15:23.8724527Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-07-27T13:15:23.9634575Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-07-27T13:15:23.9634575Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-07-27T13:15:23.9634907Z 100 38732  100 38732    0     0   107k      0 --:--:-- --:--:-- --:--:--  107k
2019-07-27T13:15:23.9711794Z + cd /emsdk-portable
2019-07-27T13:15:23.9712899Z + ./emsdk update
2019-07-27T13:15:25.5861174Z Downloading: /emsdk-portable/zips/emsdk_unix_update.tar.gz from https://s3.amazonaws.com/mozilla-games/emscripten/packages/emsdk_unix_update.tar.gz, 38729 Bytes
2019-07-27T13:15:25.5861519Z Unpacking '/emsdk-portable/zips/emsdk_unix_update.tar.gz' to '/emsdk-portable'
2019-07-27T13:15:25.5861666Z Fetching all tags from Emscripten Github repository...
2019-07-27T13:15:25.5861749Z Done. 168 tagged releases available, latest is 1.38.40.
2019-07-27T13:15:25.5861812Z Fetching all tags from Binaryen Github repository...
2019-07-27T13:15:25.5861892Z Done. 87 tagged Binaryen releases available, latest is 1.38.32.
2019-07-27T13:15:25.5862126Z Fetching all precompiled tagged releases..
2019-07-27T13:15:25.5862452Z Downloading: /emsdk-portable/llvm-tags-32bit.txt from https://s3.amazonaws.com/mozilla-games/emscripten/packages/llvm/tag/linux_32bit/index.txt
2019-07-27T13:15:25.5862777Z Downloading: /emsdk-portable/llvm-tags-64bit.txt from https://s3.amazonaws.com/mozilla-games/emscripten/packages/llvm/tag/linux_64bit/index.txt, 2379 Bytes
2019-07-27T13:15:25.5972074Z + hide_output ./emsdk install sdk-1.38.15-64bit
2019-07-27T13:15:51.2971736Z /scripts/emscripten.sh: line 3:    25 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-07-27T13:15:51.2971736Z /scripts/emscripten.sh: line 3:    25 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-07-27T13:15:51.2974101Z + ./emsdk activate sdk-1.38.15-64bit
2019-07-27T13:15:51.4636183Z Writing .emscripten configuration file to user home directory /root/
2019-07-27T13:15:51.4636511Z The Emscripten configuration file /root/.emscripten has been rewritten with the following contents:
2019-07-27T13:15:51.4636673Z import os
2019-07-27T13:15:51.4636969Z LLVM_ROOT='/emsdk-portable/clang/e1.38.15_64bit'
2019-07-27T13:15:51.4636969Z LLVM_ROOT='/emsdk-portable/clang/e1.38.15_64bit'
2019-07-27T13:15:51.4637252Z EMSCRIPTEN_NATIVE_OPTIMIZER='/emsdk-portable/clang/e1.38.15_64bit/optimizer'
2019-07-27T13:15:51.4637538Z BINARYEN_ROOT='/emsdk-portable/clang/e1.38.15_64bit/binaryen'
2019-07-27T13:15:51.4637793Z NODE_JS='/emsdk-portable/node/8.9.1_64bit/bin/node'
2019-07-27T13:15:51.4638091Z EMSCRIPTEN_ROOT='/emsdk-portable/emscripten/1.38.15'
2019-07-27T13:15:51.4638315Z SPIDERMONKEY_ENGINE = ''
2019-07-27T13:15:51.4638541Z V8_ENGINE = ''
2019-07-27T13:15:51.4638741Z TEMP_DIR = '/tmp'
2019-07-27T13:15:51.4638837Z COMPILER_ENGINE = NODE_JS
2019-07-27T13:15:51.4638901Z JS_ENGINES = [NODE_JS]
2019-07-27T13:15:51.4638960Z 
2019-07-27T13:15:51.4639340Z To conveniently access the selected set of tools from the command line, consider adding the following directories to PATH, or call 'source ./emsdk_env.sh' to do this for you.
2019-07-27T13:15:51.4639591Z 
2019-07-27T13:15:51.4640120Z    /emsdk-portable:/emsdk-portable/clang/e1.38.15_64bit:/emsdk-portable/node/8.9.1_64bit/bin:/emsdk-portable/emscripten/1.38.15
2019-07-27T13:15:51.4640234Z Set the following tools as active:
2019-07-27T13:15:51.4640450Z    clang-e1.38.15-64bit
2019-07-27T13:15:51.4640854Z    node-8.9.1-64bit
2019-07-27T13:15:51.4641074Z    emscripten-1.38.15
2019-07-27T13:15:51.4711094Z + source ./emsdk_env.sh
2019-07-27T13:15:51.4711772Z ++ SRC=./emsdk_env.sh
2019-07-27T13:15:51.4711772Z ++ SRC=./emsdk_env.sh
2019-07-27T13:15:51.4712144Z ++ '[' ./emsdk_env.sh = '' ']'
2019-07-27T13:15:51.4717238Z +++ dirname ./emsdk_env.sh
2019-07-27T13:15:51.4748950Z ++ pushd .
2019-07-27T13:15:52.2982065Z ++ unset SRC
2019-07-27T13:15:52.2982183Z ++ ./emsdk construct_env
2019-07-27T13:15:52.2982658Z PATH += /emsdk-portable
2019-07-27T13:15:52.2983446Z PATH += /emsdk-portable/clang/e1.38.15_64bit
2019-07-27T13:15:52.2983946Z PATH += /emsdk-portable/node/8.9.1_64bit/bin
2019-07-27T13:15:52.2984230Z PATH += /emsdk-portable/emscripten/1.38.15
2019-07-27T13:15:52.2984230Z PATH += /emsdk-portable/emscripten/1.38.15
2019-07-27T13:15:52.2984303Z 
2019-07-27T13:15:52.2984367Z Setting environment variables:
2019-07-27T13:15:52.2985041Z EMSDK = /emsdk-portable
2019-07-27T13:15:52.2985116Z EM_CONFIG = /root/.emscripten
2019-07-27T13:15:52.2985407Z BINARYEN_ROOT = /emsdk-portable/clang/e1.38.15_64bit/binaryen
2019-07-27T13:15:52.2985695Z EMSCRIPTEN = /emsdk-portable/emscripten/1.38.15
2019-07-27T13:15:52.2985773Z 
2019-07-27T13:15:52.2986000Z ++ . ./emsdk_set_env.sh
2019-07-27T13:15:52.2986463Z +++ export PATH=/emsdk-portable:/emsdk-portable/clang/e1.38.15_64bit:/emsdk-portable/node/8.9.1_64bit/bin:/emsdk-portable/emscripten/1.38.15:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-07-27T13:15:52.2987092Z +++ PATH=/emsdk-portable:/emsdk-portable/clang/e1.38.15_64bit:/emsdk-portable/node/8.9.1_64bit/bin:/emsdk-portable/emscripten/1.38.15:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-07-27T13:15:52.2987390Z +++ export EMSDK=/emsdk-portable
2019-07-27T13:15:52.2987729Z +++ EMSDK=/emsdk-portable
2019-07-27T13:15:52.2987964Z +++ export EM_CONFIG=/root/.emscripten
2019-07-27T13:15:52.2988208Z +++ EM_CONFIG=/root/.emscripten
2019-07-27T13:15:52.2988878Z +++ export BINARYEN_ROOT=/emsdk-portable/clang/e1.38.15_64bit/binaryen
2019-07-27T13:15:52.2989201Z +++ BINARYEN_ROOT=/emsdk-portable/clang/e1.38.15_64bit/binaryen
2019-07-27T13:15:52.2989522Z +++ export EMSCRIPTEN=/emsdk-portable/emscripten/1.38.15
2019-07-27T13:15:52.2989902Z +++ EMSCRIPTEN=/emsdk-portable/emscripten/1.38.15
2019-07-27T13:15:52.2990409Z ++ popd
2019-07-27T13:15:52.2990664Z + echo 'main(){}'
2019-07-27T13:15:52.2990911Z + HOME=/emsdk-portable/
2019-07-27T13:15:52.2991036Z + emcc a.c
2019-07-27T13:15:52.2991719Z INFO:root: - ok
2019-07-27T13:15:52.2991990Z INFO:root:(Emscripten: Running sanity checks)
2019-07-27T13:15:52.2992555Z WARNING:root:java does not seem to exist, required for closure compiler, which is optional (define JAVA in /root/.emscripten if you want it)
2019-07-27T13:15:52.2992663Z WARNING:root:closure compiler will not be available
2019-07-27T13:15:52.2992663Z WARNING:root:closure compiler will not be available
2019-07-27T13:15:52.2993574Z a.c:1:1: warning: type specifier missing, defaults to 'int' [-Wimplicit-int]
2019-07-27T13:15:52.2993668Z main(){}
2019-07-27T13:15:52.2993998Z 1 warning generated.
2019-07-27T13:15:52.2993998Z 1 warning generated.
2019-07-27T13:15:52.2994397Z INFO:root:generating system library: dlmalloc.bc... (this will be cached in "/emsdk-portable/.emscripten_cache/asmjs/dlmalloc.bc" for subsequent builds)
2019-07-27T13:15:52.4291080Z INFO:root: - ok
2019-07-27T13:15:52.4295547Z INFO:root:generating system library: libc.bc... (this will be cached in "/emsdk-portable/.emscripten_cache/asmjs/libc.bc" for subsequent builds)
2019-07-27T13:16:43.5687027Z INFO:root: - ok
2019-07-27T13:16:43.5691205Z INFO:root:generating system library: compiler-rt.a... (this will be cached in "/emsdk-portable/.emscripten_cache/asmjs/compiler-rt.a" for subsequent builds)
2019-07-27T13:16:43.8733074Z INFO:root: - ok
2019-07-27T13:16:43.8733779Z INFO:root:generating system library: wasm-libc.bc... (this will be cached in "/emsdk-portable/.emscripten_cache/asmjs/wasm-libc.bc" for subsequent builds)
2019-07-27T13:16:45.6154030Z INFO:root: - ok
2019-07-27T13:16:45.8104018Z INFO:root:generating system asset: generated_struct_info.json... (this will be cached in "/emsdk-portable/.emscripten_cache/asmjs/generated_struct_info.json" for subsequent builds)
2019-07-27T13:16:46.8586676Z + HOME=/emsdk-portable/
2019-07-27T13:16:46.8586676Z + HOME=/emsdk-portable/
2019-07-27T13:16:46.8587150Z + emcc -s BINARYEN=1 a.c
2019-07-27T13:16:46.9509328Z a.c:1:1: warning: type specifier missing, defaults to 'int' [-Wimplicit-int]
2019-07-27T13:16:46.9509694Z main(){}
2019-07-27T13:16:46.9518861Z 1 warning generated.
2019-07-27T13:16:46.9518861Z 1 warning generated.
2019-07-27T13:16:47.4853503Z + rm -f a.c a.out.js a.out.wasm
2019-07-27T13:16:47.4863895Z + cp /root/.emscripten /emsdk-portable
2019-07-27T13:16:47.4877625Z + chmod a+rxw -R /emsdk-portable
2019-07-27T13:17:25.2486035Z Removing intermediate container 93db3dc5b620
2019-07-27T13:17:25.2486585Z Step 5/18 : COPY scripts/sccache.sh /scripts/
2019-07-27T13:17:27.5582116Z  ---> 91e6fd6847e9
2019-07-27T13:17:27.5582977Z Step 6/18 : RUN sh /scripts/sccache.sh
2019-07-27T13:17:27.7055282Z  ---> Running in 49cc278405e9
---
2019-07-27T14:52:56.2765096Z test [ui] ui/imports/import-from-missing.rs ... ok
2019-07-27T14:52:56.8590718Z test [ui] ui/imports/import-crate-with-invalid-spans/main.rs ... ok
2019-07-27T14:52:57.9022193Z test [ui] ui/imports/import-from.rs ... ok
2019-07-27T14:52:57.9557626Z test [ui] ui/imports/import-glob-0.rs ... ok
2019-07-27T14:52:58.6313316Z test [ui] ui/imports/import-glob-0-rpass.rs ... ok
2019-07-27T14:52:59.5757690Z test [ui] ui/imports/import-glob-1.rs ... ok
2019-07-27T14:53:00.4963567Z test [ui] ui/imports/import-glob-crate.rs ... ok
2019-07-27T14:53:00.5323688Z test [ui] ui/imports/import-loop-2.rs ... ok
2019-07-27T14:53:00.5656355Z test [ui] ui/imports/import-loop.rs ... ok
---
2019-07-27T15:28:38.2003608Z test [ui] ui/unsized-locals/reference-unsized-locals.rs ... ok
2019-07-27T15:28:38.2455175Z test [ui] ui/unsized-locals/unsized-exprs.rs ... ok
2019-07-27T15:28:38.3029433Z test [ui] ui/unsized-locals/unsized-exprs2.rs ... ok
2019-07-27T15:28:38.3853615Z test [ui] ui/unsized-locals/unsized-exprs3.rs ... ok
2019-07-27T15:28:39.8587755Z test [ui] ui/unsized-locals/unsized-exprs-rpass.rs ... ok
2019-07-27T15:28:41.6679059Z test [ui] ui/unsized-locals/unsized-parameters.rs ... ok
2019-07-27T15:28:42.1397069Z test [ui] ui/unsized-tuple-impls.rs ... ok
2019-07-27T15:28:42.1822470Z test [ui] ui/unsized/unsized-bare-typaram.rs ... ok
2019-07-27T15:28:42.2244479Z test [ui] ui/unsized/unsized-enum.rs ... ok
---
2019-07-27T15:29:31.1527618Z 
2019-07-27T15:29:31.1528158Z ---- [ui] ui/error-codes/E0661.rs stdout ----
2019-07-27T15:29:31.1528240Z diff of stderr:
2019-07-27T15:29:31.1528302Z 
2019-07-27T15:29:31.1528636Z 4 LL |     asm!("nop" : "r"(a));
2019-07-27T15:29:31.1528912Z 6 
2019-07-27T15:29:31.1529720Z - error: aborting due to previous error
2019-07-27T15:29:31.1529720Z - error: aborting due to previous error
2019-07-27T15:29:31.1529844Z + error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1530096Z +   --> $DIR/E0661.rs:5:5
2019-07-27T15:29:31.1530157Z +    |
2019-07-27T15:29:31.1530231Z + LL |     asm!("nop" : "r"(a));
2019-07-27T15:29:31.1530965Z + 
2019-07-27T15:29:31.1531042Z + error: aborting due to 2 previous errors
2019-07-27T15:29:31.1531216Z 8 
2019-07-27T15:29:31.1531348Z 9 
2019-07-27T15:29:31.1531348Z 9 
2019-07-27T15:29:31.1531383Z 
2019-07-27T15:29:31.1531519Z 
2019-07-27T15:29:31.1531740Z The actual stderr differed from the expected stderr.
2019-07-27T15:29:31.1532116Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0661/E0661.stderr
2019-07-27T15:29:31.1532415Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T15:29:31.1532705Z To only update this specific test, also pass `--test-args error-codes/E0661.rs`
2019-07-27T15:29:31.1532815Z error: 1 errors occurred comparing output.
2019-07-27T15:29:31.1532894Z status: exit code: 1
2019-07-27T15:29:31.1532894Z status: exit code: 1
2019-07-27T15:29:31.1533845Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0661.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0661" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0661/auxiliary" "-A" "unused"
2019-07-27T15:29:31.1534349Z ------------------------------------------
2019-07-27T15:29:31.1534393Z 
2019-07-27T15:29:31.1534984Z ------------------------------------------
2019-07-27T15:29:31.1536015Z stderr:
2019-07-27T15:29:31.1536015Z stderr:
2019-07-27T15:29:31.1536637Z ------------------------------------------
2019-07-27T15:29:31.1536913Z error[E0661]: output operand constraint lacks '=' or '+'
2019-07-27T15:29:31.1537283Z    |
2019-07-27T15:29:31.1537283Z    |
2019-07-27T15:29:31.1537363Z LL |     asm!("nop" : "r"(a));
2019-07-27T15:29:31.1537491Z 
2019-07-27T15:29:31.1537491Z 
2019-07-27T15:29:31.1537741Z error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1538194Z    |
2019-07-27T15:29:31.1538194Z    |
2019-07-27T15:29:31.1538277Z LL |     asm!("nop" : "r"(a));
2019-07-27T15:29:31.1538617Z 
2019-07-27T15:29:31.1538684Z error: aborting due to 2 previous errors
2019-07-27T15:29:31.1538748Z 
2019-07-27T15:29:31.1538783Z 
2019-07-27T15:29:31.1538783Z 
2019-07-27T15:29:31.1539341Z ------------------------------------------
2019-07-27T15:29:31.1539524Z 
2019-07-27T15:29:31.1539614Z 
2019-07-27T15:29:31.1539902Z ---- [ui] ui/error-codes/E0662.rs stdout ----
2019-07-27T15:29:31.1540096Z diff of stderr:
2019-07-27T15:29:31.1540222Z 
2019-07-27T15:29:31.1540323Z 4 LL |          : "=test"("a")
2019-07-27T15:29:31.1540501Z 6 
2019-07-27T15:29:31.1540789Z - error: aborting due to previous error
2019-07-27T15:29:31.1540789Z - error: aborting due to previous error
2019-07-27T15:29:31.1540998Z + error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1541335Z +   --> $DIR/E0662.rs:4:5
2019-07-27T15:29:31.1541400Z +    |
2019-07-27T15:29:31.1541626Z + LL | /     asm!("xor %eax, %eax"
2019-07-27T15:29:31.1541719Z + LL | |          :
2019-07-27T15:29:31.1541817Z + LL | |          : "=test"("a")
2019-07-27T15:29:31.1541878Z + LL | |         );
2019-07-27T15:29:31.1542328Z + 
2019-07-27T15:29:31.1542404Z + error: aborting due to 2 previous errors
2019-07-27T15:29:31.1542588Z 8 
2019-07-27T15:29:31.1542705Z 9 
2019-07-27T15:29:31.1542705Z 9 
2019-07-27T15:29:31.1542739Z 
2019-07-27T15:29:31.1542789Z 
2019-07-27T15:29:31.1542953Z The actual stderr differed from the expected stderr.
2019-07-27T15:29:31.1543397Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0662/E0662.stderr
2019-07-27T15:29:31.1543675Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T15:29:31.1543968Z To only update this specific test, also pass `--test-args error-codes/E0662.rs`
2019-07-27T15:29:31.1544237Z error: 1 errors occurred comparing output.
2019-07-27T15:29:31.1544523Z status: exit code: 1
2019-07-27T15:29:31.1544523Z status: exit code: 1
2019-07-27T15:29:31.1545644Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0662.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0662" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0662/auxiliary" "-A" "unused"
2019-07-27T15:29:31.1546954Z ------------------------------------------
2019-07-27T15:29:31.1547165Z 
2019-07-27T15:29:31.1547530Z ------------------------------------------
2019-07-27T15:29:31.1547760Z stderr:
2019-07-27T15:29:31.1547760Z stderr:
2019-07-27T15:29:31.1548072Z ------------------------------------------
2019-07-27T15:29:31.1548525Z error[E0662]: input operand constraint contains '='
2019-07-27T15:29:31.1549362Z    |
2019-07-27T15:29:31.1549362Z    |
2019-07-27T15:29:31.1549535Z LL |          : "=test"("a") //~ ERROR E0662
2019-07-27T15:29:31.1549681Z 
2019-07-27T15:29:31.1549681Z 
2019-07-27T15:29:31.1549747Z error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1550335Z    |
2019-07-27T15:29:31.1550335Z    |
2019-07-27T15:29:31.1550407Z LL | /     asm!("xor %eax, %eax"
2019-07-27T15:29:31.1550467Z LL | |          :
2019-07-27T15:29:31.1550546Z LL | |          : "=test"("a") //~ ERROR E0662
2019-07-27T15:29:31.1550609Z LL | |         );
2019-07-27T15:29:31.1550719Z 
2019-07-27T15:29:31.1550773Z error: aborting due to 2 previous errors
2019-07-27T15:29:31.1550834Z 
2019-07-27T15:29:31.1550866Z 
2019-07-27T15:29:31.1550866Z 
2019-07-27T15:29:31.1551085Z ------------------------------------------
2019-07-27T15:29:31.1551147Z 
2019-07-27T15:29:31.1551179Z 
2019-07-27T15:29:31.1551424Z ---- [ui] ui/error-codes/E0663.rs stdout ----
2019-07-27T15:29:31.1551708Z diff of stderr:
2019-07-27T15:29:31.1551747Z 
2019-07-27T15:29:31.1551847Z 4 LL |          : "+test"("a")
2019-07-27T15:29:31.1552018Z 6 
2019-07-27T15:29:31.1552276Z - error: aborting due to previous error
2019-07-27T15:29:31.1552276Z - error: aborting due to previous error
2019-07-27T15:29:31.1552494Z + error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1552966Z +   --> $DIR/E0663.rs:4:5
2019-07-27T15:29:31.1553030Z +    |
2019-07-27T15:29:31.1553233Z + LL | /     asm!("xor %eax, %eax"
2019-07-27T15:29:31.1553322Z + LL | |          :
2019-07-27T15:29:31.1553400Z + LL | |          : "+test"("a")
2019-07-27T15:29:31.1553557Z + LL | |         );
2019-07-27T15:29:31.1553736Z + 
2019-07-27T15:29:31.1553812Z + error: aborting due to 2 previous errors
2019-07-27T15:29:31.1554041Z 8 
2019-07-27T15:29:31.1554203Z 9 
2019-07-27T15:29:31.1554203Z 9 
2019-07-27T15:29:31.1554258Z 
2019-07-27T15:29:31.1554329Z 
2019-07-27T15:29:31.1554402Z The actual stderr differed from the expected stderr.
2019-07-27T15:29:31.1555060Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0663/E0663.stderr
2019-07-27T15:29:31.1556120Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T15:29:31.1556638Z To only update this specific test, also pass `--test-args error-codes/E0663.rs`
2019-07-27T15:29:31.1556996Z error: 1 errors occurred comparing output.
2019-07-27T15:29:31.1557068Z status: exit code: 1
2019-07-27T15:29:31.1557068Z status: exit code: 1
2019-07-27T15:29:31.1558122Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0663.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0663" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0663/auxiliary" "-A" "unused"
2019-07-27T15:29:31.1559072Z ------------------------------------------
2019-07-27T15:29:31.1559123Z 
2019-07-27T15:29:31.1559518Z ------------------------------------------
2019-07-27T15:29:31.1559605Z stderr:
2019-07-27T15:29:31.1559605Z stderr:
2019-07-27T15:29:31.1559996Z ------------------------------------------
2019-07-27T15:29:31.1560394Z error[E0663]: input operand constraint contains '+'
2019-07-27T15:29:31.1561004Z    |
2019-07-27T15:29:31.1561004Z    |
2019-07-27T15:29:31.1561151Z LL |          : "+test"("a") //~ ERROR E0663
2019-07-27T15:29:31.1561273Z 
2019-07-27T15:29:31.1561273Z 
2019-07-27T15:29:31.1561332Z error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1561705Z    |
2019-07-27T15:29:31.1561705Z    |
2019-07-27T15:29:31.1561891Z LL | /     asm!("xor %eax, %eax"
2019-07-27T15:29:31.1562130Z LL | |          :
2019-07-27T15:29:31.1562278Z LL | |          : "+test"("a") //~ ERROR E0663
2019-07-27T15:29:31.1562353Z LL | |         );
2019-07-27T15:29:31.1562464Z 
2019-07-27T15:29:31.1562683Z error: aborting due to 2 previous errors
2019-07-27T15:29:31.1562727Z 
2019-07-27T15:29:31.1562758Z 
2019-07-27T15:29:31.1562758Z 
2019-07-27T15:29:31.1563242Z ------------------------------------------
2019-07-27T15:29:31.1563287Z 
2019-07-27T15:29:31.1563320Z 
2019-07-27T15:29:31.1563567Z ---- [ui] ui/error-codes/E0664.rs stdout ----
2019-07-27T15:29:31.1563634Z diff of stderr:
2019-07-27T15:29:31.1563690Z 
2019-07-27T15:29:31.1563879Z 4 LL |          : "{eax}"
2019-07-27T15:29:31.1564077Z 6 
2019-07-27T15:29:31.1564357Z - error: aborting due to previous error
2019-07-27T15:29:31.1564357Z - error: aborting due to previous error
2019-07-27T15:29:31.1564567Z + error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1564891Z +   --> $DIR/E0664.rs:4:5
2019-07-27T15:29:31.1564957Z +    |
2019-07-27T15:29:31.1565032Z + LL | /     asm!("mov $$0x200, %eax"
2019-07-27T15:29:31.1565225Z + LL | |          :
2019-07-27T15:29:31.1565355Z + LL | |          :
2019-07-27T15:29:31.1565416Z + LL | |          : "{eax}"
2019-07-27T15:29:31.1565496Z + LL | |         );
2019-07-27T15:29:31.1565629Z + 
2019-07-27T15:29:31.1565687Z + error: aborting due to 2 previous errors
2019-07-27T15:29:31.1565766Z 8 
2019-07-27T15:29:31.1566428Z 9 
2019-07-27T15:29:31.1566428Z 9 
2019-07-27T15:29:31.1566518Z 
2019-07-27T15:29:31.1566555Z 
2019-07-27T15:29:31.1566666Z The actual stderr differed from the expected stderr.
2019-07-27T15:29:31.1567222Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0664/E0664.stderr
2019-07-27T15:29:31.1567724Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T15:29:31.1568204Z To only update this specific test, also pass `--test-args error-codes/E0664.rs`
2019-07-27T15:29:31.1594785Z error: 1 errors occurred comparing output.
2019-07-27T15:29:31.1597756Z status: exit code: 1
2019-07-27T15:29:31.1597756Z status: exit code: 1
2019-07-27T15:29:31.1599519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0664.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0664" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0664/auxiliary" "-A" "unused"
2019-07-27T15:29:31.1600687Z ------------------------------------------
2019-07-27T15:29:31.1600864Z 
2019-07-27T15:29:31.1601214Z ------------------------------------------
2019-07-27T15:29:31.1601425Z stderr:
2019-07-27T15:29:31.1601425Z stderr:
2019-07-27T15:29:31.1601793Z ------------------------------------------
2019-07-27T15:29:31.1601990Z error[E0664]: clobber should not be surrounded by braces
2019-07-27T15:29:31.1602579Z    |
2019-07-27T15:29:31.1602579Z    |
2019-07-27T15:29:31.1602741Z LL |          : "{eax}" //~ ERROR E0664
2019-07-27T15:29:31.1603029Z 
2019-07-27T15:29:31.1603029Z 
2019-07-27T15:29:31.1603173Z error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1603760Z    |
2019-07-27T15:29:31.1603760Z    |
2019-07-27T15:29:31.1603918Z LL | /     asm!("mov $$0x200, %eax"
2019-07-27T15:29:31.1604226Z LL | |          :
2019-07-27T15:29:31.1604226Z LL | |          :
2019-07-27T15:29:31.1604387Z LL | |          : "{eax}" //~ ERROR E0664
2019-07-27T15:29:31.1604535Z LL | |         );
2019-07-27T15:29:31.1604807Z 
2019-07-27T15:29:31.1604964Z error: aborting due to 2 previous errors
2019-07-27T15:29:31.1605084Z 
2019-07-27T15:29:31.1605311Z 
2019-07-27T15:29:31.1605311Z 
2019-07-27T15:29:31.1606269Z ------------------------------------------
2019-07-27T15:29:31.1606498Z 
2019-07-27T15:29:31.1606633Z 
2019-07-27T15:29:31.1607106Z ---- [ui] ui/feature-gates/feature-gate-asm.rs stdout ----
2019-07-27T15:29:31.1607338Z diff of stderr:
2019-07-27T15:29:31.1607471Z 
2019-07-27T15:29:31.1607892Z 7    = note: for more information, see https://github.com/rust-lang/rust/issues/29722
2019-07-27T15:29:31.1608153Z 8    = help: add `#![feature(asm)]` to the crate attributes to enable
2019-07-27T15:29:31.1608716Z - error: aborting due to previous error
2019-07-27T15:29:31.1608716Z - error: aborting due to previous error
2019-07-27T15:29:31.1609048Z + error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1609596Z +   --> $DIR/feature-gate-asm.rs:3:9
2019-07-27T15:29:31.1610038Z +    |
2019-07-27T15:29:31.1612997Z + LL |         asm!("");
2019-07-27T15:29:31.1613366Z + 
2019-07-27T15:29:31.1613526Z + error: aborting due to 2 previous errors
2019-07-27T15:29:31.1613690Z 11 
2019-07-27T15:29:31.1614170Z 12 For more information about this error, try `rustc --explain E0658`.
2019-07-27T15:29:31.1614170Z 12 For more information about this error, try `rustc --explain E0658`.
2019-07-27T15:29:31.1614394Z 13 
2019-07-27T15:29:31.1614693Z 
2019-07-27T15:29:31.1614800Z 
2019-07-27T15:29:31.1614960Z The actual stderr differed from the expected stderr.
2019-07-27T15:29:31.1616112Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm/feature-gate-asm.stderr
2019-07-27T15:29:31.1622439Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T15:29:31.1623153Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-asm.rs`
2019-07-27T15:29:31.1623507Z error: 1 errors occurred comparing output.
2019-07-27T15:29:31.1623660Z status: exit code: 1
2019-07-27T15:29:31.1623660Z status: exit code: 1
2019-07-27T15:29:31.1625009Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-asm.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm/auxiliary" "-A" "unused"
2019-07-27T15:29:31.1626459Z ------------------------------------------
2019-07-27T15:29:31.1626675Z 
2019-07-27T15:29:31.1627052Z ------------------------------------------
2019-07-27T15:29:31.1627276Z stderr:
2019-07-27T15:29:31.1627276Z stderr:
2019-07-27T15:29:31.1627640Z ------------------------------------------
2019-07-27T15:29:31.1628172Z error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
2019-07-27T15:29:31.1628884Z    |
2019-07-27T15:29:31.1628884Z    |
2019-07-27T15:29:31.1629072Z LL |         asm!(""); //~ ERROR inline assembly is not stable enough
2019-07-27T15:29:31.1629615Z    |
2019-07-27T15:29:31.1630014Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29722
2019-07-27T15:29:31.1630014Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29722
2019-07-27T15:29:31.1630218Z    = help: add `#![feature(asm)]` to the crate attributes to enable
2019-07-27T15:29:31.1630347Z 
2019-07-27T15:29:31.1630514Z error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1631088Z    |
2019-07-27T15:29:31.1631088Z    |
2019-07-27T15:29:31.1631253Z LL |         asm!(""); //~ ERROR inline assembly is not stable enough
2019-07-27T15:29:31.1631545Z 
2019-07-27T15:29:31.1631684Z error: aborting due to 2 previous errors
2019-07-27T15:29:31.1631805Z 
2019-07-27T15:29:31.1632184Z For more information about this error, try `rustc --explain E0658`.
---
2019-07-27T15:29:31.1633524Z ---- [ui] ui/feature-gates/feature-gate-asm2.rs stdout ----
2019-07-27T15:29:31.1633742Z diff of stderr:
2019-07-27T15:29:31.1633862Z 
2019-07-27T15:29:31.1634257Z 7    = note: for more information, see https://github.com/rust-lang/rust/issues/29722
2019-07-27T15:29:31.1634463Z 8    = help: add `#![feature(asm)]` to the crate attributes to enable
2019-07-27T15:29:31.1635121Z - error: aborting due to previous error
2019-07-27T15:29:31.1635121Z - error: aborting due to previous error
2019-07-27T15:29:31.1635322Z + error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1635649Z +   --> $DIR/feature-gate-asm2.rs:5:26
2019-07-27T15:29:31.1636231Z +    |
2019-07-27T15:29:31.1636448Z + LL |         println!("{:?}", asm!(""));
2019-07-27T15:29:31.1636794Z + 
2019-07-27T15:29:31.1636960Z + error: aborting due to 2 previous errors
2019-07-27T15:29:31.1637139Z 11 
2019-07-27T15:29:31.1637584Z 12 For more information about this error, try `rustc --explain E0658`.
2019-07-27T15:29:31.1637584Z 12 For more information about this error, try `rustc --explain E0658`.
2019-07-27T15:29:31.1637829Z 13 
2019-07-27T15:29:31.1637955Z 
2019-07-27T15:29:31.1638102Z 
2019-07-27T15:29:31.1638262Z The actual stderr differed from the expected stderr.
2019-07-27T15:29:31.1638772Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/feature-gate-asm2.stderr
2019-07-27T15:29:31.1639446Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T15:29:31.1640270Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-asm2.rs`
2019-07-27T15:29:31.1640799Z error: 1 errors occurred comparing output.
2019-07-27T15:29:31.1640962Z status: exit code: 1
2019-07-27T15:29:31.1640962Z status: exit code: 1
2019-07-27T15:29:31.1642306Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-asm2.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/auxiliary" "-A" "unused"
2019-07-27T15:29:31.1643181Z ------------------------------------------
2019-07-27T15:29:31.1643367Z 
2019-07-27T15:29:31.1643714Z ------------------------------------------
2019-07-27T15:29:31.1643919Z stderr:
2019-07-27T15:29:31.1643919Z stderr:
2019-07-27T15:29:31.1644465Z ------------------------------------------
2019-07-27T15:29:31.1644919Z error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
2019-07-27T15:29:31.1646015Z    |
2019-07-27T15:29:31.1646015Z    |
2019-07-27T15:29:31.1646257Z LL |         println!("{:?}", asm!("")); //~ ERROR inline assembly is not stable
2019-07-27T15:29:31.1646624Z    |
2019-07-27T15:29:31.1647105Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29722
2019-07-27T15:29:31.1647105Z    = note: for more information, see https://github.com/rust-lang/rust/issues/29722
2019-07-27T15:29:31.1647334Z    = help: add `#![feature(asm)]` to the crate attributes to enable
2019-07-27T15:29:31.1647495Z 
2019-07-27T15:29:31.1647659Z error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1648286Z    |
2019-07-27T15:29:31.1648286Z    |
2019-07-27T15:29:31.1648473Z LL |         println!("{:?}", asm!("")); //~ ERROR inline assembly is not stable
2019-07-27T15:29:31.1648805Z 
2019-07-27T15:29:31.1648961Z error: aborting due to 2 previous errors
2019-07-27T15:29:31.1649114Z 
2019-07-27T15:29:31.1649745Z For more information about this error, try `rustc --explain E0658`.
---
2019-07-27T15:29:31.1651197Z diff of stderr:
2019-07-27T15:29:31.1651316Z 
2019-07-27T15:29:31.1651883Z - error[E0669]: invalid value for constraint in inline assembly
2019-07-27T15:29:31.1652290Z -   --> $DIR/issue-53787-inline-assembler-macro.rs:21:16
2019-07-27T15:29:31.1652510Z + error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1652879Z +   --> $DIR/issue-53787-inline-assembler-macro.rs:8:13
2019-07-27T15:29:31.1653105Z 3    |
2019-07-27T15:29:31.1653466Z - LL |     fake_jump!("FirstFunc");
2019-07-27T15:29:31.1654132Z + LL | /             asm!(
2019-07-27T15:29:31.1654296Z + LL | |             "
2019-07-27T15:29:31.1654296Z + LL | |             "
2019-07-27T15:29:31.1654462Z + LL | |             jmp $0
2019-07-27T15:29:31.1654612Z + LL | |             lea eax, [ebx]
2019-07-27T15:29:31.1654980Z + ...  |
2019-07-27T15:29:31.1655293Z + LL | |             $0:
2019-07-27T15:29:31.1655779Z + LL | |             "::"0"($id)::"volatile", "intel");
2019-07-27T15:29:31.1656612Z + ...
2019-07-27T15:29:31.1656612Z + ...
2019-07-27T15:29:31.1656787Z + LL |       fake_jump!("FirstFunc");
2019-07-27T15:29:31.1657461Z 6 
2019-07-27T15:29:31.1657618Z 7 error: aborting due to previous error
2019-07-27T15:29:31.1657797Z 8 
2019-07-27T15:29:31.1657921Z 
2019-07-27T15:29:31.1657921Z 
2019-07-27T15:29:31.1658045Z 
2019-07-27T15:29:31.1658231Z The actual stderr differed from the expected stderr.
2019-07-27T15:29:31.1658791Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53787-inline-assembler-macro/issue-53787-inline-assembler-macro.stderr
2019-07-27T15:29:31.1671845Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T15:29:31.1672212Z To only update this specific test, also pass `--test-args issues/issue-53787-inline-assembler-macro.rs`
2019-07-27T15:29:31.1672510Z error: 1 errors occurred comparing output.
2019-07-27T15:29:31.1672576Z status: exit code: 1
2019-07-27T15:29:31.1672576Z status: exit code: 1
2019-07-27T15:29:31.1673725Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53787-inline-assembler-macro.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53787-inline-assembler-macro" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53787-inline-assembler-macro/auxiliary" "-A" "unused"
2019-07-27T15:29:31.1677314Z ------------------------------------------
2019-07-27T15:29:31.1677405Z 
2019-07-27T15:29:31.1677680Z ------------------------------------------
2019-07-27T15:29:31.1677776Z stderr:
2019-07-27T15:29:31.1677776Z stderr:
2019-07-27T15:29:31.1678011Z ------------------------------------------
2019-07-27T15:29:31.1678109Z error[E0472]: asm! is unsupported on this target
2019-07-27T15:29:31.1678500Z    |
2019-07-27T15:29:31.1678565Z LL | /             asm!(
2019-07-27T15:29:31.1678650Z LL | |             "
2019-07-27T15:29:31.1678716Z LL | |             jmp $0
2019-07-27T15:29:31.1678716Z LL | |             jmp $0
2019-07-27T15:29:31.1678801Z LL | |             lea eax, [ebx]
2019-07-27T15:29:31.1679098Z LL | |             $0:
2019-07-27T15:29:31.1679098Z LL | |             $0:
2019-07-27T15:29:31.1679167Z LL | |             "::"0"($id)::"volatile", "intel");
2019-07-27T15:29:31.1679473Z ...
2019-07-27T15:29:31.1679473Z ...
2019-07-27T15:29:31.1679565Z LL |       fake_jump!("FirstFunc"); //~ ERROR invalid value for constraint in inline assembly
2019-07-27T15:29:31.1679941Z 
2019-07-27T15:29:31.1679999Z error: aborting due to previous error
2019-07-27T15:29:31.1680056Z 
2019-07-27T15:29:31.1680089Z 
---
2019-07-27T15:29:31.1682857Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:534:22
2019-07-27T15:29:31.1682946Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-27T15:29:31.1683019Z 
2019-07-27T15:29:31.1683051Z 
2019-07-27T15:29:31.1684860Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "ui" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/8.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-27T15:29:31.1686148Z 
2019-07-27T15:29:31.1686188Z 
2019-07-27T15:29:31.1686645Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/ui src/test/run-fail src/libstd src/liballoc src/libcore
2019-07-27T15:29:31.1686788Z Build completed unsuccessfully in 2:07:02
2019-07-27T15:29:31.1686788Z Build completed unsuccessfully in 2:07:02
2019-07-27T15:29:31.8037421Z ##[error]Bash exited with code '1'.
2019-07-27T15:29:31.8083632Z ##[section]Starting: Upload CPU usage statistics
2019-07-27T15:29:31.8090270Z ==============================================================================
2019-07-27T15:29:31.8090357Z Task         : Bash
2019-07-27T15:29:31.8090441Z Description  : Run a Bash script on macOS, Linux, or Windows
