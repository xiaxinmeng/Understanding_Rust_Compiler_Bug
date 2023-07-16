rust
my_long_method_chain
   // ... many methods later
   .tap(|my_option| if my_option.is_none() { print_my_warning() })
  // continuing my method chain...
