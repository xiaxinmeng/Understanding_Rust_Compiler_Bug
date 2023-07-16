
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/pass/concurrency/sync.rs" "--error-format=json" "-Zmiri-disable-isolation" "-Zmiri-strict-provenance"
Pass got exit status: 1
actual output differed from expected tests/pass/concurrency/sync.stderr
Diff< left / right > :
>error: the main thread terminated without waiting for all remaining threads
>
>note: pass `-Zmiri-ignore-leaks` to disable this check
>
>error: aborting due to previous error
