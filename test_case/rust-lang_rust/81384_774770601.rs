rust
Ordering::then_with(
  Option::unwrap_or(
    PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
    Ordering::Greater
  ),
  || Ordering::then_with(
    Option::unwrap_or(
      PartialOrd::partial_cmp(&(*__self_0_1), &(*__self_1_1)),
      Ordering::Greater
    ),
    || Option::unwrap_or(
        PartialOrd::partial_cmp(&(*__self_0_2), &(*__self_1_2)),
        Ordering::Greater
    )
  )
) == Ordering::Less,
