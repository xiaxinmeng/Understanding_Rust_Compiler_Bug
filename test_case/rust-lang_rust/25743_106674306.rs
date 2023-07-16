
src/test/compile-fail/match-ill-type1.rs:13:9: 13:13 error: internal compiler error: error comparing literals in range pattern
src/test/compile-fail/match-ill-type1.rs:13         1...2_usize => 1, //~ ERROR mismatched types in range
