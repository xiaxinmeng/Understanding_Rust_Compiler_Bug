 rust
#![allow(unused_imports)]

use std::collections::hashmap::{HashSet, SetItems};
use std::hash::Hash;

#[cfg(not(illustrate_poor_error_msg), not(illustrate_fine_error_msg))]
fn iter_to_vec<'a, 'b, X:Clone>(i: SetItems<'b, X>) -> Vec<X> {
    //                  ^~~~~~
    //                  missing clone is the only difference
    let i = i.map(|x| x.clone());
    //                  ^~~~~~
    //                  Here is the call to clone
    let mut i = i;
    i.collect()
}

#[cfg(illustrate_poor_error_msg)]
fn iter_to_vec<'a, 'b, X      >(i: SetItems<'b, X>) -> Vec<X> {
    //                  ^~~~~~
    //                  missing clone is the only difference
    let i = i.map(|x| x.clone());
    //                  ^~~~~~
    //                  Here is the call to clone
    let mut i = i;
    i.collect()
}

#[cfg(illustrate_fine_error_msg)]
fn iter_to_vec<'a, 'b, X      >(i: SetItems<'b, X>) -> Vec<X> {
    //                  ^~~~~~
    //                  missing clone is the only difference
    let i = i.map(|x| x.enolc());
    //                  ^~~~~~
    //                  Here is a call to something other than clone
    let mut i = i;
    i.collect()
}

#[cfg(illustrate_original_poor_error_msg)]
fn set_to_vec<X:Eq+Hash      >(s: &HashSet<X>) -> Vec<X> {
    //                 ^~~~~~
    //                 missing clone is the only difference
    let i = s.iter();
    let mut i = i.map(|x| x.clone());
    i.collect()
}

#[cfg(not(illustrate_original_poor_error_msg))]
fn set_to_vec<X:Eq+Hash+Clone>(s: &HashSet<X>) -> Vec<X> {
    //                 ^~~~~~
    //                 (the aforementioned clone)
    let i = s.iter();
    let mut i = i.map(|x| x.clone());
    i.collect()
}

fn main() {
    let mut s = HashSet::new();
    s.insert(1i);
    s.insert(100);
    s.insert(10211201);
    println!("{}", set_to_vec(&s));
}
