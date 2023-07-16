rust
macro_rules! define_queries {
  (Category1 { $(queries1:tt);* }
   Category2 { $(queries2:tt);* }
   ... ) => {
    define_queries_inner!(
      $(category<Category1> $queries1)*
      $(category<Category2> $queries2)*
      ...
    );
  }
}
