toml
[package]
name = "main_crate"
version = "0.1.0"
authors = ["Ryan Levick <me@ryanlevick.com>"]
edition = "2018"

[dependencies]
# Renaming the dependency seems to be the key here
dep = { package = "dependency", path = "dependency",  version = "0.1.0"  }
