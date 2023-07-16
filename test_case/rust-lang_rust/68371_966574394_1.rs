rust
let a = [1, 2, 3];
assert_eq!(a.iter().state_before(None,    |state, &item| state * item).eq(&[(1, 1), (2, 2), (6, 3)]));
assert_eq!(a.iter().state_before(Some(4), |state, &item| state * item).eq(&[(4, 1), (8, 2), (24, 3)]));
assert_eq!(a.iter().state_after(None,     |state, &item| state * item).eq(&[(1, 1), (1, 2), (2, 3)]));
assert_eq!(a.iter().state_after(Some(4),  |state, &item| state * item).eq(&[(4, 1), (4, 2), (8, 3)]));
