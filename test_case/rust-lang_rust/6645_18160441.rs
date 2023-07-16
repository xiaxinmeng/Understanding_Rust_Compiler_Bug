
compile_and_link: x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/libcore.dylib
/Users/steve/src/rust/src/libcore/vec.rs:332:28: 332:56 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:332                 result.push(slice(v, start, i).to_vec());
                                                                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
/Users/steve/src/rust/src/libcore/vec.rs:337:16: 337:45 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:337     result.push(slice(v, start, ln).to_vec());
                                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/Users/steve/src/rust/src/libcore/vec.rs:356:28: 356:56 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:356                 result.push(slice(v, start, i).to_vec());
                                                                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
/Users/steve/src/rust/src/libcore/vec.rs:363:16: 363:45 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:363     result.push(slice(v, start, ln).to_vec());
                                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/Users/steve/src/rust/src/libcore/vec.rs:381:28: 381:58 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:381                 result.push(slice(v, i + 1, end).to_vec());
                                                                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/Users/steve/src/rust/src/libcore/vec.rs:386:16: 386:43 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:386     result.push(slice(v, 0u, end).to_vec());
                                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~
/Users/steve/src/rust/src/libcore/vec.rs:406:28: 406:59 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:406                 result.push(slice(v, i + 1u, end).to_vec());
                                                                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/Users/steve/src/rust/src/libcore/vec.rs:413:16: 413:43 error: type `&['a]` does not implement any method in scope named `to_vec`
/Users/steve/src/rust/src/libcore/vec.rs:413     result.push(slice(v, 0u, end).to_vec());
