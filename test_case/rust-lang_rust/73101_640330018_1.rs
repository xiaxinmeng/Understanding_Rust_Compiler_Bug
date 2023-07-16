
// by default
 Documenting a v0.1.0 (/home/joshua/src/test-rustdoc/a)
       Fresh a v0.1.0 (/home/joshua/src/test-rustdoc/a)
     Running `rustdoc --edition=2018 --crate-type lib --crate-name a /home/joshua/src/test-rustdoc/a/src/lib.rs -o /home/joshua/src/test-rustdoc/b/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/joshua/src/test-rustdoc/b/debug/deps`
// with --no-deps
    Checking a v0.1.0 (/home/joshua/src/test-rustdoc/a)
     Running `rustc --crate-name a --edition=2018 /home/joshua/src/test-rustdoc/a/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C debuginfo=2 -C metadata=d962d8f2c027d5fc -C extra-filename=-d962d8f2c027d5fc --out-dir /home/joshua/src/test-rustdoc/b/debug/deps -C incremental=/home/joshua/src/test-rustdoc/b/debug/incremental -L dependency=/home/joshua/src/test-rustdoc/b/debug/deps`
