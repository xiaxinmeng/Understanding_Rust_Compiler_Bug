bash
cargo clean
rm src/main.rs
cp template.rs src/main.rs
sed -i 's/VARIABLE/$t/' src/main.rs
cargo build
rm src/main.rs
cp versions/template.rs src/main.rs
sed -i 's/VARIABLE/u8/' src/main.rs
cargo build
