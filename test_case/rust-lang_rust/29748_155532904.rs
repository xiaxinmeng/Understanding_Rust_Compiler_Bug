
match x {
    pat1(a) | pat2(a) | pat3(a) => {}
         ^ the first `a` is here, `a`s have to be compared hygienically
}
