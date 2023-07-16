console
$ command time -f 'took %Us' rustc +A proj-exp.rs --emit=metadata --cfg 'depth = "8"'
took 0.05s
$ command time -f 'took %Us' rustc +A proj-exp.rs --emit=metadata --cfg 'depth = "9"'
took 0.05s
