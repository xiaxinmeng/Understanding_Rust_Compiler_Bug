
docker pull shepmaster/rust-nightly
docker run -it --rm --entrypoint bash rust-nightly
echo 'fn main() { println!("Hello2"); }' > src/main.rs
time cargo build
