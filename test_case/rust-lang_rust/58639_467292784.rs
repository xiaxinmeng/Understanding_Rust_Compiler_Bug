diff
-    let poll_next = |waker: &_| Pin::new(&mut *stream).poll_next(waker);
-    let future_next = poll_fn(poll_next);
+    let future_next = poll_fn(|waker| Pin::new(&mut *stream).poll_next(waker));
