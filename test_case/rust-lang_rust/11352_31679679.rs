
$ cat foo.c
#include <sys/filio.h>
#include <stdio.h>

int main() {
    int x = 3;
}
$ clang -g foo.c -o foo
$ lldb ./foo
2014-01-06 11:40:04.437 lldb[89479:212f] Metadata.framework [Error]: couldn't get the client port
Current executable set to './foo' (x86_64).
(lldb) b main
Breakpoint 1: where = foo`main + 9 at foo.c:5, address = 0x0000000100000f89
(lldb) r
Process 89484 launched: './foo' (x86_64)
Process 89484 stopped
* thread #1: tid = 0x76b78b, 0x0000000100000f89 foo`main + 9 at foo.c:5, queue = 'com.apple.main-thread, stop reason = breakpoint 1.1
    frame #0: 0x0000000100000f89 foo`main + 9 at foo.c:5
   2    #include <stdio.h>
   3   
   4    int main() {
-> 5        int x = 3;
   6    }
