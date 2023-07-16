toml
# Build triple for the original snapshot compiler. This must be a compiler that
# nightlies are already produced for. The current platform must be able to run
# binaries of this build triple and the nightly will be used to bootstrap the
# first compiler.
#
# Defaults to platform where `x.py` is run.
build = "aarch64-apple-darwin" # because Rosetta confuses bootstrap
