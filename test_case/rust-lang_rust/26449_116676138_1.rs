
$ rustc --crate-type=lib --emit=link -O -C soft-float -C target-feature=-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2 -g --emit=obj -C soft-float -C no-vectorize-slp -C no-vectorize-loops --crate-name core lib.rs
