sh
git clone https://github.com/cryptocorrosion/cryptocorrosion
cd cryptocorrosion
RUSTFLAGS="-Z save-analysis" cargo +nightly check
