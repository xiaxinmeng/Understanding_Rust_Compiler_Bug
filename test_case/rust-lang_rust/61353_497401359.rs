plain
travis_time:end:00e95dca:start=1559228274886992502,finish=1559228277556573537,duration=2669581035
travis_fold:end:before_install.3
travis_fold:start:install
travis_time:start:1e66022e
$ case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$SCRIPT" == "./x.py dist" ]]; then travis_retry brew update && travis_retry brew install xz && travis_retry brew install swig; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac
travis_fold:end:install
travis_fold:start:before_script.1
travis_time:start:08809bbe
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
[01:51:39] 
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:168:45
[01:51:39]     |
[01:51:39] 168 |     Angle(winit::Window, angle::Context, Rc<gl::Gl>),
[01:51:39] 
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:169:34
[01:51:39]     |
---
[01:51:39] 
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:263:35
[01:51:39]     |
[01:51:39] 263 |             let init = |context: &glutin::GlContext| {
[01:51:39]     |                                   ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn glutin::GlContext`
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:339:28
[01:51:39]     |
[01:51:39] 339 |     fn clone(&self) -> Box<RenderNotifier> {
[01:51:39] 339 |     fn clone(&self) -> Box<RenderNotifier> {
[01:51:39]     |                            ^^^^^^^^^^^^^^ help: use `dyn`: `dyn RenderNotifier`
[01:51:39] 
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:364:30
[01:51:39]     |
[01:51:39] 364 | fn create_notifier() -> (Box<RenderNotifier>, Receiver<NotifierEvent>) {
[01:51:39] 
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:571:35
[01:51:39]     |
[01:51:39]     |
[01:51:39] 571 |         Box::new(captured) as Box<WrenchThing>
[01:51:39]     |                                   ^^^^^^^^^^^ help: use `dyn`: `dyn WrenchThing`
[01:51:39] 
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:580:80
[01:51:39]     |
[01:51:39] 580 |             "yaml" => Box::new(YamlFrameReader::new_from_args(subargs)) as Box<WrenchThing>,
[01:51:39] 
[01:51:39] warning: trait objects without an explicit `dyn` are deprecated
[01:51:39]    --> wrench/src/main.rs:581:81
[01:51:39]     |
---
[01:55:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:55:08] expected success, got: exit code: 101
[01:55:08] 
[01:55:08] 
[01:55:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:55:08] Build completed unsuccessfully in 1:51:56
[01:55:08] Makefile:50: recipe for target 'check-aux' failed
[01:55:08] make: *** [check-aux] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0566b604
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 16:53:07 UTC 2019
---
travis_time:end:02bea4a6:start=1559235191293812176,finish=1559235191305146168,duration=11333992
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1116d536
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.28194.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 28198]
[New LWP 28194]
warning: Could not load shared library symbols for 8 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `rustc --crate-name foo src/lib.rs --color never --crate-type lib --emit=dep-inf'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007f6c0e934428 in ?? ()
[Current thread is 1 (LWP 28198)]
#0  0x00007f6c0e934428 in ?? ()
#1  0x00007f6c0e93602a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_time:end:1116d536:start=1559235191312367565,finish=1559235193133102579,duration=1820735014
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09b3b5b0
travis_time:start:09b3b5b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a465fa0
$ dmesg | grep -i kill
