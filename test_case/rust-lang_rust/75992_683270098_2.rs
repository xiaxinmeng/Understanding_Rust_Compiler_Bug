bash
#!/bin/bash
set -e
rustc --version >> result.txt
echo "Test 1" >> result.txt
time timeout 7 rustc --edition 2018 src/main.rs
echo "Test 2" >> result.txt
time timeout 7 rustc --edition 2018 src/main.rs
echo "Test 3" >> result.txt
time timeout 7 rustc --edition 2018 src/main.rs
