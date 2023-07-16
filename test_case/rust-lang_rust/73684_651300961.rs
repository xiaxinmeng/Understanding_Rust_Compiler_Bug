bash
$ git fetch origin # grab the latest upstream changes
$ # assuming you're already on the latest commit of your branch with changes
$ git rebase -i origin/master # take the commits in your branch and replay them on top of origin/master
$ # this is the point where you can edit the git todo file and squash the commits together
$ git push fork -f # If you don't have a matching branch set up, then use `git push fork -f HEAD:{branch_name}`
