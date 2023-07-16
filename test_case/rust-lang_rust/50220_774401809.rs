shell
git clone https://github.com/fede1024/rust-rdkafka.git && cd rust-rdkafka
git co -b repro d7c2baf3
cargo build --features=cmake-build
