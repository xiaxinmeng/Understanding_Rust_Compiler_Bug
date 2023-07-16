
PKG_NAME=lib1
cd $PKG_NAME
cargo clean
cargo build --release
cd target/release
ar x lib${PKG_NAME}.a
LANG= sed -i ‘’ “s/rust_eh_personality/rust_eh_personaliti/g” $(ls ${PKG_NAME}-*.rcgu.o)
ar cr lib${PKG_NAME}.a *.o
