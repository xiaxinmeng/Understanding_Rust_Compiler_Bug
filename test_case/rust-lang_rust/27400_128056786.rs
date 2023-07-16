
$ time ./binary-jemalloc 18
stretch tree of depth 19         check: -1
524288   trees of depth 4        check: -524288
131072   trees of depth 6        check: -131072
32768    trees of depth 8        check: -32768
8192     trees of depth 10       check: -8192
2048     trees of depth 12       check: -2048
512      trees of depth 14       check: -512
128      trees of depth 16       check: -128
32       trees of depth 18       check: -32
long lived tree of depth 18      check: -1
./binary-jemalloc 18  5.88s user 0.57s system 375% cpu 1.715 total
$ time ./binary-system 18   
stretch tree of depth 19         check: -1
524288   trees of depth 4        check: -524288
131072   trees of depth 6        check: -131072
32768    trees of depth 8        check: -32768
8192     trees of depth 10       check: -8192
2048     trees of depth 12       check: -2048
512      trees of depth 14       check: -512
128      trees of depth 16       check: -128
32       trees of depth 18       check: -32
long lived tree of depth 18      check: -1
./binary-system 18  5.14s user 0.02s system 370% cpu 1.395 total
