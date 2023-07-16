
Linkcheck stage2 (i686-pc-windows-gnu)
panic_abort\fn.rust_eh_personality.html:50: broken link - src\panic_abort\lib.rs.html
panic_abort\fn.__rust_maybe_catch_panic.html:50: broken link - src\panic_abort\lib.rs.html
panic_abort\fn.__rust_start_panic.html:50: broken link - src\panic_abort\lib.rs.html
panic_abort\index.html:50: broken link - src\panic_abort\lib.rs.html
panic_unwind\fn.rust_eh_register_frames.html:50: broken link - src\panic_unwind\gcc\mod.rs.html
panic_unwind\fn.rust_eh_unregister_frames.html:50: broken link - src\panic_unwind\gcc\mod.rs.html
panic_unwind\fn.__rust_maybe_catch_panic.html:50: broken link - src\panic_unwind\lib.rs.html
panic_unwind\fn.__rust_start_panic.html:50: broken link - src\panic_unwind\lib.rs.html
panic_unwind\index.html:50: broken link - src\panic_unwind\lib.rs.html


command did not execute successfully: "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\stage2-rustc\\i686-pc-windows-gnu\\release\\linkchecker.exe" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\doc"
expected success, got: exit code: 101


Makefile:40: recipe for target 'check' failed
Zombie process: "\Device\HarddiskVolume1\dojob.exe"
thread '<main>' panicked at 'found some broken links', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\tools\linkchecker\main.rs:54
note: Run with `RUST_BACKTRACE=1` for a backtrace.
make: *** [check] Error 1
