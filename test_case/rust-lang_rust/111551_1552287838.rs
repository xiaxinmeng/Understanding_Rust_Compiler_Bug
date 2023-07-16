plain
error: make failed
status: exit code: 2
command: "make"
--- stdout -------------------------------
make[1]: Entering directory '/c/a/rust/rust/tests/run-make/extern-fn-explicit-align'
'C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64\cl.exe' -nologo -MT -Brepro -c -Fo:`cygpath -w /c/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o` test.c
test.c
test.c(10): error C2143: syntax error: missing ')' before '('
test.c(10): error C2091: function returns function
test.c(10): error C2143: syntax error: missing ')' before 'constant'
test.c(10): error C2143: syntax error: missing '{' before 'constant'
test.c(10): error C2059: syntax error: 'constant'
test.c(10): error C2059: syntax error: ')'
make[1]: Leaving directory '/c/a/rust/rust/tests/run-make/extern-fn-explicit-align'
--- stderr -------------------------------
--- stderr -------------------------------
make[1]: *** [../tools.mk:187: /c/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o] Error 2



failures:
failures:
    [run-make] tests\run-make\extern-fn-explicit-align

test result: FAILED. 239 passed; 1 failed; 80 ignored; 0 measured; 0 filtered out; finished in 78.00s

Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Build completed unsuccessfully in 0:59:58
make: *** [Makefile:68: ci-subset-1] Error 1
