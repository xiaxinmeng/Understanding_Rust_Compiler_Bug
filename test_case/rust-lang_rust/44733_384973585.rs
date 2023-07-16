
error[E0658]: use of unstable library feature 'entry_and_modify' (see issue #44733)
   --> src/models/mod.rs:155:34
    |
155 |                                 .and_modify(|&mut tup| {tup.1 = idx}) ;
    |                                  ^^^^^^^^^^

