
error[E0451]: field `start` of struct `std::ops::RangeInclusive` is private
   --> librustc/ty/layout.rs:646:34
    |
646 |             self.valid_range == (0..=1)
    |                                  ^ field `start` is private

error[E0451]: field `end` of struct `std::ops::RangeInclusive` is private
   --> librustc/ty/layout.rs:646:38
    |
646 |             self.valid_range == (0..=1)
    |                                      ^ field `end` is private
