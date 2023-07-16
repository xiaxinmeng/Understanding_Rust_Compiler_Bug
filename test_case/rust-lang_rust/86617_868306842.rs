
git rebase -i origin --exec 'cargo tree > /dev/null && git add Cargo.lock && git commit --amend --no-edit'
