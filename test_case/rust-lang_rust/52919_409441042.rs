
error[E0050]: method `exec` has 4 parameters but the declaration in trait `cargo::core::compiler::Executor::exec` has 5
   --> tools\rls\src\build\cargo.rs:333:75
    |
333 |     fn exec(&self, mut cargo_cmd: ProcessBuilder, id: &PackageId, target: &Target) -> CargoResult<()> {
    |                                                                           ^^^^^^^ expected 5 parameters, found 4
    |
    = note: `exec` from trait: `fn(&Self, cargo::util::process_builder::ProcessBuilder, &cargo::core::package_id::PackageId, &cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode) -> std::result::Result<(), failure::error::Error>`
error: aborting due to previous error
