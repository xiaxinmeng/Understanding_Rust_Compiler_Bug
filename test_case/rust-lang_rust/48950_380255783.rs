rust
        match panic::catch_unwind(|| { TargetSharedLibrary::each(|_| panic!("uh oh")); }) {
            Ok(()) => panic!("Expected a panic, but didn't get one"),
            Err(any) => {
                assert!(any.is::<&'static str>(), "panic value should be a &'static str");
                assert_eq!(*any.downcast_ref::<&'static str>().unwrap(), "uh oh");
            }
        }
