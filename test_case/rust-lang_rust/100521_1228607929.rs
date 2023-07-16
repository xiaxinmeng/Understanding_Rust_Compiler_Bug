
git clone https://github.com/Turbo87/crates.io.git
cd crates.io
git checkout ice
cargo check --test all
git checkout HEAD~1
cargo check --test all
