
match ch {
    => Matched(next , ( )) ,
    _ => __state.mark_failure ( __pos , "[]" ),
}
