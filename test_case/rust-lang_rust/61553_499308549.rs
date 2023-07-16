
# when compiling from `t`
     Running `rustc --crate-name x x/src/lib.rs --color always --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=ba89e226349d96d7 -C extra-filename=-ba89e226349d96d7 --out-dir /tmp/cargo-issue-1244/target/debug/deps -C incremental=/tmp/cargo-issue-1244/target/debug/incremental -L dependency=/tmp/cargo-issue-1244/target/debug/deps`
# when compiling from `z`
     Running `rustc --crate-name x /tmp/cargo-issue-1244/x/src/lib.rs --color always --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=2b52060268f81743 -C extra-filename=-2b52060268f81743 --out-dir /tmp/cargo-issue-1244/z/target/debug/deps -C incremental=/tmp/cargo-issue-1244/z/target/debug/incremental -L dependency=/tmp/cargo-issue-1244/z/target/debug/deps`
