plain
[00:19:12]    Compiling rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:19:12]    Compiling rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[00:19:32]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:19:33]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
            sess.abort_if_errors();
[00:20:15] 482 | |             }
[00:20:15] 483 | |         }
[00:20:15]     | |_________^ expected (), found tuple
[00:20:15]     |
[00:20:15]     |
[00:20:15]     = note: expected type `()`
[00:20:15]                found type `(std::result::Result<(), _>, std::option::Option<_>)`
[00:20:15] 
o --extern rustc_passes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-9838343e2adbcee9.so --extern rustc_plugin=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin-fd0079a3967183e9.so --extern rustc_privacy=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-5b0749a532338fe8.so --extern rustc_resolve=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-cc472b54563acd7d.so --extern rustc_save_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-cbe791ad1ac1ddea.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern rustc_traits=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-b46bf5b3351d287f.so --extern rustc_typeck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-a0e39128708a13f4.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-6a496b98b1d59ff3.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnus || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23321e97
$ dmesg | grep -i kill
