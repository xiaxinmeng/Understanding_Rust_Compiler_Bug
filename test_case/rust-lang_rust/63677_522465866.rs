
cd $(mktemp -d)
git clone https://github.com/shootingsyh/rxstream
cd rxstream
wget https://github.com/rust-lang/rust/files/3514385/patch.txt
patch -p1 < patch.txt
cargo build
