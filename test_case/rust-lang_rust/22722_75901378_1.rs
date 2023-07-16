 rust
src/main.rs:47:26: 47:42 error: `args.flag_listen` does not live long enough
src/main.rs:47     match server::start(&args.flag_listen) {
                                        ^~~~~~~~~~~~~~~~
src/main.rs:38:11: 51:2 note: reference must be valid for the destruction scope surrounding block at 38:10...
src/main.rs:38 fn main() {
src/main.rs:39     let docopt = Docopt::new(USAGE).unwrap();
src/main.rs:40     let args: Args = docopt.decode().unwrap();
src/main.rs:41 
src/main.rs:42     if args.flag_version {
src/main.rs:43         println!("{}", VERSION);
               ...
src/main.rs:40:46: 51:2 note: ...but borrowed value is only valid for the block suffix following statement 1 at 40:45
src/main.rs:40     let args: Args = docopt.decode().unwrap();
src/main.rs:41 
src/main.rs:42     if args.flag_version {
src/main.rs:43         println!("{}", VERSION);
src/main.rs:44         return
src/main.rs:45     }
               ...
