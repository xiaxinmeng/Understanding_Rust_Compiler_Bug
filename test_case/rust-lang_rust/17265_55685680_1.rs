
            fn fail(output: &mut file::FileDesc) -> ! {
                let errno = os::errno();
                let bytes = [
                    (errno << 24) as u8,
                    (errno << 16) as u8,
                    (errno <<  8) as u8,
                    (errno <<  0) as u8,
                ];
                assert!(output.inner_write(bytes).is_ok());
                unsafe { libc::_exit(1) }
            }
