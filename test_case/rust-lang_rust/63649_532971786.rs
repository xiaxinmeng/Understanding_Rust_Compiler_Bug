plain
2019-09-19T01:34:12.0153923Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T01:34:12.0345340Z ##[command]git config gc.auto 0
2019-09-19T01:34:12.0425954Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T01:34:12.0493637Z ##[command]git config --get-all http.proxy
2019-09-19T01:34:12.0627516Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63649/merge:refs/remotes/pull/63649/merge
---
2019-09-19T01:38:55.2286860Z Step 3/15 : COPY scripts/emscripten.sh /scripts/
2019-09-19T01:38:57.0194206Z  ---> b5d32898e7cc
2019-09-19T01:38:57.0194431Z Step 4/15 : RUN bash /scripts/emscripten.sh
2019-09-19T01:38:57.2661964Z  ---> Running in 69435b1a8079
2019-09-19T01:38:57.8364841Z + git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
2019-09-19T01:38:57.8419171Z Cloning into '/emsdk-portable'...
2019-09-19T01:38:58.5257077Z + cd /emsdk-portable
2019-09-19T01:38:58.5258352Z + hide_output ./emsdk install 1.38.45-upstream
2019-09-19T01:39:28.5321926Z Thu Sep 19 01:39:28 UTC 2019 - building ...
2019-09-19T01:39:37.2572880Z /scripts/emscripten.sh: line 3:    16 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-19T01:39:37.2572880Z /scripts/emscripten.sh: line 3:    16 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-19T01:39:37.2575055Z + ./emsdk activate 1.38.45-upstream
2019-09-19T01:39:37.6132031Z Writing .emscripten configuration file to user home directory /root/
2019-09-19T01:39:37.6132215Z The Emscripten configuration file /root/.emscripten has been rewritten with the following contents:
2019-09-19T01:39:37.6132366Z import os
2019-09-19T01:39:37.6132366Z import os
2019-09-19T01:39:37.6132672Z LLVM_ROOT = '/emsdk-portable/upstream/bin'
2019-09-19T01:39:37.6133305Z BINARYEN_ROOT = '/emsdk-portable/upstream'
2019-09-19T01:39:37.6133610Z EMSCRIPTEN_ROOT = '/emsdk-portable/upstream/emscripten'
2019-09-19T01:39:37.6133886Z NODE_JS = '/emsdk-portable/node/12.9.1_64bit/bin/node'
2019-09-19T01:39:37.6134117Z SPIDERMONKEY_ENGINE = ''
2019-09-19T01:39:37.6134355Z V8_ENGINE = ''
2019-09-19T01:39:37.6134579Z TEMP_DIR = '/tmp'
2019-09-19T01:39:37.6134631Z COMPILER_ENGINE = NODE_JS
2019-09-19T01:39:37.6134681Z JS_ENGINES = [NODE_JS]
2019-09-19T01:39:37.6134734Z 
2019-09-19T01:39:37.6135136Z To conveniently access the selected set of tools from the command line, consider adding the following directories to PATH, or call 'source ./emsdk_env.sh' to do this for you.
2019-09-19T01:39:37.6135191Z 
2019-09-19T01:39:37.6135523Z    /emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin
2019-09-19T01:39:37.6135613Z Set the following tools as active:
2019-09-19T01:39:37.6135901Z    releases-upstream-f3030d9fffcc9e1287cb6b8e72982e94ece31d71-64bit
2019-09-19T01:39:37.6136170Z    node-12.9.1-64bit
2019-09-19T01:39:37.6136204Z 
2019-09-19T01:39:37.6136204Z 
2019-09-19T01:39:37.6219901Z + source ./emsdk_env.sh
2019-09-19T01:39:37.6220537Z ++ SRC=./emsdk_env.sh
2019-09-19T01:39:37.6220786Z ++ '[' ./emsdk_env.sh = '' ']'
2019-09-19T01:39:37.6221022Z +++ pwd
2019-09-19T01:39:37.6221235Z ++ CURDIR=/emsdk-portable
2019-09-19T01:39:37.6226230Z +++ dirname ./emsdk_env.sh
2019-09-19T01:39:37.6239338Z ++ unset SRC
2019-09-19T01:39:37.6249069Z +++ mktemp
2019-09-19T01:39:37.6249069Z +++ mktemp
2019-09-19T01:39:37.6262580Z ++ tmpfile=/tmp/tmp.SNd2CrVyPV
2019-09-19T01:39:37.6262942Z ++ ./emsdk construct_env /tmp/tmp.SNd2CrVyPV
2019-09-19T01:39:37.7999682Z PATH += /emsdk-portable
2019-09-19T01:39:37.7999682Z PATH += /emsdk-portable
2019-09-19T01:39:37.7999927Z PATH += /emsdk-portable/upstream/emscripten
2019-09-19T01:39:37.8000501Z 
2019-09-19T01:39:37.8000575Z Setting environment variables:
2019-09-19T01:39:37.8000856Z EMSDK = /emsdk-portable
2019-09-19T01:39:37.8000905Z EM_CONFIG = /root/.emscripten
2019-09-19T01:39:37.8000905Z EM_CONFIG = /root/.emscripten
2019-09-19T01:39:37.8001158Z EMSDK_NODE = /emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-19T01:39:37.8001198Z 
2019-09-19T01:39:37.8065674Z ++ . /tmp/tmp.SNd2CrVyPV
2019-09-19T01:39:37.8066368Z +++ export PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-19T01:39:37.8066793Z +++ PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-19T01:39:37.8067047Z +++ export EMSDK=/emsdk-portable
2019-09-19T01:39:37.8067245Z +++ EMSDK=/emsdk-portable
2019-09-19T01:39:37.8067296Z +++ export EM_CONFIG=/root/.emscripten
2019-09-19T01:39:37.8067359Z +++ EM_CONFIG=/root/.emscripten
2019-09-19T01:39:37.8067615Z +++ export EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-19T01:39:37.8067850Z +++ EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-19T01:39:37.8068075Z ++ rm -f /tmp/tmp.SNd2CrVyPV
2019-09-19T01:39:37.8077285Z ++ cd /emsdk-portable
2019-09-19T01:39:37.8077518Z + echo 'main(){}'
2019-09-19T01:39:37.8077726Z + HOME=/emsdk-portable/
2019-09-19T01:39:37.8077810Z + emcc a.c
2019-09-19T01:39:37.8710021Z cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/emsdk-portable/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-19T01:39:37.8833912Z cache:INFO:  - ok
2019-09-19T01:39:37.9597429Z a.c:1:1: warning: type specifier missing, defaults to 'int' [-Wimplicit-int]
2019-09-19T01:39:37.9597580Z main(){}
2019-09-19T01:39:37.9628933Z 1 warning generated.
2019-09-19T01:39:37.9628933Z 1 warning generated.
2019-09-19T01:39:37.9856314Z cache:INFO: generating system library: libc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc.a" for subsequent builds)
2019-09-19T01:40:50.9530787Z cache:INFO:  - ok
2019-09-19T01:40:50.9535011Z cache:INFO: generating system library: libcompiler_rt.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt.a" for subsequent builds)
2019-09-19T01:40:51.3750578Z cache:INFO:  - ok
2019-09-19T01:40:51.3752113Z cache:INFO: generating system library: libc-wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc-wasm.a" for subsequent builds)
2019-09-19T01:40:54.1114330Z cache:INFO:  - ok
2019-09-19T01:40:54.1129995Z cache:INFO: generating system library: libdlmalloc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libdlmalloc.a" for subsequent builds)
2019-09-19T01:40:55.1742360Z cache:INFO:  - ok
2019-09-19T01:40:55.1745061Z cache:INFO: generating system library: libpthreads_stub.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libpthreads_stub.a" for subsequent builds)
2019-09-19T01:40:55.4915598Z cache:INFO:  - ok
2019-09-19T01:40:55.4922823Z cache:INFO: generating system library: libcompiler_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a" for subsequent builds)
2019-09-19T01:41:00.2813501Z cache:INFO:  - ok
2019-09-19T01:41:00.2817684Z cache:INFO: generating system library: libc_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc_rt_wasm.a" for subsequent builds)
2019-09-19T01:41:02.6077969Z cache:INFO:  - ok
2019-09-19T01:41:02.6343150Z cache:INFO: generating system asset: generated_struct_info.json... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/generated_struct_info.json" for subsequent builds)
2019-09-19T01:41:04.4502700Z cache:INFO:  - ok
2019-09-19T01:41:04.7434582Z + rm -f a.c a.out.js a.out.wasm
2019-09-19T01:41:04.7444443Z + cp /root/.emscripten /emsdk-portable
2019-09-19T01:41:04.7463156Z + chmod a+rxw -R /emsdk-portable
2019-09-19T01:41:30.3887738Z  ---> d008c95bcec5
2019-09-19T01:41:30.3887833Z Step 5/15 : COPY scripts/sccache.sh /scripts/
2019-09-19T01:41:32.1763167Z  ---> 44b245e81471
2019-09-19T01:41:32.1763327Z Step 6/15 : RUN sh /scripts/sccache.sh
---
2019-09-19T01:41:36.2237188Z Step 7/15 : ENV PATH=$PATH:/emsdk-portable
2019-09-19T01:41:36.4181243Z  ---> Running in 4d5cfe531281
2019-09-19T01:41:37.8606308Z Removing intermediate container 4d5cfe531281
2019-09-19T01:41:37.8607303Z  ---> 2020babf348d
2019-09-19T01:41:37.8607687Z Step 8/15 : ENV PATH=$PATH:/emsdk-portable/upstream/emscripten/
2019-09-19T01:41:39.8216876Z Removing intermediate container de9c595d81e1
2019-09-19T01:41:39.8217786Z  ---> 195dac97f7ae
2019-09-19T01:41:39.8218470Z Step 9/15 : ENV PATH=$PATH:/emsdk-portable/node/12.9.1_64bit/bin/
2019-09-19T01:41:39.9667044Z  ---> Running in 27d9c519337d
---
2019-09-19T01:41:51.7898323Z Successfully built 6f53585cdbb5
2019-09-19T01:41:51.8774206Z Successfully tagged rust-ci:latest
2019-09-19T01:41:51.9065904Z Built container sha256:6f53585cdbb5564cafa40d83467ebc8799c793bee4516eb11a397fb097205872
2019-09-19T01:41:51.9082439Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/b00c92a889cf916884e8edff8b9a3fefc8aa3192713ae8fb19e87ece5dc1beceaeff53ad6cf282e0375890176a09a5ab3e4ac3d5bf4720b955873d72323d34b6
2019-09-19T01:43:25.1782235Z upload failed: - to s3://rust-lang-ci-sccache2/docker/b00c92a889cf916884e8edff8b9a3fefc8aa3192713ae8fb19e87ece5dc1beceaeff53ad6cf282e0375890176a09a5ab3e4ac3d5bf4720b955873d72323d34b6 Unable to locate credentials
2019-09-19T01:43:26.0728407Z [CI_JOB_NAME=asmjs]
2019-09-19T01:43:26.0765390Z == clock drift check ==
2019-09-19T01:43:26.0777134Z   local time: Thu Sep 19 01:43:26 UTC 2019
2019-09-19T01:43:26.2257501Z   network time: Thu, 19 Sep 2019 01:43:26 GMT
---
2019-09-19T01:43:26.3251553Z configure: build.locked-deps    := True
2019-09-19T01:43:26.3251757Z configure: llvm.ccache          := sccache
2019-09-19T01:43:26.3252180Z configure: build.cargo-native-static := True
2019-09-19T01:43:26.3252653Z configure: dist.missing-tools   := True
2019-09-19T01:43:26.3253125Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-19T01:43:26.3253514Z configure: writing `config.toml` in current directory
2019-09-19T01:43:26.3253688Z configure: 
2019-09-19T01:43:26.3254109Z configure: run `python /checkout/x.py --help`
2019-09-19T01:43:26.3254316Z configure: 
---
2019-09-19T02:49:25.1311527Z running: "ar" "crs" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/rust_test_helpers.o"
2019-09-19T02:49:25.1325424Z exit code: 0
2019-09-19T02:49:25.1331537Z Building test helpers
2019-09-19T02:49:25.1336815Z running: "emcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-o" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
2019-09-19T02:49:25.2227119Z cargo:warning=cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/home/user/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-19T02:49:25.2372218Z cargo:warning=cache:INFO:  - ok
2019-09-19T02:49:25.3783859Z cargo:warning=shared:INFO: (Emscripten: Running sanity checks)
2019-09-19T02:49:25.3834823Z cargo:warning=shared:WARNING: java does not seem to exist, required for closure compiler, which is optional (define JAVA in /emsdk-portable/.emscripten if you want it)
2019-09-19T02:49:25.3834986Z cargo:warning=shared:WARNING: closure compiler will not be available
2019-09-19T02:49:25.4702447Z running: "emar" "crs" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o"
2019-09-19T02:49:25.5450291Z exit code: 0
2019-09-19T02:49:25.5464171Z Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
2019-09-19T02:49:25.8685046Z    Compiling proc-macro2 v0.4.30
---
2019-09-19T02:51:35.6890257Z 
2019-09-19T02:51:35.6891428Z running 9024 tests
2019-09-19T02:52:35.6909111Z test [ui] ui/abi/abi-sysv64-register-usage.rs has been running for over 60 seconds
2019-09-19T02:52:35.6911285Z test [ui] ui/abi/abi-sysv64-arg-passing.rs has been running for over 60 seconds
2019-09-19T02:55:51.6960476Z .i.....ii..i.......i......i........i.iiii.....................................ii...............i.... 100/9024
2019-09-19T02:58:27.3147620Z ..ii.........i.......................i....iiiiii.................................................... 200/9024
2019-09-19T03:02:30.6529534Z ..............................................i.........................i........................... 400/9024
2019-09-19T03:02:30.6529534Z ..............................................i.........................i........................... 400/9024
2019-09-19T03:04:12.0866009Z ..................................i...............................................iii............... 500/9024
2019-09-19T03:09:16.2340788Z ...........................................................i........................................ 700/9024
2019-09-19T03:09:55.4118185Z .................................................................................................... 800/9024
2019-09-19T03:10:38.0616754Z .................................................................i.................................. 900/9024
2019-09-19T03:12:45.2165455Z .......i............................................ii.........................................i.... 1000/9024
2019-09-19T03:12:45.2165455Z .......i............................................ii.........................................i.... 1000/9024
2019-09-19T03:14:17.5134371Z ....i.i........................................i.................................................... 1100/9024
2019-09-19T03:15:08.0332921Z .................................................................................................... 1200/9024
2019-09-19T03:15:58.3202588Z .................i.ii............................................................................... 1300/9024
2019-09-19T03:18:13.5786171Z .............................................................................i...................... 1400/9024
2019-09-19T03:20:04.1821899Z .................................................................................................... 1500/9024
2019-09-19T03:22:08.9227741Z ..........................ii.i...................................................................... 1600/9024
2019-09-19T03:24:22.3640482Z .......................................i......i..................i...............i....iii........... 1700/9024
2019-09-19T03:25:55.6778823Z .................................................................................................... 1800/9024
2019-09-19T03:27:29.2968852Z ........................................................iiiii.......................i.........ii.... 1900/9024
2019-09-19T03:28:58.5105346Z .................................................................................................... 2000/9024
2019-09-19T03:29:22.4351994Z ..............iii................................................................................... 2100/9024
2019-09-19T03:29:24.7151499Z .................................................................................................... 2200/9024
2019-09-19T03:29:37.1676754Z .............................................................iiii......................i............ 2300/9024
2019-09-19T03:31:41.9350475Z ........i..........ii............................................................................... 2500/9024
2019-09-19T03:33:45.6326045Z .................................................................................................... 2600/9024
2019-09-19T03:33:45.6326045Z .................................................................................................... 2600/9024
2019-09-19T03:36:31.1875675Z .......i....i..........................................................i...........ii.....ii.i...... 2700/9024
2019-09-19T03:39:25.1480238Z ..........i......................................................................................... 2900/9024
2019-09-19T03:41:11.0996370Z .................................................................................................... 3000/9024
2019-09-19T03:42:45.7053596Z .................................................................................................... 3100/9024
2019-09-19T03:42:45.7053596Z .................................................................................................... 3100/9024
2019-09-19T03:44:31.7229492Z .......ii..ii..................i....i............i....i.................i........................... 3200/9024
2019-09-19T03:49:05.8874483Z ......................i.............i.....i.........................................i.......i....... 3400/9024
2019-09-19T03:51:21.0418865Z .i............................i..................................................................... 3500/9024
2019-09-19T03:53:41.3119572Z .............................................i...................................................... 3600/9024
2019-09-19T03:55:57.5946190Z ...........i...........................................................i............................ 3700/9024
---
2019-09-19T04:16:25.8756304Z ............................................ii..............i....................................... 4700/9024
2019-09-19T04:18:41.2224879Z ...................................................................................i................ 4800/9024
2019-09-19T04:20:46.2568684Z ................................i...............................i................................... 4900/9024
2019-09-19T04:23:10.7077318Z .............................................................................i...........i.......... 5000/9024
2019-09-19T04:24:58.1527410Z ....i.......................i..i..i.i.i............................................................. 5100/9024
2019-09-19T04:26:55.8928441Z .................................................................................................... 5300/9024
2019-09-19T04:26:55.8928441Z .................................................................................................... 5300/9024
2019-09-19T04:28:57.9922612Z .i...............i......................................................ii..................ii...... 5400/9024
2019-09-19T04:32:27.8422833Z .................................................................................................... 5600/9024
2019-09-19T04:32:27.8422833Z .................................................................................................... 5600/9024
2019-09-19T04:34:15.4318379Z ...............i............i...i......................................................ii...i..ii... 5700/9024
2019-09-19T04:36:33.6766688Z ........i...........................i........i.i.................................................... 5800/9024
2019-09-19T04:38:52.1392732Z .................................................................................................... 6000/9024
2019-09-19T04:38:52.1392732Z .................................................................................................... 6000/9024
2019-09-19T04:39:28.7999405Z ........................................................i...........i....................i..ii...... 6100/9024
2019-09-19T04:41:56.6364108Z ..........................i.....ii.................................................................. 6200/9024
2019-09-19T04:44:07.4575375Z ...............................i...............................i....................iii..i..ii.iiii. 6300/9024
2019-09-19T04:44:26.2347301Z iiiii...........................................i................................................... 6400/9024
2019-09-19T04:44:33.4626231Z ....................i............................................................................... 6600/9024
2019-09-19T04:45:21.9198777Z ..........................................i............................i............................ 6700/9024
2019-09-19T04:47:11.8168184Z .....................................................................................i.............. 6800/9024
2019-09-19T04:47:11.8168184Z .....................................................................................i.............. 6800/9024
2019-09-19T04:48:40.7906578Z .................................................iiiiiiii........................................... 6900/9024
2019-09-19T04:52:13.9772775Z .................................................................................................... 7100/9024
2019-09-19T04:53:07.3689209Z .................................................................................................... 7200/9024
2019-09-19T04:53:51.2683879Z .............................i...................................................................... 7300/9024
2019-09-19T04:56:09.3889418Z .....................................................................................i.......i...... 7400/9024
2019-09-19T04:56:09.3889418Z .....................................................................................i.......i...... 7400/9024
2019-09-19T04:57:55.9068583Z .................................................................................................... 7500/9024
2019-09-19T04:59:39.8717844Z .........................................i..i.............iii.....i....iiiiiiiiiii.i................ 7600/9024
2019-09-19T05:01:52.9323567Z ............................................................................................i.i..... 7800/9024
2019-09-19T05:05:51.5229749Z ................................................................i................................... 7900/9024
2019-09-19T05:07:29.5317929Z ..............................i..................................................................... 8000/9024
2019-09-19T05:07:29.5317929Z ..............................i..................................................................... 8000/9024
2019-09-19T05:10:05.0507034Z .............................................................iii.....i.....i.........i........ii.... 8100/9024
2019-09-19T05:12:31.4077494Z i.i.....iiiiii.....iiiiiiii.ii...ii.iii..iiii....................................................... 8200/9024
2019-09-19T05:17:15.8126988Z .......................................................................................i............ 8400/9024
2019-09-19T05:18:30.3341495Z .................................................................................................... 8500/9024
2019-09-19T05:19:55.7629577Z .................................................................................................... 8600/9024
2019-09-19T05:21:47.5806333Z ...............................i.................................................................... 8700/9024
---
2019-09-19T05:27:01.2905822Z failures:
2019-09-19T05:27:01.2906138Z 
2019-09-19T05:27:01.2907086Z ---- [compile-fail] compile-fail/weak-lang-item.rs stdout ----
2019-09-19T05:27:01.2907400Z 
2019-09-19T05:27:01.2908182Z error: error pattern ' language item required, but not found: `eh_personality`' not found!
2019-09-19T05:27:01.2908509Z status: exit code: 1
2019-09-19T05:27:01.2909562Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/weak-lang-item.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/weak-lang-item" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/weak-lang-item/auxiliary" "-A" "unused"
2019-09-19T05:27:01.2910321Z ------------------------------------------
2019-09-19T05:27:01.2910517Z 
2019-09-19T05:27:01.2910942Z ------------------------------------------
2019-09-19T05:27:01.2911157Z stderr:
2019-09-19T05:27:01.2911157Z stderr:
2019-09-19T05:27:01.2911565Z ------------------------------------------
2019-09-19T05:27:01.2911803Z error[E0259]: the name `core` is defined multiple times
2019-09-19T05:27:01.2912240Z   --> /checkout/src/test/compile-fail/weak-lang-item.rs:8:1
2019-09-19T05:27:01.2912463Z    |
2019-09-19T05:27:01.2912649Z LL | extern crate core;
2019-09-19T05:27:01.2912810Z    | ^^^^^^^^^^^^^^^^^^ `core` reimported here
2019-09-19T05:27:01.2912964Z    |
2019-09-19T05:27:01.2913145Z    = note: `core` must be defined only once in the type namespace of this module
2019-09-19T05:27:01.2914432Z    |
2019-09-19T05:27:01.2914596Z LL | extern crate core as other_core;
2019-09-19T05:27:01.2914753Z    |
2019-09-19T05:27:01.2914885Z 
2019-09-19T05:27:01.2914885Z 
2019-09-19T05:27:01.2915064Z error: `#[panic_handler]` function required, but not found
2019-09-19T05:27:01.2915357Z error: aborting due to 2 previous errors
2019-09-19T05:27:01.2916350Z 
2019-09-19T05:27:01.2917748Z For more information about this error, try `rustc --explain E0259`.
2019-09-19T05:27:01.2918317Z 
---
2019-09-19T05:27:01.2922228Z 
2019-09-19T05:27:01.2922638Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-19T05:27:01.2922680Z 
2019-09-19T05:27:01.2923046Z 
2019-09-19T05:27:01.2926717Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "compile-fail" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-19T05:27:01.2927368Z 
2019-09-19T05:27:01.2927402Z 
2019-09-19T05:27:01.2927825Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten
2019-09-19T05:27:01.2927909Z Build completed unsuccessfully in 3:40:50
2019-09-19T05:27:01.2927909Z Build completed unsuccessfully in 3:40:50
2019-09-19T05:27:01.2983407Z == clock drift check ==
2019-09-19T05:27:01.2998027Z   local time: Thu Sep 19 05:27:01 UTC 2019
2019-09-19T05:27:01.4488846Z   network time: Thu, 19 Sep 2019 05:27:01 GMT
2019-09-19T05:27:01.4492388Z == end clock drift check ==
2019-09-19T05:27:02.1883404Z ##[error]Bash exited with code '1'.
2019-09-19T05:27:02.1921925Z ##[section]Starting: Checkout
2019-09-19T05:27:02.1923720Z ==============================================================================
2019-09-19T05:27:02.1923775Z Task         : Get sources
2019-09-19T05:27:02.1923822Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
