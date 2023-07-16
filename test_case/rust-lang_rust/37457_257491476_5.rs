
gcc -L. -l:libfringetest.so test.c && LD_DEBUG=all LD_LIBRARY_PATH=. LD_BIND_NOW=all ./a.out 2> out.txt
