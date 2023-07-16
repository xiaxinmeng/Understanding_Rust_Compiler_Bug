
git clone git@github.com:bburdette/zknotes
cd zknotes
git checkout c7f6b00114f5bc85503fdd2b7d9ce6b84d210854
git submodule update --init --recursive
cargo +stable build
