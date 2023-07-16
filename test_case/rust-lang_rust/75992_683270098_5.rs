bash
#!/bin/bash
set -e
cargo clean
time timeout 35 cargo build
