
git fetch git@github.com:huonw/rust.git is_utf8_iter

git checkout FETCH_HEAD
echo new
make check-stage1-std TESTNAME=str::bench::from_utf8_lossy

git checkout HEAD^ 
echo old
make check-stage1-std NO_REBUILD=1 TESTNAME=str::bench::from_utf8_lossy
