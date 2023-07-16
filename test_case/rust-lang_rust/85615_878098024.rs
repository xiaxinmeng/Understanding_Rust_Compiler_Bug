
warning: unresolved link to `Context::value`
  --> core/lib/src/form/context.rs:26:10
   |
26 | /// via [`Context::value()`] or [`Context::values()`]. Data fields do not have
   |          ^^^^^^^^^^^^^^^^^^ the struct `Context` has no function named `value`

warning: incompatible link kind for `Context::values`
  --> core/lib/src/form/context.rs:26:34
   |
26 | /// via [`Context::value()`] or [`Context::values()`]. Data fields do not have
   |                                  ^^^^^^^^^^^^^^^^^^^
   |
   = note: this link resolved to a field, which is not a function
