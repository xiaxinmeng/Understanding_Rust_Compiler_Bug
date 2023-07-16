
git clone -n git@github.com:parisholley/Device-Detection.git
cd Device-Detection
git checkout rust
git lfs fetch
cd rust
cargo test --features pattern pattern_init -- --nocapture
