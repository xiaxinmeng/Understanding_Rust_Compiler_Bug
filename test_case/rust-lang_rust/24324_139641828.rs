 rust
    #[allow(deprecated)]
    #[stable(feature = "rust1", since = "1.0.0")]
    impl AsRawFd for old_io::fs::File {
        fn as_raw_fd(&self) -> RawFd {
            self.as_inner().fd()
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl AsRawFd for fs::File {
        fn as_raw_fd(&self) -> RawFd {
            self.as_inner().fd().raw()
        }
    }
