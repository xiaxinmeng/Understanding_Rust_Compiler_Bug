plain
[00:07:00] 6551 | |         Ok(ident)
[00:07:00] 6552 | |     }
[00:07:00]      | |_____- defined here
[00:07:00] ...
[00:07:00] 6566 |           let orig_name = self.parse_crate_name_with_dashes()?;
[00:07:00] 
[00:07:01] error: aborting due to previous error
[00:07:01] 
[00:07:01] For more information about this error, try `rustc --explain E0061`.
[00:07:01] For more information about this error, try `rustc --explain E0061`.
[00:07:01] error: Could not compile `syntax`.
[00:07:01] 
[00:07:01] Caused by:
[00:07:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=5bf9074d2dea15d4 -C extra-filename=-5bf9074d2dea15d4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1ae79b79f441d17a.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-eaf85f66e467efb7.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3f4afd8f1fa86b47.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c1fe7cbcb67aaf3e.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-1508bcc57003426f.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-505c3c1344630e33.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-555c0a62ece4d8d0.so` (exit code: 101)
[00:07:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:01] expected success, got: exit code: 101
[00:07:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:07:01] travis_fold:end:stage0-rustc

[00:07:01] travis_time:end:stage0-rustc:start=1532722824844815578,finish=1532722890169492172,duration=65324676594


[00:07:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:01] Build completed unsuccessfully in 0:02:04
[00:07:01] Makefile:28: recipe for target 'all' failed
[00:07:01] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c501cfc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c2cecfc:start=1532722890696536133,finish=1532722890702997128,duration=6460995
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06099369
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:174e5410
travis_time:start:174e5410
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33bcdf6c
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.462521] init: failsafe main process (1095) killed by TERM signal
[   41.754579] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
