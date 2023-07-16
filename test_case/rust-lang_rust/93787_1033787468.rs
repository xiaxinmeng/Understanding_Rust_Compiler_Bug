
...
[target.'cfg(parallel_compiler)'.dependencies]
rayon = { version = "0.3.2", package = "rustc-rayon" }
rayon-core = { version = "0.3.2", package = "rustc-rayon-core" }
indexmap = { version = "1.8.0", features = ["rustc-rayon"] }

[target.'cfg(not(parallel_compiler))'.dependencies]
indexmap = { version = "1.8.0" }
