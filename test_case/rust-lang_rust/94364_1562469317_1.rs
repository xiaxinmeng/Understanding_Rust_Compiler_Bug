toml
[package]
name = "test"
version = "1.0.0"

[dependencies]
openssl = "*"
diesel = { version = "2.0", features = [
	"postgres",
] }
