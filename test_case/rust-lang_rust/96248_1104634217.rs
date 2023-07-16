diff
- format!("{}{}", snippet, ".collect::<Vec<_>>()"),
+ format!("{}.collect::<Vec<_>>()", snippet),
