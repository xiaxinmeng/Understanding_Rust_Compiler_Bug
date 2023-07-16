 sh
$ clang -x c - <<< 'int main() { main(); return 0; }' && ./a.out
Segmentation fault
$ clang -O -x c - <<< 'int main() { main(); return 0; }' && ./a.out # runs fine
$ rustc - <<< 'fn main() { main() }' && ./rust_out
rust: task efc060 ran out of stack
Aborted
$ rustc -O - <<< 'fn main() { main() }' && ./rust_out # runs fine
