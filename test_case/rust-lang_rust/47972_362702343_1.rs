sh
cd /tmp
git clone https://github.com/PaulGrandperrin/reactfs.git
cd reactfs/
git checkout rustc-issue-47972
cargo run --features "instrumentation"
git checkout HEAD^ # remove a comment, see commit below
cargo run --features "instrumentation"
# BOOM rustc panic
cargo clean
cargo run --features "instrumentation"
# no panics
