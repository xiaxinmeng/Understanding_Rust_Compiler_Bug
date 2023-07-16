
error: expected one of `!`, `(`, `)`, `,`, `::`, or `<`, found `dep_tracking`
    --> src/librustc/session/config.rs:290:51
     |
290  |                                 $opt_expr as &r#dyn dep_tracking::DepTrackingHash).is_some() {
     |                                                     ^^^^^^^^^^^^ expected one of `!`, `(`, `)`, `,`, `::`, or `<` here
...
1145 | / options! {DebuggingOptions, DebuggingSetter, basic_debugging_options,
1146 | |           build_debugging_options, "Z", "debugging",
1147 | |           DB_OPTIONS, db_type_desc, dbsetters,
1148 | |     codegen_backend: Option<String> = (None, parse_opt_string, [TRACKED],
...    |
1401 | |          the same values as the target option of the same name"),
1402 | | }
     | |_- in this macro invocation
