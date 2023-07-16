console
$ cat a.sh
#!/bin/bash
rustc -O -C debug-assertions=on a.rs && ./a
$ cargo-bisect-rustc --preserve --without-cargo --script ./a.sh
...
********************************************************************************
Regression in bc4b39c271bbd36736cbf1c0a1ac23d5df38d365
********************************************************************************
