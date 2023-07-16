
$ RUSTFLAGS="" cargo bench
   Compiling bench v0.1.0 (D:\msys64\home\mateusz\bench)
    Finished bench [optimized] target(s) in 2.44s
     Running target\release\deps\bench-2ddd0f70613232dd.exe

running 4 tests
test ends_with_char   ... bench:         288 ns/iter (+/- 16)
test ends_with_str    ... bench:         487 ns/iter (+/- 9)
test starts_with_char ... bench:         285 ns/iter (+/- 3)
test starts_with_str  ... bench:         289 ns/iter (+/- 15)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out

$ RUSTFLAGS="-Ctarget-cpu=skylake" cargo bench
   Compiling bench v0.1.0 (D:\msys64\home\mateusz\bench)
    Finished bench [optimized] target(s) in 0.86s
     Running target\release\deps\bench-2ddd0f70613232dd.exe

running 4 tests
test ends_with_char   ... bench:         288 ns/iter (+/- 9)
test ends_with_str    ... bench:         413 ns/iter (+/- 8)
test starts_with_char ... bench:         287 ns/iter (+/- 18)
test starts_with_str  ... bench:         287 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
