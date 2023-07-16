
$ rm -rf tmp
$ make check
cfg: version 1.0.0-dev (0a3e700e3 2015-04-13) (built 2015-04-14)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: disabling rustc optimization (CFG_DISABLE_OPTIMIZE)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: enabling valgrind run-pass tests (CFG_ENABLE_VALGRIND_RPASS)
cfg: valgrind-rpass command set to "/usr/bin/valgrind" --error-exitcode=100 --soname-synonyms=somalloc=NONE --quiet --suppressions=/home/alex/code/rust4/src/etc/x86.supp  --tool=memcheck --leak-check=full
cfg: no xelatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
cfg: including test rules
cfg: javac not available, skipping lexer test...
cfg: flex not available, skipping parser test...
touch: cannot touch 'tmp/install-debugger-scripts1_H_x86_64-unknown-linux-gnu-gdb.done.start_time': No such file or directory
make: *** [tmp/install-debugger-scripts1_H_x86_64-unknown-linux-gnu-gdb.done] Error 1
