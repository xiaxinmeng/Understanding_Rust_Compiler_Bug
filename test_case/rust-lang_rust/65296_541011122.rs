bash
export TOOLS="rustc-nightly-x86_64-unknown-linux-gnu.tar.gz rust-std-nightly-x86_64-unknown-linux-gnu.tar.gz cargo-nightly-x86_64-unknown-linux-gnu.tar.gz rustc-nightly-x86_64-pc-windows-msvc.tar.gz rust-std-nightly-x86_64-pc-windows-msvc.tar.gz cargo-nightly-x86_64-pc-windows-msvc.tar.gz"

for TOOL in $TOOLS; do printf "| $TOOL |"; for COMMIT in 898f36c83cc28d7921a1d7b3605323dc5cfcf533 000d90b11f7be70ffb7812680f7abc6deb52ec88; do curl -I -s https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rustc-builds/${COMMIT}/${TOOL} | rg Content-Length | tr -d '\r' | tr -d '\n' | sed 's/.*: //;s/\(.*\)/ \1 |/'; done; echo ; done
