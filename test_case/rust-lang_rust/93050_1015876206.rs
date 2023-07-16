rust
let deadline_passed = optional_deadline.is_some_with(|d| d < now);

let deadline_passed = matches!(optional_deadline, Some(d) if d < now);

let deadline_passed = if let Some(d) = optional_deadline && d < now { true } else { false };
