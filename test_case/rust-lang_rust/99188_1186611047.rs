console
$ command time -f 'took %Us' rustc +A proj-exp.rs --emit=metadata --cfg 'depth = "8"'
took 1.72s
$ command time -f 'took %Us' rustc +A proj-exp.rs --emit=metadata --cfg 'depth = "9"'
took 7.32s
