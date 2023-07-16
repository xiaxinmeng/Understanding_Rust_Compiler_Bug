
root@atlantis:~> clang-7 floattest.c -o floattest-llvm -Xclang -target-feature -Xclang +spe
root@atlantis:~> ./floattest-llvm
The result of 3.400000 + 7.200000 is 10.600000:
root@atlantis:~> clang-4.0 floattest.c -o floattest-llvm-4 -Xclang -target-feature -Xclang +spe -I /usr/include/powerpc-linux-gnuspe/
root@atlantis:~> ./floattest-llvm-4
The result of 7.199997 + 7.000000 is 0.000000:
root@atlantis:~>
