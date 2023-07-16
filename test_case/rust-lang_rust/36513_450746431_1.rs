
error[E0599]: no method named `to_string` found for type `rocket_contrib::json::Json<MTT2Game>` in the current scope
  --> src/main.rs:43:26
   |
43 |     println!( "{}", game.to_string() );
   |                          ^^^^^^^^^
   |
   = note: the method `to_string` exists but the following trait bounds were not satisfied:
           `MTT2Game : std::string::ToString`
           `rocket_contrib::json::Json<MTT2Game> : std::string::ToString`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `to_string`, perhaps you need to implement it:
           candidate #1: `std::string::ToString`
