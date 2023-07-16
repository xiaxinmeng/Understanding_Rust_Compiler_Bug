
$ rustc +nightly -Zsave-analysis --crate-type=lib -Adead_code 1.rs
ERROR:rustc_save_analysis::dump_visitor: Mis-calculated spans for path 'An::U'. Found 1 spans, expected 2. Found spans:
ERROR:rustc_save_analysis::dump_visitor:     'U' in 1.rs, line 7
ERROR:rustc_save_analysis::dump_visitor:     master span: 1.rs:7:24: 7:36: `<A as An>::U`
