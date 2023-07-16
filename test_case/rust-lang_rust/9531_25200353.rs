
% rm -rf *
% ../configure --enable-debug --enable-optimize  --enable-ccache --enable-clang --prefix=~/opt/rust-dbg
%  remake --trace -j16
...
note: ld: library not found for -lmorestack
clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error
remake: *** [x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/libstd.dylib] Error 101
Command-line invocation:
    "/Users/pnkfelix/opt-remake/bin/remake --trace -j16"
