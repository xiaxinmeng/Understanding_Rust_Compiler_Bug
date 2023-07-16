
rust: ~"(each_path) yielding reexported item: Bar"
rust: ~"(building reduced graph for external crate) found path entry: Bar (dl_def(def_trait({crate: 2, node: 6})))"
rust: ~"(building reduced graph for external crate) building type Bar"
rust: ~"(building reduced graph for external crate) ... adding trait method \'bar\'"
...
rust: ~"(each_path) yielding explicit item: Bar"
rust: ~"(building reduced graph for external crate) found path entry: Baz (dl_def(def_trait({crate: 2, node: 6})))"
rust: ~"(building reduced graph for external crate) building type Bar"
rust: ~"(building reduced graph for external crate) ... adding trait method \'bar\'"
uust: ~"(each_path) yielding explicit item: Bar::bar"
rust: ~"(building reduced graph for external crate) found path entry: Bar::bar (dl_def(def_static_method({crate: 2, node: 5}, Some({crate: 2, node: 6}), impure_fn)))"
