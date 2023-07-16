 bash
$ make -j1 -- all VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y' RUST_BACKTRACE=1

cfg: version 1.5.0-dev (e518c057f 2015-10-23)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
/usr/bin/python2 /home/zazdxscf/build/1nonpkgs/rust/rust/src/etc/get-snapshot.py x86_64-unknown-linux-gnu 
determined most recent snapshot: rust-stage0-2015-08-11-1af31d4-linux-x86_64-7df8ba9dec63ec77b857066109d4b6250f3d222f.tar.bz2
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 15.0M  100 15.0M    0     0  1842k      0  0:00:08  0:00:08 --:--:-- 3133k
got download with ok hash
...

