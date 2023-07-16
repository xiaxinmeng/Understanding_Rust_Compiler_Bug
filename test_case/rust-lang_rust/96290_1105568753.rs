plain
   |         ----^^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `-D unused-mut` implied by `-D warnings`

error[E0507]: cannot move out of `opts.logfile.0` which is behind a shared reference
   |
90 |     let mut log_out = match opts.logfile {
   |                             ^^^^^^^^^^^^ help: consider borrowing here: `&opts.logfile`
91 |         None => None,
91 |         None => None,
92 |         Some(path) => Some(OutputLocation::Raw(File::create(path)?)),
   |              |
   |              data moved here
   |              data moved here
   |              move occurs because `path` has type `PathBuf`, which does not implement the `Copy` trait
error[E0382]: use of moved value
   --> library/test/src/console.rs:115:21
    |
    |
115 |         if let Some(log_output) = log_out {
    |                     ^^^^^^^^^^ value moved here, in previous iteration of loop
    |
    = note: move occurs because value has type `OutputLocation<File>`, which does not implement the `Copy` trait
help: borrow this field in the pattern to avoid moving `log_out.0`
    |
115 |         if let Some(ref log_output) = log_out {

error[E0382]: use of moved value
   --> library/test/src/console.rs:136:21
    |
    |
130 |             if let Some(log_output) = log_out {
    |                         ---------- value moved here
...
136 |         if let Some(log_output) = log_out {
    |                     ^^^^^^^^^^ value used here after move
    |
    = note: move occurs because value has type `OutputLocation<File>`, which does not implement the `Copy` trait
help: borrow this field in the pattern to avoid moving `log_out.0`
    |
130 |             if let Some(ref log_output) = log_out {


error[E0596]: cannot borrow `log_output` as mutable, as it is not declared as mutable
    |
498 | / macro_rules! write {
498 | / macro_rules! write {
499 | |     ($dst:expr, $($arg:tt)*) => {
500 | |         $dst.write_fmt($crate::format_args!($($arg)*))
    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
502 | | }
    | |_- in this expansion of `$crate::write!` (#2)
...
552 | / macro_rules! writeln {
552 | / macro_rules! writeln {
553 | |     ($dst:expr $(,)?) => {
554 | |         $crate::write!($dst, "\n")
555 | |     };
...   |
558 | |     };
559 | | }
559 | | }
    | |_- in this expansion of `writeln!` (#1)
    |
   ::: library/test/src/console.rs:130:25
    |
130 |               if let Some(log_output) = log_out {
    |                           ---------- help: consider changing this to be mutable: `mut log_output`
131 |                   writeln!(log_output)?;


error[E0596]: cannot borrow `log_output` as mutable, as it is not declared as mutable
    |
552 | / macro_rules! writeln {
552 | / macro_rules! writeln {
553 | |     ($dst:expr $(,)?) => {
554 | |         $crate::write!($dst, "\n")
555 | |     };
556 | |     ($dst:expr, $($arg:tt)*) => {
557 | |         $dst.write_fmt($crate::format_args_nl!($($arg)*))
    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
559 | | }
    | |_- in this expansion of `writeln!`
    |
   ::: library/test/src/console.rs:115:21
   ::: library/test/src/console.rs:115:21
    |
115 |           if let Some(log_output) = log_out {
    |                       ---------- help: consider changing this to be mutable: `mut log_output`
116 |               writeln!(log_output, "{name}: {fntype}")?;


error[E0596]: cannot borrow `log_output` as mutable, as it is not declared as mutable
    |
552 | / macro_rules! writeln {
552 | / macro_rules! writeln {
553 | |     ($dst:expr $(,)?) => {
554 | |         $crate::write!($dst, "\n")
555 | |     };
556 | |     ($dst:expr, $($arg:tt)*) => {
557 | |         $dst.write_fmt($crate::format_args_nl!($($arg)*))
    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
559 | | }
    | |_- in this expansion of `writeln!`
    |
   ::: library/test/src/console.rs:136:21
   ::: library/test/src/console.rs:136:21
    |
136 |           if let Some(log_output) = log_out {
    |                       ---------- help: consider changing this to be mutable: `mut log_output`
137 |               writeln!(log_output, "{}, {}", plural(ntest, "test"), plural(nbench, "benchmark"))?;


error[E0507]: cannot move out of `log_out.0`, as `log_out` is a captured variable in an `FnMut` closure
    |
    |
245 |       let mut log_out: Option<Box<dyn OutputFormatter>> = match opts.logfile {
    |           ----------- captured outer variable
...
262 |       run_tests(opts, tests, |x| {
263 | |         on_test_event(
264 | |             &x,
265 | |             &mut st,
266 | |             &mut *out,
266 | |             &mut *out,
267 | |             match log_out {
    | |                   ^^^^^^^ help: consider borrowing here: `&log_out`
268 | |                 None => None,
269 | |                 Some(log_output) => Some(&mut *log_output),
    | |                      |
    | |                      data moved here
    | |                      data moved here
    | |                      move occurs because `log_output` has type `Box<dyn OutputFormatter>`, which does not implement the `Copy` trait
271 | |         )
272 | |     })?;
272 | |     })?;
    | |_____- captured by this `FnMut` closure

error[E0596]: cannot borrow `*log_output` as mutable, as `log_output` is not declared as mutable
    |
    |
269 |                 Some(log_output) => Some(&mut *log_output),
    |                      ----------          ^^^^^^^^^^^^^^^^ cannot borrow as mutable
    |                      |
    |                      help: consider changing this to be mutable: `mut log_output`

error[E0597]: `*log_output` does not live long enough
    |
263 |         on_test_event(
    |         ------------- borrow later used by call
...
...
269 |                 Some(log_output) => Some(&mut *log_output),
    |                                          ^^^^^^^^^^^^^^^^- `*log_output` dropped here while still borrowed
    |                                          borrowed value does not live long enough

error: variable does not need to be mutable
   --> library/test/src/console.rs:245:9
   --> library/test/src/console.rs:245:9
    |
245 |     let mut log_out: Option<Box<dyn OutputFormatter>> = match opts.logfile {
    |         |
    |         help: remove this `mut`


error[E0507]: cannot move out of `opts.logfile.0` which is behind a shared reference
    |
    |
245 |     let mut log_out: Option<Box<dyn OutputFormatter>> = match opts.logfile {
    |                                                               ^^^^^^^^^^^^ help: consider borrowing here: `&opts.logfile`
246 |         None => None,
247 |         Some(path) => Some(createFormatter(
    |              |
    |              data moved here
    |              data moved here
    |              move occurs because `path` has type `PathBuf`, which does not implement the `Copy` trait
error[E0382]: use of moved value
   --> library/test/src/console.rs:280:14
    |
    |
245 |     let mut log_out: Option<Box<dyn OutputFormatter>> = match opts.logfile {
    |         ----------- move occurs because `log_out` has type `Option<Box<dyn OutputFormatter>>`, which does not implement the `Copy` trait
...
262 |     run_tests(opts, tests, |x| {
    |                            --- value moved into closure here
267 |             match log_out {
    |                   ------- variable moved due to use in closure
...
...
280 |         Some(log_output) => log_output.write_run_finish(&st),
    |              ^^^^^^^^^^ value used here after move

error[E0596]: cannot borrow `*log_output` as mutable, as `log_output` is not declared as mutable
    |
    |
280 |         Some(log_output) => log_output.write_run_finish(&st),
    |              ----------     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
    |              |
    |              help: consider changing this to be mutable: `mut log_output`
error[E0310]: the parameter type `T` may not live long enough
   --> library/test/src/console.rs:292:33
    |
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
Some errors have detailed explanations: E0310, E0382, E0507, E0596, E0597.
For more information about an error, try `rustc --explain E0310`.
error: could not compile `test` due to 18 previous errors
Build completed unsuccessfully in 0:01:30
