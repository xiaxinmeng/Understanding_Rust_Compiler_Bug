
$ diff rustup-orig.sh  rustup.sh
191c191
< MXgiy7zESksMnVFMulIJJhR3eB0wx2GitibjY/ZhQ7tD3i0yy9ILR07dFz4pgkVM
---
> MXgiy7zMSksMnVFMulIJJhR3eB0wx2GitibjY/ZhQ7tD3i0yy9ILR07dFz4pgkVM
1447c1447
<     run rm -R "$_workdir" || return 1
---
>     # run rm -R "$_workdir" || return 1
1696c1696
<         ignore rm -R "$_cache_dir"
---
>         # ignore rm -R "$_cache_dir"
