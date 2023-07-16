rs
#![feature(const_btree_new)]

use std::collections::BTreeMap;

const fn f() -> BTreeMap<(), ()> {
    let x = BTreeMap::new();
    let l = x.len();
    x
}
