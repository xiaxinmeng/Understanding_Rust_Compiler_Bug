
DEBUG:rustc::middle::dependency_format: adding dylib: sync
DEBUG:rustc::middle::dependency_format: adding RequireDynamic: rustrt
DEBUG:rustc::middle::dependency_format: adding dylib: rand
DEBUG:rustc::middle::dependency_format: adding dylib: core
DEBUG:rustc::middle::dependency_format: adding dylib: libc
DEBUG:rustc::middle::dependency_format: adding dylib: rustrt
DEBUG:rustc::middle::dependency_format: adding RequireStatic: core
error: cannot satisfy dependencies so `core` only shows up once
note: having upstream crates all available in one format will likely make this go away
DEBUG:rustc::middle::dependency_format: adding RequireStatic: alloc
DEBUG:rustc::middle::dependency_format: adding RequireStatic: libc
error: cannot satisfy dependencies so `libc` only shows up once
note: having upstream crates all available in one format will likely make this go away
DEBUG:rustc::middle::dependency_format: adding RequireStatic: collections
DEBUG:rustc::middle::dependency_format: adding RequireStatic: unicode
error: aborting due to 2 previous errors
