console
$ git checkout master
$ git pull origin master --ff-only    # synchronize master branch with this repository
$ git checkout div-by-zero
$ git rebase master      # rebase on top of the latest master
$ git rebase -i master   # do more cleanup e.g. squash everything down to a single commit
$ git push cjkenn div-by-zero --force-with-lease
