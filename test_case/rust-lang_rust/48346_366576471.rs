
$ time ./no-pgo 10000000
10000000
-0.169075164
-0.169077842
./no-pgo 10000000  2.01s user 0.00s system 99% cpu 2.010 total
$ time ./pgo 10000000
10000000
-0.169075164
-0.169077842
./pgo 10000000  0.43s user 0.00s system 99% cpu 0.433 total
