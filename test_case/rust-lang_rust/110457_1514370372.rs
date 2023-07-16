bash
cargo bisect-rustc --start 2021-01-01 --regress success --script ./repro.sh --access github
