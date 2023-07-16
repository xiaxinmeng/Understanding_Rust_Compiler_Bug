
time rustc +nightly --crate-type lib privacy_checking.rs -Ztime-passes --cfg 'feature="pub"' --cfg 'feature="T6"' --cfg 'feature="I5"'
