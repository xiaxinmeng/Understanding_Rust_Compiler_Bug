
clang-format -style=file --assume-filename=../../contrib/clang-format rust-diagnostics.cc  > /dev/shm/aa.cc
diff /dev/shm/aa.cc rust-diagnostics.cc
