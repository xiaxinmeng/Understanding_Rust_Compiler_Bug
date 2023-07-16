bash
#!/bin/bash
rustc src/lib.rs --crate-type lib 2>&1 | not grep unreachable

#cargo-bisect-rustc --script=./bisect.sh --without-cargo --start=2020-07-10 --end=2020-08-27
