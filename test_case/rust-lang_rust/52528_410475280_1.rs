
= note: ld: warning: directory not found for option '-L~/GitHub/rust/build/x86_64-apple-darwin/test/run-pass/panic-runtime/abort/auxiliary'
          Undefined symbols for architecture x86_64:
            "_$LT$core..ptr..Unique$LT$T$GT$$GT$::as_ptr::h40c70ffcb9fa71ff", referenced from:
                core::ptr::drop_in_place::h13ea34cb88507684 in libstd-1b5266ff4a5237d6.rlib(std-1b5266ff4a5237d6.2ujrhqd1it5k4nvl.rcgu.o)
                alloc::alloc::box_free::hf8887d7620bb9fc3 in libstd-1b5266ff4a5237d6.rlib(std-1b5266ff4a5237d6.2ujrhqd1it5k4nvl.rcgu.o)
                std::rt::lang_start_internal::h298a301fc3e8b601 in libstd-1b5266ff4a5237d6.rlib(std-1b5266ff4a5237d6.2ujrhqd1it5k4nvl.rcgu.o)
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
