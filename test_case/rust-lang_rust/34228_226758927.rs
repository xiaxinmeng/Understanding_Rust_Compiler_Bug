 sh
mkdir -- '/tmp/llvm/' &&
cd -- "$_" &&
curl --location --time-cond 'llvm-master.tar.gz' --output 'llvm-master.tar.gz' 'https://codeload.github.com/rust-lang/llvm/tar.gz/master' &&
tar -x -z -f 'llvm-master.tar.gz' &&
cd -
