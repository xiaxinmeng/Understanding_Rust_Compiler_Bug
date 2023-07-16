bash

$ cat test.sh
#!/bin/sh
rustc --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 main.rs

cargo bisect-rustc --start=2022-07-01 --end=2023-01-31 --script test.sh --regress ice 
