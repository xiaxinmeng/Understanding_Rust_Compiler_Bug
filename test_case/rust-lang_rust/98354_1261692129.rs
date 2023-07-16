plain
[RUSTC-TIMING] libffi test:false 1.127
error[E0308]: mismatched types
   --> src/tools/miri/src/stacked_borrows/stack.rs:214:54
    |
214 |         let found = self.unknown_bottom.is_some_and(|&unknown_limit| {
    |                                                      ^-------------
    |                                                      |expected due to this
    |                                                      expected struct `stacked_borrows::SbTag`, found reference
    |
    = note: expected struct `stacked_borrows::SbTag`
    = note: expected struct `stacked_borrows::SbTag`
            found reference `&_`
help: consider removing `&` from the pattern
    |
214 -         let found = self.unknown_bottom.is_some_and(|&unknown_limit| {
214 +         let found = self.unknown_bottom.is_some_and(|unknown_limit| {

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] miri test:false 3.036
error: could not compile `miri` due to previous error
error: could not compile `miri` due to previous error
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: InTree, extra_features: [] } -- 12.291
thread 'main' panicked at 'in-tree tool', test.rs:489:14
[TIMING] tool::Miri { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, extra_features: [] } -- 0.000
Build completed unsuccessfully in 0:00:13
