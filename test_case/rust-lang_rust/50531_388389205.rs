bash
# get the latest version of master
git fetch upstream

# rebase your branch onto master
git rebase upstream/master

# resolve any conflicts that arise during rebase and require manual resolving
...

# Push your changes so they show up here on github.
git push -f
