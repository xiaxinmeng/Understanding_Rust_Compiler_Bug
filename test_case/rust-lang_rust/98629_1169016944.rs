bash
cargo bisect-rustc --start=2022-04-01 --script=./test.sh 

$ cat test.sh 
#!/bin/bash
cargo check 2>&1 | grep "from trait"
