
[01:19:20] [0m[1m[38;5;9merror[E0053][0m[0m[1m: method `exec` has an incompatible type for trait[0m

[01:19:20] [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/rls/src/build/cargo.rs:413:5[0m

[01:19:20] [0m    [0m[0m[1m[38;5;12m|[0m

[01:19:20] [0m[1m[38;5;12m413[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;9m/[0m[0m [0m[0m    fn exec([0m

[01:19:20] [0m[1m[38;5;12m414[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;9m|[0m[0m [0m[0m        &self,[0m

[01:19:20] [0m[1m[38;5;12m415[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;9m|[0m[0m [0m[0m        mut cargo_cmd: ProcessBuilder,[0m

[01:19:20] [0m[1m[38;5;12m416[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;9m|[0m[0m [0m[0m        id: &PackageId,[0m

[01:19:20] [0m[1m[38;5;12m...[0m[0m   [0m[0m[1m[38;5;9m|[0m

[01:19:20] [0m[1m[38;5;12m608[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;9m|[0m[0m [0m[0m        Ok(())[0m

[01:19:20] [0m[1m[38;5;12m609[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;9m|[0m[0m [0m[0m    }[0m

[01:19:20] [0m    [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;9m|_____^[0m[0m [0m[0m[1m[38;5;9mexpected struct `cargo::core::package_id::PackageId`, found reference[0m

[01:19:20] [0m    [0m[0m[1m[38;5;12m|[0m

[01:19:20] [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: expected type `[0m[0m[1mfn(&build::cargo::RlsExecutor, cargo::util::process_builder::ProcessBuilder, cargo::core::package_id::PackageId, &cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode) -> std::result::Result<(), failure::error::Error>[0m[0m`[0m

[01:19:20] [0m               found type `[0m[0m[1mfn(&build::cargo::RlsExecutor, cargo::util::process_builder::ProcessBuilder, &cargo::core::package_id::PackageId, &cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode) -> std::result::Result<(), failure::error::Error>[0m[0m`[0m

[01:19:20] 

[01:19:21] [0m[1m[38;5;9merror[0m[0m[1m: aborting due to previous error[0m
