sh
#!/bin/bash

set -ex

input="
#[no_mangle]
pub extern fn foo() {
    panic!(\"foo\");
}
"

rustc="rustc /dev/stdin -O -C lto -C panic=abort -C codegen-units=1"
rustc="$rustc --emit llvm-ir,obj --crate-type staticlib"

for i in `seq 1 100`; do
  rm -rf a b
  mkdir a b
  $rustc --out-dir a <<< $input
  $rustc --out-dir b <<< $input
  a=$(md5sum a/stdin.ll | awk '{print $1}')
  b=$(md5sum b/stdin.ll | awk '{print $1}')
  if [ "$a" != "$b" ]; then
    echo IR is different
    exit 1
  fi
  a=$(md5sum a/stdin.o | awk '{print $1}')
  b=$(md5sum b/stdin.o | awk '{print $1}')
  if [ "$a" != "$b" ]; then
    echo object is different
    exit 1
  fi
done
