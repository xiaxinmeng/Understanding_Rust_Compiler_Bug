
cargo new --lib foo
cd foo
cargo new --lib dep
rm dep/src/lib.rs
cat << EOF > dep/src/lib.rs
#[macro_export]
macro_rules! make_item {
    () => {
        pub fn f() {}
    }
}
EOF

echo 'dep = {path="dep"}' >> Cargo.toml
echo 'dep::make_item!{}' >> src/lib.rs
RUSTFLAGS="--remap-path-prefix=`pwd`=/" cargo doc -v  --no-deps
