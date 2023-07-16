rs
        // without `self`-based APIs, this would lead to a "`.unfilled()`-temporary dropped" issue
        let buf = borrow_buf.unfilled().ensure_init().init_mut(); // this ought to be typical
        