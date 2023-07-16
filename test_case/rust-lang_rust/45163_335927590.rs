
[00:44:38] /checkout/src/liballoc_jemalloc/../jemalloc/src/tsd.c:20:35: error: __emutls_t.tsd_tls causes a section type conflict with tsd_tls
[00:44:38]  __thread tsd_t JEMALLOC_TLS_MODEL tsd_tls = TSD_INITIALIZER;
[00:44:38]                                    ^
[00:44:38] /checkout/src/liballoc_jemalloc/../jemalloc/src/tsd.c:20:35: note: 'tsd_tls' was declared here
[00:44:38] make: *** [src/tsd.sym.o] Error 1
[00:44:38] make: *** Waiting for unfinished jobs....
[00:44:38] 
[00:44:38] [m[m[33m[1mwarning:[m build failed, waiting for other jobs to finish...
[00:44:41] [m[m[31m[1merror:[m build failed
[00:44:41] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:44:41] expected success, got: exit code: 101', /checkout/src/bootstrap/compile.rs:883:8
[00:44:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target arm-linux-androideabi,armv7-linux-androideabi,i686-linux-android,aarch64-linux-android,x86_64-linux-android
[00:44:41] Build completed unsuccessfully in 0:42:32
