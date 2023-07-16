rust
vec.into_iter().map(Foo::something_returning_result).collect::<Result<Vec<_>, _>>()
