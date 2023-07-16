
$ diff rustup-orig.sh  rustup.sh
1447c1447
<     run rm -R "$_workdir" || return 1
---
>     # run rm -R "$_workdir" || return 1
1708c1708
<         ignore rm -R "$_cache_dir"
---
>         # ignore rm -R "$_cache_dir"
