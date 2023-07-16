toml
# `release` but with enough debuginfo for `perf record --call-graph=dwarf`.
[profile.release-profiling]
inherits = "release"
debug = 1
