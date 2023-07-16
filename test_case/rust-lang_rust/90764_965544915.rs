
./configure \
  --enable-local-rust \
  --local-rust-root=/home/abuild/rpmbuild/BUILD/rust-1.56.1-x86_64-unknown-linux-gnu/usr \
  --llvm-root=/usr \
  --disable-codegen-tests \
  --enable-vendor \
  --release-channel=stable

./x.py --help

./x.py test
