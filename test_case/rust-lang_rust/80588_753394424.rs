
$ time /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --help >/dev/null
real	0m0.050s
user	0m0.026s
sys	0m0.020s
$ time rustc --help >/dev/null
real	0m0.157s
user	0m0.129s
sys	0m0.028s
$ time pip --help >/dev/null
real	0m0.206s
user	0m0.178s
sys	0m0.028s
$ time go help >/dev/null
real	0m0.022s
user	0m0.008s
sys	0m0.017s