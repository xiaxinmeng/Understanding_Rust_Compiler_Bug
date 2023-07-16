bash
#!/bin/bash

set -e

while true; do
    cargo +nightly clean
    cargo +nightly build --all-targets
    echo >> src/lib.rs
    RUST_BACKTRACE=1 cargo +nightly build --all-targets
done
