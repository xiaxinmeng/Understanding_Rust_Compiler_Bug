toml

[features]
default = ["runtime-async-std"]
runtime-async-std = ["async-native-tls/runtime-async-std"]
runtime-tokio = ["async-native-tls/runtime-tokio"]

[dependencies]
#runtime
async-std = { version = "1.6.0", features = ["attributes"] }
tokio = { version = "0.2", features = ["full"] }
async-native-tls = { version = "0.3.2", default-features = false,  features = ["runtime-tokio"]  }

