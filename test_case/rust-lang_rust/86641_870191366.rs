
> cat rustc.sh 
exec rustc +beta "$@"
> RUSTC=./rustc.sh cargo build
   Compiling hello v0.1.0 (/home/joshua/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
> RUSTC=./rustc.sh cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
> cat rustc.sh 
exec rustc +stable "$@"
> RUSTC=./rustc.sh cargo build
   Compiling hello v0.1.0 (/home/joshua/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
