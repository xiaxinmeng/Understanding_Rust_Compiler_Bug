
error[E0596]: cannot borrow `*card` as mutable, as it is behind a `&` reference
 --> src/lib.rs:6:38
  |
5 |         for card in &bingo_game.cards {
  |                     ----------------- this iterator yields `&` references
6 |             if card.still_playing && card.mark_number(number) {
  |                                      ^^^^^^^^^^^^^^^^^^^^^^^^ `card` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
