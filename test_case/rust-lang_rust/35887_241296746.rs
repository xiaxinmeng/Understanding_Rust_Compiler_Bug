
$ mkdir foo
$ echo "int main() { return 0; }" >foo.cpp
$ g++ foo.cpp -o foo
ld: can't open output file for writing: foo, errno=21 for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
