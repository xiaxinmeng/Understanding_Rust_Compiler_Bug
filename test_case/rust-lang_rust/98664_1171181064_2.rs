 bash
brew install ninja

git branch
* master

git log | head -1
commit 3fcf43bb0f3e86c16a88f239da18a1729a94d244

./x.py clean

./x.py test library/std  # ~ 18 GB disk space needed and ~ 3 hours on a slow ntb

cd ./build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps

ln -sn deps ../lib

valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./std-b55978f9760549f9 --exact time::tests::instant_math
