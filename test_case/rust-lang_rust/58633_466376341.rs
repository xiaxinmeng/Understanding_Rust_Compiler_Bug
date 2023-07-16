
# Indicates whether the LLVM assertions are enabled or not
assertions = true

# Number of codegen units to use for each compiler invocation. A value of 0
# means "the number of cores on this machine", and 1+ is passed through to the
# compiler.
codegen-units = 0

# Sets the number of codegen units to build the standard library with,
# regardless of what the codegen-unit setting for the rest of the compiler is.
#codegen-units-std = 1

# Whether or not debug assertions are enabled for the compiler and standard
# library. Also enables compilation of debug! and trace! logging macros.
debug-assertions = true

# Whether or not debuginfo is emitted
debuginfo = true

# Whether or not line number debug information is emitted
debuginfo-lines = true

# Whether to always use incremental compilation when building rustc
incremental = true

