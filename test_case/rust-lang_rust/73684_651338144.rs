shell
$ git checkout master
$ git fetch
$ git pull
$ git checkout <dev-branch-for-current-rust-lang-PR>
$ git rebase master
$ git push -f   # will try --force-with-lease next time
