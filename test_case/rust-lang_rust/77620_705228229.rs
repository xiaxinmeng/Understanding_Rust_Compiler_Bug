
git diff --name-only | while read dir; do cd $dir && git checkout -f HEAD && git clean -d -f && cd -; done
