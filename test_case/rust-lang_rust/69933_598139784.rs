
rustup update nightly-2020-03-11 # actually installs a rustc where --version reports 2020-03-10
rustup default nightly-2020-03-11
git clone https://github.com/mozilla/glean
cd glean
git reset --hard a30e58cebe6888a6c15fcb58d32dc00a9c38bc17
RUSTFLAGS="-Z save-analysis" cargo check
