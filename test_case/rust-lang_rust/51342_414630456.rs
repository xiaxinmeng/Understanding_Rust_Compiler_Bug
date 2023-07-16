
> rustup override add nightly
> rustup target add thumbv7em-none-eabihf
> cargo build                                 # works fine
> cargo rustc --release -- -C lto             # works fine
> cargo build --release                       # errors
