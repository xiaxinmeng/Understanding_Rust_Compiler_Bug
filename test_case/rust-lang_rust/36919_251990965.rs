
unset CDPATH
cargo new some-directory || exit 1
cd some-directory || exit 1
trap 'cd .. && rm -r some-directory' EXIT
cat > src/lib.rs <<\EOF
#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
         panic!()
    }
}
EOF
RUST_BACKTRACE=1 cargo test
