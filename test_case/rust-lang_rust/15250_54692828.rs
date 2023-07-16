
retep998@Peter-Desktop ~/rust
$ make check-fast
cfg: build triple x86_64-w64-mingw32
cfg: host triples x86_64-w64-mingw32
cfg: target triples x86_64-w64-mingw32
cfg: enabling more debugging (CFG_ENABLE_DEBUG)
cfg: host for x86_64-w64-mingw32 is x86_64
cfg: os for x86_64-w64-mingw32 is w64-mingw32
cfg: using CC=gcc (CFG_CC)
cfg: disabling valgrind due to its unreliability on this platform
/home/retep998/rust/mk/stage0.mk:7: warning: overriding recipe for target 'x86_64-w64-mingw32/stage0/bin/'
/home/retep998/rust/mk/stage0.mk:4: warning: ignoring old recipe for target 'x86_64-w64-mingw32/stage0/bin/'
cfg: no pdflatex found, deferring to xelatex
cfg: no xelatex found, deferring to lualatex
cfg: no lualatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
cfg: no llnextgen found, omitting grammar-verification
cfg: including test rules
cfg: javac not available, skipping lexer test...
make: Circular /home/retep998/rust/src/libuv/libuv.a <- /home/retep998/rust/src/libuv/libuv.a dependency dropped.
maketest: bootstrap-from-c-with-green
----- /home/retep998/rust/src/test/run-make/bootstrap-from-c-with-green/ --------------------
------ stdout ---------------------------------------------
make[1]: Entering directory '/home/retep998/rust/src/test/run-make/bootstrap-from-c-with-green'
PATH="/home/retep998/rust/x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green:/home/retep998/rust/x86_64-w64-mingw32/stage2/bin:/mingw64/bin:/usr/local/bin:/usr/bin:/bin:/c/WINDOWS/system32:/c/WINDOWS:/a/rust64/bin:/a/mingw/mingw64/bin:/a/mingw/mingw64/opt/bin:/a/cmake/bin:/a/git/bin:/usr/bin/vendor_perl:/usr/bin/core_perl" /home/retep998/rust/x86_64-w64-mingw32/stage2/bin/rustc.exe --out-dir /home/retep998/rust/x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green -L /home/retep998/rust/x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green lib.rs
gcc -Wall -Werror -g -m64 -D_WIN32_WINNT=0x0600  -L /home/retep998/rust/x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green main.c -o /home/retep998/rust/x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green/main  -lboot
PATH="/mingw64/bin:/usr/local/bin:/usr/bin:/bin:/c/WINDOWS/system32:/c/WINDOWS:/a/rust64/bin:/a/mingw/mingw64/bin:/a/mingw/mingw64/opt/bin:/a/cmake/bin:/a/git/bin:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/retep998/rust/x86_64-w64-mingw32/stage1/bin/rustlib/x86_64-w64-mingw32/lib:/home/retep998/rust/x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green" /home/retep998/rust/x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green/main
Makefile:8: recipe for target 'all' failed
make[1]: Leaving directory '/home/retep998/rust/src/test/run-make/bootstrap-from-c-with-green'

------ stderr ---------------------------------------------
make[1]: *** [all] Error 127

------        ---------------------------------------------

/home/retep998/rust/mk/tests.mk:1011: recipe for target 'x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green-2-T-x86_64-w64-mingw32-H-x86_64-w64-mingw32.ok' failed
make: *** [x86_64-w64-mingw32/test/run-make/bootstrap-from-c-with-green-2-T-x86_64-w64-mingw32-H-x86_64-w64-mingw32.ok] Error 2

retep998@Peter-Desktop ~/rust
$
