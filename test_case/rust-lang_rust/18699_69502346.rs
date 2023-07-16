
$ rustc --version
rustc 1.0.0-dev (6a697325c 2015-01-09 18:56:14 -0500)
$ ./build.sh 
plugin.rs:4:14: 4:31 warning: use of unstable item: use the crates.io `log` library instead, #[warn(unstable)] on by default
plugin.rs:4 #[macro_use] extern crate log;
                         ^~~~~~~~~~~~~~~~~
plugin.rs:5:1: 5:20 warning: use of unstable item, #[warn(unstable)] on by default
plugin.rs:5 extern crate rustc;
            ^~~~~~~~~~~~~~~~~~~
plugin.rs:10:25: 10:28 warning: unused variable: `reg`, #[warn(unused_variables)] on by default
plugin.rs:10 pub fn plugin_registrar(reg: &mut Registry) {
                                     ^~~
main.rs:5:11: 5:31 error: couldn't find crate `plugin` with expected target triple arm-unknown-linux-gnueabi
main.rs:5 #[plugin] extern crate plugin;
                    ^~~~~~~~~~~~~~~~~~~~
main.rs:5:31: 5:31 note: crate `plugin`, path #1, triple x86_64-unknown-linux-gnu: /home/ben/68e4f3c431cee9e6976a/libplugin.so
error: aborting due to previous error
