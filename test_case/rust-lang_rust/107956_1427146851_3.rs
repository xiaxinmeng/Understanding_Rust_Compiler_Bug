sh
 ~/devspace/stuff/hello  stable $ cargo +issue-101691 run
error: process didn't exit successfully: `rustc -vV` (exit status: 127)
--- stderr
/home/nimda/.rustup/toolchains/issue-101691/bin/rustc: error while loading shared libraries: librustc_driver-c78d16bcb49dc5d0.so: cannot open shared object file: No such file or directory
