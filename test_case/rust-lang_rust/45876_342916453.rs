
for r in $(git rev-list origin/master~10..origin/master); do git checkout $r; echo $r; cargo build; done
