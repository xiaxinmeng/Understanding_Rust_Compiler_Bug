
$ cat test.sh
#!/usr/bin/env bash
cargo build 2>&1 | grep "internal compiler error"
