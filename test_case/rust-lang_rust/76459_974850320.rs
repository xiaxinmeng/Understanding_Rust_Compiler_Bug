
[dependencies]
git2 = "0.13"

[patch.'https://github.com/rust-lang/bar']
git2 = { path = "bar" }

[patch.alt-registry]
git2 = { path = "bar" }
