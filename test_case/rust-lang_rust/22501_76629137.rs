 sh
# make a fresh git working dir
cd ..; mkdir rust-unsuffix-data; cd rust-unsuffix-data; git init
# add a commit for the "before" dataset (sed removes comments which are only diff noise)
cp ../rust/before-int-suffix-removal/* .; sed -i -e 's|//.*$||' *.rs; git add *; git commit -m 'before'
# add a commit for the "after" dataset, on top of "before" (same sed to remove comments)
git rm *; cp ../rust/after-int-suffix-removal/* .; sed -i -e 's|//.*$||' *.rs; git add *; git commit -m 'after'
