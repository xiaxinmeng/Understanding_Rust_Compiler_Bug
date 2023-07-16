
error[E0658]: use of unstable library feature 'assoc_unix_epoch' (see issue #49502)
   --> src/main.rs:127:38
    |
127 |     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unix time can't be calculated").as_secs() as u32
    |                                      ^^^^^^^^^^^^^^^^^^^^^^
