
warning: field `config` is never read
 --> src/component/python.rs:6:5
  |
5 | pub struct Python {
  |            ------ field in this struct
6 |     config: Rc<Config>,
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `myenv` (lib) generated 1 warning
warning: associated function `new` is never used
  --> src/component/python.rs:10:12
   |
10 |     pub fn new(config: Rc<Config>) -> Python {
   |            ^^^

warning: `myenv` (bin "myenv") generated 2 warnings (1 duplicate)
    Finished dev [unoptimized + debuginfo] target(s) in 15.24s
