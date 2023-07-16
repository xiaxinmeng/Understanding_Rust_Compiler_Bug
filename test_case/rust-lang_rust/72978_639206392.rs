
"-Wl,-rpath,$ORIGIN/../lib"[0m
2020-06-04T17:18:41.4210399Z [0m  [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libfiletime-cd4b67c5479ae270.rlib(filetime-cd4b67c5479ae270.filetime.4z3loc0p-cgu.4.rcgu.o): In function `filetime::imp::utimes::set_times::hb28e25c679b44a1a':[0m
2020-06-04T17:18:41.4211763Z [0m          filetime.4z3loc0p-cgu.4:(.text._ZN8filetime3imp6utimes9set_times17hb28e25c679b44a1aE+0x287): warning: warning: lutimes is not implemented and will always fail[0m
2020-06-04T17:18:41.4215999Z [0m          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libinotify-cd7a29090309d6ed.rlib(inotify-cd7a29090309d6ed.inotify.bxraksv4-cgu.3.rcgu.o): In function `inotify::inotify::Inotify::init::he432cafdcf6fb733':[0m
2020-06-04T17:18:41.4217235Z [0m          inotify.bxraksv4-cgu.3:(.text._ZN7inotify7inotify7Inotify4init17he432cafdcf6fb733E+0x10): undefined reference to `inotify_init1'[0m
2020-06-04T17:18:41.4218077Z [0m          clang-9: error: linker command failed with exit code 1 (use -v to see invocation)[0m
2020-06-04T17:18:41.4218761Z [0m          [0m
2020-06-04T17:18:41.4218902Z 
2020-06-04T17:18:41.4243114Z [0m[1m[38;5;9merror[0m[0m[1m: aborting due to previous error[0m
2020-06-04T17:18:41.4243571Z 
2020-06-04T17:18:41.4450968Z [0m[0m[1m[31merror[0m[1m:[0m could not compile `rust-analyzer`.
