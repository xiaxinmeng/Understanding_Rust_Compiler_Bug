
[target.'cfg(any(windows, unix))']
rustflags = ["-C", "target-cpu=native"
, "-Z", "treat-err-as-bug=5"                                                        
, "-Z", "external-macro-backtrace"
]
