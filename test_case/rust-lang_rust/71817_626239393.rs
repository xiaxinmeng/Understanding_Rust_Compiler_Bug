sh
clang -mllvm -print-after-all -mllvm -debug match.c -O2 -S -emit-llvm -o match.ll 2> match.txt
