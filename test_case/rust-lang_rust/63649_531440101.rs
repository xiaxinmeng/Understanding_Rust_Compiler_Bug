plain
2019-09-13T22:41:43.4319121Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-13T22:41:44.2640983Z ##[command]git config gc.auto 0
2019-09-13T22:41:44.2644410Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-13T22:41:44.2646178Z ##[command]git config --get-all http.proxy
2019-09-13T22:41:44.2648744Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63649/merge:refs/remotes/pull/63649/merge
---
2019-09-13T22:46:14.0107747Z  ---> 225969a551cc
2019-09-13T22:46:14.0107811Z Step 3/13 : COPY scripts/emscripten.sh /scripts/
2019-09-13T22:46:15.5183525Z  ---> 34a65d8998ea
2019-09-13T22:46:15.5223551Z Step 4/13 : RUN bash /scripts/emscripten.sh
2019-09-13T22:46:15.7116833Z  ---> Running in 23270edeb8b6
2019-09-13T22:46:16.2088472Z + git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
2019-09-13T22:46:16.2149611Z Cloning into '/emsdk-portable'...
2019-09-13T22:46:16.9318891Z + cd /emsdk-portable
2019-09-13T22:46:16.9319343Z + hide_output ./emsdk install 1.38.45-upstream
2019-09-13T22:46:46.9376129Z Fri Sep 13 22:46:46 UTC 2019 - building ...
2019-09-13T22:46:53.6268045Z /scripts/emscripten.sh: line 3:    17 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-13T22:46:53.6268045Z /scripts/emscripten.sh: line 3:    17 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-13T22:46:53.6268527Z + ./emsdk activate 1.38.45-upstream
2019-09-13T22:46:53.9734998Z Writing .emscripten configuration file to user home directory /root/
2019-09-13T22:46:53.9735175Z The Emscripten configuration file /root/.emscripten has been rewritten with the following contents:
2019-09-13T22:46:53.9735276Z import os
2019-09-13T22:46:53.9735276Z import os
2019-09-13T22:46:53.9735500Z LLVM_ROOT = '/emsdk-portable/upstream/bin'
2019-09-13T22:46:53.9735685Z BINARYEN_ROOT = '/emsdk-portable/upstream'
2019-09-13T22:46:53.9735900Z EMSCRIPTEN_ROOT = '/emsdk-portable/upstream/emscripten'
2019-09-13T22:46:53.9736097Z NODE_JS = '/emsdk-portable/node/12.9.1_64bit/bin/node'
2019-09-13T22:46:53.9736267Z SPIDERMONKEY_ENGINE = ''
2019-09-13T22:46:53.9736447Z V8_ENGINE = ''
2019-09-13T22:46:53.9736609Z TEMP_DIR = '/tmp'
2019-09-13T22:46:53.9736652Z COMPILER_ENGINE = NODE_JS
2019-09-13T22:46:53.9736711Z JS_ENGINES = [NODE_JS]
2019-09-13T22:46:53.9736738Z 
2019-09-13T22:46:53.9737038Z To conveniently access the selected set of tools from the command line, consider adding the following directories to PATH, or call 'source ./emsdk_env.sh' to do this for you.
2019-09-13T22:46:53.9737085Z 
2019-09-13T22:46:53.9738363Z    /emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin
2019-09-13T22:46:53.9738460Z Set the following tools as active:
2019-09-13T22:46:53.9738756Z    releases-upstream-f3030d9fffcc9e1287cb6b8e72982e94ece31d71-64bit
2019-09-13T22:46:53.9738965Z    node-12.9.1-64bit
2019-09-13T22:46:53.9738998Z 
2019-09-13T22:46:53.9738998Z 
2019-09-13T22:46:53.9812218Z + source ./emsdk_env.sh
2019-09-13T22:46:53.9812857Z ++ SRC=./emsdk_env.sh
2019-09-13T22:46:53.9813084Z ++ '[' ./emsdk_env.sh = '' ']'
2019-09-13T22:46:53.9816606Z +++ pwd
2019-09-13T22:46:53.9820363Z ++ CURDIR=/emsdk-portable
2019-09-13T22:46:53.9824681Z +++ dirname ./emsdk_env.sh
2019-09-13T22:46:53.9846559Z ++ unset SRC
2019-09-13T22:46:53.9846731Z +++ mktemp
2019-09-13T22:46:53.9846731Z +++ mktemp
2019-09-13T22:46:53.9861071Z ++ tmpfile=/tmp/tmp.tBiD1gQMCd
2019-09-13T22:46:53.9861196Z ++ ./emsdk construct_env /tmp/tmp.tBiD1gQMCd
2019-09-13T22:46:54.1499450Z PATH += /emsdk-portable
2019-09-13T22:46:54.1499450Z PATH += /emsdk-portable
2019-09-13T22:46:54.1499695Z PATH += /emsdk-portable/upstream/emscripten
2019-09-13T22:46:54.1499965Z 
2019-09-13T22:46:54.1500038Z Setting environment variables:
2019-09-13T22:46:54.1500243Z EMSDK = /emsdk-portable
2019-09-13T22:46:54.1500296Z EM_CONFIG = /root/.emscripten
2019-09-13T22:46:54.1500296Z EM_CONFIG = /root/.emscripten
2019-09-13T22:46:54.1500559Z EMSDK_NODE = /emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-13T22:46:54.1500596Z 
2019-09-13T22:46:54.1559272Z ++ . /tmp/tmp.tBiD1gQMCd
2019-09-13T22:46:54.1560036Z +++ export PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-13T22:46:54.1560502Z +++ PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-13T22:46:54.1560919Z +++ export EMSDK=/emsdk-portable
2019-09-13T22:46:54.1566757Z +++ EMSDK=/emsdk-portable
2019-09-13T22:46:54.1566872Z +++ export EM_CONFIG=/root/.emscripten
2019-09-13T22:46:54.1566918Z +++ EM_CONFIG=/root/.emscripten
2019-09-13T22:46:54.1567197Z +++ export EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-13T22:46:54.1567954Z +++ EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-13T22:46:54.1568188Z ++ rm -f /tmp/tmp.tBiD1gQMCd
2019-09-13T22:46:54.1666652Z ++ cd /emsdk-portable
2019-09-13T22:46:54.1666876Z + echo 'main(){}'
2019-09-13T22:46:54.1667344Z + HOME=/emsdk-portable/
2019-09-13T22:46:54.1667417Z + emcc a.c
2019-09-13T22:46:54.2192963Z cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/emsdk-portable/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-13T22:46:54.2304340Z cache:INFO:  - ok
2019-09-13T22:46:54.3062893Z a.c:1:1: warning: type specifier missing, defaults to 'int' [-Wimplicit-int]
2019-09-13T22:46:54.3063074Z main(){}
2019-09-13T22:46:54.3094157Z 1 warning generated.
2019-09-13T22:46:54.3094157Z 1 warning generated.
2019-09-13T22:46:54.3320755Z cache:INFO: generating system library: libc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc.a" for subsequent builds)
2019-09-13T22:48:04.3090485Z cache:INFO:  - ok
2019-09-13T22:48:04.3095573Z cache:INFO: generating system library: libcompiler_rt.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt.a" for subsequent builds)
2019-09-13T22:48:04.7301886Z cache:INFO:  - ok
2019-09-13T22:48:04.7302627Z cache:INFO: generating system library: libc-wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc-wasm.a" for subsequent builds)
2019-09-13T22:48:07.4612567Z cache:INFO:  - ok
2019-09-13T22:48:07.4632296Z cache:INFO: generating system library: libdlmalloc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libdlmalloc.a" for subsequent builds)
2019-09-13T22:48:08.5611730Z cache:INFO:  - ok
2019-09-13T22:48:08.5620794Z cache:INFO: generating system library: libpthreads_stub.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libpthreads_stub.a" for subsequent builds)
2019-09-13T22:48:08.8651694Z cache:INFO:  - ok
2019-09-13T22:48:08.8657350Z cache:INFO: generating system library: libcompiler_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a" for subsequent builds)
2019-09-13T22:48:13.4533550Z cache:INFO:  - ok
2019-09-13T22:48:13.4534029Z cache:INFO: generating system library: libc_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc_rt_wasm.a" for subsequent builds)
2019-09-13T22:48:15.7374572Z cache:INFO:  - ok
2019-09-13T22:48:15.7646087Z cache:INFO: generating system asset: generated_struct_info.json... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/generated_struct_info.json" for subsequent builds)
2019-09-13T22:48:17.6650720Z cache:INFO:  - ok
2019-09-13T22:48:17.9661485Z + rm -f a.c a.out.js a.out.wasm
2019-09-13T22:48:17.9667821Z + cp /root/.emscripten /emsdk-portable
2019-09-13T22:48:17.9682976Z + chmod a+rxw -R /emsdk-portable
2019-09-13T22:48:43.0934618Z Removing intermediate container 23270edeb8b6
2019-09-13T22:48:43.0935772Z  ---> 8e51d9eb0e9a
2019-09-13T22:48:44.8952980Z  ---> 241886181758
2019-09-13T22:48:44.8953442Z Step 6/13 : RUN sh /scripts/sccache.sh
2019-09-13T22:48:45.0655131Z  ---> Running in fc613a549a33
2019-09-13T22:48:45.4834688Z + curl -fo /usr/local/bin/sccache https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/2018-04-02-sccache-x86_64-unknown-linux-musl
---
2019-09-13T22:48:49.3034241Z Step 7/13 : ENV PATH=$PATH:/emsdk-portable
2019-09-13T22:48:49.7528746Z  ---> Running in 76ec8dfe0325
2019-09-13T22:48:51.2943797Z Removing intermediate container 76ec8dfe0325
2019-09-13T22:48:51.2944620Z  ---> 9a4fcafedb91
2019-09-13T22:48:51.2944918Z Step 8/13 : ENV PATH=$PATH:/emsdk-portable/upstream/emscripten/
2019-09-13T22:48:51.5997457Z  ---> Running in ac9ac5a4b2e7
2019-09-13T22:48:53.0048707Z Removing intermediate container ac9ac5a4b2e7
2019-09-13T22:48:53.0051265Z Step 9/13 : ENV PATH=$PATH:/emsdk-portable/node/12.9.1_64bit/bin/
2019-09-13T22:48:53.2233524Z  ---> Running in bf95f347ad38
2019-09-13T22:48:54.5927273Z Removing intermediate container bf95f347ad38
2019-09-13T22:48:54.5929248Z  ---> a3355047feab
---
2019-09-13T22:49:00.9416768Z  ---> fad64a385cb1
2019-09-13T22:49:00.9491480Z Successfully built fad64a385cb1
2019-09-13T22:49:01.0247003Z Successfully tagged rust-ci:latest
2019-09-13T22:49:01.0970354Z Built container sha256:fad64a385cb18126f9924e33c9bffd7acf9e53e83598fa370411301507552237
2019-09-13T22:49:01.0987693Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/7bfc17d02707b3e62f122fb47893a47314e94fe81f2628b66d98ea5f533d9a55e0e9e946bf7fd5e02d1dc5f983a79ed61bc451b8bbf8e8ba70dd60789e6c2358
2019-09-13T22:50:32.5563145Z upload failed: - to s3://rust-lang-ci-sccache2/docker/7bfc17d02707b3e62f122fb47893a47314e94fe81f2628b66d98ea5f533d9a55e0e9e946bf7fd5e02d1dc5f983a79ed61bc451b8bbf8e8ba70dd60789e6c2358 Unable to locate credentials
2019-09-13T22:50:33.3512554Z [CI_JOB_NAME=wasm32]
2019-09-13T22:50:33.5628135Z [CI_JOB_NAME=wasm32]
2019-09-13T22:50:33.5672875Z   local time: Fri Sep 13 22:50:33 UTC 2019
2019-09-13T22:50:33.8478435Z   network time: Fri, 13 Sep 2019 22:50:33 GMT
2019-09-13T22:50:33.8482394Z == end clock drift check ==
2019-09-13T22:50:33.8506914Z Starting sccache server...
---
2019-09-13T22:50:33.9428769Z configure: build.locked-deps    := True
2019-09-13T22:50:33.9428817Z configure: llvm.ccache          := sccache
2019-09-13T22:50:33.9429012Z configure: build.cargo-native-static := True
2019-09-13T22:50:33.9429396Z configure: dist.missing-tools   := True
2019-09-13T22:50:33.9429619Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-13T22:50:33.9429728Z configure: writing `config.toml` in current directory
2019-09-13T22:50:33.9429770Z configure: 
2019-09-13T22:50:33.9430014Z configure: run `python /checkout/x.py --help`
2019-09-13T22:50:33.9430087Z configure: 
---
2019-09-14T00:23:37.6321132Z exit code: 0
2019-09-14T00:23:37.6323197Z running: "ar" "crs" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/rust_test_helpers.o"
2019-09-14T00:23:37.6342308Z exit code: 0
2019-09-14T00:23:37.6346272Z Building test helpers
2019-09-14T00:23:37.6354048Z running: "emcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-o" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
2019-09-14T00:23:37.7385451Z cargo:warning=cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/home/user/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-14T00:23:37.7535983Z cargo:warning=cache:INFO:  - ok
2019-09-14T00:23:37.8911050Z cargo:warning=shared:INFO: (Emscripten: Running sanity checks)
2019-09-14T00:23:37.8962899Z cargo:warning=shared:WARNING: java does not seem to exist, required for closure compiler, which is optional (define JAVA in /emsdk-portable/.emscripten if you want it)
2019-09-14T00:23:37.8963032Z cargo:warning=shared:WARNING: closure compiler will not be available
2019-09-14T00:23:37.9812571Z running: "emar" "crs" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o"
2019-09-14T00:23:38.0458364Z exit code: 0
2019-09-14T00:23:38.0472954Z Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
2019-09-14T00:23:38.3403679Z    Compiling proc-macro2 v0.4.30
---
2019-09-14T00:25:50.6269834Z 
2019-09-14T00:25:50.6270805Z running 9012 tests
2019-09-14T00:26:50.6293439Z test [ui] ui/abi/abi-sysv64-arg-passing.rs has been running for over 60 seconds
2019-09-14T00:26:50.6294611Z test [ui] ui/abi/abi-sysv64-register-usage.rs has been running for over 60 seconds
2019-09-14T00:28:41.3626088Z .i.....ii..i.......i......i......i.iiiiiF....................................ii...............i..... 100/9012
2019-09-14T00:29:58.9422066Z .ii.........i.......................i....iiiiii..................................................... 200/9012
2019-09-14T00:32:07.0799409Z .............................................i...................................................... 400/9012
2019-09-14T00:32:07.0799409Z .............................................i...................................................... 400/9012
2019-09-14T00:32:56.3566931Z ...............................i..............................................iii................... 500/9012
2019-09-14T00:35:27.1181140Z .......................................................i............................................ 700/9012
2019-09-14T00:35:48.7859873Z .................................................................................................... 800/9012
2019-09-14T00:36:12.4731983Z .............................................................i...................................... 900/9012
2019-09-14T00:37:20.1842809Z ...i............................................ii.........................................i........ 1000/9012
---
2019-09-14T00:41:06.4156904Z .................................................................................................... 1500/9012
2019-09-14T00:42:04.4538778Z ....................ii.i............................................................................ 1600/9012
2019-09-14T00:43:11.3719053Z ................................i......i..................i...............i.....ii.................. 1700/9012
2019-09-14T00:44:08.2328896Z .................................................................................................... 1800/9012
2019-09-14T00:44:51.2861863Z .................................................iiiii.......................i.........ii........... 1900/9012
2019-09-14T00:45:43.6289515Z .................................................................................................... 2000/9012
2019-09-14T00:45:52.6688927Z .......iii.......................................................................................... 2100/9012
2019-09-14T00:45:55.8784894Z .................................................................................................... 2200/9012
2019-09-14T00:46:09.7621059Z ......................................................iiii......................i................... 2300/9012
2019-09-14T00:47:05.9327118Z .........................................................................i.......................... 2400/9012
2019-09-14T00:47:10.7806541Z .i..........ii...................................................................................... 2500/9012
2019-09-14T00:48:21.9120915Z .................................................................................................... 2600/9012
2019-09-14T00:49:42.1023740Z i....i......................................................................ii.....ii.i............. 2700/9012
2019-09-14T00:51:14.1598158Z ..i................................................................................................. 2900/9012
2019-09-14T00:52:04.6231172Z .................................................................................................... 3000/9012
2019-09-14T00:52:47.2078347Z ...................................................................................................i 3100/9012
2019-09-14T00:52:47.2078347Z ...................................................................................................i 3100/9012
2019-09-14T00:53:44.8000173Z i..ii.................i....i............i....i.................i.................................... 3200/9012
2019-09-14T00:56:03.7711590Z .............i.............i.....i.........................................i.......i........i....... 3400/9012
2019-09-14T00:57:01.2529529Z .....................i.............................................................................. 3500/9012
2019-09-14T00:58:08.3860227Z ....................................i............................................................... 3600/9012
2019-09-14T00:59:10.4649324Z ..i...........................................................i..............................i...... 3700/9012
---
2019-09-14T01:09:09.6085594Z ....................................i...............i............................................... 4700/9012
2019-09-14T01:10:17.9743114Z ...........................................................................i........................ 4800/9012
2019-09-14T01:11:18.6604220Z ........................i........................................................................... 4900/9012
2019-09-14T01:12:30.3073218Z ....................................................................i...........i..............i.... 5000/9012
2019-09-14T01:13:40.1178438Z ...................i..i..i.i.i...................................................................... 5100/9012
2019-09-14T01:14:40.7582832Z ............................................................................................i....... 5300/9012
2019-09-14T01:14:40.7582832Z ............................................................................................i....... 5300/9012
2019-09-14T01:15:40.3791897Z ........i......................................................ii..................ii............... 5400/9012
2019-09-14T01:16:47.0810512Z .........................................................i...i...................................... 5500/9012
2019-09-14T01:17:24.2402101Z .................................................................................................... 5600/9012
2019-09-14T01:18:21.9880773Z ......i............i...i......................................................ii...i..ii...........i 5700/9012
2019-09-14T01:19:25.8674118Z ...........................i........i.i............................................................. 5800/9012
2019-09-14T01:20:29.0490106Z .................................................................................................... 6000/9012
2019-09-14T01:20:29.0490106Z .................................................................................................... 6000/9012
2019-09-14T01:20:49.8276991Z ...............................................i...........i....................i..ii............... 6100/9012
2019-09-14T01:22:06.2478775Z .................i.....ii........................................................................... 6200/9012
2019-09-14T01:23:07.0382649Z ......................i...............................i.....................iii..i.ii.iiii.iiiii.... 6300/9012
2019-09-14T01:23:18.2404047Z .................................................................................................... 6500/9012
2019-09-14T01:23:21.1620050Z ...........i........................................................................................ 6600/9012
2019-09-14T01:23:48.8649907Z .................................i............................i..................................... 6700/9012
2019-09-14T01:24:51.7888675Z ............................................................................i....................... 6800/9012
2019-09-14T01:24:51.7888675Z ............................................................................i....................... 6800/9012
2019-09-14T01:25:45.6283490Z .............i..........................iiiiiiii..............................................i..... 6900/9012
2019-09-14T01:27:24.2766274Z .................................................................................................... 7100/9012
2019-09-14T01:27:47.4144235Z .................................................................................................... 7200/9012
2019-09-14T01:28:12.4178638Z ....................i............................................................................... 7300/9012
2019-09-14T01:29:29.5501698Z ..........................................................................i.......i................. 7400/9012
2019-09-14T01:29:29.5501698Z ..........................................................................i.......i................. 7400/9012
2019-09-14T01:30:20.4011085Z .................................................................................................... 7500/9012
2019-09-14T01:31:11.2571607Z ................................i.............iii.....i....iiiiiiiiiii.i.........................i.. 7600/9012
2019-09-14T01:32:21.1670050Z ................................................................................i.i................. 7800/9012
2019-09-14T01:34:26.3324551Z ....................................................i............................................... 7900/9012
2019-09-14T01:35:04.4383328Z ..................i................................................................................. 8000/9012
2019-09-14T01:35:04.4383328Z ..................i................................................................................. 8000/9012
2019-09-14T01:36:11.2729780Z .................................................iii.....i.....i.........i........ii....i.i.....iiii 8100/9012
2019-09-14T01:37:25.0324062Z ii.....iiiiiiii.ii...ii.iii..iiii............................................................i...... 8200/9012
2019-09-14T01:39:45.2107965Z ...........................................................................i........................ 8400/9012
2019-09-14T01:40:17.2918768Z .................................................................................................... 8500/9012
2019-09-14T01:41:05.0201488Z .................................................................................................... 8600/9012
2019-09-14T01:42:03.5439901Z .................................................................................................... 8700/9012
---
2019-09-14T01:44:38.6031228Z ---- [ui] ui/abi/statics/static-mut-foreign.rs stdout ----
2019-09-14T01:44:38.6031315Z 
2019-09-14T01:44:38.6031592Z error: test compilation failed although it shouldn't!
2019-09-14T01:44:38.6031916Z status: exit code: 1
2019-09-14T01:44:38.6032808Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/statics/static-mut-foreign.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/auxiliary"
2019-09-14T01:44:38.6033157Z ------------------------------------------
2019-09-14T01:44:38.6033190Z 
2019-09-14T01:44:38.6033391Z ------------------------------------------
2019-09-14T01:44:38.6033435Z stderr:
2019-09-14T01:44:38.6033435Z stderr:
2019-09-14T01:44:38.6033806Z ------------------------------------------
2019-09-14T01:44:38.6033852Z error: linking with `emcc` failed: exit code: 1
2019-09-14T01:44:38.6033900Z    |
2019-09-14T01:44:38.6037841Z    = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.2.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.3lin0r3ymdvpj22v.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_abort-a699a14fc732c57a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "DISABLE_EXCEPTION_CATCHING=1"
2019-09-14T01:44:38.6039282Z    = note: wasm-ld: warning: function signature mismatch: lseek64
2019-09-14T01:44:38.6039662Z            >>> defined as (i32, i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-14T01:44:38.6039954Z            >>> defined as (i32, i64, i32) -> i64 in /home/user/.emscripten_cache/wasm-obj/libc.a(lseek.c.o)
2019-09-14T01:44:38.6040004Z            
2019-09-14T01:44:38.6040208Z            wasm-ld: warning: function signature mismatch: ftruncate64
2019-09-14T01:44:38.6040559Z            >>> defined as (i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-14T01:44:38.6040804Z            >>> defined as (i32, i64) -> i32 in /home/user/.emscripten_cache/wasm-obj/libc.a(ftruncate.c.o)
2019-09-14T01:44:38.6040855Z            
2019-09-14T01:44:38.6041075Z            wasm-ld: warning: function signature mismatch: pwrite64
2019-09-14T01:44:38.6041594Z            >>> defined as (i32, i32, i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-14T01:44:38.6041891Z            >>> defined as (i32, i32, i32, i64) -> i32 in /home/user/.emscripten_cache/wasm-obj/libc.a(pwrite.c.o)
2019-09-14T01:44:38.6041940Z            
2019-09-14T01:44:38.6042328Z            wasm-ld: warning: function signature mismatch: pread64
2019-09-14T01:44:38.6042705Z            >>> defined as (i32, i32, i32, i32) -> i32 in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib(std-4e65788bae25c659.std.l78fir96-cgu.0.rcgu.o)
2019-09-14T01:44:38.6043134Z            >>> defined as (i32, i32, i32, i64) -> i32 in /home/user/.emscripten_cache/wasm-obj/libc.a(pread.c.o)
2019-09-14T01:44:38.6043472Z            wasm-ld: /b/s/w/ir/cache/builder/emscripten-releases/llvm-project/lld/wasm/Symbols.cpp:116: void lld::wasm::Symbol::setGOTIndex(uint32_t): Assertion `gotIndex == INVALID_INDEX' failed.
2019-09-14T01:44:38.6043533Z            Stack dump:
2019-09-14T01:44:38.6047925Z            0.    Program arguments: /emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp_GPKN1R/a.wasm --allow-undefined --import-memory --import-table --lto-O0 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.1.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.2.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.3lin0r3ymdvpj22v.rcgu.o /checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_abort-a699a14fc732c57a.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib /home/user/.emscripten_cache/wasm-obj/libc.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt.a /home/user/.emscripten_cache/wasm-obj/libc-wasm.a /home/user/.emscripten_cache/wasm-obj/libc-extras.a /home/user/.emscripten_cache/wasm-obj/libdlmalloc.a /home/user/.emscripten_cache/wasm-obj/libpthreads_stub.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a /home/user/.emscripten_cache/wasm-obj/libc_rt_wasm.a -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024 
2019-09-14T01:44:38.6049204Z             #0 0x00007fc808e37ec4 PrintStackTraceSignalHandler(void*) (/emsdk-portable/upstream/bin/../lib/libLLVM-10svn.so+0x6f6ec4)
2019-09-14T01:44:38.6049643Z             #1 0x00007fc808e35c4e llvm::sys::RunSignalHandlers() (/emsdk-portable/upstream/bin/../lib/libLLVM-10svn.so+0x6f4c4e)
2019-09-14T01:44:38.6050171Z             #2 0x00007fc808e38178 SignalHandler(int) (/emsdk-portable/upstream/bin/../lib/libLLVM-10svn.so+0x6f7178)
2019-09-14T01:44:38.6050424Z             #3 0x00007fc80bd94390 __restore_rt (/lib/x86_64-linux-gnu/libpthread.so.0+0x11390)
2019-09-14T01:44:38.6050670Z             #4 0x00007fc807b0b428 raise (/lib/x86_64-linux-gnu/libc.so.6+0x35428)
2019-09-14T01:44:38.6050935Z             #5 0x00007fc807b0d02a abort (/lib/x86_64-linux-gnu/libc.so.6+0x3702a)
2019-09-14T01:44:38.6051165Z             #6 0x00007fc807b03bd7 (/lib/x86_64-linux-gnu/libc.so.6+0x2dbd7)
2019-09-14T01:44:38.6051547Z             #7 0x00007fc807b03c82 (/lib/x86_64-linux-gnu/libc.so.6+0x2dc82)
2019-09-14T01:44:38.6051949Z             #8 0x00000000006babcb (/emsdk-portable/upstream/bin/wasm-ld+0x6babcb)
2019-09-14T01:44:38.6052204Z             #9 0x00000000006d675b lld::wasm::GlobalSection::assignIndexes() (/emsdk-portable/upstream/bin/wasm-ld+0x6d675b)
2019-09-14T01:44:38.6052453Z            #10 0x00000000006c04b6 (anonymous namespace)::Writer::run() (/emsdk-portable/upstream/bin/wasm-ld+0x6c04b6)
2019-09-14T01:44:38.6052778Z            #11 0x00000000006bc191 lld::wasm::writeResult() (/emsdk-portable/upstream/bin/wasm-ld+0x6bc191)
2019-09-14T01:44:38.6053057Z            #12 0x000000000069effe (anonymous namespace)::LinkerDriver::link(llvm::ArrayRef<char const*>) (/emsdk-portable/upstream/bin/wasm-ld+0x69effe)
2019-09-14T01:44:38.6053465Z            #13 0x0000000000699b28 lld::wasm::link(llvm::ArrayRef<char const*>, bool, llvm::raw_ostream&) (/emsdk-portable/upstream/bin/wasm-ld+0x699b28)
2019-09-14T01:44:38.6053694Z            #14 0x000000000041eefb main (/emsdk-portable/upstream/bin/wasm-ld+0x41eefb)
2019-09-14T01:44:38.6053918Z            #15 0x00007fc807af6830 __libc_start_main (/lib/x86_64-linux-gnu/libc.so.6+0x20830)
2019-09-14T01:44:38.6054159Z            #16 0x000000000041ea89 _start (/emsdk-portable/upstream/bin/wasm-ld+0x41ea89)
2019-09-14T01:44:38.6058140Z            shared:ERROR: '/emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp_GPKN1R/a.wasm --allow-undefined --import-memory --import-table --lto-O0 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.1.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.static_mut_foreign.7rcbfp3g-cgu.2.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/statics/static-mut-foreign/a.3lin0r3ymdvpj22v.rcgu.o /checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-4e65788bae25c659.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_abort-a699a14fc732c57a.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib /home/user/.emscripten_cache/wasm-obj/libc.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt.a /home/user/.emscripten_cache/wasm-obj/libc-wasm.a /home/user/.emscripten_cache/wasm-obj/libc-extras.a /home/user/.emscripten_cache/wasm-obj/libdlmalloc.a /home/user/.emscripten_cache/wasm-obj/libpthreads_stub.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a /home/user/.emscripten_cache/wasm-obj/libc_rt_wasm.a -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024' failed (-6)
2019-09-14T01:44:38.6059074Z 
2019-09-14T01:44:38.6059118Z error: aborting due to previous error
2019-09-14T01:44:38.6059371Z 
2019-09-14T01:44:38.6059419Z 
---
2019-09-14T01:44:38.6101534Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-14T01:44:38.6101614Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-14T01:44:38.6126493Z 
2019-09-14T01:44:38.6126608Z 
2019-09-14T01:44:38.6132329Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "ui" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-14T01:44:38.6132679Z 
2019-09-14T01:44:38.6132714Z 
2019-09-14T01:44:38.6193723Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten
2019-09-14T01:44:38.6193843Z Build completed unsuccessfully in 2:51:37
2019-09-14T01:44:38.6193843Z Build completed unsuccessfully in 2:51:37
2019-09-14T01:44:38.6200986Z == clock drift check ==
2019-09-14T01:44:38.6214169Z   local time: Sat Sep 14 01:44:38 UTC 2019
2019-09-14T01:44:38.9048457Z   network time: Sat, 14 Sep 2019 01:44:38 GMT
2019-09-14T01:44:38.9051439Z == end clock drift check ==
2019-09-14T01:44:39.5552445Z ##[error]Bash exited with code '1'.
2019-09-14T01:44:39.5591223Z ##[section]Starting: Checkout
2019-09-14T01:44:39.5593016Z ==============================================================================
2019-09-14T01:44:39.5593105Z Task         : Get sources
2019-09-14T01:44:39.5593152Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
