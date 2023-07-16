toml
[dependencies]
# using the actual dependency compiles fine:
# sqlx = { version = "0.4.0-beta.1", default-features = false, features = ["postgres", "runtime-tokio", "macros", "chrono"] }
# but using this or leaving it out causes the ICE
sqlx = "0.3.5" # also ICE on "0.4.0-beta.1" 
ssh2 = "0.8"
