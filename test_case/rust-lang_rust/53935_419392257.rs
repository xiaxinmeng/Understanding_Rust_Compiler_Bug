
[01:11:16] error[E0277]: a collection of type `std::collections::HashMap<&std::path::Path, (cargo::core::package_id::PackageId, cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode)>` cannot be built from an iterator over elements of type `(&cargo::core::manifest::TargetSourcePath, (cargo::core::package_id::PackageId, cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode))`
[01:11:16]    --> tools\rls\src\build\plan.rs:190:14
[01:11:16]     |
[01:11:16] 190 |             .collect();
[01:11:16]     |              ^^^^^^^ a collection of type `std::collections::HashMap<&std::path::Path, (cargo::core::package_id::PackageId, cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode)>` cannot be built from `std::iter::Iterator<Item=(&cargo::core::manifest::TargetSourcePath, (cargo::core::package_id::PackageId, cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode))>`
[01:11:16]     |
[01:11:16]     = help: the trait `std::iter::FromIterator<(&cargo::core::manifest::TargetSourcePath, (cargo::core::package_id::PackageId, cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode))>` is not implemented for `std::collections::HashMap<&std::path::Path, (cargo::core::package_id::PackageId, cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode)>`
[01:11:16] 
[01:11:16] error[E0599]: no method named `parent` found for type `&cargo::core::manifest::TargetSourcePath` in the current scope
[01:11:16]    --> tools\rls\src\build\plan.rs:200:26
[01:11:16]     |
[01:11:16] 200 |                         .parent()
[01:11:16]     |                          ^^^^^^
[01:11:16] 
[01:11:18] error[E0308]: mismatched types
[01:11:18]   --> tools\rls\src\project_model.rs:77:22
[01:11:18]    |
[01:11:18] 77 |                   lib: cargo_pkg
[01:11:18]    |  ______________________^
[01:11:18] 78 | |                     .targets()
[01:11:18] 79 | |                     .iter()
[01:11:18] 80 | |                     .find(|t| t.is_lib())
[01:11:18] 81 | |                     // racer expect name 'underscored'(crate) name
[01:11:18] 82 | |                     .map(|t| (t.src_path().to_owned(), t.name().replace('-', "_"))),
[01:11:18]    | |___________________________________________________________________________________^ expected struct `std::path::PathBuf`, found enum `cargo::core::manifest::TargetSourcePath`
[01:11:18]    |
[01:11:18]    = note: expected type `std::option::Option<(std::path::PathBuf, std::string::String)>`
[01:11:18]               found type `std::option::Option<(cargo::core::manifest::TargetSourcePath, std::string::String)>`
