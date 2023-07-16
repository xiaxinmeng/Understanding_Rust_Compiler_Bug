 shell
$ as rust_u64.s -o rust_u64.o
$ gcc rust_u64.o -L path_to_your_libstd-*.so -o rust_u64
$ ./rust_u64
