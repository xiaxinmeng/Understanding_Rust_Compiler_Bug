
$ rpm -q llvm
llvm-7.0.1-1.fc30.x86_64
$ llvm-config --link-static --system-libs
-lz -lrt -ldl -ltinfo -lpthread -lm
