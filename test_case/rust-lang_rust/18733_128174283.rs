
$ make check-stage1-std NO_REBUILD=1 NO_BENCH=1
cfg: version 1.4.0-dev (d03456183 2015-08-05)
cfg: build triple x86_64-pc-windows-gnu
cfg: host triples x86_64-pc-windows-gnu
cfg: target triples x86_64-pc-windows-gnu
cfg: host for x86_64-pc-windows-gnu is x86_64
cfg: os for x86_64-pc-windows-gnu is pc-windows-gnu
cfg: good valgrind for x86_64-pc-windows-gnu is
cfg: using CC=gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
cfg: no xelatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
cfg: including test rules
cfg: javac not available, skipping lexer test...
run: x86_64-pc-windows-gnu/stage1/test/stdtest-x86_64-pc-windows-gnu.exe
/c/Users/Diggory/Projects/rust/x86_64-pc-windows-gnu/stage1/test/stdtest-x86_64-pc-windows-gnu.exe: error while loading shared libraries: test-a5fc0d6c.dll: cannot open shared object file: No such file or directory
                                                      /c/Users/Diggory/Projects/rust/mk/tests.mk:450: recipe for target 'tmp/check-stage1-T-x86_64-pc-windows-gnu-H-x86_64-pc-windows-gnu-std.ok' failed
make: *** [tmp/check-stage1-T-x86_64-pc-windows-gnu-H-x86_64-pc-windows-gnu-std.ok] Error 127

Diggory@Diggory-Laptop MINGW64 /c/Users/Diggory/Projects/rust
$ rustc
bash: /usr/local/bin/rustc: No such file or directory
