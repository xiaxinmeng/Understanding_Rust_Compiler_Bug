
match vec {
    [ref mut a, _] => match vec {
        [_, ref mut b] => { ... }
    }
}
