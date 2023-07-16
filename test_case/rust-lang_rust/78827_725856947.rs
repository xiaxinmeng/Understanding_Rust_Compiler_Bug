
git clone https://github.com/Firstyear/demo_r_link.git
cd demo_r_link
cargo build
readelf -Ws target/debug/libdemo.so | grep wrapper_add
