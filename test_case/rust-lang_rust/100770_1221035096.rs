`
echo "pub fn main() {}" > a.rs
rustc ./a.rs -o/tmp/ -Zunpretty=ast-tree
