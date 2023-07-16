
$ git log --oneline
859d3b67 (HEAD -> no-mangle-method-wip) Ignore `#[no_mangle]` and `#[export_name = "..."]` on generic functions and trait methods
a563b825 Adjust `check_no_mangle` and `check_export_name` to warn/error on `#[no_mangle]`/`#[export_name]` on trait methods
0bb2ea65 Adjust `#[no_mangle]`-related checks and lints for `impl` items
c84beefd Add associated functions that have custom linkage to `reachable_set`
eb2226b1 (grafted, origin/master, master) Auto merge of #85296 - bjorn3:plugin_cleanup, r=petrochenkov
