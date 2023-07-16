Bash
RUSTFLAGS="-g -C opt-level=0" cargo build
g++ main.cpp -L ./target/debug/ -lfinalfusion_tf -o run -g -O0
valgrind --leak-check=full --show-leak-kinds=all  --trace-children=yes ./run
