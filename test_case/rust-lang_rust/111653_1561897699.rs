
wget https://static.rust-lang.org/dist/rustc-1.69.0-src.tar.gz
tar zxvf rustc-1.69.0-src.tar.gz
cd rustc-1.69.0-src
wget https://github.com/rust-lang/rust/pull/109256.patch
wget https://github.com/rust-lang/rust/pull/110906.patch
patch -p1 < 109256.patch
patch -p1 < 110906.patch
./configure --prefix=/path/to/installation/using/a/non/root/user/rust-1.69.0 --enable-extended
python3 ./x.py build
python3 ./x.py install
