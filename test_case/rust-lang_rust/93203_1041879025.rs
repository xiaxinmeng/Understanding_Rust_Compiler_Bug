text
    thread::scope(…)
    //             |
    //             | 'env cannot end before this call, and has no reason to end later.


    thread::scope(|scope| {
        …
    }) ---------
    // 'env cannot end before this call, and has no reason to end later.
    