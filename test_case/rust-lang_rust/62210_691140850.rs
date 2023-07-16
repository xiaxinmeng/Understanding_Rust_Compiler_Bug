
sudo amazon-linux-extras install -y rust1
wget https://github.com/xiph/rav1e/archive/v0.3.4.tar.gz -O rav1e-v0.3.4.tar.gz
tar xvf rav1e-v0.3.4.tar.gz 
cd rav1e-v0.3.4.tar.gz
cargo install cargo-c
