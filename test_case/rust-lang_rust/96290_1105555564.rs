plain
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
error: unused imports: `TestExecTime`, `bench::fmt_bench_samples`
   |
   |
9  |     bench::fmt_bench_samples,
...
...
18 |     time::{TestExecTime, TestSuiteExecTime},
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0277]: the `?` operator can only be used in a closure that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
   |
91 |     let mut log_out = opts.logfile.map(|path| OutputLocation::Raw(File::create(path)?));
   |                                        ---------------------------------------------^-
   |                                        |                                            |
   |                                        |                                            cannot use the `?` operator in a closure that returns `OutputLocation<File>`
   |                                        this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `OutputLocation<File>`

error[E0277]: the `?` operator can only be used in a closure that returns `Result` or `Option` (or another type that implements `FromResidual`)
    |
    |
243 |       let mut log_out: Option<Box<dyn OutputFormatter>> = opts.logfile.map(|path| {
244 | |         createFormatter(
244 | |         createFormatter(
245 | |             OutputLocation::Raw(File::create(path)?),
    | |                                                   ^ cannot use the `?` operator in a closure that returns `Box<dyn OutputFormatter>`
246 | |             opts,
249 | |         )
250 | |     });
250 | |     });
    | |_____- this function should return `Result` or `Option` to accept `?`
    |
    = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `Box<dyn OutputFormatter>`
error[E0308]: `match` arms have incompatible types
   --> library/test/src/console.rs:277:29
    |
    |
275 |       let log_out_result = match log_out {
276 | |         None => Ok(()),
    | |                 ------ this is found to be of type `Result<(), _>`
    | |                 ------ this is found to be of type `Result<(), _>`
277 | |         Some(log_output) => log_output.write_run_finish(&st),
    | |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `bool`
    | |_____- `match` arms have incompatible types
    |
    = note: expected type `Result<(), _>`
               found enum `Result<bool, std::io::Error>`
