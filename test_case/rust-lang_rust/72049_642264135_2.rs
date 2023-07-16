sh
gcc -c foo.c
ar rvs libfoo.a foo.o
rustc main.rs -L .
