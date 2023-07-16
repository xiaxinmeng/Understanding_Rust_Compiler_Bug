
wget https://gist.github.com/jvns/c89b3cae890e11ecbc5f/raw/7d3d351f6f661ed289115ea4adbeb01e6d94c72e/example.rs
git clone https://github.com/thestinger/rust-core.git
rustc example.rs --cfg libc
