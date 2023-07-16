bash
wget http://download.angelcam.com/toolchains/mips64-buildroot-linux-gnu.tar.gz
tar xzf mips64-buildroot-linux-gnu.tar.gz
export PATH=$(pwd)/mips64-buildroot-linux-gnu/usr/bin:$PATH
export CC_mips64_unknown_linux_gnu=mips64-buildroot-linux-gnu-gcc
