
git clone git://github.com/rust-lang/llvm
cd llvm
git remote add llvm git://github.com/llvm-mirror/llvm
git fetch llvm
git checkout -b rust-llvm-$date rust-llvm-$old_date
git rebase llvm/master
# fix conflicts and make a PR to rust-lang/llvm
