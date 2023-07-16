
$ time rustc main.rs
real	0m2.223s
user	0m2.115s
sys	0m0.107s
$ time rustc -C llvm-args=--x86-experimental-lvi-inline-asm-hardening -C target-feature=+lvi-cfi,+lvi-load-hardening main.rs
real	1m47.199s
user	1m47.015s
sys	0m0.158s
