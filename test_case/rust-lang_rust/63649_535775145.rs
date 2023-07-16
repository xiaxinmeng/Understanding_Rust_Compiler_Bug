plain
2019-09-26T23:49:02.5694142Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T23:49:02.5894359Z ##[command]git config gc.auto 0
2019-09-26T23:49:02.5962994Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T23:49:02.6020701Z ##[command]git config --get-all http.proxy
2019-09-26T23:49:02.6186574Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63649/merge:refs/remotes/pull/63649/merge
---
2019-09-26T23:53:45.6729771Z  ---> 791fa8cc9a50
2019-09-26T23:53:45.6730374Z Step 3/13 : COPY scripts/emscripten.sh /scripts/
2019-09-26T23:53:47.5943128Z  ---> 664e83963d56
2019-09-26T23:53:47.5943334Z Step 4/13 : RUN bash /scripts/emscripten.sh
2019-09-26T23:53:47.7693700Z  ---> Running in 399f4b10a52e
2019-09-26T23:53:48.2554285Z + git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
2019-09-26T23:53:48.2575820Z Cloning into '/emsdk-portable'...
2019-09-26T23:53:48.9238351Z + cd /emsdk-portable
2019-09-26T23:53:48.9240330Z + hide_output ./emsdk install 1.38.46-upstream
2019-09-26T23:54:17.7770882Z /scripts/emscripten.sh: line 3:    18 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-26T23:54:17.7770882Z /scripts/emscripten.sh: line 3:    18 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-26T23:54:17.7772778Z + ./emsdk activate 1.38.46-upstream
2019-09-26T23:54:18.1633207Z Writing .emscripten configuration file to user home directory /root/
2019-09-26T23:54:18.1633408Z The Emscripten configuration file /root/.emscripten has been rewritten with the following contents:
2019-09-26T23:54:18.1633495Z import os
2019-09-26T23:54:18.1633495Z import os
2019-09-26T23:54:18.1633770Z LLVM_ROOT = '/emsdk-portable/upstream/bin'
2019-09-26T23:54:18.1634021Z BINARYEN_ROOT = '/emsdk-portable/upstream'
2019-09-26T23:54:18.1634556Z EMSCRIPTEN_ROOT = '/emsdk-portable/upstream/emscripten'
2019-09-26T23:54:18.1634865Z NODE_JS = '/emsdk-portable/node/12.9.1_64bit/bin/node'
2019-09-26T23:54:18.1635119Z SPIDERMONKEY_ENGINE = ''
2019-09-26T23:54:18.1635313Z V8_ENGINE = ''
2019-09-26T23:54:18.1635510Z TEMP_DIR = '/tmp'
2019-09-26T23:54:18.1635575Z COMPILER_ENGINE = NODE_JS
2019-09-26T23:54:18.1635623Z JS_ENGINES = [NODE_JS]
2019-09-26T23:54:18.1635693Z 
2019-09-26T23:54:18.1636049Z To conveniently access the selected set of tools from the command line, consider adding the following directories to PATH, or call 'source ./emsdk_env.sh' to do this for you.
2019-09-26T23:54:18.1636125Z 
2019-09-26T23:54:18.1636403Z    /emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin
2019-09-26T23:54:18.1636503Z Set the following tools as active:
2019-09-26T23:54:18.1637930Z    releases-upstream-c89919d252f7cea00d944bdf3bd630cd3c7e7388-64bit
2019-09-26T23:54:18.1638423Z    node-12.9.1-64bit
2019-09-26T23:54:18.1638898Z 
2019-09-26T23:54:18.1638898Z 
2019-09-26T23:54:18.1734546Z + source ./emsdk_env.sh
2019-09-26T23:54:18.1741997Z ++ SRC=./emsdk_env.sh
2019-09-26T23:54:18.1748082Z ++ '[' ./emsdk_env.sh = '' ']'
2019-09-26T23:54:18.1748853Z +++ pwd
2019-09-26T23:54:18.1749332Z ++ CURDIR=/emsdk-portable
2019-09-26T23:54:18.1749792Z +++ dirname ./emsdk_env.sh
2019-09-26T23:54:18.1763690Z ++ unset SRC
2019-09-26T23:54:18.1771046Z +++ mktemp
2019-09-26T23:54:18.1771046Z +++ mktemp
2019-09-26T23:54:18.1786829Z ++ tmpfile=/tmp/tmp.SPwf47rStE
2019-09-26T23:54:18.1788108Z ++ ./emsdk construct_env /tmp/tmp.SPwf47rStE
2019-09-26T23:54:18.3617098Z PATH += /emsdk-portable
2019-09-26T23:54:18.3617098Z PATH += /emsdk-portable
2019-09-26T23:54:18.3617808Z PATH += /emsdk-portable/upstream/emscripten
2019-09-26T23:54:18.3618137Z 
2019-09-26T23:54:18.3618219Z Setting environment variables:
2019-09-26T23:54:18.3618460Z EMSDK = /emsdk-portable
2019-09-26T23:54:18.3618513Z EM_CONFIG = /root/.emscripten
2019-09-26T23:54:18.3618513Z EM_CONFIG = /root/.emscripten
2019-09-26T23:54:18.3618798Z EMSDK_NODE = /emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-26T23:54:18.3618843Z 
2019-09-26T23:54:18.3655686Z ++ . /tmp/tmp.SPwf47rStE
2019-09-26T23:54:18.3656145Z +++ export PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-26T23:54:18.3657198Z +++ PATH=/emsdk-portable:/emsdk-portable/upstream/emscripten:/emsdk-portable/node/12.9.1_64bit/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2019-09-26T23:54:18.3657593Z +++ export EMSDK=/emsdk-portable
2019-09-26T23:54:18.3657841Z +++ EMSDK=/emsdk-portable
2019-09-26T23:54:18.3657893Z +++ export EM_CONFIG=/root/.emscripten
2019-09-26T23:54:18.3657940Z +++ EM_CONFIG=/root/.emscripten
2019-09-26T23:54:18.3658527Z +++ export EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-26T23:54:18.3664955Z +++ EMSDK_NODE=/emsdk-portable/node/12.9.1_64bit/bin/node
2019-09-26T23:54:18.3665299Z ++ rm -f /tmp/tmp.SPwf47rStE
2019-09-26T23:54:18.3665612Z ++ cd /emsdk-portable
2019-09-26T23:54:18.3665829Z + echo 'main(){}'
2019-09-26T23:54:18.3666040Z + HOME=/emsdk-portable/
2019-09-26T23:54:18.3666106Z + emcc a.c
2019-09-26T23:54:18.4356023Z cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/emsdk-portable/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-26T23:54:18.4488718Z cache:INFO:  - ok
2019-09-26T23:54:18.5208106Z a.c:1:1: warning: type specifier missing, defaults to 'int' [-Wimplicit-int]
2019-09-26T23:54:18.5208250Z main(){}
2019-09-26T23:54:18.5237244Z 1 warning generated.
2019-09-26T23:54:18.5237244Z 1 warning generated.
2019-09-26T23:54:18.6002143Z cache:INFO: generating system library: libc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc.a" for subsequent builds)
2019-09-26T23:55:23.0389680Z cache:INFO:  - ok
2019-09-26T23:55:23.0395161Z cache:INFO: generating system library: libcompiler_rt.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libcompiler_rt.a" for subsequent builds)
2019-09-26T23:55:27.6883790Z cache:INFO:  - ok
2019-09-26T23:55:27.6890582Z cache:INFO: generating system library: libc-wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc-wasm.a" for subsequent builds)
2019-09-26T23:55:30.1729662Z cache:INFO:  - ok
2019-09-26T23:55:30.1751721Z cache:INFO: generating system library: libdlmalloc.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libdlmalloc.a" for subsequent builds)
2019-09-26T23:55:31.2992225Z cache:INFO:  - ok
2019-09-26T23:55:31.2998781Z cache:INFO: generating system library: libpthread_stub.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libpthread_stub.a" for subsequent builds)
2019-09-26T23:55:31.5958107Z cache:INFO:  - ok
2019-09-26T23:55:31.5958735Z cache:INFO: generating system library: libc_rt_wasm.a... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/libc_rt_wasm.a" for subsequent builds)
2019-09-26T23:55:33.7322903Z cache:INFO:  - ok
2019-09-26T23:55:33.7466211Z cache:INFO: generating system asset: generated_struct_info.json... (this will be cached in "/emsdk-portable/.emscripten_cache/wasm-obj/generated_struct_info.json" for subsequent builds)
2019-09-26T23:55:35.6083578Z cache:INFO:  - ok
2019-09-26T23:55:35.8044839Z + rm -f a.c a.out.js a.out.wasm
2019-09-26T23:55:35.8055953Z + cp /root/.emscripten /emsdk-portable
2019-09-26T23:55:35.8071908Z + chmod a+rxw -R /emsdk-portable
2019-09-26T23:56:02.0904126Z Removing intermediate container 399f4b10a52e
2019-09-26T23:56:02.0906293Z Step 5/13 : COPY scripts/sccache.sh /scripts/
2019-09-26T23:56:06.5569500Z  ---> 0dd96cdf444a
2019-09-26T23:56:06.5570306Z Step 6/13 : RUN sh /scripts/sccache.sh
2019-09-26T23:56:06.6959597Z  ---> Running in 9a6fecf9d415
---
2019-09-26T23:56:10.9045529Z Step 7/13 : ENV PATH=$PATH:/emsdk-portable
2019-09-26T23:56:11.0742442Z  ---> Running in afbf480f5ca0
2019-09-26T23:56:12.6418928Z Removing intermediate container afbf480f5ca0
2019-09-26T23:56:12.6420048Z  ---> f725b11e172c
2019-09-26T23:56:12.6420742Z Step 8/13 : ENV PATH=$PATH:/emsdk-portable/upstream/emscripten/
2019-09-26T23:56:12.8323241Z  ---> Running in e9dc00e80ad0
2019-09-26T23:56:14.3664810Z Removing intermediate container e9dc00e80ad0
2019-09-26T23:56:14.3667338Z Step 9/13 : ENV PATH=$PATH:/emsdk-portable/node/12.9.1_64bit/bin/
2019-09-26T23:56:14.5358698Z  ---> Running in 4bca3c207b24
2019-09-26T23:56:15.9981314Z Removing intermediate container 4bca3c207b24
2019-09-26T23:56:15.9982168Z  ---> 88d5f237a16e
---
2019-09-26T23:56:22.7314739Z Successfully built d7a5444aa826
2019-09-26T23:56:22.8878079Z Successfully tagged rust-ci:latest
2019-09-26T23:56:22.9177355Z Built container sha256:d7a5444aa8269737eebca0e55e61dc655e6fb334424ee68fde4d4503f9b35338
2019-09-26T23:56:22.9196264Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/886433b54c6398520261ed9b7ac82c02d8885da1b96fe485b962935cb649261f34684468f47c027ab1349b97d1fe2648ca888fb24d3fe85b8c8aab35faa55531
2019-09-26T23:57:56.3413893Z upload failed: - to s3://rust-lang-ci-sccache2/docker/886433b54c6398520261ed9b7ac82c02d8885da1b96fe485b962935cb649261f34684468f47c027ab1349b97d1fe2648ca888fb24d3fe85b8c8aab35faa55531 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-09-26T23:57:57.2593108Z [CI_JOB_NAME=wasm32]
2019-09-26T23:57:57.4733620Z [CI_JOB_NAME=wasm32]
2019-09-26T23:57:57.4788322Z   local time: Thu Sep 26 23:57:57 UTC 2019
2019-09-26T23:57:57.5632114Z   network time: Thu, 26 Sep 2019 23:57:57 GMT
2019-09-26T23:57:57.5632408Z == end clock drift check ==
2019-09-26T23:57:57.5657333Z Starting sccache server...
---
2019-09-26T23:57:57.6630058Z configure: build.locked-deps    := True
2019-09-26T23:57:57.6630277Z configure: llvm.ccache          := sccache
2019-09-26T23:57:57.6630665Z configure: build.cargo-native-static := True
2019-09-26T23:57:57.6631105Z configure: dist.missing-tools   := True
2019-09-26T23:57:57.6631603Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-26T23:57:57.6631990Z configure: writing `config.toml` in current directory
2019-09-26T23:57:57.6632148Z configure: 
2019-09-26T23:57:57.6632585Z configure: run `python /checkout/x.py --help`
2019-09-26T23:57:57.6633028Z configure: 
---
2019-09-27T01:34:50.2280664Z exit code: 0
2019-09-27T01:34:50.2284807Z running: "ar" "crs" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/rust_test_helpers.o"
2019-09-27T01:34:50.2302291Z exit code: 0
2019-09-27T01:34:50.2306229Z Building test helpers
2019-09-27T01:34:50.2316712Z running: "emcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-o" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
2019-09-27T01:34:50.3314894Z cargo:warning=cache:INFO: generating system asset: is_vanilla.txt... (this will be cached in "/home/user/.emscripten_cache/is_vanilla.txt" for subsequent builds)
2019-09-27T01:34:50.7599666Z cargo:warning=cache:INFO:  - ok
2019-09-27T01:34:51.0796428Z cargo:warning=shared:INFO: (Emscripten: Running sanity checks)
2019-09-27T01:34:51.0852260Z cargo:warning=shared:WARNING: java does not seem to exist, required for closure compiler, which is optional (define JAVA in /emsdk-portable/.emscripten if you want it)
2019-09-27T01:34:51.0852706Z cargo:warning=shared:WARNING: closure compiler will not be available
2019-09-27T01:34:51.3833976Z running: "emar" "crs" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/librust_test_helpers.a" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers/rust_test_helpers.o"
2019-09-27T01:34:51.4532129Z exit code: 0
2019-09-27T01:34:51.4546718Z Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
2019-09-27T01:34:51.7532694Z    Compiling proc-macro2 v0.4.30
---
2019-09-27T01:37:07.4675219Z 
2019-09-27T01:37:07.4675992Z running 9049 tests
2019-09-27T01:38:07.4698306Z test [ui] ui/abi/abi-sysv64-arg-passing.rs has been running for over 60 seconds
2019-09-27T01:38:07.4698695Z test [ui] ui/abi/abi-sysv64-register-usage.rs has been running for over 60 seconds
2019-09-27T01:39:52.6566978Z .i.....ii..i.......i......i......i.iiiiii.....................................ii...............i.... 100/9049
2019-09-27T01:41:17.4943747Z ..ii.........i.......................i....iiiiii.................................................... 200/9049
2019-09-27T01:43:32.5957032Z ..............................................i..................................................... 400/9049
2019-09-27T01:43:32.5957032Z ..............................................i..................................................... 400/9049
2019-09-27T01:44:26.1475033Z .....................................i...............................................iii............ 500/9049
2019-09-27T01:47:06.8734561Z ..............................................................i..................................... 700/9049
2019-09-27T01:47:28.0596116Z .................................................................................................... 800/9049
2019-09-27T01:47:54.2906826Z ....................................................................i............................... 900/9049
2019-09-27T01:47:54.2906826Z ....................................................................i............................... 900/9049
2019-09-27T01:49:02.0533164Z ..........i............................................ii.........................................i. 1000/9049
2019-09-27T01:49:50.4445612Z .......i.i........................................i................................................. 1100/9049
2019-09-27T01:50:48.1793286Z ....................i.ii............................................................................ 1300/9049
2019-09-27T01:51:57.4210525Z .................................................................................i.................. 1400/9049
2019-09-27T01:52:57.3338179Z .................................................................................................... 1500/9049
2019-09-27T01:54:00.2136916Z ..................................ii.i.............................................................. 1600/9049
2019-09-27T01:54:00.2136916Z ..................................ii.i.............................................................. 1600/9049
2019-09-27T01:55:14.5915859Z ................................................i......i..................i...............i.....ii.. 1700/9049
2019-09-27T01:55:59.7699239Z .................................................................................................... 1800/9049
2019-09-27T01:56:53.6294510Z .................................................................iiiii.......................i...... 1900/9049
2019-09-27T01:57:54.9469984Z ...ii............................................................................................... 2000/9049
2019-09-27T01:58:09.9829999Z .......................iii.......................................................................... 2100/9049
2019-09-27T01:58:13.0851480Z .................................................................................................... 2200/9049
2019-09-27T01:58:18.0565765Z ......................................................................iiii......................i... 2300/9049
2019-09-27T01:59:30.2250397Z .................i..........ii...................................................................... 2500/9049
2019-09-27T02:00:33.8646124Z .................................................................................................... 2600/9049
2019-09-27T02:00:33.8646124Z .................................................................................................... 2600/9049
2019-09-27T02:02:00.7446268Z ................i....i......................................................................ii.....i 2700/9049
2019-09-27T02:03:08.5987740Z i.i..............................................................i.................................. 2800/9049
2019-09-27T02:04:31.1893774Z .................................................................................................... 3000/9049
2019-09-27T02:05:22.1435140Z .................................................................................................... 3100/9049
2019-09-27T02:05:22.1435140Z .................................................................................................... 3100/9049
2019-09-27T02:06:13.0851822Z ..................ii..ii..................i....i............i....i.................i................ 3200/9049
2019-09-27T02:08:34.8125469Z .................................i.............i.....i.........................................i.... 3400/9049
2019-09-27T02:09:46.1422123Z ...i........i............................i.......................................................... 3500/9049
2019-09-27T02:11:00.5409303Z ........................................................i........................................... 3600/9049
2019-09-27T02:12:04.4243198Z ......................i...........................................................i................. 3700/9049
---
2019-09-27T02:22:54.5604802Z .......i................................................i...............i........................... 4700/9049
2019-09-27T02:24:00.0402836Z ...............................................................................................i.... 4800/9049
2019-09-27T02:25:08.7382383Z ............................................i....................................................... 4900/9049
2019-09-27T02:26:16.0063276Z ............................................................................................i....... 5000/9049
2019-09-27T02:27:52.7291377Z ....i..............i.......................i..i..i.i.i.............................................. 5100/9049
2019-09-27T02:28:58.1293747Z .................................................................................................... 5300/9049
2019-09-27T02:29:54.8243508Z .................i...............i......................................................ii.......... 5400/9049
2019-09-27T02:31:08.0529914Z ........ii........................................................................i...i............. 5500/9049
2019-09-27T02:31:42.3479646Z .................................................................................................... 5600/9049
2019-09-27T02:31:42.3479646Z .................................................................................................... 5600/9049
2019-09-27T02:32:51.1349943Z ...............................i............i...i................................................... 5700/9049
2019-09-27T02:33:50.2556423Z ...ii...i..ii...........i...........................i........i.i.................................... 5800/9049
2019-09-27T02:35:16.6426357Z .................................................................................................... 6000/9049
2019-09-27T02:35:36.4467403Z .........................................................................i...........i.............. 6100/9049
2019-09-27T02:35:36.4467403Z .........................................................................i...........i.............. 6100/9049
2019-09-27T02:36:51.7425961Z ......i..ii................................i.....ii................................................. 6200/9049
2019-09-27T02:38:05.6563481Z ................................................i...............................i................... 6300/9049
2019-09-27T02:38:17.1685321Z .iii..i..ii.iiii.iiiii............................................i................................. 6400/9049
2019-09-27T02:38:25.0996445Z ......................................i............................................................. 6600/9049
2019-09-27T02:38:44.6456423Z ............................................................i............................i.......... 6700/9049
2019-09-27T02:39:47.4915732Z .................................................................................................... 6800/9049
2019-09-27T02:39:47.4915732Z .................................................................................................... 6800/9049
2019-09-27T02:40:38.9958777Z ...i....................................i..........................iiiiiiii......................... 6900/9049
2019-09-27T02:42:30.3622253Z .................................................................................................... 7100/9049
2019-09-27T02:43:10.3180676Z .................................................................................................... 7200/9049
2019-09-27T02:43:34.8736638Z ................................................i................................................... 7300/9049
2019-09-27T02:44:41.3423152Z .................................................................................................... 7400/9049
2019-09-27T02:44:41.3423152Z .................................................................................................... 7400/9049
2019-09-27T02:45:51.1525106Z ...i.......i........................................................................................ 7500/9049
2019-09-27T02:46:35.8549129Z ..............................................................i.............iii.....i....iiiiiiiiiii 7600/9049
2019-09-27T02:47:03.3377867Z .i.........................i........................................................................ 7700/9049
2019-09-27T02:49:42.5470550Z ..........i.i.....................................................................i................. 7900/9049
2019-09-27T02:50:52.7181921Z ................................................i................................................... 8000/9049
2019-09-27T02:50:52.7181921Z ................................................i................................................... 8000/9049
2019-09-27T02:51:38.1870288Z ................................................................................iii.....i.....i..... 8100/9049
2019-09-27T02:53:19.0536077Z ....i........ii....i.i.....iiiiii.....iiiiiiii.ii...ii.iii..iiii.................................... 8200/9049
2019-09-27T02:55:49.3486903Z .................................................................................................... 8400/9049
2019-09-27T02:56:31.5655295Z ..........i......................................................................................... 8500/9049
2019-09-27T02:57:03.7756149Z .................................................................................................... 8600/9049
2019-09-27T02:58:16.5129126Z .................................................................................................... 8700/9049
---
2019-09-27T03:07:51.0400099Z     Finished release [optimized] target(s) in 1m 21s
2019-09-27T03:07:51.0548868Z      Running build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-emscripten/release/deps/std-caa4512fd56c78b6.js
2019-09-27T03:07:51.2946940Z 
2019-09-27T03:07:51.3050078Z running 396 tests
2019-09-27T03:07:51.3110462Z exception thrown: 5453192 - Exception catching is disabled, this exception cannot be caught. Compile with -s DISABLE_EXCEPTION_CATCHING=0 or DISABLE_EXCEPTION_CATCHING=2 to catch.
2019-09-27T03:07:52.9416542Z error: test failed, to rerun pass '-p std --lib'
2019-09-27T03:07:52.9419336Z 
2019-09-27T03:07:52.9419336Z 
2019-09-27T03:07:52.9420444Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "wasm32-unknown-emscripten" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--quiet"
2019-09-27T03:07:52.9423775Z 
2019-09-27T03:07:52.9423993Z 
2019-09-27T03:07:52.9481102Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten src/test/ui src/libstd src/liballoc src/libcore
2019-09-27T03:07:52.9481611Z Build completed unsuccessfully in 3:07:12
2019-09-27T03:07:52.9481611Z Build completed unsuccessfully in 3:07:12
2019-09-27T03:07:52.9488120Z == clock drift check ==
2019-09-27T03:07:52.9505162Z   local time: Fri Sep 27 03:07:52 UTC 2019
2019-09-27T03:07:53.2273206Z   network time: Fri, 27 Sep 2019 03:07:53 GMT
2019-09-27T03:07:53.2278139Z == end clock drift check ==
2019-09-27T03:07:53.8305905Z ##[error]Bash exited with code '1'.
2019-09-27T03:07:53.8358510Z ##[section]Starting: Checkout
2019-09-27T03:07:53.8360607Z ==============================================================================
2019-09-27T03:07:53.8360661Z Task         : Get sources
2019-09-27T03:07:53.8360702Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
