plain
2019-09-20T17:37:32.5299224Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T17:37:32.5504982Z ##[command]git config gc.auto 0
2019-09-20T17:37:32.5577016Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T17:37:32.5635417Z ##[command]git config --get-all http.proxy
2019-09-20T17:37:32.5789943Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63649/merge:refs/remotes/pull/63649/merge
---
2019-09-20T17:42:26.6086259Z  ---> a0b394aa6827
2019-09-20T17:42:26.6086331Z Step 3/15 : COPY scripts/emscripten.sh /scripts/
2019-09-20T17:42:28.3468327Z  ---> 0c99687a38c0
2019-09-20T17:42:28.3468526Z Step 4/15 : RUN bash /scripts/emscripten.sh
2019-09-20T17:42:28.5059040Z  ---> Running in 81188b4bc613
2019-09-20T17:42:29.0744974Z + git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
2019-09-20T17:42:29.0800534Z Cloning into '/emsdk-portable'...
2019-09-20T17:42:29.9515898Z + cd /emsdk-portable
2019-09-20T17:42:29.9516272Z + hide_output ./emsdk install 1.38.45-upstream
2019-09-20T17:42:59.0690515Z /scripts/emscripten.sh: line 3:    16 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-20T17:42:59.0690515Z /scripts/emscripten.sh: line 3:    16 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-20T17:42:59.0691623Z + ./emsdk activate 1.38.45-upstream
2019-09-20T17:42:59.5368720Z Writing .emscripten configuration file to user home directory /root/
2019-09-20T17:42:59.5369660Z The Emscripten configuration file /root/.emscripten has been rewritten with the following contents:
2019-09-20T17:42:59.5369807Z import os
2019-09-20T17:42:59.5369807Z import os
2019-09-20T17:42:59.5370259Z LLVM_ROOT = '/emsdk-portable/upstream/bin'
2019-09-20T17:42:59.5370776Z BINARYEN_ROOT = '/emsdk-portable/upstream'
2019-09-20T17:42:59.5371275Z EMSCRIPTEN_ROOT = '/emsdk-portable/upstream/emscripten'
2019-09-20T17:42:59.5371821Z NODE_JS = '/emsdk-portable/node/12.9.1_64bit/bin/node'
2019-09-20T17:42:59.5372276Z SPIDERMONKEY_ENGINE = ''
2019-09-20T17:42:59.5372713Z V8_ENGINE = ''
2019-09-20T17:42:59.5373218Z TEMP_DIR = '/tmp'
2019-09-20T17:42:59.5373768Z COMPILER_ENGINE = NODE_JS
2019-09-20T17:42:59.5373888Z JS_ENGINES = [NODE_JS]
2019-09-20T17:42:59.5373954Z 
2019-09-20T17:42:59.5374458Z To conveniently access the selected set of tools from the command line, consider adding the following directories to PATH, or call 'source ./emsdk_env.sh' to do this for you.
2019-09-20T17:42:59.5374709Z 
2019-09-20T17:42:59.5375174Z    /emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin
2019-09-20T17:42:59.5375442Z Set the following tools as active:
2019-09-20T17:42:59.5375865Z    releases-upstream-f3030d9fffcc9e1287cb6b8e72982e94ece31d71-64bit
2019-09-20T17:42:59.5376094Z    node-12.9.1-64bit
2019-09-20T17:42:59.5376129Z 
2019-09-20T17:42:59.5376129Z 
2019-09-20T17:42:59.5464005Z + source ./emsdk_env.sh
2019-09-20T17:42:59.5464644Z ++ SRC=./emsdk_env.sh
2019-09-20T17:42:59.5465432Z ++ '[' ./emsdk_env.sh = '' ']'
2019-09-20T17:42:59.5475095Z +++ pwd
2019-09-20T17:42:59.5475403Z ++ CURDIR=/emsdk-portable
2019-09-20T17:42:59.5479757Z +++ dirname ./emsdk_env.sh
2019-09-20T17:43:00.1394221Z ++ unset SRC
2019-09-20T17:43:00.1395031Z +++ mktemp
2019-09-20T17:43:00.1395031Z +++ mktemp
2019-09-20T17:43:00.1395542Z ++ tmpfile=/tmp/tmp.0JRqXAd1Qo
2019-09-20T17:43:00.1396147Z ++ ./emsdk construct_env /tmp/tmp.0JRqXAd1Qo
2019-09-20T17:43:00.1397103Z PATH += /emsdk-portable
2019-09-20T17:43:00.1397103Z PATH += /emsdk-portable
2019-09-20T17:43:00.1397568Z PATH += /emsdk-portable/upstream/emscripten
2019-09-20T17:43:00.1398258Z 
2019-09-20T17:43:00.1398427Z Setting environment variables:
2019-09-20T17:43:00.1399137Z EMSDK = /emsdk-portable
2019-09-20T17:43:00.1399475Z EM_CONFIG = /root/.emscripten
2019-09-20T17:43:00.1399475Z EM_CONFIG = /root/.emscripten
2019-09-20T17:43:00.1399988Z EMSDK_NODE = /emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-20T17:43:00.1400200Z 
2019-09-20T17:43:00.1401178Z ++ . /tmp/tmp.0JRqXAd1Qo
2019-09-20T17:43:00.1401817Z +++ export PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-20T17:43:00.1402640Z +++ PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-20T17:43:00.1403300Z +++ export EMSDK=/emsdk-portable
2019-09-20T17:43:00.1404180Z +++ EMSDK=/emsdk-portable
2019-09-20T17:43:00.1404817Z +++ export EM_CONFIG=/root/.emscripten
2019-09-20T17:43:00.1405299Z +++ EM_CONFIG=/root/.emscripten
2019-09-20T17:43:00.1405805Z +++ export EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-20T17:43:00.1406332Z +++ EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-20T17:43:00.1406796Z ++ rm -f /tmp/tmp.0JRqXAd1Qo
2019-09-20T17:43:00.6393360Z ++ cd /emsdk-portable
2019-09-20T17:43:00.6395822Z + echo 'main(){}'
2019-09-20T17:43:00.6400380Z + HOME=/emsdk-portable/
2019-09-20T17:43:00.6401049Z + emcc a.c
2019-09-20T17:43:00.7151743Z cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/emsdk-portable/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-20T17:43:00.7286729Z cache:INFO:  - ok
2019-09-20T17:43:00.8092708Z a.c:1:1: warning: type specifier missing, defaults to 'int' [-Wimplicit-int]
2019-09-20T17:43:00.8092809Z main(){}
2019-09-20T17:43:00.8123809Z 1 warning generated.
2019-09-20T17:43:00.8123809Z 1 warning generated.
2019-09-20T17:43:00.8360330Z cache:INFO: generating system library: libc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc.a" for subsequent builds)
2019-09-20T17:44:15.3318135Z cache:INFO:  - ok
2019-09-20T17:44:15.3322945Z cache:INFO: generating system library: libcompiler_rt.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt.a" for subsequent builds)
2019-09-20T17:44:15.7510506Z cache:INFO:  - ok
2019-09-20T17:44:15.7517007Z cache:INFO: generating system library: libc-wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc-wasm.a" for subsequent builds)
2019-09-20T17:44:18.5895648Z cache:INFO:  - ok
2019-09-20T17:44:18.5917950Z cache:INFO: generating system library: libdlmalloc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libdlmalloc.a" for subsequent builds)
2019-09-20T17:44:19.6570350Z cache:INFO:  - ok
2019-09-20T17:44:19.6573833Z cache:INFO: generating system library: libpthreads_stub.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libpthreads_stub.a" for subsequent builds)
2019-09-20T17:44:19.9637336Z cache:INFO:  - ok
2019-09-20T17:44:19.9642649Z cache:INFO: generating system library: libcompiler_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a" for subsequent builds)
2019-09-20T17:44:24.7502812Z cache:INFO:  - ok
2019-09-20T17:44:24.7507545Z cache:INFO: generating system library: libc_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc_rt_wasm.a" for subsequent builds)
2019-09-20T17:44:27.0801047Z cache:INFO:  - ok
2019-09-20T17:44:27.1122799Z cache:INFO: generating system asset: generated_struct_info.json... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/generated_struct_info.json" for subsequent builds)
2019-09-20T17:44:28.8932422Z cache:INFO:  - ok
2019-09-20T17:44:29.2216172Z + rm -f a.c a.out.js a.out.wasm
2019-09-20T17:44:29.2229148Z + cp /root/.emscripten /emsdk-portable
2019-09-20T17:44:29.2246604Z + chmod a+rxw -R /emsdk-portable
2019-09-20T17:44:54.7691569Z Removing intermediate container 81188b4bc613
2019-09-20T17:44:54.7692273Z  ---> 3b0547fa887d
2019-09-20T17:44:56.5975358Z  ---> 7f59cd7e20b7
2019-09-20T17:44:56.5975505Z Step 6/15 : RUN sh /scripts/sccache.sh
2019-09-20T17:44:56.7731869Z  ---> Running in c09c21fa5ecd
2019-09-20T17:44:58.1675457Z + curl -fo /usr/local/bin/sccache https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/2018-04-02-sccache-x86_64-unknown-linux-musl
---
2019-09-20T17:45:00.8198561Z Step 7/15 : ENV PATH=$PATH:/emsdk-portable
2019-09-20T17:45:01.0184878Z  ---> Running in 61fb49ef2595
2019-09-20T17:45:02.4707151Z Removing intermediate container 61fb49ef2595
2019-09-20T17:45:02.4708764Z  ---> 7a6e0f740e02
2019-09-20T17:45:02.4709314Z Step 8/15 : ENV PATH=$PATH:/emsdk-portable/upstream/emscripten/
2019-09-20T17:45:02.6669206Z  ---> Running in 46774db4e9d5
2019-09-20T17:45:04.0625918Z Removing intermediate container 46774db4e9d5
2019-09-20T17:45:04.0628109Z Step 9/15 : ENV PATH=$PATH:/emsdk-portable/node/12.9.1_64bit/bin/
2019-09-20T17:45:04.2667452Z  ---> Running in 88a91997ed08
2019-09-20T17:45:05.6775423Z Removing intermediate container 88a91997ed08
2019-09-20T17:45:05.6776698Z  ---> 3133ea3006b1
---
2019-09-20T17:45:15.2470316Z  ---> 3aad510c5da0
2019-09-20T17:45:15.2510547Z Successfully built 3aad510c5da0
2019-09-20T17:45:15.3574939Z Successfully tagged rust-ci:latest
2019-09-20T17:45:15.4479720Z Built container sha256:3aad510c5da04c6fddd1b9186125ba299e41191cb8a4dc14f212e1947206b4c6
2019-09-20T17:45:15.4538542Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/9069a8ae71b54c926e2a8002679e7618c86055bb388d267d806c6caebcd0ade8c08692ea699e039b70cf69938549b5e3aa2d924b0a69bcb52be16ba4bc6a194a
2019-09-20T17:46:48.9061133Z upload failed: - to s3://rust-lang-ci-sccache2/docker/9069a8ae71b54c926e2a8002679e7618c86055bb388d267d806c6caebcd0ade8c08692ea699e039b70cf69938549b5e3aa2d924b0a69bcb52be16ba4bc6a194a An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-09-20T17:46:49.9087588Z [CI_JOB_NAME=asmjs]
2019-09-20T17:46:49.9130648Z == clock drift check ==
2019-09-20T17:46:49.9139734Z   local time: Fri Sep 20 17:46:49 UTC 2019
2019-09-20T17:46:50.0690002Z   network time: Fri, 20 Sep 2019 17:46:50 GMT
---
2019-09-20T17:46:50.1694228Z configure: build.locked-deps    := True
2019-09-20T17:46:50.1694277Z configure: llvm.ccache          := sccache
2019-09-20T17:46:50.1694508Z configure: build.cargo-native-static := True
2019-09-20T17:46:50.1694751Z configure: dist.missing-tools   := True
2019-09-20T17:46:50.1695016Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-20T17:46:50.1695140Z configure: writing `config.toml` in current directory
2019-09-20T17:46:50.1695188Z configure: 
2019-09-20T17:46:50.1695448Z configure: run `python /checkout/x.py --help`
2019-09-20T17:46:50.1695516Z configure: 
---
2019-09-20T18:52:30.0604828Z running: "ar" "crs" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/rust_test_helpers.o"
2019-09-20T18:52:30.0621656Z exit code: 0
2019-09-20T18:52:30.0624693Z Building test helpers
2019-09-20T18:52:30.0630007Z running: "emcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-o" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
2019-09-20T18:52:30.1517895Z cargo:warning=cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/home/user/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-20T18:52:30.1661448Z cargo:warning=cache:INFO:  - ok
2019-09-20T18:52:30.3081138Z cargo:warning=shared:INFO: (Emscripten: Running sanity checks)
2019-09-20T18:52:30.3137877Z cargo:warning=shared:WARNING: java does not seem to exist, required for closure compiler, which is optional (define JAVA in /emsdk-portable/.emscripten if you want it)
2019-09-20T18:52:30.3138055Z cargo:warning=shared:WARNING: closure compiler will not be available
2019-09-20T18:52:30.3999060Z running: "emar" "crs" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o"
2019-09-20T18:52:30.4720159Z exit code: 0
2019-09-20T18:52:30.4737997Z Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
2019-09-20T18:52:30.7846184Z    Compiling proc-macro2 v0.4.30
---
2019-09-20T18:54:40.5139640Z 
2019-09-20T18:54:40.5140774Z running 9026 tests
2019-09-20T18:55:40.5158670Z test [ui] ui/abi/abi-sysv64-arg-passing.rs has been running for over 60 seconds
2019-09-20T18:55:40.5160516Z test [ui] ui/abi/abi-sysv64-register-usage.rs has been running for over 60 seconds
2019-09-20T18:58:55.7388201Z .i.....ii..i.......i......i........i.iiii.....................................ii...............i.... 100/9026
2019-09-20T19:01:30.4543603Z ..ii.........i.......................i....iiiiii.................................................... 200/9026
2019-09-20T19:05:34.4587756Z ..............................................i.........................i........................... 400/9026
2019-09-20T19:05:34.4587756Z ..............................................i.........................i........................... 400/9026
2019-09-20T19:07:15.5204180Z ..................................i...............................................iii............... 500/9026
2019-09-20T19:12:21.0112685Z ...........................................................i........................................ 700/9026
2019-09-20T19:13:01.0296450Z .................................................................................................... 800/9026
2019-09-20T19:13:43.4426270Z .................................................................i.................................. 900/9026
2019-09-20T19:15:51.1418990Z .......i............................................ii.........................................i.... 1000/9026
2019-09-20T19:15:51.1418990Z .......i............................................ii.........................................i.... 1000/9026
2019-09-20T19:17:22.8796956Z ....i.i........................................i.................................................... 1100/9026
2019-09-20T19:18:13.0963582Z .................................................................................................... 1200/9026
2019-09-20T19:19:03.6108614Z .................i.ii............................................................................... 1300/9026
2019-09-20T19:21:19.9675003Z .............................................................................i...................... 1400/9026
2019-09-20T19:23:11.6942282Z .................................................................................................... 1500/9026
2019-09-20T19:25:15.3974262Z ..........................ii.i...................................................................... 1600/9026
2019-09-20T19:27:31.8433825Z ........................................i......i..................i...............i....iii.......... 1700/9026
2019-09-20T19:29:02.9510636Z .................................................................................................... 1800/9026
2019-09-20T19:30:38.2496634Z .........................................................iiiii.......................i.........ii... 1900/9026
2019-09-20T19:32:08.2293771Z .................................................................................................... 2000/9026
2019-09-20T19:32:32.1383582Z ...............iii.................................................................................. 2100/9026
2019-09-20T19:32:34.3762605Z .................................................................................................... 2200/9026
2019-09-20T19:32:45.5959882Z ...............................................................iiii.....................i........... 2300/9026
2019-09-20T19:34:52.8844888Z .........i..........ii.............................................................................. 2500/9026
2019-09-20T19:36:56.7050529Z .................................................................................................... 2600/9026
2019-09-20T19:36:56.7050529Z .................................................................................................... 2600/9026
2019-09-20T19:39:42.8271838Z ........i....i..........................................................i...........ii.....ii.i..... 2700/9026
2019-09-20T19:42:34.4635783Z ...........i........................................................................................ 2900/9026
2019-09-20T19:44:24.6299006Z .................................................................................................... 3000/9026
2019-09-20T19:45:58.7166879Z .................................................................................................... 3100/9026
2019-09-20T19:45:58.7166879Z .................................................................................................... 3100/9026
2019-09-20T19:47:43.3430696Z ........ii..ii..................i....i............i....i.................i.......................... 3200/9026
2019-09-20T19:52:17.4235965Z .......................i.............i.....i.........................................i.......i...... 3400/9026
2019-09-20T19:54:36.5628550Z ..i............................i.................................................................... 3500/9026
2019-09-20T19:56:57.6285884Z ..............................................i..................................................... 3600/9026
2019-09-20T19:59:14.2550455Z ............i...........................................................i........................... 3700/9026
---
2019-09-20T20:19:46.0172215Z .............................................ii..............i...................................... 4700/9026
2019-09-20T20:21:57.7053180Z ....................................................................................i............... 4800/9026
2019-09-20T20:24:05.2744732Z .................................i...............................i.................................. 4900/9026
2019-09-20T20:26:30.7797876Z ..............................................................................i...........i......... 5000/9026
2019-09-20T20:28:19.3144514Z .....i.......................i..i..i.i.i............................................................ 5100/9026
2019-09-20T20:30:13.5826493Z .................................................................................................... 5300/9026
2019-09-20T20:30:13.5826493Z .................................................................................................... 5300/9026
2019-09-20T20:32:19.9502771Z ..i...............i......................................................ii..................ii..... 5400/9026
2019-09-20T20:35:51.0791171Z .................................................................................................... 5600/9026
2019-09-20T20:35:51.0791171Z .................................................................................................... 5600/9026
2019-09-20T20:37:37.1645125Z ................i............i...i......................................................ii...i..ii.. 5700/9026
2019-09-20T20:39:56.3707720Z .........i...........................i........i.i................................................... 5800/9026
2019-09-20T20:42:18.8748193Z .................................................................................................... 6000/9026
2019-09-20T20:42:18.8748193Z .................................................................................................... 6000/9026
2019-09-20T20:42:54.6926087Z .........................................................i...........i....................i..ii..... 6100/9026
2019-09-20T20:45:23.4435388Z ...........................i.....ii................................................................. 6200/9026
2019-09-20T20:47:34.2661878Z ................................i...............................i....................iii..i..ii.iiii 6300/9026
2019-09-20T20:47:53.2055791Z .iiiii...........................................i.................................................. 6400/9026
2019-09-20T20:48:00.5034582Z .....................i.............................................................................. 6600/9026
2019-09-20T20:48:44.6427755Z ...........................................i............................i........................... 6700/9026
2019-09-20T20:50:39.1379886Z ......................................................................................i............. 6800/9026
2019-09-20T20:50:39.1379886Z ......................................................................................i............. 6800/9026
2019-09-20T20:52:05.9232221Z ..................................................iiiiiiii.......................................... 6900/9026
2019-09-20T20:55:38.8398405Z .................................................................................................... 7100/9026
2019-09-20T20:56:35.3481350Z .................................................................................................... 7200/9026
2019-09-20T20:57:18.6335509Z ...............................i.................................................................... 7300/9026
2019-09-20T20:59:35.4683449Z .......................................................................................i.......i.... 7400/9026
2019-09-20T20:59:35.4683449Z .......................................................................................i.......i.... 7400/9026
2019-09-20T21:01:25.1516099Z .................................................................................................... 7500/9026
2019-09-20T21:03:04.1133487Z ...........................................i..i.............iii.....i....iiiiiiiiiii.i.............. 7600/9026
2019-09-20T21:05:19.0761250Z ..............................................................................................i.i... 7800/9026
2019-09-20T21:09:15.4630380Z ..................................................................i................................. 7900/9026
2019-09-20T21:10:59.2731428Z ................................i................................................................... 8000/9026
2019-09-20T21:10:59.2731428Z ................................i................................................................... 8000/9026
2019-09-20T21:13:35.6631787Z ...............................................................iii.....i.....i.........i........ii.. 8100/9026
2019-09-20T21:16:00.6335518Z ..i.i.....iiiiii.....iiiiiiii.ii...ii.iii..iiii..................................................... 8200/9026
2019-09-20T21:20:48.4375590Z .........................................................................................i.......... 8400/9026
2019-09-20T21:22:03.1444850Z .................................................................................................... 8500/9026
2019-09-20T21:23:24.2989068Z .................................................................................................... 8600/9026
2019-09-20T21:25:19.3074597Z .................................i.................................................................. 8700/9026
---
2019-09-20T21:33:28.9125187Z    Compiling rand_core v0.5.0
2019-09-20T21:33:29.1754178Z    Compiling rand_hc v0.2.0
2019-09-20T21:33:29.3463379Z    Compiling rand v0.7.0
2019-09-20T21:33:32.1663078Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-09-20T21:33:32.6872615Z error: linking with `emcc` failed: exit code: 1
2019-09-20T21:33:32.6898945Z   |
2019-09-20T21:33:32.6949462Z   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/run_time_detect-41ac5c05a452285c.run_time_detect.2g7r8ovf-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/run_time_detect-41ac5c05a452285c.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/run_time_detect-41ac5c05a452285c.31edbhgvps6ybmht.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libtest-3072b8356e59e60a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libterm-ec3e883f321bb27d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libgetopts-f91bd61645eaf2dd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunicode_width-7fae48598d197287.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_std-f536514bf94e11db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libstd-4e65788bae25c659.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-353ec216f4d15524.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib" "-l" "c" "-Wl,-rpath,$ORIGIN/../lib" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-s" "WASM=0"
2019-09-20T21:33:32.6950922Z   = note: wasm-ld: error: unknown argument: -rpath
2019-09-20T21:33:32.6951228Z           wasm-ld: error: cannot open $ORIGIN/../lib: No such file or directory
2019-09-20T21:33:32.6955546Z           shared:ERROR: '/emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp_fuSfIY/run_time_detect-41ac5c05a452285c.wasm --allow-undefined --import-memory --import-table --lto-O0 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/run_time_detect-41ac5c05a452285c.run_time_detect.2g7r8ovf-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/run_time_detect-41ac5c05a452285c.31edbhgvps6ybmht.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libtest-3072b8356e59e60a.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libterm-ec3e883f321bb27d.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libgetopts-f91bd61645eaf2dd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunicode_width-7fae48598d197287.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_std-f536514bf94e11db.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libstd-4e65788bae25c659.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-353ec216f4d15524.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib -rpath $ORIGIN/../lib /home/user/.emscripten_cache/wasm-obj/libc.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt.a /home/user/.emscripten_cache/wasm-obj/libc-wasm.a /home/user/.emscripten_cache/wasm-obj/libc-extras.a /home/user/.emscripten_cache/wasm-obj/libdlmalloc.a /home/user/.emscripten_cache/wasm-obj/libpthreads_stub.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a /home/user/.emscripten_cache/wasm-obj/libc_rt_wasm.a -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024' failed (1)
2019-09-20T21:33:32.6956646Z 
2019-09-20T21:33:32.6956918Z error: aborting due to previous error
2019-09-20T21:33:32.6956955Z 
2019-09-20T21:33:32.6957186Z error: Could not compile `std`.
2019-09-20T21:33:32.6957186Z error: Could not compile `std`.
2019-09-20T21:33:32.6957455Z warning: build failed, waiting for other jobs to finish...
2019-09-20T21:33:33.2242884Z error: linking with `emcc` failed: exit code: 1
2019-09-20T21:33:33.2246389Z   |
2019-09-20T21:33:33.2257033Z   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/env-684ee6cb0d06c9e5.env.84zftx17-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/env-684ee6cb0d06c9e5.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/env-684ee6cb0d06c9e5.kr1um5bq5n6zzlm.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libtest-3072b8356e59e60a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libterm-ec3e883f321bb27d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libgetopts-f91bd61645eaf2dd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunicode_width-7fae48598d197287.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_std-f536514bf94e11db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/librand-af75f0f0f533354e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/librand_hc-f75a59e834501b5b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/librand_core-a96c867b08c65e9f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libgetrandom-320f122525bdc2f4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libstd-4e65788bae25c659.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-353ec216f4d15524.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib" "-l" "c" "-Wl,-rpath,$ORIGIN/../lib" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-s" "WASM=0"
2019-09-20T21:33:33.2259049Z   = note: wasm-ld: error: unknown argument: -rpath
2019-09-20T21:33:33.2259376Z           wasm-ld: error: cannot open $ORIGIN/../lib: No such file or directory
2019-09-20T21:33:33.2264834Z           shared:ERROR: '/emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp_b_n7qX/env-684ee6cb0d06c9e5.wasm --allow-undefined --import-memory --import-table --lto-O0 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/env-684ee6cb0d06c9e5.env.84zftx17-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/env-684ee6cb0d06c9e5.kr1um5bq5n6zzlm.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libtest-3072b8356e59e60a.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libterm-ec3e883f321bb27d.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libgetopts-f91bd61645eaf2dd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunicode_width-7fae48598d197287.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_std-f536514bf94e11db.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/librand-af75f0f0f533354e.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/librand_hc-f75a59e834501b5b.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/librand_core-a96c867b08c65e9f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libgetrandom-320f122525bdc2f4.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libstd-4e65788bae25c659.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-353ec216f4d15524.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-9b60830bbe2d581e.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-ba59cbcb6e7217de.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-c7227c1ea0e624f6.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-ec10ee0429285dbd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-11efea36ab74a158.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-537a31a71a1667e5.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bf44b0c00855d2df.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-ac926a854a4b8f07.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-6243923ce9b44c89.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-18771975efaea389.rlib -rpath $ORIGIN/../lib /home/user/.emscripten_cache/wasm-obj/libc.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt.a /home/user/.emscripten_cache/wasm-obj/libc-wasm.a /home/user/.emscripten_cache/wasm-obj/libc-extras.a /home/user/.emscripten_cache/wasm-obj/libdlmalloc.a /home/user/.emscripten_cache/wasm-obj/libpthreads_stub.a /home/user/.emscripten_cache/wasm-obj/libcompiler_rt_wasm.a /home/user/.emscripten_cache/wasm-obj/libc_rt_wasm.a -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024' failed (1)
2019-09-20T21:33:33.2266209Z 
2019-09-20T21:33:33.2278673Z error: aborting due to previous error
2019-09-20T21:33:33.2278968Z 
2019-09-20T21:33:33.2391159Z error: Could not compile `std`.
2019-09-20T21:33:33.2391159Z error: Could not compile `std`.
2019-09-20T21:33:33.2391236Z 
2019-09-20T21:33:33.2391544Z To learn more, run the command again with --verbose.
2019-09-20T21:33:33.2412784Z 
2019-09-20T21:33:33.2412910Z 
2019-09-20T21:33:33.2413745Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "asmjs-unknown-emscripten" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--quiet"
2019-09-20T21:33:33.2413882Z 
2019-09-20T21:33:33.2413916Z 
2019-09-20T21:33:33.2426507Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/ui src/libstd src/liballoc src/libcore
2019-09-20T21:33:33.2426651Z Build completed unsuccessfully in 3:44:18
2019-09-20T21:33:33.2426651Z Build completed unsuccessfully in 3:44:18
2019-09-20T21:33:33.2484437Z == clock drift check ==
2019-09-20T21:33:33.2506929Z   local time: Fri Sep 20 21:33:33 UTC 2019
2019-09-20T21:33:33.3361958Z   network time: Fri, 20 Sep 2019 21:33:33 GMT
2019-09-20T21:33:33.3363728Z == end clock drift check ==
2019-09-20T21:33:34.7260520Z ##[error]Bash exited with code '1'.
2019-09-20T21:33:34.7307093Z ##[section]Starting: Checkout
2019-09-20T21:33:34.7309070Z ==============================================================================
2019-09-20T21:33:34.7309151Z Task         : Get sources
2019-09-20T21:33:34.7309502Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
