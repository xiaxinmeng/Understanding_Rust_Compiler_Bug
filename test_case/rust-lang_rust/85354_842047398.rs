
git clone https://github.com/ZuseZ4/perf_comp
cd perf_comp
git checkout simple_example
cargo run --release 
vim src/main.rs // now call the fast path on line 82 instead of the slow on line 81.
cargo run --release // the exec time changed from 4s to 1.3s  
