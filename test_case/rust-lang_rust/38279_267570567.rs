
cd rust
git checkout master
git checkout -b tmp
git merge issue-8521 --no-ff
git reset master
git add src
git commit -m "allow path fragments to be parsed as type parameter bounds"
git branch -D issue-8521
git branch -m issue-8521
git push origin issue-8521 --force
