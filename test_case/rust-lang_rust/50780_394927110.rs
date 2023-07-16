
wesley@endurance:~/code/rust/rustc-serialize> RUSTFLAGS="-Zself-profile -Zprofile-json" cargo build
   Compiling rustc-serialize v0.3.24 (file:///home/wesley/code/rust/rustc-serialize)
Self profiling results for rustc_serialize:

| Phase            | Time (ms)      | Queries        | Hits (%) |
| ---------------- | -------------- | -------------- | -------- |
| Parsing          | 27             |                |          |
| Expansion        | 132            |                |          |
| TypeChecking     | 2182           | 3321830        | 96.17    |
| BorrowChecking   | 144            | 28872          | 69.71    |
| Codegen          | 2065           | 110571         | 79.61    |
| Linking          | 17             | 11320          | 83.46    |
| Other            | 590            | 4922566        | 97.07    |

Optimization level: No
Incremental: on
    Finished dev [unoptimized + debuginfo] target(s) in 5.29 secs
