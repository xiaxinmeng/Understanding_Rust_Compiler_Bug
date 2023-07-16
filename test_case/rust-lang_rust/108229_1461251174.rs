
> ./configure --set profile=compiler
configure: processing command line
configure: 
configure: profile              := compiler
configure: build.configure-args := ['--set', 'profile=compiler']
configure: 
configure: writing `config.toml` in current directory
configure: 
configure: run `python /home/gh-jyn514/rust1/x.py --help`
(bash@dev-desktop-us-1.infra.rust-lang.org) ~/rust1 [04:08:29]
> x c
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
failed to parse TOML configuration 'config.toml': invalid TOML value, did you mean to use a quoted string? at line 11 column 11
Build completed unsuccessfully in 0:00:00
