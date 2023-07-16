
executor::block_on(async {
            let read_line = |_context| -> Poll<String> {
                Poll::Ready("Hello, World!".into())
            };

            let read_future = futures::future::poll_fn(read_line);
            assert_eq!(await!(read_future), "Hello, World!".to_owned());
        });
