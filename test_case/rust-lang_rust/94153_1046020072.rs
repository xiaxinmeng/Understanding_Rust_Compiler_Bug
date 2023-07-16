plain
    --> src/tools/rustfmt/src/items.rs:1005:13
     |
1005 |           let ast::Trait {
     |  _____________^
1006 | |             is_auto,
1007 | |             unsafety,
1008 | |             ref generics,
1009 | |             ref bounds,
1010 | |             ref items,
1011 | |         } = **trait_kind;
     | |_________^ missing field `constness`
help: include the missing field in the pattern
     |
     |
1010 |             ref items, constness } = **trait_kind;
     |                      ~~~~~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
     |
1010 |             ref items, .. } = **trait_kind;

For more information about this error, try `rustc --explain E0027`.
error: could not compile `rustfmt-nightly` due to previous error
warning: build failed, waiting for other jobs to finish...
