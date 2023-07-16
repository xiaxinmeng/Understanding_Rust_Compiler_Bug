
Nam@Nam-PC ~/rust/build
$ make check-stage2-cfail-full TESTNAME=syntax-extension-regex-invalid
cfg: build triple i686-pc-mingw32
cfg: host triples i686-pc-mingw32
cfg: target triples i686-pc-mingw32
cfg: enabling more debugging (CFG_ENABLE_DEBUG)
cfg: host for i686-pc-mingw32 is i386
cfg: os for i686-pc-mingw32 is pc-mingw32
cfg: using CC=gcc (CFG_CC)
cfg: disabling valgrind due to its unreliability on this platform
/home/Nam/rust/mk/stage0.mk:7: warning: overriding recipe for target 'i686-pc-mingw32/stage0/bin/'
/home/Nam/rust/mk/stage0.mk:4: warning: ignoring old recipe for target 'i686-pc-mingw32/stage0/bin/'
cfg: no pdflatex found, deferring to xelatex
cfg: no xelatex found, deferring to lualatex
cfg: no lualatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
cfg: no llnextgen found, omitting grammar-verification
cfg: disabling doc build (CFG_DISABLE_DOCS)
cfg: including test rules
run cfail-full [i686-pc-mingw32]: i686-pc-mingw32/stage2/bin/compiletest.exe

running 1 test
test [compile-fail] compile-fail-fulldeps/syntax-extension-regex-invalid.rs ... ok

using metrics ratchet: tmp\check-stage2-T-i686-pc-mingw32-H-i686-pc-mingw32-cfail-full-metrics.json
result of ratchet: 0 metrics added, 0 removed, 0 improved, 0 regressed, 0 noise
updated ratchet file

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
