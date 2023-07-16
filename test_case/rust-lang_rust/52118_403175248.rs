
error[E0599]: no variant named `Init` found for type `State` in the current scope
  --> src/main.rs:11:13
   |
1  | pub enum State {
   | -------------- variant `Init` not found here
...
11 |             Self::Init => 1,
   |             ^^^^^^^^^^ variant not found in `State`
   |
   = note: did you mean `State::Init`?
