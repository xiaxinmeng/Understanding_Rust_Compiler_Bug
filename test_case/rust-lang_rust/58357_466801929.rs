plain
[01:05:57]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:05:58] [RUSTC-TIMING] panic_unwind test:false 0.296
[01:05:58] warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`
[01:05:58] 
[01:05:58] error[E0412]: cannot find type `IoVecMut` in this scope
[01:05:58]    --> src/libstd/sys/sgx/net.rs:106:44
[01:05:58]     |
[01:05:58] 106 |     pub fn read_vectored(&self, buf: &mut [IoVecMut<'_>]) -> io::Result<usize> {
[01:05:58] help: possible candidates are found in other modules, you can import them into scope
[01:05:58]     |
[01:05:58]     |
[01:05:58] 1   | use io::IoVecMut;
[01:05:58]     |
[01:05:58] 1   | use sys::sgx::io::IoVecMut;
[01:05:58] 
[01:05:58] 
[01:05:58] error[E0412]: cannot find type `IoVec` in this scope
[01:05:58]    --> src/libstd/sys/sgx/net.rs:118:41
[01:05:58]     |
[01:05:58] 118 |     pub fn write_vectored(&self, buf: &[IoVec<'_>]) -> io::Result<usize> {
[01:05:58] help: possible candidates are found in other modules, you can import them into scope
[01:05:58]     |
[01:05:58]     |
[01:05:58] 1   | use io::IoVec;
[01:05:58]     |
[01:05:58] 1   | use sys::sgx::io::IoVec;
[01:05:58] 
[01:06:02] error: aborting due to 2 previous errors
[01:06:02] 
[01:06:02] For more information about this error, try `rustc --explain E0412`.
---
travis_time:end:14fc59cb:start=1551032370250743857,finish=1551032370261280786,duration=10536929
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:335a4be4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b551e9f
travis_time:start:0b551e9f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:072a1a58
$ dmesg | grep -i kill
