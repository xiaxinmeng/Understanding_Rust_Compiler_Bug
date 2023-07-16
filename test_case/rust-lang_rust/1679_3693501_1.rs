
enum tuple_like {
    tup()
    rec()
}

enum lvalue {
    path()
    index()
}

enum expr < tuple_like, lvalue {
    some_other_case()
}
