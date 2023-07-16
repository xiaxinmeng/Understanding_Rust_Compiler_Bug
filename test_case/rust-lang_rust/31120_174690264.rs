
src/libstd/process.rs:849:5: 866:6 error: duplicate definition of value `test_inherit_env` [E0428]
src/libstd/process.rs:849     fn test_inherit_env() {
src/libstd/process.rs:850         use env;
src/libstd/process.rs:851 
src/libstd/process.rs:852         let mut result = env_cmd().output().unwrap();
src/libstd/process.rs:853         let output = String::from_utf8(result.stdout).unwrap();
src/libstd/process.rs:854 
                          ...
src/libstd/process.rs:849:5: 866:6 help: run `rustc --explain E0428` to see a detailed explanation
src/libstd/process.rs:829:5: 845:6 note: first definition of value `test_inherit_env` here
src/libstd/process.rs:829     fn test_inherit_env() {
src/libstd/process.rs:830         use env;
src/libstd/process.rs:831 
src/libstd/process.rs:832         let result = env_cmd().output().unwrap();
src/libstd/process.rs:833         let output = String::from_utf8(result.stdout).unwrap();
src/libstd/process.rs:834 
