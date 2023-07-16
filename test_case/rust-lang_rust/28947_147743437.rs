
$ rustc -g --emit llvm-bc src/libsyntax/lib.rs
$ opt -O2 -disable-output syntax.bc
LLVM ERROR: Broken function found, compilation aborted!
$ opt -O2 -verify-each -disable-output syntax.bc
