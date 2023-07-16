
# Reset to the commit before merge
git reset --hard d644e652e65eea4e104c4df63a56781bc5648803
# Add rust-lang/rust as a remote in case this wasn't done before
git remote add upstream https://github.com/rust-lang/rust.git
# Fetch everything from rust-lang/rust
git fetch --all
# Rebase your changes on top of rust-lang/rust master
# In case of conflicts use `git rebase --continue` after resolving them
git rebase upstream/master
# Update this PR
git push -f origin master
