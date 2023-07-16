plain
    Checking test v0.0.0 (/checkout/library/test)
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:293:33
    |
293 |           OutputFormat::Pretty => Box::new(PrettyFormatter::new(
294 | |             output,
294 | |             output,
295 | |             opts.use_color(),
296 | |             max_name_len,
297 | |             is_multithreaded,
298 | |             opts.time_options,
299 | |         )),
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:301:13
    |
    |
301 |             Box::new(TerseFormatter::new(output, opts.use_color(), max_name_len, is_multithreaded))
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:303:31
    |
    |
303 |         OutputFormat::Json => Box::new(JsonFormatter::new(output)),
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:304:32
    |
    |
304 |         OutputFormat::Junit => Box::new(JunitFormatter::new(output)),
    |
    |
    = help: consider adding an explicit lifetime bound `T: 'static`...
For more information about this error, try `rustc --explain E0310`.
error: could not compile `test` due to 4 previous errors
Build completed unsuccessfully in 0:01:26
