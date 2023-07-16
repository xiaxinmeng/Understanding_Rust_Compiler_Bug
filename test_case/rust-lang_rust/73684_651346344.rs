bash
$ git checkout <dev-branch-for-current-rust-lang-PR>
$ git fetch origin # make the latest upstream changes available in your local repo
$ git rebase origin/master # Rebase your changes on top of upstream. NOTE: origin/branch means use branch as it is on origin, not your local repo. Therefore, you don't have to checkout master and pull it before rebasing. 
$ git push -f   # will try --force-with-lease next time
