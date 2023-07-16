
$ rustc slow-typeck.rs -Z time-passes --cfg equality | grep 'type checking'
time: 0.219     type checking
$ rustc slow-typeck.rs -Z time-passes | grep 'type checking'
time: 0.014     type checking
