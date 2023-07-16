 bash
$ pwd
/home/emacs/build/rust/x86_64-unknown-linux-gnu/stage2/bin

$ ./rustc
./rustc: error while loading shared libraries: librustc_driver-4e7c5e5c.so: cannot open shared object file: No such file or directory

#or as the make help says: run it directly from the build directory (taken literally)
$ pwd
/home/emacs/build/rust
$ ./x86_64-unknown-linux-gnu/stage2/bin/rustc
./x86_64-unknown-linux-gnu/stage2/bin/rustc: error while loading shared libraries: librustc_driver-4e7c5e5c.so: cannot open shared object file: No such file or directory
