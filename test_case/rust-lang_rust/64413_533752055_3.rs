
$ strace -e write sh -c 'exec ./test 2> /dev/null'
write(2, "partial ", 8)                 = 8
write(2, "line\n", 5)                   = 5
write(2, "a one and a two and a 3 and a 4."..., 39) = 39
