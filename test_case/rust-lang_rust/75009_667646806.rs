sh
git fetch origin master
git checkout master
git reset --hard origin/master
cd library/stdarch
git checkout d6822f9c433bd70f786b157f17beaf64ee28d83a
cd -
git add library/stdarch
git commit
git push georgio master --force-with-lease
