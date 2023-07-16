
error[E0609]: no field `items` on type `B`
  --> src/main.rs:17:14
   |
17 |     B::new().items.contains("");
   |              ^^^^^ unknown field
   |
   = note: available fields are: `a`, `map`
help: one of the expressions' fields has a field of the same name
   |
17 |     B::new().a.items.contains("");
   |              ++
help: one of the expressions' fields has a field of the same name
   |
17 |     B::new().map.base.table.table.items.contains("");
   |              +++++++++++++++++++++
