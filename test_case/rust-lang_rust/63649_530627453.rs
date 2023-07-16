plain
2019-09-11T20:29:34.7358943Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T20:29:34.7568036Z ##[command]git config gc.auto 0
2019-09-11T20:29:34.7654293Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T20:29:34.7705187Z ##[command]git config --get-all http.proxy
2019-09-11T20:29:34.7844471Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63649/merge:refs/remotes/pull/63649/merge
---
2019-09-11T20:34:34.0673521Z  ---> 07309c031cec
2019-09-11T20:34:34.0673589Z Step 3/13 : COPY scripts/emscripten.sh /scripts/
2019-09-11T20:34:35.9760122Z  ---> 8ed93b5c0e1f
2019-09-11T20:34:35.9760300Z Step 4/13 : RUN bash /scripts/emscripten.sh
2019-09-11T20:34:36.1286178Z  ---> Running in f401f42199d8
2019-09-11T20:34:36.6585069Z + git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
2019-09-11T20:34:36.6610281Z Cloning into '/emsdk-portable'...
2019-09-11T20:34:37.8845565Z + cd /emsdk-portable
2019-09-11T20:34:37.8845961Z + hide_output ./emsdk install 1.38.42-upstream
2019-09-11T20:35:07.2805297Z Wed Sep 11 20:35:07 UTC 2019 - building ...
2019-09-11T20:35:11.9956159Z /scripts/emscripten.sh: line 3:    17 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-11T20:35:11.9956159Z /scripts/emscripten.sh: line 3:    17 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-11T20:35:11.9961514Z + ./emsdk activate 1.38.42-upstream
2019-09-11T20:35:12.3851752Z Writing .emscripten configuration file to user home directory /root/
2019-09-11T20:35:12.3852734Z The Emscripten configuration file /root/.emscripten has been rewritten with the following contents:
2019-09-11T20:35:12.3853453Z import os
2019-09-11T20:35:12.3853453Z import os
2019-09-11T20:35:12.3854090Z LLVM_ROOT = '/emsdk-portable/upstream/bin'
2019-09-11T20:35:12.3855042Z BINARYEN_ROOT = '/emsdk-portable/upstream'
2019-09-11T20:35:12.3855712Z EMSCRIPTEN_ROOT = '/emsdk-portable/upstream/emscripten'
2019-09-11T20:35:12.3856267Z NODE_JS = '/emsdk-portable/node/12.9.1_64bit/bin/node'
2019-09-11T20:35:12.3856784Z SPIDERMONKEY_ENGINE = ''
2019-09-11T20:35:12.3857308Z V8_ENGINE = ''
2019-09-11T20:35:12.3857798Z TEMP_DIR = '/tmp'
2019-09-11T20:35:12.3858085Z COMPILER_ENGINE = NODE_JS
2019-09-11T20:35:12.3858332Z JS_ENGINES = [NODE_JS]
2019-09-11T20:35:12.3858559Z 
2019-09-11T20:35:12.3859178Z To conveniently access the selected set of tools from the command line, consider adding the following directories to PATH, or call 'source ./emsdk_env.sh' to do this for you.
2019-09-11T20:35:12.3859488Z 
2019-09-11T20:35:12.3860021Z    /emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin
2019-09-11T20:35:12.3860567Z Set the following tools as active:
2019-09-11T20:35:12.3860567Z Set the following tools as active:
2019-09-11T20:35:12.3861078Z    releases-upstream-05f8c01394ddd0838d3cb484ba8ca97a3efc1ac9-64bit
2019-09-11T20:35:12.3861892Z 
2019-09-11T20:35:12.3943740Z + source ./emsdk_env.sh
2019-09-11T20:35:12.3952269Z ++ SRC=./emsdk_env.sh
2019-09-11T20:35:12.3952269Z ++ SRC=./emsdk_env.sh
2019-09-11T20:35:12.3953130Z ++ '[' ./emsdk_env.sh = '' ']'
2019-09-11T20:35:12.3953731Z +++ pwd
2019-09-11T20:35:12.3954179Z ++ CURDIR=/emsdk-portable
2019-09-11T20:35:12.3954787Z +++ dirname ./emsdk_env.sh
2019-09-11T20:35:12.3961693Z ++ unset SRC
2019-09-11T20:35:12.3962297Z +++ mktemp
2019-09-11T20:35:12.3962297Z +++ mktemp
2019-09-11T20:35:12.3974091Z ++ tmpfile=/tmp/tmp.nya847z9Tq
2019-09-11T20:35:12.3974204Z ++ ./emsdk construct_env /tmp/tmp.nya847z9Tq
2019-09-11T20:35:12.5743752Z PATH += /emsdk-portable
2019-09-11T20:35:12.5743752Z PATH += /emsdk-portable
2019-09-11T20:35:12.5744044Z PATH += /emsdk-portable/upstream/emscripten
2019-09-11T20:35:12.5746364Z 
2019-09-11T20:35:12.5746649Z Setting environment variables:
2019-09-11T20:35:12.5747123Z EMSDK = /emsdk-portable
2019-09-11T20:35:12.5747423Z EM_CONFIG = /root/.emscripten
2019-09-11T20:35:12.5747423Z EM_CONFIG = /root/.emscripten
2019-09-11T20:35:12.5747941Z EMSDK_NODE = /emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-11T20:35:12.5748212Z 
2019-09-11T20:35:12.5836312Z ++ . /tmp/tmp.nya847z9Tq
2019-09-11T20:35:12.5837162Z +++ export PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-11T20:35:12.5837643Z +++ PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-11T20:35:12.5837878Z +++ export EMSDK=/emsdk-portable
2019-09-11T20:35:12.5838079Z +++ EMSDK=/emsdk-portable
2019-09-11T20:35:12.5838149Z +++ export EM_CONFIG=/root/.emscripten
2019-09-11T20:35:12.5838195Z +++ EM_CONFIG=/root/.emscripten
2019-09-11T20:35:12.5838439Z +++ export EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-11T20:35:12.5838692Z +++ EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-11T20:35:12.5838903Z ++ rm -f /tmp/tmp.nya847z9Tq
2019-09-11T20:35:12.5839181Z ++ cd /emsdk-portable
2019-09-11T20:35:12.5839412Z + echo 'main(){}'
2019-09-11T20:35:12.5839619Z + HOME=/emsdk-portable/
2019-09-11T20:35:12.5839813Z + emcc a.c
2019-09-11T20:35:12.6474530Z cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/emsdk-portable/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-11T20:35:12.6595793Z cache:INFO:  - ok
2019-09-11T20:35:12.7704050Z a.c:1:1: warning: type specifier missing, defaults to 'int' [-Wimplicit-int]
2019-09-11T20:35:12.7704207Z main(){}
2019-09-11T20:35:12.7735953Z 1 warning generated.
2019-09-11T20:35:12.7735953Z 1 warning generated.
2019-09-11T20:35:12.7986717Z cache:INFO: generating system library: libc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc.a" for subsequent builds)
2019-09-11T20:36:38.1394874Z cache:INFO:  - ok
2019-09-11T20:36:38.1400468Z cache:INFO: generating system library: libcompiler_rt.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt.a" for subsequent builds)
2019-09-11T20:36:38.6683106Z cache:INFO:  - ok
2019-09-11T20:36:38.6688413Z cache:INFO: generating system library: libc-wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc-wasm.a" for subsequent builds)
2019-09-11T20:36:41.8604825Z cache:INFO:  - ok
2019-09-11T20:36:41.8625187Z cache:INFO: generating system library: libdlmalloc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libdlmalloc.a" for subsequent builds)
2019-09-11T20:36:42.9365296Z cache:INFO:  - ok
2019-09-11T20:36:42.9368542Z cache:INFO: generating system library: libpthreads_stub.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libpthreads_stub.a" for subsequent builds)
2019-09-11T20:36:43.2790869Z cache:INFO:  - ok
2019-09-11T20:36:43.2796957Z cache:INFO: generating system library: libcompiler_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a" for subsequent builds)
2019-09-11T20:36:48.7870347Z cache:INFO:  - ok
2019-09-11T20:36:48.7875281Z cache:INFO: generating system library: libc_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc_rt_wasm.a" for subsequent builds)
2019-09-11T20:36:51.4781014Z cache:INFO:  - ok
2019-09-11T20:36:51.4986050Z cache:INFO: generating system asset: generated_struct_info.json... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/generated_struct_info.json" for subsequent builds)
2019-09-11T20:36:53.3463985Z cache:INFO:  - ok
2019-09-11T20:36:53.7122136Z + rm -f a.c a.out.js a.out.wasm
2019-09-11T20:36:53.7134258Z + cp /root/.emscripten /emsdk-portable
2019-09-11T20:36:53.7153671Z + chmod a+rxw -R /emsdk-portable
2019-09-11T20:37:19.2429172Z Removing intermediate container f401f42199d8
2019-09-11T20:37:19.2430792Z Step 5/13 : COPY scripts/sccache.sh /scripts/
2019-09-11T20:37:21.1692182Z  ---> eafc23691bb5
2019-09-11T20:37:21.1692959Z Step 6/13 : RUN sh /scripts/sccache.sh
2019-09-11T20:37:21.3381294Z  ---> Running in c6f5288c6eb3
---
2019-09-11T20:37:25.9398096Z Step 7/13 : ENV PATH=$PATH:/emsdk-portable
2019-09-11T20:37:26.1900896Z  ---> Running in 09d2483b5452
2019-09-11T20:37:27.6394090Z Removing intermediate container 09d2483b5452
2019-09-11T20:37:27.6394995Z  ---> 28f6d4aaf6e9
2019-09-11T20:37:27.6395390Z Step 8/13 : ENV PATH=$PATH:/emsdk-portable/upstream/emscripten/
2019-09-11T20:37:27.8438349Z  ---> Running in 368b7f6ab589
2019-09-11T20:37:29.3749311Z Removing intermediate container 368b7f6ab589
2019-09-11T20:37:29.3751280Z Step 9/13 : ENV PATH=$PATH:/emsdk-portable/node/12.9.1_64bit/bin/
2019-09-11T20:37:29.5830795Z  ---> Running in 32f1b7f1f455
2019-09-11T20:37:31.0638093Z Removing intermediate container 32f1b7f1f455
2019-09-11T20:37:31.0638892Z  ---> 62931d59260e
---
2019-09-11T20:37:37.8954536Z Removing intermediate container a95de16d5118
2019-09-11T20:37:37.8956063Z  ---> 15e72cfdcb3d
2019-09-11T20:37:37.9000516Z Successfully built 15e72cfdcb3d
2019-09-11T20:37:38.0007868Z Successfully tagged rust-ci:latest
2019-09-11T20:37:38.1186564Z Built container sha256:15e72cfdcb3d8d4a6dee09906e10d7ecc3b2834cb90a2efbc5c24b01a816847a
2019-09-11T20:37:38.1202897Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/8ee7d461869c9704c3ba0346ea7f0e68dceeee7beed3d0aa54d8ec45631045dd591f036407da64f446872e712c15d85f81ffb5caf5ad710b37c3e97022542921
2019-09-11T20:39:12.3090762Z upload failed: - to s3://rust-lang-ci-sccache2/docker/8ee7d461869c9704c3ba0346ea7f0e68dceeee7beed3d0aa54d8ec45631045dd591f036407da64f446872e712c15d85f81ffb5caf5ad710b37c3e97022542921 Unable to locate credentials
2019-09-11T20:39:13.4438594Z [CI_JOB_NAME=asmjs]
2019-09-11T20:39:13.4496606Z == clock drift check ==
2019-09-11T20:39:13.4505453Z   local time: Wed Sep 11 20:39:13 UTC 2019
2019-09-11T20:39:13.7153640Z   network time: Wed, 11 Sep 2019 20:39:13 GMT
---
2019-09-11T20:39:13.8173171Z configure: build.locked-deps    := True
2019-09-11T20:39:13.8173226Z configure: llvm.ccache          := sccache
2019-09-11T20:39:13.8174314Z configure: build.cargo-native-static := True
2019-09-11T20:39:13.8174564Z configure: dist.missing-tools   := True
2019-09-11T20:39:13.8174854Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-11T20:39:13.8175007Z configure: writing `config.toml` in current directory
2019-09-11T20:39:13.8175286Z configure: 
2019-09-11T20:39:13.8175605Z configure: run `python /checkout/x.py --help`
2019-09-11T20:39:13.8175657Z configure: 
---
2019-09-11T22:14:43.1098874Z running: "ar" "crs" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/rust_test_helpers.o"
2019-09-11T22:14:43.1122301Z exit code: 0
2019-09-11T22:14:43.1127467Z Building test helpers
2019-09-11T22:14:43.1134205Z running: "emcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-o" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
2019-09-11T22:14:43.2132934Z cargo:warning=cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/home/user/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-11T22:14:43.6318995Z cargo:warning=cache:INFO:  - ok
2019-09-11T22:14:43.9530416Z cargo:warning=shared:INFO: (Emscripten: Running sanity checks)
2019-09-11T22:14:43.9574596Z cargo:warning=shared:WARNING: java does not seem to exist, required for closure compiler, which is optional (define JAVA in /emsdk-portable/.emscripten if you want it)
2019-09-11T22:14:43.9578583Z cargo:warning=shared:WARNING: closure compiler will not be available
2019-09-11T22:14:44.2583482Z running: "emar" "crs" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o"
2019-09-11T22:14:44.3387089Z exit code: 0
2019-09-11T22:14:44.3408917Z Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
2019-09-11T22:14:44.6587980Z    Compiling proc-macro2 v0.4.30
---
2019-09-11T22:16:59.6674018Z 
2019-09-11T22:16:59.6674255Z running 9009 tests
2019-09-11T22:17:59.0632821Z test [ui] ui/abi/abi-sysv64-arg-passing.rs has been running for over 60 seconds
2019-09-11T22:17:59.0633817Z test [ui] ui/abi/abi-sysv64-register-usage.rs has been running for over 60 seconds
2019-09-11T22:22:07.8779501Z .i.....ii..i.......i......i........i.iiiF....................................ii...............i..... 100/9009
2019-09-11T22:25:17.0788977Z .ii.........i.......................i....iiiiii..................................................... 200/9009
2019-09-11T22:30:15.5063204Z .............................................i........................i............................. 400/9009
2019-09-11T22:30:15.5063204Z .............................................i........................i............................. 400/9009
2019-09-11T22:32:18.5203921Z ...............................i..............................................iii................... 500/9009
2019-09-11T22:38:33.6343215Z .......................................................i............................................ 700/9009
2019-09-11T22:39:23.6729320Z .................................................................................................... 800/9009
2019-09-11T22:40:20.0465517Z .............................................................i...................................... 900/9009
2019-09-11T22:43:02.7074490Z ...i............................................ii.........................................i........ 1000/9009
2019-09-11T22:43:02.7074490Z ...i............................................ii.........................................i........ 1000/9009
2019-09-11T22:44:45.4974014Z i.i........................................i........................................................ 1100/9009
2019-09-11T22:45:47.2023502Z .................................................................................................... 1200/9009
2019-09-11T22:46:58.2525783Z ............i.ii.................................................................................... 1300/9009
2019-09-11T22:49:39.3164007Z ......................................................................i............................. 1400/9009
2019-09-11T22:52:05.2073937Z .................................................................................................... 1500/9009
2019-09-11T22:54:30.5463942Z ...................ii.i............................................................................. 1600/9009
2019-09-11T22:57:06.7962259Z ...............................i......i..................i...............i....iii................... 1700/9009
2019-09-11T22:59:20.6322764Z .................................................................................................... 1800/9009
2019-09-11T23:00:55.4875419Z ................................................iiiii.......................i.........ii............ 1900/9009
2019-09-11T23:02:55.8877353Z .................................................................................................... 2000/9009
2019-09-11T23:03:14.6215595Z ......iii........................................................................................... 2100/9009
2019-09-11T23:03:17.7055104Z .................................................................................................... 2200/9009
2019-09-11T23:03:50.9326958Z .....................................................iiii......................i.................... 2300/9009
2019-09-11T23:06:02.8634428Z ..............................i.........................................i........................... 2400/9009
2019-09-11T23:06:11.3047693Z i..........ii....................................................................................... 2500/9009
2019-09-11T23:08:59.9646107Z ...................................................................................................i 2600/9009
2019-09-11T23:12:18.5209441Z ....i..........................................................i...........ii.....ii.i.............. 2700/9009
2019-09-11T23:16:03.4707617Z .i.................................................................................................. 2900/9009
2019-09-11T23:18:03.1762485Z ................................F................................................................... 3000/9009
2019-09-11T23:19:45.2626563Z ..................................................................................................ii 3100/9009
2019-09-11T23:19:45.2626563Z ..................................................................................................ii 3100/9009
2019-09-11T23:22:10.5805687Z ..ii.................i....i............i....i.................i..................................... 3200/9009
2019-09-11T23:27:59.9442713Z ............i.............i.....i.........................................i.......i........i........ 3400/9009
2019-09-11T23:30:20.0047624Z ....................i............................................................................... 3500/9009
2019-09-11T23:33:17.4834993Z ...................................i................................................................ 3600/9009
2019-09-11T23:36:08.6030461Z .i...........................................................i..............................i....... 3700/9009
---
2019-09-12T00:07:40.4977530Z .......................i...............................i............................................ 4900/9009
2019-09-12T00:10:50.2911045Z ...................................................................i...........i..............i..... 5000/9009
2019-09-12T00:12:23.5532908Z .....................test [ui] ui/iterators/iter-count-overflow-debug.rs has been running for over 60 seconds
2019-09-12T00:13:01.5971713Z test [ui] ui/iterators/iter-position-overflow-debug.rs has been running for over 60 seconds
2019-09-12T00:15:12.8609927Z FF.i.i.i.........................................test [ui] ui/iterators/iter-position-overflow-ndebug.rs has been running for over 60 seconds
2019-09-12T00:16:52.0440582Z .....................................i.............................................................. 5200/9009
2019-09-12T00:18:26.8447577Z ..........................................................................................i......... 5300/9009
2019-09-12T00:18:26.8447577Z ..........................................................................................i......... 5300/9009
2019-09-12T00:21:06.6965868Z ......i......................................................ii..................ii................. 5400/9009
2019-09-12T00:25:37.1653229Z .................................................................................................... 5600/9009
2019-09-12T00:25:37.1653229Z .................................................................................................... 5600/9009
2019-09-12T00:27:59.8162522Z ...i............i...i......................................................ii...i..ii...........i... 5700/9009
2019-09-12T00:30:29.4574864Z ........................i........i.i................................................................ 5800/9009
2019-09-12T00:33:02.5256978Z .................................................................................................... 6000/9009
2019-09-12T00:33:02.5256978Z .................................................................................................... 6000/9009
2019-09-12T00:34:03.1528911Z ............................................i...........i....................i..ii.................. 6100/9009
2019-09-12T00:37:08.0283744Z ..............i.....ii.............................................................................. 6200/9009
2019-09-12T00:39:29.3171604Z ...................i...............................i....................iii..i..ii.iiii.iiiii....... 6300/9009
2019-09-12T00:39:46.5007201Z .................................................................................................... 6500/9009
2019-09-12T00:39:49.6227140Z ........i........................................................................................... 6600/9009
2019-09-12T00:40:50.0539464Z ..............................i............................i........................................ 6700/9009
2019-09-12T00:43:10.6034760Z .........................................................................i.......................... 6800/9009
2019-09-12T00:43:10.6034760Z .........................................................................i.......................... 6800/9009
2019-09-12T00:45:00.8985103Z .....................................iiiiiiii..............................................i........ 6900/9009
2019-09-12T00:49:13.6901119Z .................................................................................................... 7100/9009
2019-09-12T00:50:09.5471074Z .................................................................................................... 7200/9009
2019-09-12T00:51:04.4727303Z .................i.................................................................................. 7300/9009
2019-09-12T00:54:20.8764363Z .......................................................................i.......i.................... 7400/9009
2019-09-12T00:54:20.8764363Z .......................................................................i.......i.................... 7400/9009
2019-09-12T00:56:13.8146294Z .................................................................................................... 7500/9009
2019-09-12T00:58:13.7041794Z ..........................i..i.............iii.....i....iiiiiiiiiii.i.........................i..... 7600/9009
2019-09-12T01:01:04.9206498Z .............................................................................i.i.................... 7800/9009
2019-09-12T01:06:05.2310799Z .................................................i.................................................. 7900/9009
2019-09-12T01:07:31.2796764Z ...............i.................................................................................... 8000/9009
2019-09-12T01:07:31.2796764Z ...............i.................................................................................... 8000/9009
2019-09-12T01:11:00.7525313Z ..............................................iii.....i.....i.........i........ii....i.i.....iiiiii. 8100/9009
2019-09-12T01:13:49.8022628Z ....iiiiiiii.ii...ii.iii..iiii............................................................i......... 8200/9009
2019-09-12T01:19:33.4522831Z ........................................................................i........................... 8400/9009
2019-09-12T01:20:48.4384345Z .................................................................................................... 8500/9009
2019-09-12T01:22:54.3563559Z .................................................................................................... 8600/9009
2019-09-12T01:25:09.5152528Z ................i................................................................................... 8700/9009
2019-09-12T01:25:09.5152528Z ................i................................................................................... 8700/9009
2019-09-12T01:28:02.0125920Z ..........................................i......................................................... 8800/9009
2019-09-12T01:29:12.0048757Z ................................ii...............................i.................................. 8900/9009
2019-09-12T01:30:36.7434804Z .............i...i.i.............................................................................ii. 9000/9009
2019-09-12T01:31:04.8044184Z failures:
2019-09-12T01:31:04.8093134Z 
2019-09-12T01:31:04.8094283Z ---- [ui] ui/abi/statics/static-mut-foreign.rs stdout ----
2019-09-12T01:31:04.8094330Z 
2019-09-12T01:31:04.8094330Z 
2019-09-12T01:31:04.8094583Z error: test compilation failed although it shouldn't!
2019-09-12T01:31:04.8094967Z status: exit code: 1
2019-09-12T01:31:04.8095822Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/statics/static-mut-foreign.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/auxiliary"
2019-09-12T01:31:04.8096379Z ------------------------------------------
2019-09-12T01:31:04.8096494Z 
2019-09-12T01:31:04.8096775Z ------------------------------------------
2019-09-12T01:31:04.8096841Z stderr:
2019-09-12T01:31:04.8096841Z stderr:
2019-09-12T01:31:04.8097057Z ------------------------------------------
2019-09-12T01:31:04.8097106Z error: linking with `emcc` failed: exit code: 1
2019-09-12T01:31:04.8097163Z    |
2019-09-12T01:31:04.8101303Z    = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.2.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.3lin0r3ymdvpj22v.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "-l" "rust_test_helpers" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_abort-a699a14fc732c57a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-s" "WASM=0"
2019-09-12T01:31:04.8104420Z    = note: wasm-ld: warning: function signature mismatch: lseek64
2019-09-12T01:31:04.8104850Z            >>> defined as (i32, i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-12T01:31:04.8105153Z            >>> defined as (i32, i64, i32) -> i64 in /home/user/.emscripten_cache/wasm-obj/libc.a(lseek.c.o)
2019-09-12T01:31:04.8105207Z            
2019-09-12T01:31:04.8105459Z            wasm-ld: warning: function signature mismatch: ftruncate64
2019-09-12T01:31:04.8105950Z            >>> defined as (i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-12T01:31:04.8106304Z            >>> defined as (i32, i64) -> i32 in /home/user/.emscripten_cache/wasm-obj/libc.a(ftruncate.c.o)
2019-09-12T01:31:04.8106355Z            
2019-09-12T01:31:04.8106588Z            wasm-ld: warning: function signature mismatch: pwrite64
2019-09-12T01:31:04.8106997Z            >>> defined as (i32, i32, i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-12T01:31:04.8107296Z            >>> defined as (i32, i32, i32, i64) -> i32 in /home/user/.emscripten_cache/wasm-obj/libc.a(pwrite.c.o)
2019-09-12T01:31:04.8107348Z            
2019-09-12T01:31:04.8107598Z            wasm-ld: warning: function signature mismatch: pread64
2019-09-12T01:31:04.8107999Z            >>> defined as (i32, i32, i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-12T01:31:04.8108542Z            >>> defined as (i32, i32, i32, i64) -> i32 in /home/user/.emscripten_cache/wasm-obj/libc.a(pread.c.o)
2019-09-12T01:31:04.8109015Z            wasm-ld: /b/s/w/ir/cache/builder/emscripten-releases/llvm-project/lld/wasm/Symbols.cpp:115: void lld::wasm::Symbol::setGOTIndex(uint32_t): Assertion `gotIndex == INVALID_INDEX' failed.
2019-09-12T01:31:04.8109569Z            Stack dump:
2019-09-12T01:31:04.8113765Z            0.    Program arguments: /emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp_FmzcAl/a.wasm --allow-undefined --import-memory --import-table --lto-O0 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.1.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.2.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.3lin0r3ymdvpj22v.rcgu.o /checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_abort-a699a14fc732c57a.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib /home/user/.emscripten_cache/wasm-obj/libc.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt.a /home/user/.emscripten_cache/wasm-obj/libc-wasm.a /home/user/.emscripten_cache/wasm-obj/libc-extras.a /home/user/.emscripten_cache/wasm-obj/libdlmalloc.a /home/user/.emscripten_cache/wasm-obj/libpthreads_stub.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a /home/user/.emscripten_cache/wasm-obj/libc_rt_wasm.a -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024 
2019-09-12T01:31:04.8116307Z             #0 0x00007f23dc4739f4 PrintStackTraceSignalHandler(void*) (/emsdk-portable/upstream/bin/../lib/libLLVM-10svn.so+0x6d79f4)
2019-09-12T01:31:04.8116851Z             #1 0x00007f23dc47173e llvm::sys::RunSignalHandlers() (/emsdk-portable/upstream/bin/../lib/libLLVM-10svn.so+0x6d573e)
2019-09-12T01:31:04.8117167Z             #2 0x00007f23dc473ca8 SignalHandler(int) (/emsdk-portable/upstream/bin/../lib/libLLVM-10svn.so+0x6d7ca8)
2019-09-12T01:31:04.8117448Z             #3 0x00007f23df345390 __restore_rt (/lib/x86_64-linux-gnu/libpthread.so.0+0x11390)
2019-09-12T01:31:04.8118045Z             #4 0x00007f23db166428 raise (/lib/x86_64-linux-gnu/libc.so.6+0x35428)
2019-09-12T01:31:04.8118384Z             #5 0x00007f23db16802a abort (/lib/x86_64-linux-gnu/libc.so.6+0x3702a)
2019-09-12T01:31:04.8118647Z             #6 0x00007f23db15ebd7 (/lib/x86_64-linux-gnu/libc.so.6+0x2dbd7)
2019-09-12T01:31:04.8118928Z             #7 0x00007f23db15ec82 (/lib/x86_64-linux-gnu/libc.so.6+0x2dc82)
2019-09-12T01:31:04.8119632Z             #8 0x00000000006ac4fb (/emsdk-portable/upstream/bin/wasm-ld+0x6ac4fb)
2019-09-12T01:31:04.8120025Z             #9 0x00000000006c691b lld::wasm::GlobalSection::assignIndexes() (/emsdk-portable/upstream/bin/wasm-ld+0x6c691b)
2019-09-12T01:31:04.8120358Z            #10 0x00000000006b1466 (anonymous namespace)::Writer::run() (/emsdk-portable/upstream/bin/wasm-ld+0x6b1466)
2019-09-12T01:31:04.8120646Z            #11 0x00000000006adaaf lld::wasm::writeResult() (/emsdk-portable/upstream/bin/wasm-ld+0x6adaaf)
2019-09-12T01:31:04.8120994Z            #12 0x0000000000690ae7 (anonymous namespace)::LinkerDriver::link(llvm::ArrayRef<char const*>) (/emsdk-portable/upstream/bin/wasm-ld+0x690ae7)
2019-09-12T01:31:04.8121334Z            #13 0x000000000068b6d8 lld::wasm::link(llvm::ArrayRef<char const*>, bool, llvm::raw_ostream&) (/emsdk-portable/upstream/bin/wasm-ld+0x68b6d8)
2019-09-12T01:31:04.8121611Z            #14 0x000000000041eadb main (/emsdk-portable/upstream/bin/wasm-ld+0x41eadb)
2019-09-12T01:31:04.8121908Z            #15 0x00007f23db151830 __libc_start_main (/lib/x86_64-linux-gnu/libc.so.6+0x20830)
2019-09-12T01:31:04.8122196Z            #16 0x000000000041e669 _start (/emsdk-portable/upstream/bin/wasm-ld+0x41e669)
2019-09-12T01:31:04.8126339Z            shared:ERROR: '/emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp_FmzcAl/a.wasm --allow-undefined --import-memory --import-table --lto-O0 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.1.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.2.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.3lin0r3ymdvpj22v.rcgu.o /checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_abort-a699a14fc732c57a.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib /home/user/.emscripten_cache/wasm-obj/libc.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt.a /home/user/.emscripten_cache/wasm-obj/libc-wasm.a /home/user/.emscripten_cache/wasm-obj/libc-extras.a /home/user/.emscripten_cache/wasm-obj/libdlmalloc.a /home/user/.emscripten_cache/wasm-obj/libpthreads_stub.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a /home/user/.emscripten_cache/wasm-obj/libc_rt_wasm.a -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024' failed (-6)
2019-09-12T01:31:04.8127090Z 
2019-09-12T01:31:04.8127133Z error: aborting due to previous error
2019-09-12T01:31:04.8127179Z 
2019-09-12T01:31:04.8127207Z 
2019-09-12T01:31:04.8127207Z 
2019-09-12T01:31:04.8127481Z ------------------------------------------
2019-09-12T01:31:04.8127514Z 
2019-09-12T01:31:04.8127539Z 
2019-09-12T01:31:04.8127791Z ---- [ui] ui/impl-trait/example-calendar.rs stdout ----
2019-09-12T01:31:04.8127826Z 
2019-09-12T01:31:04.8127871Z error: test run failed!
2019-09-12T01:31:04.8127930Z status: exit code: 1
2019-09-12T01:31:04.8128257Z command: "/emsdk-portable/node/12.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js"
2019-09-12T01:31:04.8128556Z ------------------------------------------
2019-09-12T01:31:04.8128602Z undefined
2019-09-12T01:31:04.8128629Z 
2019-09-12T01:31:04.8128842Z ------------------------------------------
2019-09-12T01:31:04.8128842Z ------------------------------------------
2019-09-12T01:31:04.8128900Z stderr:
2019-09-12T01:31:04.8129181Z ------------------------------------------
2019-09-12T01:31:04.8129434Z thread 'main' panicked at 'assertion failed: `(left == right)`
2019-09-12T01:31:04.8129502Z   left: `[((2012, 52), 7), ((2013, 1), 7)]`,
2019-09-12T01:31:04.8129788Z  right: `[((2013, 1), 6), ((2013, 2), 7)]`', /checkout/src/test/ui/impl-trait/example-calendar.rs:461:5
2019-09-12T01:31:04.8129908Z undefined
2019-09-12T01:31:04.8129908Z undefined
2019-09-12T01:31:04.8129953Z exception thrown: abort(undefined) at Error
2019-09-12T01:31:04.8130333Z     at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:23:15885)
2019-09-12T01:31:04.8130696Z     at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:23:16056)
2019-09-12T01:31:04.8130996Z     at Object.abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:23:130488)
2019-09-12T01:31:04.8131284Z     at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:23:90291)
2019-09-12T01:31:04.8131584Z     at Nd (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:9:3395)
2019-09-12T01:31:04.8131867Z     at be (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:9:11231)
2019-09-12T01:31:04.8132147Z     at ae (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:9:9935)
2019-09-12T01:31:04.8132455Z     at hc (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:5:132379)
2019-09-12T01:31:04.8132989Z     at Array.Ma (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:5:64312)
2019-09-12T01:31:04.8133639Z     at Array.bb (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a.js:5:90120)
2019-09-12T01:31:04.8133918Z ------------------------------------------
2019-09-12T01:31:04.8133949Z 
2019-09-12T01:31:04.8133973Z 
2019-09-12T01:31:04.8134197Z ---- [ui] ui/issues/issue-33992.rs stdout ----
2019-09-12T01:31:04.8134197Z ---- [ui] ui/issues/issue-33992.rs stdout ----
2019-09-12T01:31:04.8134227Z 
2019-09-12T01:31:04.8134437Z error: test compilation failed although it shouldn't!
2019-09-12T01:31:04.8134481Z status: signal: 6
2019-09-12T01:31:04.8135142Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-33992.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33992/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33992/auxiliary"
2019-09-12T01:31:04.8135449Z ------------------------------------------
2019-09-12T01:31:04.8135480Z 
2019-09-12T01:31:04.8135682Z ------------------------------------------
2019-09-12T01:31:04.8135739Z stderr:
---
2019-09-12T01:31:04.8136762Z ---- [ui] ui/iterators/iter-position-overflow-debug.rs stdout ----
2019-09-12T01:31:04.8136808Z 
2019-09-12T01:31:04.8136846Z error: test run failed!
2019-09-12T01:31:04.8136901Z status: exit code: 1
2019-09-12T01:31:04.8137204Z command: "/emsdk-portable/node/12.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js"
2019-09-12T01:31:04.8137476Z ------------------------------------------
2019-09-12T01:31:04.8137519Z undefined
2019-09-12T01:31:04.8137543Z 
2019-09-12T01:31:04.8137737Z ------------------------------------------
2019-09-12T01:31:04.8137737Z ------------------------------------------
2019-09-12T01:31:04.8137792Z stderr:
2019-09-12T01:31:04.8137989Z ------------------------------------------
2019-09-12T01:31:04.8138243Z thread 'main' panicked at 'attempt to add with overflow', /checkout/src/libcore/ops/arith.rs:100:45
2019-09-12T01:31:04.8138355Z undefined
2019-09-12T01:31:04.8138355Z undefined
2019-09-12T01:31:04.8138394Z exception thrown: abort(undefined) at Error
2019-09-12T01:31:04.8138806Z     at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:19:14633)
2019-09-12T01:31:04.8139171Z     at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:19:14804)
2019-09-12T01:31:04.8139625Z     at Object.abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:19:127901)
2019-09-12T01:31:04.8139947Z     at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:19:89039)
2019-09-12T01:31:04.8140243Z     at Nb (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:5:74266)
2019-09-12T01:31:04.8140552Z     at _a (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:5:48949)
2019-09-12T01:31:04.8140855Z     at Kb (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:5:72877)
2019-09-12T01:31:04.8141273Z     at ra (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:5:9595)
2019-09-12T01:31:04.8141583Z     at ea (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:5:271)
2019-09-12T01:31:04.8141885Z     at Array.je (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-position-overflow-debug/a.js:5:130833)
2019-09-12T01:31:04.8142151Z ------------------------------------------
2019-09-12T01:31:04.8142183Z 
2019-09-12T01:31:04.8142208Z 
2019-09-12T01:31:04.8142440Z ---- [ui] ui/iterators/iter-count-overflow-debug.rs stdout ----
2019-09-12T01:31:04.8142440Z ---- [ui] ui/iterators/iter-count-overflow-debug.rs stdout ----
2019-09-12T01:31:04.8142473Z 
2019-09-12T01:31:04.8142529Z error: test run failed!
2019-09-12T01:31:04.8142570Z status: exit code: 1
2019-09-12T01:31:04.8143134Z command: "/emsdk-portable/node/12.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js"
2019-09-12T01:31:04.8143410Z ------------------------------------------
2019-09-12T01:31:04.8143451Z undefined
2019-09-12T01:31:04.8143476Z 
2019-09-12T01:31:04.8143688Z ------------------------------------------
2019-09-12T01:31:04.8143688Z ------------------------------------------
2019-09-12T01:31:04.8143730Z stderr:
2019-09-12T01:31:04.8143925Z ------------------------------------------
2019-09-12T01:31:04.8144285Z thread 'main' panicked at 'attempt to add with overflow', /checkout/src/libcore/ops/arith.rs:100:45
2019-09-12T01:31:04.8144376Z undefined
2019-09-12T01:31:04.8144376Z undefined
2019-09-12T01:31:04.8144429Z exception thrown: abort(undefined) at Error
2019-09-12T01:31:04.8144692Z     at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:19:14633)
2019-09-12T01:31:04.8144953Z     at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:19:14804)
2019-09-12T01:31:04.8145246Z     at Object.abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:19:127901)
2019-09-12T01:31:04.8145508Z     at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:19:89039)
2019-09-12T01:31:04.8145761Z     at Ob (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:5:75244)
2019-09-12T01:31:04.8146029Z     at Ya (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:5:48734)
2019-09-12T01:31:04.8146281Z     at Jb (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:5:73656)
2019-09-12T01:31:04.8146535Z     at ra (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:5:9595)
2019-09-12T01:31:04.8146802Z     at ea (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:5:271)
2019-09-12T01:31:04.8147144Z     at Array.Mc (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/iter-count-overflow-debug/a.js:5:99356)
2019-09-12T01:31:04.8147415Z ------------------------------------------
2019-09-12T01:31:04.8147443Z 
2019-09-12T01:31:04.8147467Z 
2019-09-12T01:31:04.8147489Z 
---
2019-09-12T01:31:04.8180336Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-12T01:31:04.8180956Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-12T01:31:04.8181132Z 
2019-09-12T01:31:04.8181296Z 
2019-09-12T01:31:04.8183080Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "ui" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-12T01:31:04.8183635Z 
2019-09-12T01:31:04.8183761Z 
2019-09-12T01:31:04.8184197Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten
2019-09-12T01:31:04.8184412Z Build completed unsuccessfully in 4:49:13
2019-09-12T01:31:04.8184412Z Build completed unsuccessfully in 4:49:13
2019-09-12T01:31:04.8212144Z == clock drift check ==
2019-09-12T01:31:04.8225120Z   local time: Thu Sep 12 01:31:04 UTC 2019
2019-09-12T01:31:04.9583540Z   network time: Thu, 12 Sep 2019 01:31:04 GMT
2019-09-12T01:31:04.9588659Z == end clock drift check ==
2019-09-12T01:31:05.7288719Z ##[error]Bash exited with code '1'.
2019-09-12T01:31:05.7339305Z ##[section]Starting: Checkout
2019-09-12T01:31:05.7341163Z ==============================================================================
2019-09-12T01:31:05.7341243Z Task         : Get sources
2019-09-12T01:31:05.7341296Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
