
[01:22:43]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:22:44] error[E0433]: failed to resolve. Use of undeclared type or module `Lrc`
[01:22:44]    --> librustc_driver/test.rs:108:40
[01:22:44]     |
[01:22:44] 108 |                                        Lrc::new(CodeMap::new(FilePathMapping::empty())));
[01:22:44]     |                                        ^^^ Use of undeclared type or module `Lrc`
[01:22:44] 
[01:22:45] error: aborting due to previous error
