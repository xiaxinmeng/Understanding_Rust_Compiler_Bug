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
...
determined most recent snapshot: rust-stage0-2015-08-11-1af31d4-linux-x86_64-7df8ba9dec63ec77b857066109d4b6250f3d222f.tar.bz2
curl: (6) Couldn't resolve host 'static.rust-lang.org'
Traceback (most recent call last):
  File "/home/zazdxscf/build/1nonpkgs/rust/rust/src/etc/get-snapshot.py", line 78, in <module>
    main(sys.argv)
  File "/home/zazdxscf/build/1nonpkgs/rust/rust/src/etc/get-snapshot.py", line 66, in main
    get_url_to_file(url, dl)
  File "/home/zazdxscf/build/1nonpkgs/rust/rust/src/etc/snapshot.py", line 163, in get_url_to_file
    raise Exception("failed to fetch url")
Exception: failed to fetch url
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/stage0.mk:17: recipe for target 'x86_64-unknown-linux-gnu/stage0/bin/rustc' failed
make: *** [x86_64-unknown-linux-gnu/stage0/bin/rustc] Error 1

real    21m4.541s
user    63m10.920s
sys 6m48.963s
