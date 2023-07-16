bash
export TOOLS="rustc-nightly-x86_64-unknown-linux-gnu.tar.gz rust-std-nightly-x86_64-unknown-linux-gnu.tar.gz cargo-nightly-x86_64-unknown-linux-gnu.tar.gz rustc-nightly-x86_64-pc-windows-msvc.tar.gz rust-std-nightly-x86_64-pc-windows-msvc.tar.gz cargo-nightly-x86_64-pc-windows-msvc.tar.gz"

for TOOL in $TOOLS; do printf "| $TOOL |"; for COMMIT in 7e498005a12548a8fd396312affde05c4d3ca085 a16dca337de610986252bb800953e57bf395863f 6576f4be5af31a5e61dfc0cf50b7130e6c6dfb35 50ffa79589600f515ff2710830c23cd2dce7cb76; do curl -I -s https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rustc-builds/${COMMIT}/${TOOL} | rg Content-Length | tr -d '\r' | tr -d '\n' | sed 's/.*: //;s/\(.*\)/ \1 |/'; done; echo ; done
