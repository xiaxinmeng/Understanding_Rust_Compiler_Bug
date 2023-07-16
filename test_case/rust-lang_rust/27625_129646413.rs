
rust $ make tidy-binaries
<snip>
check: binaries
rust $ cp x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/
rust $ make tidy-binaries
<snip>
check: binaries
Binaries checked into src:
/home/wthrowe/computing/rust/src/test/rustc
/home/wthrowe/computing/rust/mk/tests.mk:269: recipe for target 'tidy-binaries' failed
make: *** [tidy-binaries] Error 123
