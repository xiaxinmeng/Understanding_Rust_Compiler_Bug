
$ clang++ -E -x c++ - -v < /dev/null
...
#include <...> search starts here:
 /usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0
 /usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/x86_64-pc-linux-gnu
 /usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/backward
 /usr/local/include
 /usr/lib/clang/8.0.1/include
 /usr/include
End of search list.
...

$ clang++ --target=x86_64-unknown-linux-gnu -E -x c++ - -v < /dev/null
...
#include <...> search starts here:
 /usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0
 /usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/x86_64-pc-linux-gnu
 /usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../include/c++/9.1.0/backward
 /usr/local/include
 /usr/lib/clang/8.0.1/include
 /usr/include
End of search list.
...
