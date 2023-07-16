
git fetch mozilla
git checkout -b my-new-branch mozilla/master
git cherry-pick 7c28c91
git push berleon my-new-branch:master -f
