shell
export MYARCH=x86_64-apple-darwin && mkdir -p build/$MYARCH/subdir && touch build/$MYARCH/subdir/file && ln -s build/$MYARCH/subdir/file build/$MYARCH/subdir/symlink
