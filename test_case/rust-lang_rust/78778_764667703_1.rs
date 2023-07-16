console
$ build/x86_64-unknown-linux-gnu/stage0-tools-bin/miri --version
build/x86_64-unknown-linux-gnu/stage0-tools-bin/miri: error while loading shared libraries: librustc_driver-110e2444bfbc1fba.so: cannot open shared object file: No such file or directory

$ export LD_LIBRARY_PATH=build/x86_64-unknown-linux-gnu/stage1/lib/
$ build/x86_64-unknown-linux-gnu/stage0-tools-bin/miri --version
rustc 1.51.0-dev
