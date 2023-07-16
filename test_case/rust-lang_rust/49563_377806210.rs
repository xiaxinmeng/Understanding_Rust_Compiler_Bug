 rust
[target.'cfg(all(target_arch = "arm", target_os = "none"))'.dependencies.cb] # THUMB
features = ["mem"]
path = "../cb"

[target.'cfg(not(all(target_arch = "arm", target_os = "none")))'.dependencies.cb] # not THUMB
path = "../cb"
