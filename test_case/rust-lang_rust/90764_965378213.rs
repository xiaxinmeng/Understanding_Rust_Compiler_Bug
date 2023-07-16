bash
wget https://static.rust-lang.org/dist/rustc-1.56.1-src.tar.gz
tar -xzf rustc-1.56.1-src.tar.gz
cd rustc-1.56.1-src
./configure --set rust.channel=stable --set build.vendor=true
# Download bootstrap
./x.py help
# Disconnect network here
./x.py test src/tools/tidy
