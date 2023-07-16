shell
$ rustup-toolchain-install-master 87b60fa8c61c1fc5243bb1bce530da8f3f694d4d
[...]
$ echo | rustc - --emit obj --crate-type lib # create dummy object file
$ ar rsT libthin.a rust_out.o # create thin archive
$ echo | rustc +stable -l static=thin - --crate-type staticlib -L .
error: failed to add native library ./libthin.a: Unsupported archive identifier

error: aborting due to previous error

$ echo | rustc +87b60fa8c61c1fc5243bb1bce530da8f3f694d4d -l static=thin - --crate-type staticlib -L .
$ echo $?
0
