shell
# First, make sure to have a clean repo.
make clean  # i.e. cargo clean
# The first run works normally.
make ci-travis  # i.e. cargo build/test/etc
# Change a file to trigger a rebuild.
# No need to actually change anything, touching is enough.
touch kernel/src/debug.rs
# The second run triggers warnings/errors about `#[warn(unused_attributes)]`.
make ci-travis  # i.e. cargo build/test/etc
