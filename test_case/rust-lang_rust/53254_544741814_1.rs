
# windows-gnu
$ rustc simd.rs && ./simd.exe; echo $?
Illegal instruction
132

# linux-gnu
$ rustc simd.rs && ./simd; echo $?
0
