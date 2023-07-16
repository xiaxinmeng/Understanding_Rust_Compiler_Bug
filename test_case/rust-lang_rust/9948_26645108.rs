
[mutex:rust/test]% vi src/hellobin/main.rs # add the change you describe
[mutex:rust/test]% rustpkg build hellobin                                                                                 master 
WARNING: The Rust package manager is experimental and may be unstable
task <unnamed> failed at 'assertion failed: os::path_exists(&pkg_src.start_dir.push_rel(p))', /home/dpc/opt/src/rust/src/librustpkg/rustpkg.rs:433
