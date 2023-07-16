rust
---- btree/map.rs - btree::map::BTreeMap<K, V>::range_mut (line 818) stdout ----
	error[E0283]: type annotations required: cannot resolve `_: std::cmp::Ord`
 --> btree/map.rs:824:25
  |
9 | for (_, balance) in map.range_mut("B".."Cheryl") {
  |                         ^^^^^^^^^

thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:296:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    btree/map.rs - btree::map::BTreeMap<K, V>::range_mut (line 818)
