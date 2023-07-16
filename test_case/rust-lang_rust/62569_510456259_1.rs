
$ rg . rust/ | head -n1
rust/bstr/rustfmt.toml:disable_all_formatting = true
$ echo ${pipestatus[1]}
0
