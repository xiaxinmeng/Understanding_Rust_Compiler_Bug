
error[E0282]: type annotations needed
   --> library/test/src/cli.rs:293:66
    |
293 |     let mut report_time_colored = report_time && colored_opt_str == Some("colored".into());
    |                                                                  ^^      ---------------- this method call resolves to `T`
    |                                                                  |
    |                                                                  cannot infer type for type parameter `B`
