
➜  learn1 git:(master) ✗ cargo run -v
warning: unused manifest key: package.debug
   Compiling learn1 v0.1.0 (/Users/nisim.j/Desktop/learn1)
     Running `rustc --crate-name learn1 --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C metadata=b039cbe887790850 --out-dir /Users/nisim.j/Desktop/learn1/target/debug/deps -C incremental=/Users/nisim.j/Desktop/learn1/target/debug/incremental -L dependency=/Users/nisim.j/Desktop/learn1/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 7.83s
     Running `target/debug/learn1`
error: could not execute process `target/debug/learn1` (never executed)

Caused by:
  No such file or directory (os error 2)
