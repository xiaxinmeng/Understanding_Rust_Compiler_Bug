
% make libfoo.so
gcc -c -Wall -Werror -fpic -o foo.o foo.c
gcc -shared -o libfoo.so foo.o
% make test-1
gcc -Wall -L/home/pnkfelix/Dev/Mozilla/rpath -o test-1 main.c -lfoo
% make test-2
gcc -Wall -L/home/pnkfelix/Dev/Mozilla/rpath -lfoo -o test-2 main.c
/tmp/ccg3Pya4.o: In function `main':
main.c:(.text+0xf): undefined reference to `foo'
collect2: error: ld returned 1 exit status
make: *** [test-2] Error 1
% 
