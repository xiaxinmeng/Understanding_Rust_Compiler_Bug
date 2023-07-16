compile_fail,E0409
let x = (0, 2);
match x {
    (0, ref y) | (y, 0) ={ /* use y */} // error: variable `y` is bound with
                                          //        different mode in pattern #2
                                          //        than in pattern #1
    _ =()
}
