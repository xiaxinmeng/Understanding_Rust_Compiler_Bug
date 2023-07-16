 sh
git checkout -b tmp 
git reset --hard fd6ec2d # your commit on this branch
git fetch origin # whatever the mozilla remote is called
git rebase origin/master # do the rebase
git push bjadamson tmp:rustc-improvements -f # force-push this branch
