sh
git checkout master

git pull upstream master --ff-only

git checkout rust 

git rebase master 

# follow on-screen instruction to fix any merge conflict

git push self rust --force-with-lease
