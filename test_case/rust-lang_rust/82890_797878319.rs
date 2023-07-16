toml
[profile.release]
opt-level = 0  # important!
lto = true 
codegen-units = 1

[dependencies]
staticvec = {git = "https://github.com/slightlyoutofphase/staticvec.git", rev="ad812d18da514fbb551aea09e37279c602742508"}
