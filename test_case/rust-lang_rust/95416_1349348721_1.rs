rs
error[E0515]: cannot return value referencing temporary value
   --> src/main.rs:338:9
    |
337 |         let page = BUFFER_POOL.lock().unwrap().get_page(page_loc);
    |                    --------------------------- temporary value created here
338 |         page
    |         ^^^^ returns a value referencing data owned by the current function
