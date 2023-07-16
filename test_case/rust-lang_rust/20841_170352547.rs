 rust
// Initial expression.
bar(Some((|x| x.sin())))
// Add type variables for bar.
bar::<$F>(Some((|x| x.sin())))
// Propagate expected type from signature of bar.
bar::<$F>(Some((|x| x.sin())) expects Option<$F>)
// Add type variables for Some.
bar::<$F>(Some::<$T>((|x| x.sin())))
// Propagate expected type from signature of Some.
bar::<$F>(Some::<$T>((|x| x.sin()) expects $T) expects Option<$F>)
