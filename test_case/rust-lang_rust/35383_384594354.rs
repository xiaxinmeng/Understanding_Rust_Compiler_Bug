
error[E0214]: parenthesized parameters may only be used with a trait
  --> src/main.rs:13:23
   |
13 |     prev_value: Option(String),
   |                       ^^^^^^^^ only traits may use parentheses
   |                        - help: did you mean to use `Option<String>` here instead?
