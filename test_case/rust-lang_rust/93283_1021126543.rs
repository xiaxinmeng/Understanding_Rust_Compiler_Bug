bash
git switch issue-93211-fix # change to branch, if not already on it
git fetch upstream # get the most recent commits from the rust-lang repo rather than your own github fork, without updating any local branches
git rebase upstream/master # rebase on the remote master branch, not your local master, you can update the local master later
# fix conflicts here if necessary
git push --force # overwrite remote branch on your github fork with your local branch
