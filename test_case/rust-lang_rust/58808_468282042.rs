
$ MIRI_SYSROOT=/home/bjorn/.cache/miri/HOST cargo run -- core symbols |& head
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rlib_view core symbols`
Reading rlib core
Rust args: ["core", "/some_nonexistent_dummy_path", "--sysroot", "/home/bjorn/.cache/miri/HOST", "-Cpanic=abort"]
error: `#[panic_handler]` function required, but not found


Exported symbols:
    Rust <core::char::ParseCharError as core::fmt::Debug>::fmt (DefId(1/0:20381 ~ core[eddd]::char[0]::convert[0]::{{impl}}[9]::fmt[0]))
    Rust core::fmt::Formatter::alternate (DefId(1/0:4257 ~ core[eddd]::fmt[0]::{{impl}}[6]::alternate[0]))
[1]    22404 illegal hardware instruction  MIRI_SYSROOT=/home/bjorn/.cache/miri/HOST cargo run -- core symbols 2>&1 | 
       22405 done                          head
