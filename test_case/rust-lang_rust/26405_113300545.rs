
$ make -j10
cfg: version 1.2.0-dev (f45181276 2015-06-18)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples i686-unknown-linux-gnu x86_64-unknown-linux-gnu
cfg: target triples i686-unknown-linux-gnu x86_64-unknown-linux-gnu
cfg: non-build host triples i686-unknown-linux-gnu
cfg: non-build target triples i686-unknown-linux-gnu
cfg: host for i686-unknown-linux-gnu is i386
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for i686-unknown-linux-gnu is unknown-linux-gnu
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for i686-unknown-linux-gnu is 1
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: enabling valgrind run-pass tests (CFG_ENABLE_VALGRIND_RPASS)
cfg: valgrind-rpass command set to "/usr/bin/valgrind" --error-exitcode=100 --fair-sched=try --quiet --soname-synonyms=somalloc=NONE --suppressions=/home/alex/code/rust2/src/etc/x86.supp  --tool=memcheck --leak-check=full
cfg: no xelatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
rustc: i686-unknown-linux-gnu/stage0/lib/rustlib/i686-unknown-linux-gnu/lib/libcore
i686-unknown-linux-gnu/stage0/bin/rustc: error while loading shared libraries: librustc_driver-d8ace771.so: cannot open shared object file: No such file or directory
make: *** [i686-unknown-linux-gnu/stage0/lib/rustlib/i686-unknown-linux-gnu/lib/stamp.core] Error 127
make: INTERNAL: Exiting with 11 jobserver tokens available; should be 10!
