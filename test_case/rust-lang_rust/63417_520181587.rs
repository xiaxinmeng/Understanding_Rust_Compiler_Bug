
cd ~/projects
git clone --depth 1 https://github.com/llvm/llvm-project llvm
cd llvm/llvm
mkdir build
cd build
cmake ..
cmake --build . -j4
