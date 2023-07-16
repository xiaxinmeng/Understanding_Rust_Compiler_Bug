
# On 32-bit x86, SSE2 is preferred if available
$ rustc -S test.rs -o - | grep mul
fmulp
$ rustc --target-feature +sse2 -S test.rs -o - | grep mul
mulsd
