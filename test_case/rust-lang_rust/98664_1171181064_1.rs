 bash
git clone https://github.com/LouisBrunner/valgrind-macos
cd valgrind-macos
git checkout feature/macos12
cp darwin20.supp darwin21.supp
./autogen.sh
./configure
make -j
sudo make install
