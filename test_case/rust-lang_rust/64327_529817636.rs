
2019-09-10T01:32:20.0705767Z ---- collections/btree/map.rs - collections::btree::map::BTreeMap<K, V>::range_mut (line 841) stdout ----
2019-09-10T01:32:20.0706521Z error[E0283]: type annotations required: cannot resolve `_: std::cmp::Ord`
2019-09-10T01:32:20.0707955Z   --> collections/btree/map.rs:848:25
2019-09-10T01:32:20.0708249Z    |
2019-09-10T01:32:20.0708436Z 10 | for (_, balance) in map.range_mut("B".."Cheryl") {
