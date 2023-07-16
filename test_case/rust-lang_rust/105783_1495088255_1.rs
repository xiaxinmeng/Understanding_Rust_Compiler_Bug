
# rustup installation

cargo install cargo-bisect-rustc

sudo setsebool selinuxuser_execstack off

cargo bisect-rustc --start=1.65.0 --end=1.68.2 --script ./test.sh
