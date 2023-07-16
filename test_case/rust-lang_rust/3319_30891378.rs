
rm -rf x86_64-unknown-linux-gnu/stage* # make sure no left over dirs exist
./configure
ln -s rustlib/ x86_64-unknown-linux-gnu/stage0/lib/rustc # fix snapshot
make
