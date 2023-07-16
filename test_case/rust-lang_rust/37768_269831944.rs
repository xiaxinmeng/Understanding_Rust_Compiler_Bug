
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
   --> src/actions/mod.rs:195:30
    |
195 |         let v: Vec<&str> = s.trim()
    |                              ^^^^
    |
help: consider using an explicit lifetime parameter as shown: fn from_str(s: &'a str) -> Result<Self, <Self>::Err>
   --> src/actions/mod.rs:193:5
    |
193 |     fn from_str(s: &str) -> Result<Self, Self::Err> {
    |     ^
