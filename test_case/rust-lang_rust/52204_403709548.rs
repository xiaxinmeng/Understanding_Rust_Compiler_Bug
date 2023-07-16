
zmd@ReflectiveCoherence:~/Code/Misc$ rustc import_suggestion.rs --edition 2015
error[E0658]: `crate` in paths is experimental (see issue #45477)
 --> import_suggestion.rs:1:5
  |
1 | use crate::inner_mod::MyInnerStruct;
  |     ^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
