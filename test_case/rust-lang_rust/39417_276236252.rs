
error[E0282]: unable to infer enough type information about `_`
   --> src/config/types.rs:608:50
    |
608 |                 let config: io::TestRefsConfig = serde_json::from_value(test_conf.config
    |                                                  ^ cannot infer type for `_`
    |
    = note: type annotations or generic parameter binding required
