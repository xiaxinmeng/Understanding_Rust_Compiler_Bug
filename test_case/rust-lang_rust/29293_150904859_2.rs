 bash
$ time strace /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -VV 2>/tmp/strace3.log 

real    0m44.451s
user    0m44.073s
sys 0m0.360s
$ cat /tmp/strace3.log
