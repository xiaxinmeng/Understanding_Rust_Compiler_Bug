
error: lifetime may not live long enough
  --> src/lib/grid.rs:70:56
   |
67 |       pub async fn add_empty<'t>(
   |                              -- lifetime `'t` defined here
68 |           target: &Target,
   |                         - let's call the lifetime of this reference `'1`
69 |           mut tx: sqlx::Transaction<'t, Postgres>,
70 |       ) -> sqlx::Result<sqlx::Transaction<'t, Postgres>> {
   |  ________________________________________________________^
71 | |         let grid_id = sqlx::query_as!(
72 | |             Grid,
73 | |             r#"
...  |
97 | |         Ok(tx)
98 | |     }
   | |_____^ associated function was supposed to return data with lifetime `'t` but it is returning data with lifetime `'1`

error: lifetime may not live long enough
  --> src/lib/grid.rs:97:9
   |
67 |     pub async fn add_empty<'t>(
   |                            -- lifetime `'t` defined here
68 |         target: &Target,
   |                       - let's call the lifetime of this reference `'1`
...
97 |         Ok(tx)
   |         ^^^^^^ associated function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'t`
