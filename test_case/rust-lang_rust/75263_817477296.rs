
git clone https://github.com/dmitry-zakablukov/rust-lang-issue-75263.git
cd rust-lang-issue-75263\symbols_test
cargo build --release
strings target\release\symbols_test.exe | grep "engine\src"
