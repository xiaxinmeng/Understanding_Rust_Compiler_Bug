
INFO:<unknown>: preparing rlib to "Z:\\Repositories\\regextester\\regextester\\target\\debug\\deps\\libversion_check-9fb617200fce9285.rlib"
error: failed to build archive: no such file or directory

 INFO 2018-05-08T08:18:23Z: cargo::ops::cargo_rustc::job_queue: end: version_check v0.1.3 => Target(lib)/Profile(build) => Host
 INFO 2018-05-08T08:18:23Z: cargo::util::job::imp: found 0 remaining processes
DEBUG 2018-05-08T08:18:23Z: cargo: exit_with_error; err=CliError { error: Some(ProcessError { desc: "process didn\'t exit successfully: `rustc --crate-name version_check C:\\Users\\willspeak\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\version_check-0.1.3\\src\\lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=9fb617200fce9285 -C extra-filename=-9fb617200fce9285 --out-dir Z:\\Repositories\\regextester\\regextester\\target\\debug\\deps -L dependency=Z:\\Repositories\\regextester\\regextester\\target\\debug\\deps --cap-lints allow` (exit code: 101)", exit: Some(ExitStatus(ExitStatus(101))), output: None }

Could not compile `version_check`.), unknown: false, exit_code: 101 }
error: Could not compile `version_check`.
