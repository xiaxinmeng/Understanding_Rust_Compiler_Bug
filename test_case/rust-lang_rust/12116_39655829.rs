
[... lots of compiler errors ...]
shell/shellprocess.rs:94:37: 94:73 error: this function takes 2 parameters but 3 parameters were supplied
shell/shellprocess.rs:94                 let maybe_process = Process::new(command, args, options);
                                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
shell/shellprocess.rs:96:21: 96:38 error: mismatched types: expected `std::result::Result<std::io::process::Process,std::io::IoError>` but found `std::option::Option<<generic #228>>` (expected enum std::result::Result but found enum std::option::Option)
shell/shellprocess.rs:96                     Some(mut process) => {
                                             ^~~~~~~~~~~~~~~~~
shell/shellprocess.rs:101:21: 101:25 error: mismatched types: expected `std::result::Result<std::io::process::Process,std::io::IoError>` but found `std::option::Option<<generic #229>>` (expected enum std::result::Result but found enum std::option::Option)
shell/shellprocess.rs:101                     None => {
                                              ^~~~
shell/shellprocess.rs:98:25: 98:74 error: type `std::comm::Sender<std::option::Option<i32>>` does not implement any method in scope named `try_send_deferred`
shell/shellprocess.rs:98                         pidchan.try_send_deferred(Some(process.get_id()));
