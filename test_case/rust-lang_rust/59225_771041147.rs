rust
error[E0223]: ambiguous associated type
  --> src/main.rs:19:45
   |
19 |     let query = diesel::update(posts.filter(posts::columns::id.eq_any(subquery)))
   |                                             ^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<hello::schema::posts::table as Trait>::columns`
