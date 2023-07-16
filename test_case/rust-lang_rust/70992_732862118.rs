
> # prepare some build tool and files
> rustup component add rust-src
> cargo install cargo-xbuild
> 
> cp /path/to/your/rust-toolchain .
> cp /path/to/your/custom_target_file.json .
> cp /path/to/your/placeholder_main.rs src/main.rs
> 
> # build the place holder project to get sysroot
> cargo xbuild --target custom_target_file
> cp -r target/sysroot/  /where/do/you/want/to/save/it
> 