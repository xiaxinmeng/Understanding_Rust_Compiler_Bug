 bash
$ make -j4 -- all VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y -Ccodegen-units=4' RUST_BACKTRACE=1
...
//with -j4 downloading the snapshot is done at startup! Cool!
...
cfg: version 1.5.0-dev (9a855668f 2015-10-23)
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
/bin/sh: x86_64-unknown-linux-gnu/stage2/bin/rustdoc: No such file or directory
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/docs.mk:138: recipe for target 'doc/grammar.html' failed
make: *** [doc/grammar.html] Error 127
make: *** Waiting for unfinished jobs....
...

