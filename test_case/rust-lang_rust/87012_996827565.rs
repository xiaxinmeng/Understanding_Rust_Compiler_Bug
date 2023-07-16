
RUSTC_LOG=[evaluate_obligation]=debug rustc --crate-name repro_crate --edition=2018 src/main.rs -Zself-profile -Zself-profile-events=default,args --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C metadata=8bb05b458def09dc -C extra-filename=-8bb05b458def09dc --out-dir /Users/carolnichols/rust/rust_issue_87012/target/debug/deps 2>traits-log
