bash
git clone https://github.com/copterust/proving-ground.git
cd proving-ground
rustup override set nightly
rustup target add thumbv7em-none-eabihf
mkdir gcc
curl -L https://developer.arm.com/-/media/Files/downloads/gnu-rm/7-2018q2/gcc-arm-none-eabi-7-2018-q2-update-linux.tar.bz2\?revision\=bc2c96c0-14b5-4bb4-9f18-bceb4050fee7\?product\=GNU%20Arm%20Embedded%20Toolchain,64-bit,,Linux,7-2018-q2-update | tar --strip-components=1 -C gcc -xj
export PATH="$PATH:$PWD/gcc/bin"
make mem=128k nodevice
RUST_BACKTRACE=1 make mem=128k all
