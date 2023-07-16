
# Outside Docker container
docker run -it --rm ubuntu:16.04

# Inside Docker container
apt-get update && apt-get install -y curl
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
time RUST_BACKTRACE=1 rustc foo.rs
