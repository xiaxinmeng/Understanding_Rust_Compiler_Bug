plain
[00:48:09] ....................................................................................................
[00:48:20] ..............................................................................................i.....
[00:48:29] ....................................................................................................
[00:48:42] ....................................................................................................
[00:48:52] ...................................................................F................................
[00:49:11] .........................................................................i..........................
[00:49:28] ....................................................................................................
[00:49:37] ....................................................................................................
[00:49:52] ...........................................ii.......................................................
---
travis_time:end:2b1ac707:start=1532612028373721577,finish=1532612028383965910,duration=10244333
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a31f4a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
