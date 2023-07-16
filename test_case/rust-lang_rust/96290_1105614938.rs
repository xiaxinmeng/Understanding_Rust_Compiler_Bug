plain
    Checking test v0.0.0 (/checkout/library/test)
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:292:33
    |
292 |           OutputFormat::Pretty => Box::new(PrettyFormatter::new(
293 | |             output,
293 | |             output,
294 | |             opts.use_color(),
295 | |             max_name_len,
296 | |             is_multithreaded,
297 | |             opts.time_options,
298 | |         )),
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:300:13
    |
    |
300 |             Box::new(TerseFormatter::new(output, opts.use_color(), max_name_len, is_multithreaded))
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:302:31
    |
    |
302 |         OutputFormat::Json => Box::new(JsonFormatter::new(output)),
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:303:32
    |
    |
303 |         OutputFormat::Junit => Box::new(JunitFormatter::new(output)),
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
For more information about this error, try `rustc --explain E0310`.
error: could not compile `test` due to 4 previous errors
Build completed unsuccessfully in 0:01:28
