rs
// before adding associated function: 
// visually cluttered with many characters distracting you from
// the main code. The other option is writing your own `is_even`
// for a trivial use case.

assert_eq!(None.filter(|n| n % 2 == 0), None);
assert_eq!(Some(3).filter(|n| n % 2 == 0), None);
assert_eq!(Some(4).filter(|n| n % 2 == 0), Some(4));

// after adding associated function:
// intent is clear without need for creating your own `is_even`
// for a common use case.

assert_eq!(None.filter(i32::is_even), None);
assert_eq!(Some(3).filter(i32::is_even), None);
assert_eq!(Some(4).filter(i32::is_even), Some(4));
