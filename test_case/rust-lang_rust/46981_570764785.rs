sh
find Cargo.* src/ static/ ... -type f | entr -r sh -c 'cargo run < /dev/null'
