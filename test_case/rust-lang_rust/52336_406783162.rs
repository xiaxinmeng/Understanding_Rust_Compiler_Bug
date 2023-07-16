sh
git branch -D dyn-rollup
git checkout master
git checkout -b dyn-rollup

# rollup the libstd PR 
git fetch origin +refs/pull/52221/merge: &&
git merge --no-ff FETCH_HEAD^2

# rollup the libcore PR 
git fetch origin +refs/pull/52222/merge: &&
git merge --no-ff FETCH_HEAD^2

# rollup the libterm PR 
git fetch origin +refs/pull/52296/merge: &&
git merge --no-ff FETCH_HEAD^2

# ...
# apply additional changes 
# ...

git push self dyn-rollup --force-with-lease
