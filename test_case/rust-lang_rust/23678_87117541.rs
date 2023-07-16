 text
cfg: version 1.0.0-dev (19bc2539c 2015-03-27) (built 2015-03-27)
cfg: build triple x86_64-apple-darwin
cfg: host triples x86_64-apple-darwin
cfg: target triples x86_64-apple-darwin
cfg: enabling more debugging (CFG_ENABLE_DEBUG)
cfg: host for x86_64-apple-darwin is x86_64
cfg: os for x86_64-apple-darwin is apple-darwin
cfg: good valgrind for x86_64-apple-darwin is 
cfg: using CC=clang (CFG_CC)
cfg: using CXX=clang++ (CFG_CXX)
cfg: enabling valgrind run-pass tests (CFG_ENABLE_VALGRIND_RPASS)
cfg: valgrind-rpass command set to "/usr/local/bin/valgrind" --error-exitcode=100 --soname-synonyms=somalloc=NONE --quiet --suppressions=/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/etc/x86.supp --suppressions=/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/etc/apple-darwin.supp --tool=memcheck --leak-check=full
cfg: no xelatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
cfg: including test rules
cfg: antlr4 not available, skipping lexer test...
The rust test suite will segfault many rustc's in the debuginfo phase.
set ALLOW_NONZERO_ULIMIT to ignore this warning
make: *** [check-sanitycheck] Error 1
