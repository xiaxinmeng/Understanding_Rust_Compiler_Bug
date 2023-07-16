
compile_and_link: i686-unknown-linux-gnu/stage0/lib/rustc/i686-unknown-linux-gnu/lib/librustc.so
terminate called after throwing an instance of 'std::bad_alloc'
  what():  std::bad_alloc
Stack dump:
0.      Running pass 'Function Pass Manager' on module 'rustc.rc'.
1.      Running pass 'Live IR Variables' on function '@_ZN6middle8borrowck11check_loans14__extensions__10check_call4anonE'
Aborted
make: *** [i686-unknown-linux-gnu/stage0/lib/rustc/i686-unknown-linux-gnu/lib/librustc.so] Error 134
