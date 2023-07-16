plain
[00:56:04] ....i......................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:56:09] .........
[00:56:41] ....................................................................................................
[00:57:10] ......................................................................ii............................
[00:58:02] .................................i....................................................i.ii.....test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:58:47] ..............................................................................................iiiiii
[00:59:14] i...................................................................................................
[00:59:44] ....................................................................................................
[01:00:10] ....................................................................................................
---
[01:33:32] travis_fold:end:stage0-linkchecker

[01:33:32] travis_time:end:stage0-linkchecker:start=1524363993598161550,finish=1524363996559484103,duration=2961322553

[01:33:35] std/intrinsics/fn.va_end.html:2: broken link fragment `#fn.va_start` pointing to `std/intrinsics/fn.va_end.html`
[01:33:35] std/intrinsics/fn.va_end.html:3: broken link fragment `#fn.va_copy` pointing to `std/intrinsics/fn.va_end.html`
[01:33:35] std/intrinsics/fn.va_start.html:2: broken link fragment `#fn.va_arg` pointing to `std/intrinsics/fn.va_start.html`
[01:33:35] std/intrinsics/index.html:1859: broken link fragment `#fn.va_start` pointing to `std/intrinsics/index.html`
[01:33:35] std/intrinsics/index.html:1860: broken link fragment `#fn.va_copy` pointing to `std/intrinsics/index.html`
[01:33:35] std/intrinsics/index.html:1868: broken link fragment `#fn.va_arg` pointing to `std/intrinsics/index.html`
[01:33:43] core/intrinsics/fn.va_end.html:2: broken link fragment `#fn.va_start` pointing to `core/intrinsics/fn.va_end.html`
[01:33:43] core/intrinsics/fn.va_end.html:3: broken link fragment `#fn.va_copy` pointing to `core/intrinsics/fn.va_end.html`
[01:33:43] core/intrinsics/fn.va_start.html:2: broken link fragment `#fn.va_arg` pointing to `core/intrinsics/fn.va_start.html`
[01:33:43] core/intrinsics/index.html:1852: broken link fragment `#fn.va_start` pointing to `core/intrinsics/index.html`
[01:33:43] core/intrinsics/index.html:1853: broken link fragment `#fn.va_copy` pointing to `core/intrinsics/index.html`
[01:33:43] core/intrinsics/index.html:1861: broken link fragment `#fn.va_arg` pointing to `core/intrinsics/index.html`

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:039409e6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
