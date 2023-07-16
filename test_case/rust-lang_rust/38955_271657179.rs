
error[E0308]: mismatched types
  --> /checkout/src/librustc_driver/test.rs:82:30
   |
82 |         remove_message(self, &db.message, db.level);
   |                              ^^^^^^^^^^^ expected str, found struct `std::vec::Vec`
   |
   = note: expected type `&str`
   = note:    found type `&std::vec::Vec<(std::string::String, errors::snippet::Style)>`
   = help: here are some functions which might fulfill your needs:
 - .remove(...)
 - .swap_remove(...)

error[E0308]: mismatched types
  --> /checkout/src/librustc_driver/test.rs:84:34
   |
84 |             remove_message(self, &child.message, child.level);
   |                                  ^^^^^^^^^^^^^^ expected str, found struct `std::vec::Vec`
   |
   = note: expected type `&str`
   = note:    found type `&std::vec::Vec<(std::string::String, errors::snippet::Style)>`
   = help: here are some functions which might fulfill your needs:
 - .remove(...)
 - .swap_remove(...)
