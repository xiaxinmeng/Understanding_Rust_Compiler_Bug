
    bb1: {
        _17 = move _2;                   // scope 0 at /app/example.rs:4:21: 9:2
        _4 = callee() -> [return: bb2, unwind: bb11]; // scope 0 at /app/example.rs:8:5: 8:13
    }

    bb2: {
        _3 = <impl Future<Output = ()> as IntoFuture>::into_future(move _4) -> [return: bb3, unwind: bb11]; // scope 0 at /app/example.rs:8:13: 8:19
    }

    bb3: {
        (((*(_1.0: &mut [static generator@/app/example.rs:4:21: 9:2])) as variant#3).0: impl std::future::Future<Output = ()>) = move _3; // scope 0 at /app/example.rs:8:13: 8:19
        goto -> bb4;                     // scope 1 at /app/example.rs:8:13: 8:19
    }

    bb4: {
        _8 = &mut (((*(_1.0: &mut [static generator@/app/example.rs:4:21: 9:2])) as variant#3).0: impl std::future::Future<Output = ()>); // scope 2 at /app/example.rs:8:13: 8:19
        _7 = &mut (*_8);                 // scope 2 at /app/example.rs:8:13: 8:19
        _6 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _7) -> [return: bb5, unwind: bb11]; // scope 2 at /app/example.rs:8:13: 8:19
    }

    bb5: {
        _11 = _17;                       // scope 2 at /app/example.rs:8:13: 8:19
        _10 = get_context(move _11) -> [return: bb6, unwind: bb11]; // scope 2 at /app/example.rs:8:5: 8:19
    }

    bb6: {
        _9 = &mut (*_10);                // scope 2 at /app/example.rs:8:5: 8:19
        _5 = <impl Future<Output = ()> as Future>::poll(move _6, move _9) -> [return: bb7, unwind: bb11]; // scope 2 at /app/example.rs:8:13: 8:19
    }
