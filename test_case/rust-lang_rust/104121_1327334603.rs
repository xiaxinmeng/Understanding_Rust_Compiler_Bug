
git fetch upstream # or whatever you named your remote that points to rust-lang/rust, not your own fork
git rebase -i upstream/master # opens a text editor, replace all `pick` with `fixup` if you want to squash them into the commit above them.
git push --force-with-lease
