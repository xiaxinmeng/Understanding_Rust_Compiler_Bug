
$ cat test.sh
#!/bin/bash

set -ex
llc \
  -filetype=obj \
   -O0 \
   $1 \
   -o foo.o

set +ex

dwarfdump -i foo.o &> foo.tmp

set -ex

if grep -q 'dwarfdump ERROR:  reference form with no valid local ref' foo.tmp; then
  echo interesting
  exit 1
fi
exit 0
$ chmod +x test.sh
$ bugpoint -compile-custom -compile-command=./test.sh ./test.test0-*.ll
...
$ llvm-dis bugpoint-reduced-simplified.bc
