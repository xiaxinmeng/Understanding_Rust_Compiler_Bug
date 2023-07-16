sh
mkdir my_toolchain
ln -s actual_toolchain/gcc my_toolchain/cc
ln -s actual_toolchain/gar my_toolchain/ar
export PATH=my_toolchain:actual_toolchain:$PATH
./x.py build
