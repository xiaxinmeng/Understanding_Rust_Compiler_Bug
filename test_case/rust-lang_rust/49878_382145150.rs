plain
Resolving deltas: 100% (616705/616705), completed with 4918 local objects.
---
[00:00:57] configure: rust.quiet-tests     := True
---
[00:46:56] .................................................................................i..................
[00:47:02] ........................i...........................................................................
---
[00:47:46] .....................i...........................................................................i..
[00:47:52] ....................................................................................................
[00:47:59] ...........ii.......................................................................................
[00:48:07] ............................................................................................i.......
---
[00:48:47] .............................................i......................................................
---
[00:52:55] ..............................i.....................................................................
[00:53:09] ...............................................................i....................................
[00:53:25] .................................................i..................................................
[00:53:46] ....................................................................................................
[00:54:08] ....................................................................................................
[00:54:30] ....................................................................................................
[00:54:56] .......i............................................................................................
[00:55:23] ...i.............................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:55:33] ...................
[00:56:06] ....................................................................................................
[00:56:39] .....................................................................ii.............................
[00:57:29] ................................i..............................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:57:37] ......i.ii...........
[00:58:21] .............................................................................................iiiiiii
---
[01:00:42] ....................i............................................................ii.iii.............
[01:00:50] ....................................................................................................
[01:00:58] .........i..............................i...........................................................
[01:01:05] ....................................................................................................
[01:01:12] ...........i........................................................................................
[01:01:21] ....................................................................................................
[01:01:30] ....................................................................................................
[01:01:41] ....................................................................................................
[01:01:52] ....................................................................................................
[01:02:06] ....................................................................................................
[01:02:14] ....i...............................................................................................
[01:02:23] ........i..ii.......................................................................................
[01:02:34] ....................................................................................................
[01:02:43] ....................................................................................................
[01:02:53] ..........................................................................i.........................
[01:03:03] ....................i...............................................................................
---
[01:03:38] ...........................i........................................................................
[01:03:39] ....................................................................i...............................
[01:03:40] ...............i.......................................................
---
[01:03:57] .............i........................
---
[01:04:28] i...i..ii....i.............ii.........iii......i..i...i...ii..i..i..ii.....
---
[01:04:31] i.......i......................i.......
---
[01:05:10] iiii.......i..i........i..i.i.............i..........iiii...........i...i..........ii.i.i.......ii..
[01:05:12] ....ii...
---
[01:13:29] ..................i.................................................................................
[01:15:14] ........i...........................................................................................
---
[01:17:21] .....................................i..............................................................
[01:17:43] ....................................................................................................
[01:18:06] ..........................................i.........................................................
---
[01:19:39] .........................................................ii.........................................
---
[01:21:06] .....................i..............................................................................
---
[01:26:42] ii..................................................................................................
[01:27:03] ....................................................................................................
[01:27:20] .......................iii......i......i...i......i.................................................
[01:27:30] ....................................................................................................
[01:27:46] .............................................iiii........ii.........................................
[01:27:58] ....................................................................................................
[01:28:15] ............................................................................................i.......
[01:28:41] ....................................................................................................
[01:28:53] ....................................................................................................
[01:29:04] ..iiii...............................................
---
[01:35:16] std/intrinsics/fn.va_end.html:2: broken link - std/intrinsics/fn.va_start
[01:35:16] std/intrinsics/fn.va_end.html:3: broken link - std/intrinsics/fn.va_copy
[01:35:16] std/intrinsics/fn.va_start.html:2: broken link - std/intrinsics/fn.va_arg
[01:35:16] std/intrinsics/index.html:1859: broken link - std/intrinsics/fn.va_start
[01:35:16] std/intrinsics/index.html:1860: broken link - std/intrinsics/fn.va_copy
[01:35:16] std/intrinsics/index.html:1868: broken link - std/intrinsics/fn.va_arg
[01:35:25] core/intrinsics/fn.va_end.html:2: broken link - core/intrinsics/fn.va_start
[01:35:25] core/intrinsics/fn.va_end.html:3: broken link - core/intrinsics/fn.va_copy
[01:35:25] core/intrinsics/fn.va_start.html:2: broken link - core/intrinsics/fn.va_arg
[01:35:25] core/intrinsics/index.html:1852: broken link - core/intrinsics/fn.va_start
[01:35:25] core/intrinsics/index.html:1853: broken link - core/intrinsics/fn.va_copy
[01:35:25] core/intrinsics/index.html:1861: broken link - core/intrinsics/fn.va_arg
[01:35:31] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
---
[01:35:31] make: *** [check] Error 1
[01:35:31] Makefile:58: recipe for target 'check' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:16f54d06:start=1523998992908371833,finish=1523998992923970785,duration=15598952
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0600ccca
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0600ccca:start=1523998992930712028,finish=1523998992937289954,duration=6577926
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:020702ae
$ dmesg | grep -i kill
[   10.678826] init: failsafe main process (1096) killed by TERM signal
