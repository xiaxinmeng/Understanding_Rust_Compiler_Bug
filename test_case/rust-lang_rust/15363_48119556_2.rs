
[~] $ go version   
go version go1.2.1 linux/amd64
[~] $ go build foo.go                
[~] $ time GOMAXPROCS=1 ./foo   
GOMAXPROCS=1 ./foo  0.04s user 0.00s system 100% cpu 0.040 total
[~] $ time GOMAXPROCS=8 ./foo              
GOMAXPROCS=8 ./foo  0.16s user 0.08s system 203% cpu 0.117 total
[~] $ rustc -v                
rustc 0.11.0-nightly (459f155f81291c46633e86a480628b50304ffb1c 2014-07-04 23:46:44 +0000)
[~] $ rustc -O foo.rs                 
[~] $ time RUST_THREADS=1 ./foo      
RUST_THREADS=1 ./foo  0.12s user 0.00s system 99% cpu 0.124 total
[~] $ time RUST_THREADS=8 ./foo   
RUST_THREADS=8 ./foo  1.01s user 0.61s system 266% cpu 0.608 total
