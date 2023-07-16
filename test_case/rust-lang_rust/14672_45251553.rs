
$ make   
gcc -c foobar.c -o fooba.o
ar crus libfoobar.a fooba.o
rustc foo.rs -L.
rustc bar.rs -L.
$ lldb ./bar      
Current executable set to './bar' (x86_64).
(lldb) b main
libc++abi.dylib: terminating with uncaught exception of type std::out_of_range: basic_string
zsh: abort      lldb ./bar
