toml
# Which triples to build libraries (core/alloc/std/test/proc_macro) for. Each of
# these triples will be bootstrapped from the build triple themselves.
#
# Defaults to `host`. If you set this explicitly, you likely want to add all
# host triples to this list as well in order for those host toolchains to be
# able to compile programs for their native target.
#target = ["x86_64-unknown-linux-gnu"] (as an example)
