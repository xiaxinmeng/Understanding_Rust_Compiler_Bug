sh
git fetch --all
git checkout master
git reset --hard rust-lang/master
git rebase master origin/master
$EDITOR src/liballoc/tests/vec.rs
git add src/liballoc/tests/vec.rs
git rebase --continue
git checkout master
git merge --ff-only $THE_COMMIT_OUTPUT_BY_CHECKOUT
git push --force
