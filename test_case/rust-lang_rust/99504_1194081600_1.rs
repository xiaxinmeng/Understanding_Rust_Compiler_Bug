
    bb1: {
        _19 = move _2;                   // scope 0 at /app/example.rs:4:21: 9:2
        (((*(_1.0: &mut [static generator@/app/example.rs:4:21: 9:2])) as variant#3).0: impl std::future::Future<Output = ()>) = callee() -> [return: bb2, unwind: bb12]; // scope 0 at /app/example.rs:6:19: 6:27
    }

    bb2: {
        _4 = &mut (((*(_1.0: &mut [static generator@/app/example.rs:4:21: 9:2])) as variant#3).0: impl std::future::Future<Output = ()>); // scope 3 at /app/example.rs:7:53: 7:61
        _3 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _4) -> [return: bb3, unwind: bb12]; // scope 3 at /app/example.rs:7:24: 7:62
    }

    bb3: {
        _6 = move _3;                    // scope 2 at /app/example.rs:8:5: 8:8
        _5 = <Pin<&mut impl Future<Output = ()>> as IntoFuture>::into_future(move _6) -> [return: bb4, unwind: bb12]; // scope 2 at /app/example.rs:8:8: 8:14
    }

    bb4: {
        (((*(_1.0: &mut [static generator@/app/example.rs:4:21: 9:2])) as variant#3).1: std::pin::Pin<&mut impl std::future::Future<Output = ()>>) = move _5; // scope 2 at /app/example.rs:8:8: 8:14
        goto -> bb5;                     // scope 4 at /app/example.rs:8:8: 8:14
    }

    bb5: {
        _10 = &mut (((*(_1.0: &mut [static generator@/app/example.rs:4:21: 9:2])) as variant#3).1: std::pin::Pin<&mut impl std::future::Future<Output = ()>>); // scope 5 at /app/example.rs:8:8: 8:14
        _9 = &mut (*_10);                // scope 5 at /app/example.rs:8:8: 8:14
        _8 = Pin::<&mut Pin<&mut impl Future<Output = ()>>>::new_unchecked(move _9) -> [return: bb6, unwind: bb12]; // scope 5 at /app/example.rs:8:8: 8:14
    }

    bb6: {
        _13 = _19;                       // scope 5 at /app/example.rs:8:8: 8:14
        _12 = get_context(move _13) -> [return: bb7, unwind: bb12]; // scope 5 at /app/example.rs:8:5: 8:14
    }

    bb7: {
        _11 = &mut (*_12);               // scope 5 at /app/example.rs:8:5: 8:14
        _7 = <Pin<&mut impl Future<Output = ()>> as Future>::poll(move _8, move _11) -> [return: bb8, unwind: bb12]; // scope 5 at /app/example.rs:8:8: 8:14
    }
