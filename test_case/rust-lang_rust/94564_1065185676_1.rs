
[root@8b906bb3417f /]# gcc -static -pthread -Wl,--whole-archive,-lpthread,--no-whole-archive ./foo.c -o foo
[root@8b906bb3417f /]# ./foo
[root@8b906bb3417f /]#
