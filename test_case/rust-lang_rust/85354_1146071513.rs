
➜  RustPerf rustc -O -C target-cpu=native main.rs && ./main
1a 5.785559692s

1b 1.999194356s

➜  RustPerf rustc -O  main.rs && ./main
1a 1.801210071s

1b 2.404140135s
