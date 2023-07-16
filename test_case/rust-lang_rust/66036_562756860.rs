bash
#!/bin/bash -x
timeout 4 /code/fuchsia-rust/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt -instcombine "$@" -o opt-out
if [ $? -eq 124 ]; then
  echo "TIMED OUT"
  exit 1
elif [ $? -eq 0 ]; then
  echo "SUCCESS"
  exit 0
else
  echo "INVALID"
  exit 0
fi
