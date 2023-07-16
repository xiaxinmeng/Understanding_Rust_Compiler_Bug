 sh
target=unknown-linux-gnu
curl -O http://static.rust-lang.org/dist/rust-nightly-x86_64-$target.tar.gz
curl -O http://static.rust-lang.org/dist/rust-nightly-i686-$target.tar.gz
tar xf rust-nightly-x86_64-$target.tar.gz
tar xf rust-nightly-i686-$target.tar.gz
cp -r rust-nightly-i686-$target/lib/rustlib/i686-$target \
      rust-nightly-x86_64-$target/lib/rustlib
(cd rust-nightly-x86_64-$target && \
  find lib/rustlib/i686-$target/lib -type f >> lib/rustlib/manifest.in)
./rust-nightly-x86_64-$target/install.sh $@
