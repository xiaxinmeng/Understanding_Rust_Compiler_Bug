
error[E0599]: no method named `init` found for struct `Registry` in the current scope
  --> src/main.rs:2:36
   |
2  |     tracing_subscriber::registry().init();
   |                                    ^^^^ method not found in `Registry`
   |
  ::: /Users/edward-shen/.cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/util.rs:89:8
   |
89 |     fn init(self) {
   |        ---- the method is available for `Registry` here
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
1  | use tracing_subscriber::prelude::_;
   |

For more information about this error, try `rustc --explain E0599`.
