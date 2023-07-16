bash
$ time ~/.cargo/bin/rustc -vv
error: Option 'verbose' given more than once.


real	0m1.161s
user	0m0.773s
sys	0m0.082s

$ time ~/.cargo/bin/rustc -vv
error: Option 'verbose' given more than once.


real	0m0.814s
user	0m0.775s
sys	0m0.040s
$ time ~/.cargo/bin/rustc -vv
error: Option 'verbose' given more than once.


real	0m0.790s
user	0m0.754s
sys	0m0.038s
