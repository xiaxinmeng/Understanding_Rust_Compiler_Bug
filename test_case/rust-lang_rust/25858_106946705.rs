
$ git clone https://github.com/rillian/rust-ffi
[...]
$ cd rust-ffi/
$ make MACOSX_DEPLOYMENT_TARGET=10.7
rustc --crate-type staticlib test.rs 2> libtest.a.out
c++ -g -Wall -std=c++1y -c TestRust.cpp
c++ -g -Wall -std=c++1y -o test TestRust.o libtest.a -lSystem -lpthread -lc -lm
$ make clean
[...]
$ make MACOSX_DEPLOYMENT_TARGET=10.6
rustc --crate-type staticlib test.rs 2> libtest.a.out
c++ -g -Wall -std=c++1y -c TestRust.cpp
c++ -g -Wall -std=c++1y -o test TestRust.o libtest.a -lSystem -lpthread -lc -lm
ld: targeted OS version does not support use of thread local variables in __ZN6thread7current20h6880357b0f286e7cVxbE for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
make: *** [test] Error 1
