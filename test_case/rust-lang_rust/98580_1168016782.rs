plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error: named argument is not used by name
   --> library/alloc/tests/fmt.rs:128:48
    |
128 |     t!(format!("{:.a$}", "aaaaaaaaaaaaaaaaaa", a = 4), "aaaa");

error: named argument is not used by name
   --> library/alloc/tests/fmt.rs:129:49
    |
    |
129 |     t!(format!("{:._a$}", "aaaaaaaaaaaaaaaaaa", _a = 4), "aaaa");

error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:18:12
