
rustup-toolchain-install-master 87781d93a3879c9931bdafd98741f3a1d2c02191
echo 'fn main() {}' > t.rs
rustc +87781d93a3879c9931bdafd98741f3a1d2c02191 -Zself-profile --crate-name t t.rs
summarize summarize --json t-*.mm_profdata
# no output, exit code 0
