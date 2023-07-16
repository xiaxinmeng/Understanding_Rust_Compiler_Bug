bash
#!/bin/bash

mkdir -p $CARGO_TARGET_DIR
cp -r target/criterion $CARGO_TARGET_DIR
cargo bench -- -b 1.53.0 --sample-size 200
