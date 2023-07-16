rust
(($expr:expr) ($dot:tt $literal:tt $($rest:tt)*) (. $unusable_literal:literal $($rest:tt)*)) => {
    $expr $dot $literal
};
