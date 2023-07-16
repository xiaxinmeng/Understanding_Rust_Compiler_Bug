
enum expr {
        ex_tup(...),
        ex_rec(...),
        ex_path(...),
        ex_index(...),
        ex_tuple_lvalue(...),
}
enum tuple_like : expr { ex_tup, ex_rec, ex_tuple_lvalue }
enum lvalue : expr { ex_path, ex_index, ex_tuple_lvalue }
