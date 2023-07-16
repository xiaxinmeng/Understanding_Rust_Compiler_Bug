
# Includes one of the default files in src/bootstrap/defaults
profile = "compiler"
changelog-seen = 2

[rust]
incremental = true

# deny-warnings = false

debug = true
debug-assertions = false
debuginfo-level = 1
debuginfo-level-rustc = 1
debuginfo-level-std = 1

run-dsymutil = false

parallel-compiler = true

[build]
compiler-docs = true
