sh
git clone https://github.com/bgourlie/rs-nes.git
cd rs-nes
git checkout 4d98499f # HEAD of remove-unstable-features at time of posting
cargo +stable check # using rustc 1.32.0 (9fda7c223 2019-01-16)
