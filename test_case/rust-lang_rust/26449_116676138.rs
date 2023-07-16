
$ rustc -O -g --crate-type=lib --emit=link --emit=obj -C target-feature=-mmx --crate-name core lib.rs
LLVM ERROR: SSE register return with SSE disabled
$ rustc -O -g --crate-type=lib --emit=link --emit=obj -C target-feature=-sse --crate-name core lib.rs
LLVM ERROR: SSE register return with SSE disabled
$ rustc -O -g --crate-type=lib --emit=link --emit=obj -C target-feature=-sse2 --crate-name core lib.rs
LLVM ERROR: SSE2 register return with SSE2 disabled
