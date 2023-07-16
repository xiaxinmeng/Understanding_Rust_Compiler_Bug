rust
[01:05:00] error[E0532]: expected unit struct/variant or constant, found tuple variant `RangeEnd::Included`
[01:05:00]   --> src\tools\rustfmt\src\patterns.rs:62:17
[01:05:00]    |
[01:05:00] 62 |                 RangeEnd::Included => rewrite_pair(&**lhs, &**rhs, "", "...", "", context, shape),
[01:05:00]    |                 ^^^^^^^^^^--------
[01:05:00]    |                           |
[01:05:00]    |                           did you mean `Excluded`?
