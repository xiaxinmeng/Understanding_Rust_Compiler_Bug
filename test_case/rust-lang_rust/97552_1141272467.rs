
[package]
name = "vec-deref-inline"
version = "0.1.0"

[profile.release]
lto = true
panic = 'abort'
