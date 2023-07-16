
$ strace -e write sh -c 'exec ./test 2> /dev/null'
write(2, "partial ", 8)                 = 8
write(2, "line", 4)                     = 4
write(2, "\n", 1)                       = 1
write(2, "a one and a two and a ", 22)  = 22
write(2, "3", 1)                        = 1
write(2, " and a ", 7)                  = 7
write(2, "4", 1)                        = 1
write(2, "\n", 1)                       = 1
