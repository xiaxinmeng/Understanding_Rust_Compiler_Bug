
[1:31:17]~/hacking/rust/src/test/bench> for s in $stacks; do for e in $exes; do echo "stacksize=$s exe=$e"; RUST_MIN_STACK=$e /usr/bin/time -f '%E' $e; done done
stacksize=4194304 exe=./silly-test-spawn-mmap
0:01.76
stacksize=4194304 exe=./silly-test-spawn
0:00.92
stacksize=8388608 exe=./silly-test-spawn-mmap
0:01.75
stacksize=8388608 exe=./silly-test-spawn
0:00.89
stacksize=1048576 exe=./silly-test-spawn-mmap
0:01.81
stacksize=1048576 exe=./silly-test-spawn
0:00.89
stacksize=8092 exe=./silly-test-spawn-mmap
0:01.72
stacksize=8092 exe=./silly-test-spawn
0:00.94
[1:31:35]~/hacking/rust/src/test/bench> for s in $stacks; do for e in $exes; do echo "stacksize=$s exe=$e"; RUST_MIN_STACK=$e /usr/bin/time -f '%E' jemalloc.sh $e; done done
stacksize=4194304 exe=./silly-test-spawn-mmap
0:01.82
stacksize=4194304 exe=./silly-test-spawn
0:01.13
stacksize=8388608 exe=./silly-test-spawn-mmap
0:01.80
stacksize=8388608 exe=./silly-test-spawn
0:01.12
stacksize=1048576 exe=./silly-test-spawn-mmap
0:01.78
stacksize=1048576 exe=./silly-test-spawn
0:01.13
stacksize=8092 exe=./silly-test-spawn-mmap
0:01.80
stacksize=8092 exe=./silly-test-spawn
0:01.12
