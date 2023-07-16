
git fetch origin
git rebase origin/master # this should just work without conflicts
git status # this should show a clean working dir!
cd src/tools/miri
git fetch origin
git reset --hard origin/rustup
cd ../../..
./x.py # updates Cargo.lock if needed
git commit -am "update miri"
