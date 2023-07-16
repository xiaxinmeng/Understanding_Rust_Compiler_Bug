bash
crate=typenum
version=1.15.0

cd /tmp
curl -L https://crates.io/api/v1/crates/${crate}/${version}/download | tar -xz
cd ${crate}-${version}
cargo +nightly-2022-06-28 rustdoc --lib -- -Z unstable-options --output-format json
