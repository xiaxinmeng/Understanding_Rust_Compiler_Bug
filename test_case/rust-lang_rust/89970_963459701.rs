
error: Missing required bounds on Keys
  --> C:\Users\Mingw\.cargo\git\checkouts\spinach-4a04cccc93d2e458\622fdb8\lib\src\collections.rs:20:5
   |
20 |     type Keys<'s>: Iterator<Item = &'s K> where K: 's;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
   |                                                      |
   |                                                      help: add the required where clauses: `, Self: 's`
