sh
cd src/tools/cargo
git checkout master
git pull origin master --ff-only
cd ../..
cargo update -p cargo
git add Cargo.lock tools/cargo
git ci -m 'Update cargo'
