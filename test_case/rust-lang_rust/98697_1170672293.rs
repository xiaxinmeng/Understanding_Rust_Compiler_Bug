console
$ cat bisect.sh 
#!/bin/bash
cargo doc
not rg "for&lt;" $CARGO_TARGET_DIR/doc

$ cargo-bisect-rustc --start 2022-03-31 --end 2022-06-30 --script ./bisect.sh
