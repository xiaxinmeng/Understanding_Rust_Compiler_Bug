 bash
$ time x86_64-unknown-linux-gnu/stage0/bin/rustc -g --emit llvm-bc -Csave-temps src/libsyntax/lib.rs

real    1m27.220s
user    1m22.487s
sys 0m4.070s

$ la *.bc
-rw-r--r-- 1 zazdxscf zazdxscf 25854804 16.10.2015 16:20 syntax.0.bc
-rw-r--r-- 1 zazdxscf zazdxscf 24978784 16.10.2015 16:20 syntax.0.no-opt.bc
-rw-r--r-- 1 zazdxscf zazdxscf 25854804 16.10.2015 16:20 syntax.bc
-rw-r--r-- 1 zazdxscf zazdxscf  3356724 16.10.2015 16:20 syntax.metadata.bc

$ time opt -O2 -disable-output syntax.0.no-opt.bc

real    4m21.647s
user    4m20.130s
sys 0m0.747s

$ time opt -O2 -disable-output syntax.bc

real    4m22.069s
user    4m20.267s
sys 0m1.027s

