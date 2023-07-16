bash
cargo bisect-rustc --start=2022-01-01 --script=./script.sh 

cat script.sh
#!/bin/sh
cargo check 2>&1 | grep "does not outlive the data it points at"
