toml
[target.'cfg(all(windows, target_arch = "x86"))'.dependencies]
ansi_term = "0.9" # 0.10 fails to compile on windows x86

[target.'cfg(not(all(windows, target_arch = "x86")))'.dependencies]
ansi_term = "0.10"
