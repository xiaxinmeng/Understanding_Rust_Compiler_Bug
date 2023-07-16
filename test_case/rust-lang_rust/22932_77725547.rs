 rust
assert_eq!((0 as $T).is_power_of_two(), false);
assert_eq!((1 as $T).is_power_of_two(), true);
assert_eq!((2 as $T).is_power_of_two(), true);
assert_eq!((3 as $T).is_power_of_two(), false);
assert_eq!((4 as $T).is_power_of_two(), true);
assert_eq!((5 as $T).is_power_of_two(), false);
assert!(($T::MAX / 2 + 1).is_power_of_two(), true);
