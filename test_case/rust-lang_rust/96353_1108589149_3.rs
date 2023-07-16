
help: consider importing this function...
   |
   |
LL | use hi_str;
LL | use hi_str;
   |
   |
help: ...and refer to it directly
   |
LL -     println!("{}", circular_modules_main::hi_str());
LL +     println!("{}", hi_str());
   | 
