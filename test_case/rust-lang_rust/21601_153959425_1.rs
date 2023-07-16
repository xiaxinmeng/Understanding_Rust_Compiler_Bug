 rust
➜  libB  rustc lib.rs --extern attribute=../libA/libattribute.so
error: cannot satisfy dependencies so `std` only shows up once
help: having upstream crates all available in one format will likely make this go away
error: cannot satisfy dependencies so `core` only shows up once
help: having upstream crates all available in one format will likely make this go away
error: cannot satisfy dependencies so `collections` only shows up once
help: having upstream crates all available in one format will likely make this go away
error: cannot satisfy dependencies so `rustc_unicode` only shows up once
help: having upstream crates all available in one format will likely make this go away
error: cannot satisfy dependencies so `alloc` only shows up once
help: having upstream crates all available in one format will likely make this go away
error: cannot satisfy dependencies so `rand` only shows up once
help: having upstream crates all available in one format will likely make this go away
error: cannot satisfy dependencies so `libc` only shows up once
help: having upstream crates all available in one format will likely make this go away
error: cannot link together two allocators: alloc_jemalloc and alloc_system
error: aborting due to 8 previous errors
