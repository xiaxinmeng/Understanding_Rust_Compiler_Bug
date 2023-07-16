
hichaelmart Hi all, a question about lifetime specifiers – can someone explain why the compiler doesn't just automatically create these in the simple cases? ie: fn my_fn<T>(items: T) -> String where T: Iterator<&str>
eddyb hichaelmart: long standing issue
eddyb it's something that should be done IMO
