
error[E0277]: a value of type `&std::collections::HashSet<char>` cannot be built from an iterator over elements of type `char`
   --> src/bin/day6.rs:153:82
    |
153 |         .fold_first(|accumulator, next| accumulator.intersection(&next).copied().collect())
    |                                                                                  ^^^^^^^ value of type `&std::collections::HashSet<char>` cannot be built from `std::iter::Iterator<Item=char>`
    |
    = help: the trait `FromIterator<char>` is not implemented for `&std::collections::HashSet<char>`
