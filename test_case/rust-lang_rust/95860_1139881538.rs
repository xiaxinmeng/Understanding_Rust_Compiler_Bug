rust
macro_rules! m {
    { $($($name:tt)+)? } => {
        $(
            ${ignore($($name)+)}
            // whatever tokens which
            // should be emitted once
        )?
    }
}
