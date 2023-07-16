\n\nWhen looking for the implementation for the trait, the compiler finds\nboth the `impl<T> MyTrait for T` where T is all types and the `impl\nMyTrait for Foo`. Since a trait cannot be implemented multiple times,\nthis is aages
24092 ./src/tools/lldb/packages/Python/lldbsuite
23856 ./src/tools/lldb/packages/Python/lldbsuite/test
23704 ./src/llvm-emscripten/test/tools
23012 ./.git/modules/src/tools/cargo
---
travis_time:end:0242df68:start=1544741210943534273,finish=1544741210947715258,duration=4180985
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:160c8b10
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:c
