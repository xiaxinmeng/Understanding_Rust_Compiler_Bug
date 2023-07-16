sh
git rebase --root -i
# It will show a screen with all your commits; change all but the first from `pick` to `squash`
git push -f
