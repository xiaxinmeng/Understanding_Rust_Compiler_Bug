
cargo +nightly build
# (build fails, no ICE)

# ...just to make sure I'm doing the repro right...
cargo +nightly-2018-03-16 build
# (ICE)
