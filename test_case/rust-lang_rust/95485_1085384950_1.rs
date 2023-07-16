rust
macro_rules! checked_sum {
    ( $first:expr $(, $rest:expr )* $(,)? ) => {
        [$($rest),*].into_iter().try_fold($first, |a, b| a.checked_add(b))
    }
}
